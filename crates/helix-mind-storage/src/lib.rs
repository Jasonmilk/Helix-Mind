pub mod sqlite_pool;
pub mod topology;
pub mod parquet_store;
pub mod deep_cold;
pub mod human_view;
pub mod node_cache;
pub mod deferred_writer;
pub mod codec;
pub mod engine;
pub mod fts;
pub mod wal_projector;

use helix_mind_core::config::StorageConfig;
use fts::FtsCommand;
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
    /// Async FTS5 index-maintenance queue (P1 M-01, ADR-0013).
    pub fts_tx: tokio::sync::mpsc::UnboundedSender<FtsCommand>,
    /// P6 (ADR-0015): 事实来源 WAL 写入器（单写入者串行）。
    /// `None` = 未启用（`:memory:` 库自动禁用，或 wal_enabled=false）。
    pub wal: tokio::sync::Mutex<Option<helix_mind_wal::WalWriter>>,
}

pub enum WritePriority {
    Critical,
    Deferred,
}

impl StorageEngine {
    pub async fn new(config: &StorageConfig) -> Result<Arc<Self>, helix_mind_core::error::MindError> {
        let sqlite = SqlitePool::new(&config.sqlite_path)?;
        sqlite.ensure_schema()?;
        // FTS5 projection: create table (in ensure_schema) then rebuild from truth source.
        fts::rebuild_fts(&sqlite)?;
        let topology = MemoryTopology::rebuild_from_sqlite(&sqlite)?;
        let topology = Arc::new(RwLock::new(topology));
        let cache = NodeCache::new(config.node_cache_capacity);
        let deferred_writer = DeferredWriter::new();
        let (fts_tx, fts_rx) = tokio::sync::mpsc::unbounded_channel();
        let worker_pool = sqlite.clone();
        tokio::spawn(fts::run_fts_worker(worker_pool, fts_rx));
        // P6 (ADR-0015): 初始化事实来源 WAL。`:memory:` 库自动禁用。
        let wal = if config.wal_enabled && config.sqlite_path != ":memory:" {
            let wal_cfg = helix_mind_wal::WalConfig::new(&config.wal_dir);
            let writer = helix_mind_wal::WalWriter::open(&wal_cfg)
                .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            Some(writer)
        } else {
            None
        };
        let engine = Arc::new(Self {
            config: config.clone(),
            sqlite,
            topology,
            cache,
            deferred_writer,
            fts_tx,
            wal: tokio::sync::Mutex::new(wal),
        });
        Ok(engine)
    }

    /// Barrier: flush all pending FTS index ops, then return. Deterministic
    /// completion for tests and for callers that need read-your-writes.
    pub async fn flush_fts_index(&self) -> Result<(), helix_mind_core::error::MindError> {
        let (ack, rx) = tokio::sync::oneshot::channel();
        let _ = self.fts_tx.send(FtsCommand::Flush(ack));
        rx.await
            .map_err(|_| helix_mind_core::error::MindError::Storage("fts flush ack dropped".into()))
    }
}
