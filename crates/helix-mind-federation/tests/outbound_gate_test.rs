//! Outbound gate integration tests (ADR-0018 P3a).
//!
//! Verifies: federation outbound is refused when `enabled = false` (default,
//! "capability not ready = feature does not exist"), and every public node
//! must pass the deterministic dual-judge review before leaving the mind
//! (fail-closed on high-risk / self-contradictory nodes).

use helix_mind_core::config::{FederationConfig, StorageConfig};
use helix_mind_core::graph::*;
use helix_mind_storage::StorageEngine;
use helix_mind_storage::WritePriority;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

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

/// Text node, or structured node carrying an `assertions` list when
/// `assertions_json` is `Some`.
fn create_l2_node(content: &str, assertions_json: Option<&str>) -> Node {
    let content = match assertions_json {
        Some(assertions) => {
            let mut map = HashMap::new();
            map.insert("assertions".into(), assertions.into());
            NodeContent::Structured(map)
        }
        None => NodeContent::Text(content.into()),
    };
    Node {
        id: Uuid::new_v4(),
        node_type: NodeType::L2,
        content,
        dominance: 0.5,
        utility: 0.5,
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
        phase_state: PhaseState::Liquid,
        subject_dependency: SubjectDependency::Low,
        meta: PhaseMeta::default(),
    }
}

fn fed_config(enabled: bool) -> FederationConfig {
    FederationConfig {
        enabled,
        outgoing_dir: "/tmp/test_fed_outgoing".to_string(),
        sandbox_dir: "/tmp/test_fed_sandbox".to_string(),
        cremation_years: 5,
        scan_interval_sec: 60,
    }
}

fn ensure_outgoing_dir() {
    std::fs::create_dir_all("/tmp/test_fed_outgoing").unwrap();
}

#[tokio::test]
async fn outbound_gate_disabled_refuses_share() {
    let storage = create_test_storage().await;
    let share = helix_mind_federation::dag_share::DagShare::new(fed_config(false), storage);
    let res = share.share(None).await;
    assert!(res.is_err(), "disabled gate must refuse outbound share");
}

#[tokio::test]
async fn outbound_gate_enabled_high_risk_node_fails_closed() {
    let storage = create_test_storage().await;
    let node = create_l2_node("run sudo systemctl reboot now", None);
    storage.write_node(node, WritePriority::Critical).await.unwrap();
    let share = helix_mind_federation::dag_share::DagShare::new(fed_config(true), storage);
    let res = share.share(None).await;
    assert!(res.is_err(), "high-risk node must fail closed on outbound");
}

#[tokio::test]
async fn outbound_gate_enabled_self_contradictory_node_fails_closed() {
    let storage = create_test_storage().await;
    let node = create_l2_node(
        "",
        Some(
            r#"[{"subject":"A","predicate":"causes","object":"B"},
                 {"subject":"A","predicate":"prevents","object":"B"}]"#,
        ),
    );
    storage.write_node(node, WritePriority::Critical).await.unwrap();
    let share = helix_mind_federation::dag_share::DagShare::new(fed_config(true), storage);
    let res = share.share(None).await;
    assert!(res.is_err(), "self-contradictory node must fail closed on outbound");
}

#[tokio::test]
async fn outbound_gate_enabled_clean_node_passes_and_writes_block() {
    ensure_outgoing_dir();
    let storage = create_test_storage().await;
    let node = create_l2_node("the earth is round", None);
    storage.write_node(node, WritePriority::Critical).await.unwrap();
    let share = helix_mind_federation::dag_share::DagShare::new(fed_config(true), storage);
    let res = share.share(None).await;
    assert!(res.is_ok(), "clean public node should pass outbound review");
    let cid = res.unwrap();
    let path = format!("/tmp/test_fed_outgoing/{}.dag.zst", cid);
    assert!(std::path::Path::new(&path).exists(), "outgoing DAG block written");
}
