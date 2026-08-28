//! WAL 投影器（P5, ADR-0015）—— 把 WAL 事件（事实来源）投影到 SQLite（液态状态）。
//!
//! 职责边界：
//! - `WalProjector::apply` 将单条 `WalEvent` 应用到 `SqlitePool`（纯 SQL 层）。
//! - 投影器**不**维护内存拓扑 / 缓存 / FTS 索引（那是 `StorageEngine` 主路径的职责；
//!   完整恢复时由 `StorageEngine::new` 从 SQLite 重建拓扑）。
//! - 事件 SQL 与主路径共用 `SqlitePool::upsert_node` 等单一源方法，避免漂移。

use helix_mind_core::error::MindError;
use helix_mind_wal::WalEvent;

use crate::sqlite_pool::SqlitePool;

/// WAL → SQLite 投影器（无状态，纯函数式）。
pub struct WalProjector;

impl WalProjector {
    /// 应用单条 WAL 事件到 SQLite。
    pub fn apply(sqlite: &SqlitePool, event: &WalEvent) -> Result<(), MindError> {
        match event {
            WalEvent::NodeWritten(node) => sqlite.upsert_node(node),
            WalEvent::EdgeAdded(edge) => sqlite.upsert_edge(edge),
            WalEvent::NodeMarkedRecessive(id) => sqlite.mark_node_recessive(id),
            WalEvent::AuditWritten(entry) => sqlite.insert_audit(entry),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StorageEngine;
    use helix_mind_core::config::StorageConfig;
    use helix_mind_core::graph::{Edge, Node, NodeContent, NodeType, RelationType};
    use helix_mind_wal::WalWriter;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("helix_proj_{}_{}", name, std::process::id()))
    }

    fn temp_cfg(name: &str) -> StorageConfig {
        let dir = temp_dir(name);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        StorageConfig {
            sqlite_path: dir.join("test.db").to_string_lossy().into_owned(),
            node_cache_capacity: 100,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn project_node_edge_recessive_audit_roundtrip() {
        let cfg = temp_cfg("project_roundtrip");
        // 用 StorageEngine::new 建内存/文件 SQLite + schema（FTS5 探针已由 engine 启动保证）。
        let engine = StorageEngine::new(&cfg).await.unwrap();

        // 构造事件。
        let node = Node {
            content: NodeContent::Text("投影目标节点".into()),
            node_type: NodeType::L2,
            ..Default::default()
        };
        let node_id = node.id;

        let edge = Edge {
            source_id: node_id,
            target_id: uuid::Uuid::new_v4(),
            relation_type: RelationType::Semantic,
            weight: 0.5,
            is_soft: true,
        };
        let edge_target = edge.target_id;

        // 写 WAL（事实来源）。
        let wal_cfg = helix_mind_wal::WalConfig::new(temp_dir("project_wal"));
        let mut writer = WalWriter::open(&wal_cfg).unwrap();
        writer.append_synced(&WalEvent::NodeWritten(node.clone())).unwrap();
        writer.append_synced(&WalEvent::EdgeAdded(edge)).unwrap();
        writer.append_synced(&WalEvent::AuditWritten(helix_mind_core::AuditEntry {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type: helix_mind_core::AuditEventType::FederationNodeMerged,
            actor: "test".into(),
            details: "projector test".into(),
        })).unwrap();
        writer.append_synced(&WalEvent::NodeMarkedRecessive(node_id)).unwrap();
        drop(writer);

        // replay 全部事件（哈希链校验）+ 投影到 SQLite。
        let reader = helix_mind_wal::WalReader::new(&wal_cfg);
        let outcome = reader.replay_all().unwrap();
        assert_eq!(outcome.records.len(), 4, "4 events replayed with valid hash chain");

        let sqlite = &engine.sqlite;
        for rec in &outcome.records {
            WalProjector::apply(sqlite, &rec.event).unwrap();
        }

        // 读回验证：节点 upsert + 隐性标记。
        let stored = engine.get_nodes_by_ids(&[node_id]).await.unwrap();
        assert_eq!(stored.len(), 1);
        match (&stored[0].content, &node.content) {
            (NodeContent::Text(a), NodeContent::Text(b)) => assert_eq!(a, b),
            _ => panic!("expected text content match"),
        }
        assert!(stored[0].is_recessive, "recessive projected");

        // 边 upsert 投影。
        let conn = sqlite.get().unwrap();
        let cnt: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM edges WHERE source_id = ?1 AND target_id = ?2",
                rusqlite::params![node_id.to_string(), edge_target.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(cnt, 1, "edge projected");

        // 审计投影。
        let cnt: i64 = conn
            .query_row("SELECT COUNT(*) FROM audit_log", [], |r| r.get(0))
            .unwrap();
        assert_eq!(cnt, 1, "audit projected");
    }
}
