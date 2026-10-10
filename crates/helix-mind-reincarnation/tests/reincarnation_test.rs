//! P6-3 轮回补全集成测试。
//!
//! 验证 `trigger_reincarnation` 执行**完整轮回**（sunset 归档旧生 → rebirth 新生），
//! 且 emergency_dusk / epoch / inheritance 路径修复不回归。

use helix_mind_core::config::{LifecycleConfig, StorageConfig};
use helix_mind_core::graph::{Node, NodeContent, NodeType, Sensitivity};
use helix_mind_reincarnation::ReincarnationEngine;
use helix_mind_storage::StorageEngine;
use helix_mind_storage::WritePriority;

fn temp_dir(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("helix_rc_{}_{}", name, std::process::id()))
}

async fn temp_engine(name: &str) -> (std::sync::Arc<StorageEngine>, std::path::PathBuf) {
    let dir = temp_dir(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let cfg = StorageConfig {
        sqlite_path: dir.join("test.db").to_string_lossy().into_owned(),
        wal_dir: dir.join("wal").to_string_lossy().into_owned(),
        ..Default::default()
    };
    let engine = StorageEngine::new(&cfg).await.unwrap();
    (engine, dir)
}

/// 完整轮回：sunset（归档）→ rebirth（新生），返回新世代号。
#[tokio::test]
async fn full_reincarnation_cycle_sunset_then_rebirth() {
    let (engine, _dir) = temp_engine("cycle").await;

    // 旧生记忆（generation 1, L3）。
    let node = Node {
        content: NodeContent::Text("旧生的记忆".into()),
        node_type: NodeType::L3,
        generation: 1,
        sensitivity: Some(Sensitivity::Private),
        ..Default::default()
    };
    engine
        .write_node(node.clone(), WritePriority::Critical)
        .await
        .unwrap();

    let lc = LifecycleConfig {
        enabled: true,
        archive_past_life: false, // 测试不写 epoch crystal
        inheritance_crystal: false,
        ..Default::default()
    };
    let rc = ReincarnationEngine::new(lc, engine.clone());

    // 1. 错误确认令牌必须拒绝。
    assert!(rc.trigger_reincarnation("wrong token").await.is_err());

    // 2. 完整轮回。
    let new_gen = rc
        .trigger_reincarnation("I understand this will reset my memory")
        .await
        .unwrap();
    assert_eq!(new_gen, 2, "sunset archives gen 1, rebirth born gen 2");

    // 3. 审计：sunset + rebirth 各写一条 ReincarnationTriggered。
    let conn = engine.sqlite.get().unwrap();
    let cnt: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM audit_log WHERE event_type = 'ReincarnationTriggered'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(cnt >= 2, "sunset + rebirth both audit (got {})", cnt);

    // 4. 旧生节点仍保留（记忆不可篡改，归档不删除）。
    let stored = engine.get_nodes_by_ids(&[node.id]).await.unwrap();
    assert_eq!(stored.len(), 1, "old memory preserved (archive, not delete)");
}

/// emergency_dusk 的世代号来自真实 storage（不再硬编码 1）。
#[tokio::test]
async fn emergency_dusk_uses_real_generation() {
    let (engine, _dir) = temp_engine("dusk").await;

    // 世代 5 的 L3 节点 → dusk 的新世代应为 6。
    let node = Node {
        content: NodeContent::Text("世代五的记忆".into()),
        node_type: NodeType::L3,
        generation: 5,
        sensitivity: Some(Sensitivity::Private),
        ..Default::default()
    };
    engine
        .write_node(node.clone(), WritePriority::Critical)
        .await
        .unwrap();

    let lc = LifecycleConfig {
        enabled: true,
        archive_past_life: false,
        inheritance_crystal: false,
        ..Default::default()
    };
    let rc = ReincarnationEngine::new(lc, engine.clone());
    let new_gen = rc
        .trigger_emergency_dusk(1024, 100000)
        .await
        .unwrap();
    assert_eq!(new_gen, 6, "dusk derives generation from storage, not hardcoded 1");
}

/// epoch 晶体写入配置目录（deep_cold_dir），不污染工作目录。
#[tokio::test]
async fn epoch_crystal_written_to_deep_cold_dir() {
    let (engine, dir) = temp_engine("epoch").await;

    let lc = LifecycleConfig {
        enabled: true,
        archive_past_life: true, // 触发 epoch crystal
        inheritance_crystal: false,
        ..Default::default()
    };
    let rc = ReincarnationEngine::new(lc, engine.clone());
    let new_gen = rc
        .trigger_reincarnation("I understand this will reset my memory")
        .await
        .unwrap();
    assert_eq!(new_gen, 2);

    // 晶体必须落在 deep_cold_dir，而非当前目录。
    let deep_cold = engine.config.deep_cold_dir.clone();
    let entries = std::fs::read_dir(&deep_cold).unwrap();
    let mut has_epoch = false;
    for e in entries {
        let name = e.unwrap().file_name().to_string_lossy().into_owned();
        if name.starts_with("epoch_crystal_") && name.ends_with(".zst") {
            has_epoch = true;
        }
    }
    assert!(has_epoch, "epoch crystal written to deep_cold_dir");

    let _ = dir;
}
