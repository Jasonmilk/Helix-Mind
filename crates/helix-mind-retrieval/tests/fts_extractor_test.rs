//! P1 M-03 integration tests: the real FTS5-trigram extractor through a real
//! (temp-file) storage engine — Chinese recall, short-word LIKE fallback, phase
//! weighting, injection sanitization + audit, and end-to-end retrieval with the
//! FTS extractor as the production default.

use helix_mind_core::config::{RetrievalConfig, StorageConfig};
use helix_mind_core::graph::{
    AutonomyLevel, CognitiveMode, EnergyContext, Node, NodeContent, NodeType, PhaseState,
    Sensitivity,
};
use helix_mind_retrieval::{FtsExtractor, RetrievalEngine, StartNodeExtractor};
use helix_mind_storage::{StorageEngine, WritePriority};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

// ── helpers ─────────────────────────────────────────────────────────────

/// Temp-file engine (NOT `:memory:`: the r2d2 pool would give each pooled
/// connection its own private in-memory DB, breaking shared writes).
async fn temp_engine() -> (Arc<StorageEngine>, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("hm_fts_{}.db", Uuid::new_v4()));
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

fn node_with(content: &str, phase: PhaseState) -> Node {
    Node {
        id: Uuid::new_v4(),
        node_type: NodeType::L2,
        content: NodeContent::Text(content.to_string()),
        phase_state: phase,
        sensitivity: Some(Sensitivity::Public),
        ..Node::default()
    }
}

fn low_threshold_retrieval_config() -> RetrievalConfig {
    // SA-Core zeroes a leaf start node above 0.5 (see PLAN.md §3.6 note); 0.2
    // keeps the extracted node observable in the end-to-end pipeline test.
    let mut cfg = RetrievalConfig::default();
    cfg.weight_threshold = 0.2;
    cfg
}

// ── M-01 / M-03 tests ───────────────────────────────────────────────────

#[tokio::test]
async fn fts_extractor_recalls_chinese_content() {
    let (engine, dir) = temp_engine().await;
    let node = node_with("认知相态的河流与催化器", PhaseState::Liquid);
    engine
        .write_node(node.clone(), WritePriority::Critical)
        .await
        .unwrap();
    engine.flush_fts_index().await.unwrap();

    let extractor = FtsExtractor::new(&engine, 20);
    let ids = extractor.extract_start_nodes("认知相态");
    assert!(
        ids.contains(&node.id),
        "FTS5 trigram should recall Chinese content, got {ids:?}"
    );
    cleanup(&dir);
}

#[tokio::test]
async fn fts_extractor_falls_back_to_like_for_short_query() {
    let (engine, dir) = temp_engine().await;
    let node = node_with("rust systems programming", PhaseState::Liquid);
    engine
        .write_node(node.clone(), WritePriority::Critical)
        .await
        .unwrap();
    engine.flush_fts_index().await.unwrap();

    let extractor = FtsExtractor::new(&engine, 20);
    let ids = extractor.extract_start_nodes("ru");
    assert!(
        ids.contains(&node.id),
        "2-char query must use LIKE fallback, got {ids:?}"
    );
    cleanup(&dir);
}

#[tokio::test]
async fn fts_extractor_ranks_crystal_above_liquid() {
    let (engine, dir) = temp_engine().await;
    let liquid = node_with("算法设计模式", PhaseState::Liquid);
    let crystal = node_with("算法设计模式", PhaseState::Crystal);
    engine
        .write_node(liquid.clone(), WritePriority::Critical)
        .await
        .unwrap();
    engine
        .write_node(crystal.clone(), WritePriority::Critical)
        .await
        .unwrap();
    engine.flush_fts_index().await.unwrap();

    let extractor = FtsExtractor::new(&engine, 20);
    let ids = extractor.extract_start_nodes("算法设计");
    assert!(ids.contains(&crystal.id) && ids.contains(&liquid.id), "both phases match");
    assert_eq!(
        ids.first(),
        Some(&crystal.id),
        "Crystal (weight 1.5) must rank above Liquid (1.0), got {ids:?}"
    );
    cleanup(&dir);
}

#[tokio::test]
async fn fts_extractor_sanitizes_injection_input_and_audits() {
    let (engine, dir) = temp_engine().await;
    let node = node_with("安全", PhaseState::Liquid);
    engine
        .write_node(node.clone(), WritePriority::Critical)
        .await
        .unwrap();
    engine.flush_fts_index().await.unwrap();

    let extractor = FtsExtractor::new(&engine, 20);
    // Attempted FTS5 / SQL injection: quotes, semicolons, DROP TABLE. The
    // whitelist strips non-alphanumeric/non-space chars (quotes, semicolons);
    // benign words like "DROP"/"TABLE" survive but form a *different literal
    // phrase* — the point is it must not crash and must not affect the index.
    let _ = extractor.extract_start_nodes("\"; DROP TABLE nodes_fts; --");

    // The sanitized input must be audited (ruling 2026-08-28).
    let conn = engine.sqlite_get().await.unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM audit_log WHERE event_type = 'fts_input_sanitized'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1, "sanitized input must produce exactly one audit row");

    // The index survived the attack: a clean query still recalls the node.
    let still_ok = extractor.extract_start_nodes("安全");
    assert!(
        still_ok.contains(&node.id),
        "index must survive after injection attempt, got {still_ok:?}"
    );
    cleanup(&dir);
}

// ── M-02 end-to-end ─────────────────────────────────────────────────────

#[tokio::test]
async fn retrieval_engine_end_to_end_with_fts_default() {
    let (engine, dir) = temp_engine().await;
    let node = node_with("认知相态扩散机制", PhaseState::Liquid);
    engine
        .write_node(node.clone(), WritePriority::Critical)
        .await
        .unwrap();
    engine.flush_fts_index().await.unwrap();

    // `RetrievalEngine::new` defaults to the real FtsExtractor (P1 M-01).
    let retrieval = RetrievalEngine::new(low_threshold_retrieval_config(), engine.clone());
    let energy = EnergyContext::default();
    let result = retrieval
        .query(
            "认知相态扩散",
            CognitiveMode::Skilled,
            &energy,
            false,
            false,
            AutonomyLevel::Open,
        )
        .await
        .unwrap();
    assert!(
        !result.nodes.is_empty(),
        "end-to-end retrieval with FTS default must return nodes"
    );
    assert!(
        result.nodes.iter().any(|n| n.id == node.id),
        "the FTS-extracted start node must appear in the result"
    );
    cleanup(&dir);
}
