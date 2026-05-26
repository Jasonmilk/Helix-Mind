use helix_mind_core::graph::*;
use helix_mind_core::config::*;
use helix_mind_storage::StorageEngine;
use helix_mind_storage::WritePriority;
use helix_mind_retrieval::RetrievalEngine;
use helix_mind_metabolism::decay::DecayEngine;
use helix_mind_metabolism::symbolic::SymbolicSolver;
use helix_mind_metabolism::symbolic::LogicAssertion;
use helix_mind_metabolism::symbolic::Predicate;
use uuid::Uuid;
use std::sync::Arc;

async fn create_test_storage() -> Arc<StorageEngine> {
    let config = StorageConfig {
        sqlite_path: ":memory:".to_string(),
        human_view_dir: "/tmp/test_human_view".to_string(),
        human_view_max_size_mb: 1,
        node_cache_capacity: 100,
        deep_cold_dir: "/tmp/test_deep_cold".to_string(),
        deferred_write_interval_sec: 60,
        l3_merge_similarity_threshold: 0.85,
        parquet_dir: "/tmp/test_parquet".to_string(),
        topology_max_nodes: 100000,
        vector_similarity_threshold: 0.7,
    };
    StorageEngine::new(&config).await.unwrap()
}

fn create_l2_node(content: &str, utility: f64) -> Node {
    Node {
        id: Uuid::new_v4(),
        node_type: NodeType::L2,
        content: NodeContent::Text(content.to_string()),
        dominance: 0.5,
        utility,
        heat: 0.5,
        is_hypothetical: false,
        is_recessive: false,
        sensitivity: Some(Sensitivity::Public),
        generation: 1,
        created_at: chrono::Utc::now(),
        last_accessed_at: chrono::Utc::now(),
        access_count: 0,
        initial_impact: 0.5,
        corrected_by: None,
        notes: None,
        corroborations: 0,
        attribution_ledger: vec![],
        source: NodeSource::Local,
        high_risk: false,
        abstract_provenance: None,
        derived_from: vec![],
    }
}

#[tokio::test]
async fn test_write_and_retrieve_node() {
    let storage = create_test_storage().await;
    let node = create_l2_node("Rust memory safety", 0.8);
    let node_id = node.id;

    storage.write_node(node.clone(), WritePriority::Critical).await.unwrap();

    let retrieved = storage.get_nodes_by_ids(&[node_id]).await.unwrap();
    assert_eq!(retrieved.len(), 1);
    assert_eq!(retrieved[0].utility, 0.8);
}

#[tokio::test]
async fn test_retrieval_engine_basic() {
    let storage = create_test_storage().await;

    let node_a = create_l2_node("A leads to B", 0.9);
    let node_b = create_l2_node("B is the result", 0.9);
    let id_a = node_a.id;
    let id_b = node_b.id;

    storage.write_node(node_a.clone(), WritePriority::Critical).await.unwrap();
    storage.write_node(node_b.clone(), WritePriority::Critical).await.unwrap();

    let edge = Edge {
        source_id: id_a,
        target_id: id_b,
        weight: 0.9,
        relation_type: RelationType::Causal,
        is_soft: false,
    };
    storage.add_edge(&edge).await.unwrap();

    let retrieval_config = RetrievalConfig {
        beam_width: 3,
        weight_threshold: 0.5,
        max_nodes_per_query: 100,
        dead_end_penalty_factor: 0.8,
        max_hops: 5,
        soft_edge_decay_factor: 0.8,
        soft_edge_min_weight: 0.1,
        tentative_edge_weight: 0.3,
    };
    let retrieval = RetrievalEngine::new(retrieval_config, storage.clone());

    let energy = EnergyContext {
        token_budget: 1000,
        heliotropism: 0.0,
        pulse: 0.3,
        vigilance: 0.2,
        latency_limit_ms: 500,
        system_load: 0.0,
        familiarity: 0.5,
        impasse_depth: 0,
    };

    let result = retrieval.query(
        "test",
        CognitiveMode::Anchor,
        &energy,
        false,
        false,
        AutonomyLevel::Open,
    ).await.unwrap();

    println!("Impasse: {:?}, stages: {}", result.impasse_level, result.stages_attempted);
}

#[tokio::test]
async fn test_damped_momentum_decay() {
    let storage = create_test_storage().await;
    let node = create_l2_node("Important knowledge", 1.0);
    storage.write_node(node.clone(), WritePriority::Critical).await.unwrap();

    let decay_engine = DecayEngine::new(0.95, 0.15, storage.clone());

    let new_utility = decay_engine.apply_decay(&node, 0.5).await.unwrap();
    assert!(new_utility > 0.94);

    let mut updated = node.clone();
    updated.utility = new_utility;
    for _ in 0..10 {
        updated.utility = decay_engine.apply_decay(&updated, 0.8).await.unwrap();
    }
    assert!(updated.utility < 0.6);
}

#[tokio::test]
async fn test_symbolic_solver_contradiction() {
    let solver = SymbolicSolver::new();

    let new = vec![LogicAssertion {
        subject: "action.rm".into(),
        predicate: Predicate::Causes,
        object: "cleanup".into(),
    }];
    let l0 = vec![LogicAssertion {
        subject: "action.rm".into(),
        predicate: Predicate::Prevents,
        object: "cleanup".into(),
    }];

    assert!(solver.check_clash(&new, &l0, &[]).is_err());
}

#[tokio::test]
async fn test_user_trait_serialization() {
    use helix_mind_core::persona::{UserTraitNode, TraitType, NodeLifecycle};

    let node = UserTraitNode {
        node_id: Uuid::new_v4(),
        trait_type: TraitType::Preference,
        confidence: 0.9,
        evidence: vec![],
        abstract_provenance: Some("Based on observations".into()),
        lifecycle: NodeLifecycle::CreatorImprint,
        evidence_solidified_at: Some(chrono::Utc::now()),
        created_generation: 1,
        last_updated_generation: 1,
    };

    let json = serde_json::to_string(&node).unwrap();
    let back: UserTraitNode = serde_json::from_str(&json).unwrap();
    assert_eq!(back.confidence, 0.9);
    assert_eq!(back.lifecycle, NodeLifecycle::CreatorImprint);
}