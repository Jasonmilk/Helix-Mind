pub mod sqlite_pool;
pub mod topology;
pub mod parquet_store;
pub mod deep_cold;
pub mod human_view;
pub mod node_cache;
pub mod deferred_writer;
pub mod codec;
pub mod engine;

use helix_mind_core::config::StorageConfig;
use sqlite_pool::SqlitePool;
use topology::MemoryTopology;
use node_cache::NodeCache;
use deferred_writer::DeferredWriter;
use std::sync::Arc;
use tokio::sync::RwLock;
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
    #[allow(dead_code)]
    deferred_writer: DeferredWriter,
}

pub enum WritePriority {
    Critical,
    Deferred,
}

impl StorageEngine {
    pub async fn new(config: &StorageConfig) -> Result<Arc<Self>, helix_mind_core::error::MindError> {
        let sqlite = SqlitePool::new(&config.sqlite_path)?;
        sqlite.ensure_schema()?;
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
}
