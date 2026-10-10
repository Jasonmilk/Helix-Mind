//! Alarm registry — P10d (ADR-0032): ana_wakeup wake-up channel.
//!
//! Mind never self-wakes (no heartbeat rule). Alarms are L2 nodes with
//! provenance `alarm#{job_id}` (deterministic name-based id, idempotent
//! upsert) and a small state machine in `notes`:
//!
//! ```text
//! pending ──(ana_wakeup, due)──► claimed ──(ack: done)──► done
//!                                      └──(ack: renewed)──► pending (due += repeat)
//! ```
//!
//! Due semantics (D2): `punctual` alarms are due at `due_at`; `jittered`
//! alarms (peak-congestion guard) are due once the window
//! `[due_at - jitter_minutes, due_at + jitter_minutes]` has opened — elastic
//! due, no randomness, no timer. The caller (Anaphase) passes the window
//! width from its own config (default 60); time judgement stays in Mind.

use std::collections::HashMap;
use std::sync::Arc;

use helix_mind_core::error::MindError;
use helix_mind_core::graph::{Node, NodeContent, NodeType};
use helix_mind_storage::{StorageEngine, WritePriority};

/// Status values kept in node notes.
const ST_PENDING: &str = "pending";
const ST_CLAIMED: &str = "claimed";
const ST_DONE: &str = "done";

/// Content field keys.
const K_DUE_AT: &str = "due_at";
const K_MODE: &str = "mode";
const K_ACTION: &str = "action";
const K_REPEAT_MINUTES: &str = "repeat_minutes";

/// One due alarm handed to the waker.
#[derive(Debug, Clone, PartialEq)]
pub struct AlarmDue {
    pub job_id: String,
    pub action: String,
    pub due_at: String,
    pub mode: String,
    pub claim_id: String,
}

fn field(map: &HashMap<String, String>, key: &str) -> Option<String> {
    map.get(key).cloned()
}

fn status(node: &Node) -> String {
    node.notes.clone().unwrap_or_default()
}

fn is_alarm(node: &Node) -> bool {
    node.abstract_provenance
        .as_deref()
        .is_some_and(|p| p.starts_with("alarm#"))
}

/// Claim id — deterministic from the alarm provenance (no extra state):
/// `claim#{job_id}`. Ack reverses it back to the provenance.
fn claim_id_for(job_id: &str) -> String {
    format!("claim#{job_id}")
}

fn job_id_from_claim(claim_id: &str) -> Option<String> {
    claim_id.strip_prefix("claim#").map(|s| s.to_string())
}

/// Parse an ISO timestamp; invalid → None (honest skip, never panic).
fn parse_iso(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&chrono::Utc))
}

/// List due alarms and atomically claim them (pending → claimed).
/// `jitter_minutes = 0` disables the elastic window (punctual semantics for
/// every alarm); otherwise jittered alarms are due once their window opens.
pub async fn list_due_alarms(
    storage: &Arc<StorageEngine>,
    jitter_minutes: u32,
) -> Result<Vec<AlarmDue>, MindError> {
    let now = chrono::Utc::now();
    let jitter = chrono::Duration::minutes(i64::from(jitter_minutes));
    let l2 = storage.get_nodes_by_type(NodeType::L2).await?;

    let mut due: Vec<AlarmDue> = Vec::new();
    for node in &l2 {
        if !is_alarm(node) || status(node) != ST_PENDING {
            continue;
        }
        let Some(content) = (match &node.content {
            NodeContent::Structured(m) => Some(m),
            _ => None,
        }) else {
            continue;
        };
        let Some(due_at) = field(content, K_DUE_AT).and_then(|s| parse_iso(&s)) else {
            continue;
        };
        let mode = field(content, K_MODE).unwrap_or_else(|| "punctual".to_string());
        // D2: jittered → window has opened (due_at - m <= now); punctual →
        // strictly due (due_at <= now). Late alarms are still returned —
        // honest, no missed-state machine (avoid over-engineering).
        let opened = if mode == "jittered" && jitter_minutes > 0 {
            now >= due_at - jitter
        } else {
            now >= due_at
        };
        if !opened {
            continue;
        }

        let job_id = node
            .abstract_provenance
            .as_deref()
            .unwrap_or_default()
            .strip_prefix("alarm#")
            .unwrap_or_default()
            .to_string();
        let action = field(content, K_ACTION).unwrap_or_default();

        // Atomic claim: update the node in place (pending → claimed).
        let mut claimed = node.clone();
        claimed.notes = Some(ST_CLAIMED.to_string());
        storage.write_node(claimed, WritePriority::Deferred).await?;

        due.push(AlarmDue {
            job_id: job_id.clone(),
            action,
            due_at: field(content, K_DUE_AT).unwrap_or_default(),
            mode,
            claim_id: claim_id_for(&job_id),
        });
    }
    Ok(due)
}

/// Acknowledge a claim: `done` closes the alarm; `renewed` rolls a repeating
/// alarm forward (due_at += repeat_minutes, back to pending). Idempotent —
/// acking an already-closed claim succeeds without side effects.
pub async fn ack_alarm(
    storage: &Arc<StorageEngine>,
    claim_id: &str,
    ack_status: &str,
) -> Result<bool, MindError> {
    let Some(job_id) = job_id_from_claim(claim_id) else {
        return Ok(false);
    };
    let provenance = format!("alarm#{job_id}");
    let l2 = storage.get_nodes_by_type(NodeType::L2).await?;
    let Some(node) = l2
        .iter()
        .find(|n| n.abstract_provenance.as_deref() == Some(provenance.as_str()))
    else {
        return Ok(false);
    };

    let current = status(node);
    if current == ST_DONE && ack_status == ST_DONE {
        return Ok(true); // already closed — idempotent
    }

    let mut updated = node.clone();
    match ack_status {
        "done" => {
            updated.notes = Some(ST_DONE.to_string());
        }
        "renewed" => {
            // D3: renewal rolls from the ORIGINAL due_at (+ repeat), never
            // from now — no drift accumulation on slow wakers.
            if let Some(mut content) = match &updated.content {
                NodeContent::Structured(m) => Some(m.clone()),
                _ => None,
            } {
                let repeat = content
                    .get(K_REPEAT_MINUTES)
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap_or(0);
                if repeat > 0 {
                    let next = parse_iso(&content[&K_DUE_AT.to_string()])
                        .map(|d| d + chrono::Duration::minutes(repeat))
                        .map(|d| d.to_rfc3339());
                    if let Some(next) = next {
                        content.insert(K_DUE_AT.to_string(), next);
                        updated.content = NodeContent::Structured(content);
                    }
                }
                updated.notes = Some(ST_PENDING.to_string());
            }
        }
        _ => return Ok(false),
    }
    storage.write_node(updated, WritePriority::Deferred).await?;
    Ok(true)
}
