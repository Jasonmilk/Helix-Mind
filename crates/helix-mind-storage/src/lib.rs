pub mod sqlite_pool;
pub mod topology;
pub mod parquet_store;
pub mod deep_cold;
pub mod human_view;
pub mod node_cache;
pub mod deferred_writer;

use helix_mind_core::graph::*;
use helix_mind_core::config::StorageConfig;
use sqlite_pool::SqlitePool;
use topology::MemoryTopology;
use node_cache::NodeCache;
use deferred_writer::DeferredWriter;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StorageStats {
    pub total_nodes: u64,
    pub total_edges: u64,
    pub total_interactions: u64,
    pub elapsed_days: u64,
}

pub struct StorageEngine {
    pub config: StorageConfig,
    pub sqlite: SqlitePool,
    pub topology: Arc<RwLock<MemoryTopology>>,
    pub cache: NodeCache,
    deferred_writer: DeferredWriter,
}

pub enum WritePriority {
    Critical,
    Deferred,
}

impl StorageEngine {
    pub async fn new(config: &StorageConfig) -> Result<Arc<Self>, helix_mind_core::error::MindError> {
        let sqlite = SqlitePool::new(&config.sqlite_path)?;
        let topology = MemoryTopology::rebuild_from_sqlite(&sqlite)?;
        let topology = Arc::new(RwLock::new(topology));
        let cache = NodeCache::new(config.node_cache_capacity);
        let deferred_writer = DeferredWriter::new();

        let engine = Arc::new(Self {
            config: config.clone(),
            sqlite,
            topology,
            cache,
            deferred_writer,
        });

        Ok(engine)
    }

    pub async fn write_node(&self, _node: Node, _priority: WritePriority) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_nodes_by_ids(&self, _ids: &[Uuid]) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_edges_between(&self, _ids: &[Uuid]) -> Result<Vec<Edge>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_node_heat(&self, _id: &Uuid) -> Result<f64, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn skilled_retrieve(
        &self,
        _start_ids: &[Uuid],
        _beam_width: usize,
        _weight_threshold: f64,
        _energy_budget: u64,
        _max_nodes: usize,
    ) -> Result<(Vec<Uuid>, bool, Option<String>), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn anchor_retrieve(
        &self,
        _start_ids: &[Uuid],
        _query_embedding: Option<Vec<f32>>,
        _beam_width: usize,
        _weight_threshold: f64,
        _energy_budget: u64,
        _max_nodes: usize,
    ) -> Result<(Vec<Uuid>, bool, Option<String>), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn imagination_retrieve(
        &self,
        _start_ids: &[Uuid],
        _temperature: f64,
        _energy_budget: u64,
        _max_nodes: usize,
    ) -> Result<(Vec<Uuid>, bool, Option<String>), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_nodes_by_type(&self, _node_type: NodeType) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_top_l3_for_crystallization(&self, _limit: usize) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn lookup_l2_by_hash(&self, _hash: &str) -> Result<Option<Uuid>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn insert_l2_content_index(&self, _hash: &str, _node_id: &Uuid) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn mark_recessive(&self, _node_id: &Uuid) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_nodes_below_heat(&self, _heat: f64, _before: chrono::DateTime<Utc>) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_expired_recessives(&self, _expired_before: chrono::DateTime<Utc>) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn delete_recessive_index(&self, _node_id: &Uuid) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_unresolved_dissonance(&self, _older_than: chrono::Duration) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn get_elapsed_days(&self) -> Result<u64, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn archive_past_life(&self, _generation: u64) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn reset_self_portrait(&self) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn record_inheritance_crystal_hash(&self, _generation: u64, _hash: &str) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn find_similar_node(&self, _node: &Node) -> Result<Option<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn add_edge(&self, _edge: &Edge) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn delete_all_nodes_by_type(&self, _node_type: NodeType) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn delete_social_graph(&self) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn delete_user_profile(&self) -> Result<(), helix_mind_core::error::MindError> {
        todo!()
    }

    pub async fn sqlite_get(&self) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>, helix_mind_core::error::MindError> {
        self.sqlite.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))
    }

    pub async fn write_audit(&self, entry: &helix_mind_core::AuditEntry) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        
        conn.execute(
            "INSERT INTO audit_log (event_id, timestamp, event_type, actor, details) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                entry.event_id.to_string(),
                entry.timestamp.to_rfc3339(),
                format!("{:?}", entry.event_type),
                entry.actor,
                entry.details,
            ],
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        
        Ok(())
    }

    pub async fn get_l2_nodes_by_generation(&self, _generation: u64) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        todo!()
    }

    /// Return overall statistics for CLI and health checks
    pub async fn get_stats(&self) -> Result<StorageStats, helix_mind_core::error::MindError> {
        let conn = self.sqlite.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let total_nodes: u64 = conn.query_row("SELECT COUNT(*) FROM nodes", [], |row| row.get(0))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let total_edges: u64 = conn.query_row("SELECT COUNT(*) FROM edges", [], |row| row.get(0))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let total_interactions: u64 = conn.query_row("SELECT COALESCE(SUM(access_count),0) FROM nodes", [], |row| row.get(0))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let elapsed_days: u64 = conn.query_row("SELECT (julianday('now') - julianday(MIN(created_at))) FROM nodes", [], |row| row.get(0))
            .unwrap_or(0) as u64;

        Ok(StorageStats {
            total_nodes,
            total_edges,
            total_interactions,
            elapsed_days,
        })
    }
}