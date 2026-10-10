//! Sleep review — P10c (ADR-0031 D3): Deep Dream review + adaptive mutation.
//!
//! Mounted in the API layer (the orchestrator): after `trigger_hibernate`
//! forgets cold L3 nodes, this module reviews L1 strategy coverage and
//! adapts the mutation rate. Full chain is deterministic and 0-token:
//!
//! ```text
//! L1 legacy coverage vs L1 fresh coverage (whitespace tokenization,
//! same semantics as DeterministicAdapter::extract_entities)
//!     → SleepReview::review_path → Stale / Viable
//!     → AdaptiveMutation::record_outcome → mutation_rate
//!     → persisted as an L2 node with provenance "mutation-state"
//!       (same storage pipeline; idempotent by name-based id)
//! ```
//!
//! Kept OUT of the metabolism crate on purpose: metabolism does not depend
//! on the cognitive crate (no dependency cycle); the API layer composes
//! both (extreme decoupling — the orchestrator orchestrates).

use std::collections::HashMap;
use std::sync::Arc;

use helix_mind_cognitive::{AdaptiveMutation, MutationConfig, ReviewConfig, SleepReview};
use helix_mind_core::error::MindError;
use helix_mind_core::graph::{Node, NodeContent, NodeType};
use helix_mind_storage::{StorageEngine, WritePriority};

/// Deterministic provenance marker for the persistent mutation state.
const MUTATION_STATE_PROVENANCE: &str = "mutation-state";

/// Result of one sleep-review pass.
#[derive(Debug, Clone, PartialEq)]
pub struct SleepReviewReport {
    /// Whether enough L1 strategy nodes existed to compare (>= 2).
    pub compared: bool,
    /// Entity coverage of the legacy (older) L1 synthesis.
    pub legacy_coverage: usize,
    /// Entity coverage of the fresh (newer) L1 synthesis.
    pub review_coverage: usize,
    /// Review verdict: Stale means the old path ossified (explore more).
    pub verdict: Option<String>,
    /// Mutation rate after adaptation (persisted).
    pub mutation_rate: f64,
    /// EMA-smoothed success rate after adaptation (persisted).
    pub ema_success: f64,
}

/// Entity coverage of a node's text content — same semantics as
/// DeterministicAdapter::extract_entities (whitespace tokenization),
/// expressed here as a pure count (no async trait needed, no cycle).
fn entity_coverage(node: &Node) -> usize {
    match &node.content {
        NodeContent::Text(t) => t.split_whitespace().count(),
        _ => 0,
    }
}

/// Load the persisted mutation state (L2 node, provenance "mutation-state").
/// Missing → fresh AdaptiveMutation defaults (protocol anchors).
async fn load_mutation(storage: &Arc<StorageEngine>) -> Result<AdaptiveMutation, MindError> {
    let l2 = storage.get_nodes_by_type(NodeType::L2).await?;
    let state = l2
        .iter()
        .find(|n| n.abstract_provenance.as_deref() == Some(MUTATION_STATE_PROVENANCE));
    match state {
        Some(node) => {
            let mut rate = None;
            let mut ema = None;
            if let NodeContent::Structured(map) = &node.content {
                rate = map.get("mutation_rate").and_then(|v| v.parse::<f64>().ok());
                ema = map.get("ema_success").and_then(|v| v.parse::<f64>().ok());
            }
            let mut mutation = AdaptiveMutation::new(MutationConfig::default());
            // Re-seed from persisted values when present (deterministic
            // replay across restarts); fall back to defaults otherwise.
            if let (Some(r), Some(e)) = (rate, ema) {
                if (MutationConfig::default().min_rate..=MutationConfig::default().max_rate)
                    .contains(&r)
                    && (0.0..=1.0).contains(&e)
                {
                    mutation = AdaptiveMutation::restore(r, e, MutationConfig::default());
                }
            }
            Ok(mutation)
        }
        None => Ok(AdaptiveMutation::new(MutationConfig::default())),
    }
}

/// Persist the mutation state as an L2 node (idempotent by name-based id).
async fn persist_mutation(
    storage: &Arc<StorageEngine>,
    mutation: &AdaptiveMutation,
) -> Result<(), MindError> {
    let state = storage.get_nodes_by_type(NodeType::L2).await?;
    let existing = state
        .iter()
        .find(|n| n.abstract_provenance.as_deref() == Some(MUTATION_STATE_PROVENANCE));

    // Deterministic node id (DNA principle 11): same provenance derives the
    // same id — upsert writes the SAME node, never a twin.
    let id = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, MUTATION_STATE_PROVENANCE.as_bytes());
    let content = NodeContent::Structured(HashMap::from_iter(vec![
        ("mutation_rate".to_string(), format!("{:.6}", mutation.mutation_rate())),
        ("ema_success".to_string(), format!("{:.6}", mutation.ema_success())),
    ]));

    match existing {
        Some(node) => {
            let mut updated = node.clone();
            updated.content = content;
            storage.write_node(updated, WritePriority::Deferred).await?;
        }
        None => {
            let node = Node {
                id,
                node_type: NodeType::L2,
                content,
                abstract_provenance: Some(MUTATION_STATE_PROVENANCE.to_string()),
                notes: Some("sleep-review mutation state (P10c, ADR-0031)".to_string()),
                ..Default::default()
            };
            storage.write_node(node, WritePriority::Deferred).await?;
        }
    }
    Ok(())
}

/// Run one sleep-review pass: compare L1 strategy coverage, adapt the
/// mutation rate, persist the state. Deterministic, 0 tokens.
pub async fn run_sleep_review(storage: &Arc<StorageEngine>) -> Result<SleepReviewReport, MindError> {
    let mut l1 = storage.get_nodes_by_type(NodeType::L1).await?;
    l1.sort_by_key(|n| n.created_at);

    if l1.len() < 2 {
        // Not enough strategy nodes to compare — honest no-op (the mutation
        // state stays untouched; no fabricated verdict).
        let mutation = load_mutation(storage).await?;
        return Ok(SleepReviewReport {
            compared: false,
            legacy_coverage: 0,
            review_coverage: 0,
            verdict: None,
            mutation_rate: mutation.mutation_rate(),
            ema_success: mutation.ema_success(),
        });
    }

    let legacy = &l1[l1.len() - 2];
    let fresh = &l1[l1.len() - 1];
    let legacy_cov = entity_coverage(legacy);
    let review_cov = entity_coverage(fresh);

    let review = SleepReview {
        config: ReviewConfig::default(),
    };
    let verdict = review.review_path(legacy_cov, review_cov);
    let viable = verdict == helix_mind_cognitive::ReviewVerdict::Viable;

    let mut mutation = load_mutation(storage).await?;
    mutation.record_outcome(viable);
    persist_mutation(storage, &mutation).await?;

    Ok(SleepReviewReport {
        compared: true,
        legacy_coverage: legacy_cov,
        review_coverage: review_cov,
        verdict: Some(format!("{verdict:?}")),
        mutation_rate: mutation.mutation_rate(),
        ema_success: mutation.ema_success(),
    })
}
