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

        // Initialize connection: enable WAL mode and optimizations
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

    pub fn get(&self) -> Result<PooledConnection<SqliteConnectionManager>, helix_mind_core::error::MindError> {
        self.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))
    }

    /// Execute operation in transaction, auto commit/rollback
    pub fn transactional_write<F, T>(&self, f: F) -> Result<T, helix_mind_core::error::MindError>
    where
        F: FnOnce(&rusqlite::Transaction) -> Result<T, helix_mind_core::error::MindError>,
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