//! FTS5 trigram index — a derived *projection* of the `nodes` table (P1 M-01,
//! ADR-0013).
//!
//! Rulings (2026-08-28):
//! - The index is rebuilt from the truth source (`nodes`) on startup, then
//!   maintained incrementally by an async write queue with batch coalescing.
//!   No FTS write ever blocks the node-write transaction.
//! - Search SQL lives here (with the schema); the retrieval `FtsExtractor` is a
//!   thin read-only consumer that owns input sanitization / escaping / audit.

use crate::codec::phase_state_str;
use crate::sqlite_pool::SqlitePool;
use helix_mind_core::error::MindError;
use helix_mind_core::graph::{NodeContent, PhaseState};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::oneshot;
use uuid::Uuid;

/// Flush once this many ops accumulate (ruling: batch coalescing).
pub const FTS_BATCH_SIZE: usize = 100;
/// Flush if ops are pending and this much time elapses (debounce).
pub const FTS_DEBOUNCE_MS: u64 = 100;

/// Index-maintenance command sent over the async write queue.
/// (No `Clone` — `Flush` carries a `oneshot::Sender` which is move-only.)
#[derive(Debug)]
pub enum FtsCommand {
    Upsert {
        node_id: Uuid,
        phase_state: PhaseState,
        content: String,
    },
    Delete {
        node_id: Uuid,
    },
    /// Barrier: flush all prior ops, then signal completion on the oneshot.
    Flush(oneshot::Sender<()>),
}

/// A search hit: node id plus its (pre-weighting) score.
#[derive(Debug, Clone)]
pub struct FtsHit {
    pub node_id: Uuid,
    pub score: f64,
}

/// Idempotent FTS5 virtual-table creation (trigram tokenizer).
pub fn create_fts_table(conn: &rusqlite::Connection) -> Result<(), MindError> {
    conn.execute_batch(
        "CREATE VIRTUAL TABLE IF NOT EXISTS nodes_fts USING fts5(
            content,
            node_id UNINDEXED,
            phase_state UNINDEXED,
            tokenize='trigram'
        );",
    )
    .map_err(|e| MindError::Storage(e.to_string()))
}

/// Rebuild the index from the truth source (`nodes`). The index is a
/// projection, so rebuilding on startup guarantees it cannot diverge from the
/// source. Incremental maintenance takes over once the worker is running.
pub fn rebuild_fts(pool: &SqlitePool) -> Result<(), MindError> {
    let conn = pool.get()?;
    conn.execute_batch(
        "DELETE FROM nodes_fts;
         INSERT INTO nodes_fts(node_id, phase_state, content)
            SELECT id, phase_state, content FROM nodes;",
    )
    .map_err(|e| MindError::Storage(e.to_string()))
}

/// Flush a batch of index ops in a single transaction.
pub fn flush_batch(pool: &SqlitePool, batch: &[FtsCommand]) -> Result<(), MindError> {
    pool.transactional_write(|tx| {
        for cmd in batch {
            match cmd {
                FtsCommand::Upsert {
                    node_id,
                    phase_state,
                    content,
                } => {
                    // FTS5 has no UPSERT on a UNINDEXED column; delete+insert is
                    // the idempotent form (bounded by the batch transaction).
                    tx.execute(
                        "DELETE FROM nodes_fts WHERE node_id = ?1",
                        rusqlite::params![node_id.to_string()],
                    )
                    .map_err(|e| MindError::Storage(e.to_string()))?;
                    tx.execute(
                        "INSERT INTO nodes_fts(node_id, phase_state, content) VALUES (?1,?2,?3)",
                        rusqlite::params![
                            node_id.to_string(),
                            phase_state_str(phase_state),
                            content,
                        ],
                    )
                    .map_err(|e| MindError::Storage(e.to_string()))?;
                }
                FtsCommand::Delete { node_id } => {
                    tx.execute(
                        "DELETE FROM nodes_fts WHERE node_id = ?1",
                        rusqlite::params![node_id.to_string()],
                    )
                    .map_err(|e| MindError::Storage(e.to_string()))?;
                }
                FtsCommand::Flush(_) => {}
            }
        }
        Ok(())
    })
}

/// Background worker: coalesces queued ops, flushes on batch-size, debounce, or
/// a `Flush` barrier (which also acks the caller for deterministic tests).
pub async fn run_fts_worker(pool: SqlitePool, mut rx: UnboundedReceiver<FtsCommand>) {
    let mut batch: Vec<FtsCommand> = Vec::new();
    let mut debounce = tokio::time::interval(Duration::from_millis(FTS_DEBOUNCE_MS));
    debounce.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            cmd = rx.recv() => match cmd {
                Some(cmd) => {
                    let is_barrier = matches!(cmd, FtsCommand::Flush(_));
                    batch.push(cmd);
                    while let Ok(c) = rx.try_recv() {
                        batch.push(c);
                    }
                    if is_barrier || batch.len() >= FTS_BATCH_SIZE {
                        if let Err(e) = flush_batch(&pool, &batch) {
                            eprintln!("[fts] flush error: {e}");
                        }
                        // Move the batch out (sender is not Clone) and ack barriers.
                        for ack in drain_acks(std::mem::take(&mut batch)) {
                            let _ = ack.send(());
                        }
                    }
                }
                None => {
                    if let Err(e) = flush_batch(&pool, &batch) {
                        eprintln!("[fts] final flush error: {e}");
                    }
                    for ack in drain_acks(std::mem::take(&mut batch)) {
                        let _ = ack.send(());
                    }
                    break;
                }
            },
            _ = debounce.tick(), if !batch.is_empty() => {
                if let Err(e) = flush_batch(&pool, &batch) {
                    eprintln!("[fts] debounce flush error: {e}");
                }
                for ack in drain_acks(std::mem::take(&mut batch)) {
                    let _ = ack.send(());
                }
            }
        }
    }
}

