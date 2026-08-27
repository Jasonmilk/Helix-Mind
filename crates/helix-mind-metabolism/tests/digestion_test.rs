//! P2a M-04/M-05 integration tests (ADR-0014): deterministic metabolism
//! through the real (temp-file) storage engine —
//! 1. digest merge loop: FTS5 candidate recall → find_similar_node →
//!    Levenshtein threshold → mark_recessive + SimilarTo edge;
//! 2. dissonance loop: Conflicts edge on structured assertions →
//!    SymbolicSolver arbitration → Corrects edge (weight -1.0) + corrected_by.

use helix_mind_core::config::{MetabolismConfig, StorageConfig};
use helix_mind_core::graph::{
    Edge, Node, NodeContent, NodeType, RelationType, Sensitivity,
};
use helix_mind_metabolism::{MetabolismEngine, MetabolismEvent};
use helix_mind_storage::{StorageEngine, WritePriority};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

// ── helpers ─────────────────────────────────────────────────────────────

/// Temp-file engine (NOT `:memory:`: the r2d2 pool would give each pooled
/// connection its own private in-memory DB).
async fn temp_engine() -> (Arc<StorageEngine>, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("hm_meta_{}.db", Uuid::new_v4()));
    let config = StorageConfig {
        sqlite_path: dir.to_string_lossy().to_string(),
        ..Default::default()
    };
    let engine = StorageEngine::new(&config).await.unwrap();
    (engine, dir)
}

fn cleanup(dir: &Path) {
    let _ = std::fs::remove_file(dir);
    let _ = std::fs::remove_file(format!("{}-wal", dir.display()));
    let _ = std::fs::remove_file(format!("{}-shm", dir.display()));
}

fn l3_text(text: &str) -> Node {
    Node {
        id: Uuid::new_v4(),
        node_type: NodeType::L3,
        content: NodeContent::Text(text.to_string()),
        sensitivity: Some(Sensitivity::Public),
        ..Node::default()
    }
}

/// Structured L3 node carrying a `{subject, predicate, object}` assertion
/// (P2a convention: `assertions` key in the Structured map).
fn l3_structured(subject: &str, predicate: &str, object: &str, heat: f64) -> Node {
    let mut map = HashMap::new();
    map.insert(
        "assertions".to_string(),
        format!(
            r#"[{{"subject":"{}","predicate":"{}","object":"{}"}}]"#,
            subject, predicate, object
        ),
    );
    Node {
        id: Uuid::new_v4(),
        node_type: NodeType::L3,
        content: NodeContent::Structured(map),
        heat,
        sensitivity: Some(Sensitivity::Public),
        ..Node::default()
    }
}

// ── M-04: digest merge loop ─────────────────────────────────────────────

#[tokio::test]
async fn digest_merges_similar_l3_nodes() {
    let (engine, dir) = temp_engine().await;

    let a = l3_text("机器学习模型训练需要大量数据");
    let b = l3_text("机器学习模型训练需要大量数据");
    engine.write_node(a.clone(), WritePriority::Critical).await.unwrap();
    engine.write_node(b.clone(), WritePriority::Critical).await.unwrap();
    engine.flush_fts_index().await.unwrap();


    let metabolism = MetabolismEngine::new(MetabolismConfig::default(), engine.clone());
    let report = metabolism
        .trigger_by_event(MetabolismEvent::MicroSleep)
        .await
        .unwrap();

    assert!(report.digest_merged >= 1, "expected >=1 merge, got {}", report.digest_merged);

    // One twin is folded (recessive), the survivor stays active, and a
    // SimilarTo edge links them.
    let after = engine.get_nodes_by_ids(&[a.id, b.id]).await.unwrap();
    let recessive = after.iter().filter(|n| n.is_recessive).count();
    let active = after.iter().filter(|n| !n.is_recessive).count();
    assert_eq!(recessive, 1, "exactly one twin should be merged/recessive");
    assert_eq!(active, 1, "exactly one twin should survive");
    let edges = engine.get_edges_between(&[a.id, b.id]).await.unwrap();
    assert!(
        edges.iter().any(|e| e.relation_type == RelationType::SimilarTo),
        "SimilarTo edge expected, got {edges:?}"
    );

    cleanup(&dir);
}

