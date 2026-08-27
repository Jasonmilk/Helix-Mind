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
                derived_from TEXT NOT NULL DEFAULT '[]',
                phase_state TEXT NOT NULL DEFAULT 'Liquid',
                subject_dependency TEXT NOT NULL DEFAULT 'High',
                concentration TEXT NOT NULL DEFAULT 'Dissolved',
                tension REAL NOT NULL DEFAULT 0.0
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

        // P0 (ADR-0011/0012): append-only schema migration for existing DBs.
        self.migrate_nodes_phase_columns(&conn)?;
        Ok(())
    }

    /// Idempotent append-only migration (ADR-0012): adds phase-state columns if
    /// missing, then materializes subject_dependency from node_type exactly once
    /// (L2→Low, else High). Runtime nodes are never re-derived (ADR-0011).
    fn migrate_nodes_phase_columns(
        &self,
        conn: &rusqlite::Connection,
    ) -> Result<(), helix_mind_core::error::MindError> {
        let mut stmt = conn
            .prepare("PRAGMA table_info(nodes)")
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let existing: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?
            .collect::<Result<_, _>>()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        let adds = [
            ("phase_state", "TEXT NOT NULL DEFAULT 'Liquid'"),
            ("subject_dependency", "TEXT NOT NULL DEFAULT 'High'"),
            ("concentration", "TEXT NOT NULL DEFAULT 'Dissolved'"),
            ("tension", "REAL NOT NULL DEFAULT 0.0"),
        ];
        let mut added_any = false;
        for (name, ddl) in adds {
            if !existing.iter().any(|c| c == name) {
                conn.execute_batch(&format!("ALTER TABLE nodes ADD COLUMN {name} {ddl};"))
                    .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
                added_any = true;
            }
        }

        // One-time materialized backfill (only right after columns were added):
        // subject_dependency is derived from node_type (L2→Low, else High).
        if added_any {
            conn.execute_batch(
                "UPDATE nodes SET subject_dependency =
                     CASE WHEN node_type = 'L2' THEN 'Low' ELSE 'High' END;",
            )
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        }

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

#[cfg(test)]
mod tests {
    use helix_mind_core::config::StorageConfig;
    use crate::StorageEngine;

    #[tokio::test]
    async fn migrate_legacy_db_backfills_phase_columns() {
        // Build a legacy (pre-P0, 22-column) schema DB by hand, with one L2 and one L3 row.
        let dir = std::env::temp_dir().join(format!("helix_mig_{}.db", uuid::Uuid::new_v4()));
        let path_str = dir.to_string_lossy().to_string();

        {
            let conn = rusqlite::Connection::open(&path_str).unwrap();
            conn.execute_batch(
                "CREATE TABLE nodes (
                    id TEXT PRIMARY KEY, node_type TEXT NOT NULL, content TEXT NOT NULL,
                    heat REAL NOT NULL DEFAULT 0.5, is_hypothetical INTEGER NOT NULL DEFAULT 0,
                    is_recessive INTEGER NOT NULL DEFAULT 0, sensitivity TEXT,
                    generation INTEGER NOT NULL DEFAULT 0, created_at TEXT NOT NULL,
                    last_accessed_at TEXT NOT NULL DEFAULT '1970-01-01T00:00:00Z',
                    access_count INTEGER NOT NULL DEFAULT 0,
                    initial_impact REAL NOT NULL DEFAULT 0.5,
                    corrected_by TEXT, notes TEXT, dominance REAL NOT NULL DEFAULT 0.5,
                    utility REAL NOT NULL DEFAULT 0.5, corroborations INTEGER NOT NULL DEFAULT 0,
                    attribution_ledger TEXT NOT NULL DEFAULT '[]',
                    source TEXT NOT NULL DEFAULT 'Local', high_risk INTEGER NOT NULL DEFAULT 0,
                    abstract_provenance TEXT NOT NULL DEFAULT '', derived_from TEXT NOT NULL DEFAULT '[]'
                );",
            )
            .unwrap();
            conn.execute(
                "INSERT INTO nodes (id, node_type, content, created_at)
                 VALUES ('l2-1', 'L2', 'legacy l2', '2026-01-01T00:00:00Z'),
                        ('l3-1', 'L3', 'legacy l3', '2026-01-01T00:00:00Z')",
                [],
            )
            .unwrap();
        }

        // Open through StorageEngine::new — runs ensure_schema + migration (ADR-0012).
        let config = StorageConfig {
            sqlite_path: path_str.clone(),
            ..Default::default()
        };
        let _engine = StorageEngine::new(&config).await.unwrap();

        // Verify the append-only columns were added and materialized once.
        let conn = rusqlite::Connection::open(&path_str).unwrap();
        let phase: String = conn
            .query_row("SELECT phase_state FROM nodes WHERE id = 'l3-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(phase, "Liquid");
        let dep_l2: String = conn
            .query_row("SELECT subject_dependency FROM nodes WHERE id = 'l2-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(dep_l2, "Low");
        let dep_l3: String = conn
            .query_row("SELECT subject_dependency FROM nodes WHERE id = 'l3-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(dep_l3, "High");
        let conc: String = conn
            .query_row("SELECT concentration FROM nodes WHERE id = 'l3-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(conc, "Dissolved");
        let tension: f64 = conn
            .query_row("SELECT tension FROM nodes WHERE id = 'l3-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(tension, 0.0);

        drop(conn);
        let _ = std::fs::remove_file(&path_str);
    }
}