/// Move the batch and pull out the `Flush` barrier senders (ack them once the
/// preceding flush has completed).
fn drain_acks(batch: Vec<FtsCommand>) -> Vec<oneshot::Sender<()>> {
    batch
        .into_iter()
        .filter_map(|c| match c {
            FtsCommand::Flush(tx) => Some(tx),
            _ => None,
        })
        .collect()
}

/// Map a node's content into the searchable text stored in the index.
pub fn node_search_text(content: &NodeContent) -> String {
    match content {
        NodeContent::Text(s) => s.clone(),
        NodeContent::Structured(m) => m.values().cloned().collect::<Vec<_>>().join(" "),
        NodeContent::Reference(s) => s.clone(),
        NodeContent::Event { event_type, payload } => {
            let mut parts = vec![event_type.clone()];
            parts.extend(payload.values().cloned());
            parts.join(" ")
        }
        NodeContent::GeneLock {
            lineage_name,
            core_principles,
            ..
        } => {
            let mut parts = vec![lineage_name.clone()];
            parts.extend(core_principles.iter().cloned());
            parts.join(" ")
        }
    }
}

/// Record sanitized-away input to the audit log (ruling 2026-08-28: Mind owns
/// "query content is harmless"). The audit_log table lives with this crate.
pub fn audit_sanitized(pool: &SqlitePool, original: &str, removed: &str) {
    if removed.is_empty() {
        return;
    }
    let Ok(conn) = pool.get() else {
        return;
    };
    let details =
        serde_json::json!({ "original": original, "removed_chars": removed }).to_string();
    let _ = conn.execute(
        "INSERT INTO audit_log (event_id, timestamp, event_type, actor, details)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            uuid::Uuid::new_v4().to_string(),
            chrono::Utc::now().to_rfc3339(),
            "fts_input_sanitized",
            "retrieval",
            details,
        ],
    );
}

/// FTS5 MATCH search with bm25 relevance and phase weighting
/// (Crystal > Liquid > Gas, ruling 2026-08-28).
///
/// Note: FTS5 `bm25()` returns a *negative* score (more negative = better), so
/// we negate it before applying the positive phase multiplier, otherwise a
/// Crystal node would be pushed *down* by its higher weight.
pub fn fts_search(
    pool: &SqlitePool,
    escaped_match: &str,
    limit: usize,
) -> Result<Vec<FtsHit>, MindError> {
    let conn = pool.get()?;
    let mut stmt = conn
        .prepare(
            "SELECT node_id, bm25(nodes_fts)
             FROM nodes_fts
             WHERE nodes_fts MATCH ?1
             ORDER BY (-bm25(nodes_fts)) * CASE phase_state
                 WHEN 'Crystal' THEN 1.5 WHEN 'Liquid' THEN 1.0 ELSE 0.5 END DESC
             LIMIT ?2",
        )
        .map_err(|e| MindError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map(
            rusqlite::params![escaped_match, limit as i64],
            |row| {
                Ok(FtsHit {
                    node_id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
                    score: row.get(1)?,
                })
            },
        )
        .map_err(|e| MindError::Storage(e.to_string()))?;
    let mut hits = Vec::new();
    for r in rows {
        hits.push(r.map_err(|e| MindError::Storage(e.to_string()))?);
    }
    Ok(hits)
}

/// 1-2 char fallback: substring `LIKE` on the nodes table, phase-weighted order.
pub fn fts_like_search(
    pool: &SqlitePool,
    pattern: &str,
    limit: usize,
) -> Result<Vec<FtsHit>, MindError> {
    let conn = pool.get()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, 1.0 FROM nodes
             WHERE content LIKE ?1 ESCAPE '\\'
             ORDER BY CASE phase_state
                 WHEN 'Crystal' THEN 0 WHEN 'Liquid' THEN 1 ELSE 2 END
             LIMIT ?2",
        )
        .map_err(|e| MindError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map(
            rusqlite::params![pattern, limit as i64],
            |row| {
                Ok(FtsHit {
                    node_id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
                    score: row.get(1)?,
                })
            },
        )
        .map_err(|e| MindError::Storage(e.to_string()))?;
    let mut hits = Vec::new();
    for r in rows {
        hits.push(r.map_err(|e| MindError::Storage(e.to_string()))?);
    }
    Ok(hits)
}