// ── M-05: dissonance loop ───────────────────────────────────────────────

#[tokio::test]
async fn digest_resolves_structured_dissonance() {
    let (engine, dir) = temp_engine().await;

    // Contradictory assertions on the same subject/object.
    let a = l3_structured("咖啡因", "increases", "心率", 0.9);
    let b = l3_structured("咖啡因", "decreases", "心率", 0.1);
    engine.write_node(a.clone(), WritePriority::Deferred).await.unwrap();
    engine.write_node(b.clone(), WritePriority::Deferred).await.unwrap();

    // Declare the conflict.
    engine
        .add_edge(&Edge {
            source_id: a.id,
            target_id: b.id,
            weight: 1.0,
            relation_type: RelationType::Conflicts,
            is_soft: false,
        })
        .await
        .unwrap();

    // Zero cooling window: the freshly written conflict is immediately
    // eligible for arbitration.
    let mut config = MetabolismConfig::default();
    config.dissonance_window_hours = 0;
    let metabolism = MetabolismEngine::new(config, engine.clone());
    metabolism
        .trigger_by_event(MetabolismEvent::MicroSleep)
        .await
        .unwrap();

    // Higher-heat node (a, 0.9) corrects lower-heat node (b, 0.1):
    // Corrects edge (weight -1.0) + corrected_by trace.
    let edges = engine.get_edges_between(&[a.id, b.id]).await.unwrap();
    assert!(
        edges.iter().any(|e| e.relation_type == RelationType::Corrects && e.weight == -1.0),
        "Corrects edge (weight -1.0) expected, got {edges:?}"
    );
    let after = engine.get_nodes_by_ids(&[b.id]).await.unwrap();
    assert_eq!(
        after[0].corrected_by,
        Some(a.id),
        "b should be traced as corrected by a"
    );

    // And the pair is no longer reported as unresolved.
    let unresolved = engine
        .get_unresolved_dissonance(chrono::Duration::hours(0))
        .await
        .unwrap();
    assert!(
        !unresolved.iter().any(|(x, y)| (*x, *y) == (a.id, b.id)),
        "resolved pair must not reappear in unresolved dissonance"
    );

    cleanup(&dir);
}

#[tokio::test]
async fn digest_skips_text_only_dissonance() {
    let (engine, dir) = temp_engine().await;

    // Text nodes with a Conflicts edge: P2a deterministic arbitration cannot
    // extract assertions from free text (honest boundary, defers to P2b LLM
    // translation) — the pair must stay unresolved, and no Corrects edge may
    // be fabricated.
    let a = l3_text("咖啡因会升高心率");
    let b = l3_text("咖啡因会降低心率");
    engine.write_node(a.clone(), WritePriority::Deferred).await.unwrap();
    engine.write_node(b.clone(), WritePriority::Deferred).await.unwrap();
    engine
        .add_edge(&Edge {
            source_id: a.id,
            target_id: b.id,
            weight: 1.0,
            relation_type: RelationType::Conflicts,
            is_soft: false,
        })
        .await
        .unwrap();

    let mut config = MetabolismConfig::default();
    config.dissonance_window_hours = 0;
    let metabolism = MetabolismEngine::new(config, engine.clone());
    metabolism
        .trigger_by_event(MetabolismEvent::MicroSleep)
        .await
        .unwrap();

    let unresolved = engine
        .get_unresolved_dissonance(chrono::Duration::hours(0))
        .await
        .unwrap();
    assert!(
        unresolved.iter().any(|(x, y)| (*x, *y) == (a.id, b.id)),
        "text-only conflict must remain unresolved (P2a boundary)"
    );
    let edges = engine.get_edges_between(&[a.id, b.id]).await.unwrap();
    assert!(
        !edges.iter().any(|e| e.relation_type == RelationType::Corrects),
        "no Corrects edge may be fabricated for text-only conflict"
    );

    cleanup(&dir);
}
