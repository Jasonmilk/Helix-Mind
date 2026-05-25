use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

#[derive(Clone)]
pub struct SqlitePool {
    pub pool: Pool<SqliteConnectionManager>,
}

impl SqlitePool {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, helix_mind_core::error::MindError> {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::builder()
            .max_size(10)
            .min_idle(Some(2))
            .build(manager)
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        let conn = pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA busy_timeout=5000;
             PRAGMA cache_size=-64000;
             PRAGMA foreign_keys=ON;
             PRAGMA mmap_size=268435456;"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(Self { pool })
    }

    /// Create tables if they don't exist. Idempotent.
    pub fn ensure_schema(&self) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                node_type TEXT NOT NULL DEFAULT 'L3',
                content TEXT NOT NULL DEFAULT '{}',
                heat REAL NOT NULL DEFAULT 0.5,
                is_hypothetical INTEGER NOT NULL DEFAULT 0,
                is_recessive INTEGER NOT NULL DEFAULT 0,
                sensitivity TEXT,
                generation INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                last_accessed_at TEXT NOT NULL,
                access_count INTEGER NOT NULL DEFAULT 0,
                initial_impact REAL NOT NULL DEFAULT 0.5,
                corrected_by TEXT,
                notes TEXT,
                dominance REAL NOT NULL DEFAULT 0.5,
                utility REAL NOT NULL DEFAULT 0.5,
                corroborations INTEGER NOT NULL DEFAULT 0,
                attribution_ledger TEXT NOT NULL DEFAULT '[]',
                source TEXT NOT NULL DEFAULT '\"Local\"',
                high_risk INTEGER NOT NULL DEFAULT 0,
                abstract_provenance TEXT NOT NULL DEFAULT '',
                derived_from TEXT NOT NULL DEFAULT '[]'
            );

            CREATE TABLE IF NOT EXISTS edges (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                weight REAL NOT NULL DEFAULT 0.5,
                relation_type TEXT NOT NULL DEFAULT 'Semantic',
                is_soft INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (source_id, target_id, relation_type)
            );

            CREATE TABLE IF NOT EXISTS audit_log (
                event_id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                event_type TEXT NOT NULL,
                actor TEXT NOT NULL DEFAULT '',
                details TEXT NOT NULL DEFAULT ''
            );

            CREATE TABLE IF NOT EXISTS life_records (
                generation INTEGER PRIMARY KEY,
                inheritance_crystal_hash TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_nodes_type ON nodes(node_type);
            CREATE INDEX IF NOT EXISTS idx_nodes_heat ON nodes(heat);
            CREATE INDEX IF NOT EXISTS idx_nodes_recessive ON nodes(is_recessive);
            CREATE INDEX IF NOT EXISTS idx_nodes_generation ON nodes(generation);
            CREATE INDEX IF NOT EXISTS idx_edges_source ON edges(source_id);
            CREATE INDEX IF NOT EXISTS idx_edges_target ON edges(target_id);
            "
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    pub fn get(&self) -> Result<PooledConnection<SqliteConnectionManager>, helix_mind_core::error::MindError> {
        self.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))
    }

    /// Execute an operation within an ACID transaction. Auto-commit on success, rollback on error.
    pub fn transactional_write<F, R>(&self, f: F) -> Result<R, helix_mind_core::error::MindError>
    where
        F: FnOnce(&rusqlite::Transaction) -> Result<R, helix_mind_core::error::MindError>,
    {
        let mut conn = self.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let tx = conn.transaction()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        match f(&tx) {
            Ok(result) => {
                tx.commit()
                    .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
                Ok(result)
            }
            Err(e) => {
                let _ = tx.rollback();
                Err(e)
            }
        }
    }
}
