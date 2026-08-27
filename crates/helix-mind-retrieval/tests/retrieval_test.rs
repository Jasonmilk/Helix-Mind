//! P0.5 retrieval pipeline tests.
//!
//! The retrieval engine's Stage-1 start-node extraction is now a pluggable
//! seam (`StartNodeExtractor`). These tests inject the deterministic
//! `FakeAdapter` so the full Stage 1..5 pipeline is exercised with real
//! in-memory SQLite data — no live LLM, no network.

use std::sync::Arc;

use helix_mind_core::config::{RetrievalConfig, StorageConfig};
use helix_mind_core::graph::*;
use helix_mind_retrieval::{FakeAdapter, RetrievalEngine};
use helix_mind_storage::{StorageEngine, WritePriority};
use uuid::Uuid;

async fn memory_storage() -> Arc<StorageEngine> {
    let config = StorageConfig {
        sqlite_path: ":memory:".to_string(),
        ..Default::default()
    };
    StorageEngine::new(&config).await.unwrap()
}

fn retrieval_config() -> RetrievalConfig {
    RetrievalConfig {
        beam_width: 3,
        // Low threshold: SA-Core keeps only nodes whose activation survives the
        // attenuation loop. A 1-hop leaf (A→B) holds 0.25; threshold 0.5 would
        // zero it, hiding traversal. 0.2 lets the test observe the edge reach.
        weight_threshold: 0.2,
        max_nodes_per_query: 100,
        dead_end_penalty_factor: 0.8,
        max_hops: 5,
        soft_edge_decay_factor: 0.8,
        soft_edge_min_weight: 0.1,
        tentative_edge_weight: 0.3,
    }
}

fn energy() -> EnergyContext {
    EnergyContext {
        token_budget: 1000,
        heliotropism: 0.0,
        pulse: 0.3,
        vigilance: 0.2,
        latency_limit_ms: 500,
        system_load: 0.0,
        familiarity: 0.5,
        impasse_depth: 0,
        budget_tier: BudgetTier::Augmentable,
    }
}

fn l2_node(content: &str, utility: f64) -> Node {
    Node {
        id: Uuid::new_v4(),
        node_type: NodeType::L2,
        content: NodeContent::Text(content.to_string()),
        dominance: 0.5,
        utility,
        // L2 → Low subject-dependency (ADR-0011), Liquid phase by default.
        subject_dependency: SubjectDependency::Low,
        ..Default::default()
    }
}

#[tokio::test]
async fn retrieval_returns_start_node_with_fake_adapter() {
    let storage = memory_storage().await;
    let node = l2_node("Helix memory architecture", 0.9);
    let node_id = node.id;
    storage
        .write_node(node, WritePriority::Critical)
        .await
        .unwrap();

    let mut fake = FakeAdapter::new();
    fake.add("memory architecture", vec![node_id]);
    let engine = RetrievalEngine::with_extractor(retrieval_config(), storage, Arc::new(fake));

    let result = engine
        .query(
            "memory architecture",
            CognitiveMode::Anchor,
            &energy(),
            false,
            false,
            AutonomyLevel::Open,
        )
        .await
        .unwrap();

    assert!(
        !result.nodes.is_empty(),
        "retrieval must return the start node (FakeAdapter maps query → node)"
    );
    assert_eq!(result.nodes[0].id, node_id);
}

#[tokio::test]
async fn retrieval_traverses_causal_edge_to_neighbor() {
    let storage = memory_storage().await;
    let a = l2_node("A leads to B", 0.9);
    let b = l2_node("B is the result", 0.9);
    let id_a = a.id;
    let id_b = b.id;
    storage.write_node(a, WritePriority::Critical).await.unwrap();
    storage.write_node(b, WritePriority::Critical).await.unwrap();
    storage
        .add_edge(&Edge {
            source_id: id_a,
            target_id: id_b,
            weight: 0.9,
            relation_type: RelationType::Causal,
            is_soft: false,
        })
        .await
        .unwrap();

    let mut fake = FakeAdapter::new();
    fake.add("A", vec![id_a]);
    let engine = RetrievalEngine::with_extractor(retrieval_config(), storage, Arc::new(fake));

    let result = engine
        .query("A", CognitiveMode::Anchor, &energy(), false, false, AutonomyLevel::Open)
        .await
        .unwrap();

    let returned: Vec<Uuid> = result.nodes.iter().map(|n| n.id).collect();
    assert!(
        returned.contains(&id_a),
        "start node A must be returned"
    );
    assert!(
        returned.contains(&id_b),
        "skilled traversal should reach B via the causal edge"
    );
}
