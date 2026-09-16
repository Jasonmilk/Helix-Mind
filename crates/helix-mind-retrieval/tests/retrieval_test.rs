//! P0.5 retrieval pipeline tests.
//!
//! The retrieval engine's Stage-1 start-node extraction is now a pluggable
//! seam (`StartNodeExtractor`). These tests inject the deterministic
//! `FakeAdapter` so the full Stage 1..5 pipeline is exercised with real
//! in-memory SQLite data — no live LLM, no network.

use std::sync::Arc;

use helix_mind_core::config::{RetrievalConfig, StorageConfig};
use helix_mind_core::graph::*;
use helix_mind_retrieval::{energy_degraded, FakeAdapter, RetrievalEngine};
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
    RetrievalConfig { stopwords: Vec::new(),
        beam_width: 3,
        // ADR-0042 D0: the threshold workaround is gone. The gate is relative
        // (τ · activation mass), so a 1-hop leaf A→B holding ≈0.25 of mass ≈1
        // survives the default τ=0.02. The previous `weight_threshold: 0.2`
        // existed solely to get under the absolute 0.8 gate — which sat ABOVE
        // the first-hop ceiling α, zeroing every non-seed node.
        max_nodes_per_query: 100,
        dead_end_penalty_factor: 0.8,
        max_hops: 5,
        soft_edge_decay_factor: 0.8,
        soft_edge_min_weight: 0.1,
        tentative_edge_weight: 0.3,
        ..Default::default()
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

// ── P10 seed-floor regression (2026-09-07) ──────────────────────────────

#[tokio::test]
async fn isolated_seed_survives_default_threshold() {
    // An isolated (edge-less) seed node decays to (1 - alpha) * 1.0 = 0.5 per hop
    // and used to be zeroed by the absolute `weight_threshold = 0.8`, silently
    // killing recall. Seed = hard evidence: it must survive.
    //
    // ADR-0042 D0 removed the absolute gate in favour of a relative one, so this
    // is no longer a close call — but the invariant is what matters, not the
    // margin, and it must hold for any gate setting (see the storage-layer
    // `seeds_survive_even_an_absurd_gate`).
    let storage = memory_storage().await;
    let node = l2_node("我叫Jason", 0.9);
    let node_id = node.id;
    storage
        .write_node(node, WritePriority::Critical)
        .await
        .unwrap();
    storage.flush_fts_index().await.unwrap();

    let mut fake = FakeAdapter::new();
    fake.add("我叫Jason", vec![node_id]);
    let engine = RetrievalEngine::with_extractor(RetrievalConfig::default(), storage, Arc::new(fake));
    let result = engine
        .query("我叫Jason", helix_mind_core::graph::CognitiveMode::Skilled, &energy(), false, false, AutonomyLevel::Agent)
        .await
        .unwrap();
    assert!(
        result.nodes.iter().any(|n| n.id == node_id),
        "seed node must survive default threshold; got {} nodes: {:?}",
        result.nodes.len(),
        result.nodes.iter().map(|n| &n.content).collect::<Vec<_>>()
    );
}


// ── K9: energy-guard thresholds live in config, not inline ──────────────

fn energy_at(system_load: f64, latency_limit_ms: u64, token_budget: u64) -> EnergyContext {
    EnergyContext {
        system_load,
        latency_limit_ms,
        token_budget,
        ..Default::default()
    }
}

#[test]
fn energy_guard_defaults_equal_the_original_literals() {
    let cfg = RetrievalConfig::default();
    assert_eq!(cfg.high_system_load, 0.9);
    assert_eq!(cfg.min_latency_limit_ms, 100);
    assert_eq!(cfg.min_token_budget, 100);
}

#[test]
fn energy_guard_degrades_only_past_the_threshold() {
    let cfg = RetrievalConfig::default();
    // Exactly at every threshold: still healthy (all three comparisons are strict).
    assert!(!energy_degraded(&energy_at(0.9, 100, 100), &cfg));
    // Past any single threshold: degraded.
    assert!(energy_degraded(&energy_at(0.91, 100, 100), &cfg));
    assert!(energy_degraded(&energy_at(0.0, 99, 100), &cfg));
    assert!(energy_degraded(&energy_at(0.0, 100, 99), &cfg));
    // Comfortably inside every budget: healthy.
    assert!(!energy_degraded(&energy_at(0.1, 5_000, 4_096), &cfg));
}

#[test]
fn energy_guard_thresholds_are_config_overridable() {
    let mut strict = RetrievalConfig::default();
    strict.high_system_load = 0.5;
    strict.min_token_budget = 2_048;
    let e = energy_at(0.6, 5_000, 1_024);
    // One energy context, two configs: the guard follows config, not a constant.
    assert!(!energy_degraded(&e, &RetrievalConfig::default()));
    assert!(energy_degraded(&e, &strict));
}

// ── SA-Core activation reaches the white-box (2026-09-17) ───────────────
//
// `HelixQueryResult.activation_vector` was reserved for this and returned empty
// everywhere, under a comment claiming the diffusion was "not yet implemented".
// It WAS implemented (storage::topology::sa_core_diffusion) and every layer above
// already carried it — the storage API returned it, proto field 13 reserved a
// seat, helix-mind-api already mapped it. The retrieval call was the one layer
// missing, so the panel's white-box reported the persisted `heat` column
// (540/540 nodes at its 0.5 construction default) instead of this cycle's
// activation.
//
// White-box honesty is the point: the answer must say what SA-Core actually
// chose AND how strongly, not just which ids came back.

#[tokio::test]
async fn activation_vector_reports_what_sa_core_actually_computed() {
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

    assert!(
        !result.activation_vector.is_empty(),
        "SA-Core ran and produced activations; the white-box must carry them"
    );
    let seed = result
        .activation_vector
        .iter()
        .find(|e| e.node_id == id_a)
        .expect("the seed must appear in the activation vector");
    assert!(
        seed.activation > 0.0,
        "the seed is hard evidence and must carry activation; got {}",
        seed.activation
    );
    let neighbour = result
        .activation_vector
        .iter()
        .find(|e| e.node_id == id_b)
        .expect("activation must spread along the causal edge to B");
    assert!(
        neighbour.activation > 0.0,
        "a node reached by diffusion must report non-zero activation; got {}",
        neighbour.activation
    );
    // Activation is a per-cycle quantity in (0,1]; it is not the persisted
    // column, which sits at its schema default for every node today.
    assert!(
        result.activation_vector.iter().all(|e| e.activation > 0.0 && e.activation <= 1.0),
        "activations stay in the protocol's stated range (0.0-1.0)"
    );
}

#[tokio::test]
async fn activation_vector_is_empty_only_when_there_is_nothing_to_seed() {
    // The honest-empty case: no start nodes means nothing was seeded, so the
    // vector is empty because that is the truth — not because a call is missing.
    let storage = memory_storage().await;
    let node = l2_node("unrelated", 0.9);
    storage.write_node(node, WritePriority::Critical).await.unwrap();

    let fake = FakeAdapter::new(); // maps nothing
    let engine = RetrievalEngine::with_extractor(retrieval_config(), storage, Arc::new(fake));

    let result = engine
        .query("nothing matches", CognitiveMode::Anchor, &energy(), false, false, AutonomyLevel::Open)
        .await
        .unwrap();

    assert!(result.nodes.is_empty(), "no start nodes -> no nodes");
    assert!(
        result.activation_vector.is_empty(),
        "nothing seeded -> genuinely empty, and that is honest"
    );
}
