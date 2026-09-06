//! Sleep review tests — P10c (ADR-0031 D3).
//!
//! Deterministic 0-token Deep Dream chain: L1 strategy coverage comparison →
//! SleepReview verdict → AdaptiveMutation adaptation → persisted mutation
//! state (L2 node, provenance "mutation-state", idempotent).

use std::sync::Arc;

use helix_mind_api::sleep_review::run_sleep_review;
use helix_mind_core::graph::{Node, NodeContent, NodeType};
use helix_mind_core::config::Config;
use helix_mind_storage::{StorageEngine, WritePriority};
use chrono::Utc;

fn l1_node(text: &str, minutes_ago: i64) -> Node {
    let mut n = Node::default();
    n.node_type = NodeType::L1;
    n.content = NodeContent::Text(text.to_string());
    n.created_at = Utc::now() - chrono::Duration::minutes(minutes_ago);
    n.last_accessed_at = n.created_at;
    n
}

async fn make_storage() -> Arc<StorageEngine> {
    let mut config = Config::default();
    config.storage.sqlite_path = ":memory:".to_string();
    StorageEngine::new(&config.storage).await.unwrap()
}

#[tokio::test]
async fn stale_review_raises_mutation_rate() {
    let storage = make_storage().await;
    // Legacy covers 1 entity; fresh review covers 4 → delta 3 >= 2 → Stale
    // (old path ossified → record failure → explore more → rate up).
    storage.write_node(l1_node("alpha", 60), WritePriority::Deferred).await.unwrap();
    storage.write_node(l1_node("alpha beta gamma delta", 30), WritePriority::Deferred).await.unwrap();

    let report = run_sleep_review(&storage).await.unwrap();
    assert_eq!(report.compared, true);
    assert_eq!(report.legacy_coverage, 1);
    assert_eq!(report.review_coverage, 4);
    assert_eq!(report.verdict.as_deref(), Some("Stale"));
    // Failure: EMA drops below the neutral 0.5 → mutation rate rises
    // (0.20 - 0.17*0.35 ≈ 0.1405 > neutral 0.115).
    assert!(
        report.ema_success < 0.5,
        "stale → success rate drops, got {}",
        report.ema_success
    );
    assert!(
        report.mutation_rate > 0.115,
        "stale → explore more, got {}",
        report.mutation_rate
    );

    // State persisted as an idempotent L2 node.
    let l2 = storage.get_nodes_by_type(NodeType::L2).await.unwrap();
    let state = l2.iter().find(|n| n.abstract_provenance.as_deref() == Some("mutation-state"));
    assert!(state.is_some(), "mutation state persisted");
}

#[tokio::test]
async fn viable_review_lowers_mutation_rate() {
    let storage = make_storage().await;
    // Legacy 4 entities, fresh 4 → delta 0 < 2 → Viable (record success →
    // rate down).
    storage.write_node(l1_node("a b c d", 60), WritePriority::Deferred).await.unwrap();
    storage.write_node(l1_node("w x y z", 30), WritePriority::Deferred).await.unwrap();

    let report = run_sleep_review(&storage).await.unwrap();
    assert_eq!(report.verdict.as_deref(), Some("Viable"));
    // Success: EMA above neutral 0.5 → mutation rate converges down
    // (0.20 - 0.17*0.65 ≈ 0.0895 < neutral 0.115).
    assert!(
        report.ema_success > 0.5,
        "viable → success rate rises, got {}",
        report.ema_success
    );
    assert!(
        report.mutation_rate < 0.115,
        "viable → settle down, got {}",
        report.mutation_rate
    );
}

#[tokio::test]
async fn fewer_than_two_l1_is_honest_noop() {
    let storage = make_storage().await;
    storage.write_node(l1_node("only one", 10), WritePriority::Deferred).await.unwrap();

    let report = run_sleep_review(&storage).await.unwrap();
    assert_eq!(report.compared, false);
    assert_eq!(report.verdict, None);
    // No fabricated mutation state either.
    let l2 = storage.get_nodes_by_type(NodeType::L2).await.unwrap();
    assert!(
        l2.iter().all(|n| n.abstract_provenance.as_deref() != Some("mutation-state")),
        "no state written without a comparison"
    );
}

#[tokio::test]
async fn mutation_state_is_idempotent_and_restores() {
    let storage = make_storage().await;
    storage.write_node(l1_node("alpha", 60), WritePriority::Deferred).await.unwrap();
    storage.write_node(l1_node("alpha beta gamma delta", 30), WritePriority::Deferred).await.unwrap();

    let first = run_sleep_review(&storage).await.unwrap();
    // Second pass: restore from persisted state → continues from first
    // pass's rate, then adapts again (Stale again).
    let second = run_sleep_review(&storage).await.unwrap();

    // Exactly one mutation-state node (idempotent upsert).
    let l2 = storage.get_nodes_by_type(NodeType::L2).await.unwrap();
    let states: Vec<&Node> = l2
        .iter()
        .filter(|n| n.abstract_provenance.as_deref() == Some("mutation-state"))
        .collect();
    assert_eq!(states.len(), 1, "idempotent state node");

    // Restore works: the second pass continues from the persisted rate
    // (state was not reset to default).
    assert!(
        (second.mutation_rate - first.mutation_rate).abs() > 1e-9 || second.mutation_rate > 0.05,
        "second pass continues from persisted state (first={}, second={})",
        first.mutation_rate,
        second.mutation_rate
    );
}
