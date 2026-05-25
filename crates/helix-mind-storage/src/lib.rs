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

    // ── Core CRUD ────────────────────────────────────────────────────

    pub async fn write_node(
        &self,
        node: Node,
        _priority: WritePriority,
    ) -> Result<(), helix_mind_core::error::MindError> {
        node.validate()?;
        let content_json =
            serde_json::to_string(&node.content).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let ledger_json =
            serde_json::to_string(&node.attribution_ledger).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let source_json =
            serde_json::to_string(&node.source).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let provenance = node.abstract_provenance.as_deref().unwrap_or("");
        let derived_json =
            serde_json::to_string(&node.derived_from).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        self.sqlite.transactional_write(|tx| {
            tx.execute(
                "INSERT INTO nodes (id, node_type, content, heat, is_hypothetical, is_recessive,
                 sensitivity, generation, created_at, last_accessed_at, access_count,
                 initial_impact, corrected_by, notes, dominance, utility, corroborations,
                 attribution_ledger, source, high_risk, abstract_provenance, derived_from)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22)
                 ON CONFLICT(id) DO UPDATE SET
                 content=excluded.content, heat=excluded.heat,
                 is_hypothetical=excluded.is_hypothetical, is_recessive=excluded.is_recessive,
                 last_accessed_at=excluded.last_accessed_at, access_count=excluded.access_count,
                 corrected_by=excluded.corrected_by, notes=excluded.notes,
                 dominance=excluded.dominance, utility=excluded.utility,
                 corroborations=excluded.corroborations, attribution_ledger=excluded.attribution_ledger,
                 source=excluded.source, high_risk=excluded.high_risk,
                 abstract_provenance=excluded.abstract_provenance, derived_from=excluded.derived_from",
                rusqlite::params![
                    node.id.to_string(),
                    node_type_str(&node.node_type),
                    content_json,
                    node.heat,
                    node.is_hypothetical,
                    node.is_recessive,
                    node.sensitivity.as_ref().map(|s| sensitivity_str(s)),
                    node.generation,
                    node.created_at.to_rfc3339(),
                    node.last_accessed_at.to_rfc3339(),
                    node.access_count,
                    node.initial_impact,
                    node.corrected_by.map(|id| id.to_string()),
                    node.notes.as_deref().unwrap_or(""),
                    node.dominance,
                    node.utility,
                    node.corroborations,
                    ledger_json,
                    source_json,
                    node.high_risk,
                    provenance,
                    derived_json,
                ],
            ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            Ok(())
        })?;

        // Update in-memory topology and cache
        {
            let mut topo = self.topology.write().await;
            topo.add_node(&node);
        }
        self.cache.put(node);
        Ok(())
    }

    pub async fn get_nodes_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut nodes = Vec::with_capacity(ids.len());
        let mut missing_ids = Vec::new();

        // Try cache first
        for id in ids {
            if let Some(node) = self.cache.get(id) {
                nodes.push(node);
            } else {
                missing_ids.push(*id);
            }
        }

        if missing_ids.is_empty() {
            return Ok(nodes);
        }

        // Fetch missing from SQLite
        let conn = self.sqlite.get()?;
        let placeholders: Vec<String> = missing_ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "SELECT id, node_type, content, heat, is_hypothetical, is_recessive,
             sensitivity, generation, created_at, last_accessed_at, access_count,
             initial_impact, corrected_by, notes, dominance, utility, corroborations,
             attribution_ledger, source, high_risk, abstract_provenance, derived_from
             FROM nodes WHERE id IN ({})",
            placeholders.join(",")
        );
        let mut stmt = conn.prepare(&sql).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let id_strings: Vec<String> = missing_ids.iter().map(|id| id.to_string()).collect();
        let params: Vec<&dyn rusqlite::types::ToSql> = id_strings.iter().map(|s| s as &dyn rusqlite::types::ToSql).collect();

        let rows = stmt.query_map(params.as_slice(), |row| {
            Ok(row_to_node(row))
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        for row in rows {
            let node = row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            self.cache.put(node.clone());
            nodes.push(node);
        }
        Ok(nodes)
    }

    pub async fn get_edges_between(
        &self,
        node_ids: &[Uuid],
    ) -> Result<Vec<Edge>, helix_mind_core::error::MindError> {
        if node_ids.is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.sqlite.get()?;
        let placeholders: Vec<String> = node_ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "SELECT source_id, target_id, weight, relation_type, is_soft
             FROM edges WHERE source_id IN ({}) OR target_id IN ({})",
            placeholders.join(","),
            placeholders.join(",")
        );
        let mut stmt = conn.prepare(&sql).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let id_strings: Vec<String> = node_ids.iter().map(|id| id.to_string()).collect();
        let mut params: Vec<String> = Vec::new();
        params.extend(id_strings.clone());
        params.extend(id_strings);
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|s| s as &dyn rusqlite::types::ToSql).collect();

        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(Edge {
                source_id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
                target_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                weight: row.get(2)?,
                relation_type: str_to_relation_type(&row.get::<_, String>(3)?),
                is_soft: row.get(4)?,
            })
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        let mut edges = Vec::new();
        for row in rows {
            edges.push(row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?);
        }
        Ok(edges)
    }

    pub async fn add_edge(&self, edge: &Edge) -> Result<(), helix_mind_core::error::MindError> {
        edge.validate()?;

        // Cycle check for hard edges
        if !edge.is_soft {
            let topo = self.topology.read().await;
            if topo.would_create_cycle(edge.source_id, edge.target_id) {
                return Err(helix_mind_core::error::MindError::CycleDetected {
                    conflict_nodes: vec![edge.source_id, edge.target_id],
                });
            }
        }

        self.sqlite.transactional_write(|tx| {
            tx.execute(
                "INSERT INTO edges (source_id, target_id, weight, relation_type, is_soft)
                 VALUES (?1,?2,?3,?4,?5)
                 ON CONFLICT(source_id, target_id, relation_type) DO UPDATE SET
                 weight=excluded.weight, is_soft=excluded.is_soft",
                rusqlite::params![
                    edge.source_id.to_string(),
                    edge.target_id.to_string(),
                    edge.weight,
                    relation_type_str(&edge.relation_type),
                    edge.is_soft,
                ],
            ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            Ok(())
        })?;

        let mut topo = self.topology.write().await;
        topo.add_edge(edge.source_id, edge.target_id, edge)?;
        Ok(())
    }

    // ── Heat / Recessive ─────────────────────────────────────────────

    pub async fn get_node_heat(&self, id: &Uuid) -> Result<f64, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let heat: f64 = conn.query_row(
            "SELECT heat FROM nodes WHERE id = ?1",
            rusqlite::params![id.to_string()],
            |row| row.get(0),
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(heat)
    }

    pub async fn mark_recessive(&self, node_id: &Uuid) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "UPDATE nodes SET is_recessive = 1 WHERE id = ?1",
            rusqlite::params![node_id.to_string()],
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let mut topo = self.topology.write().await;
        topo.mark_recessive(node_id);
        self.cache.invalidate(node_id);
        Ok(())
    }

    /// Update node utility weight (for DecayEngine).
    pub async fn update_node_utility(
        &self,
        node_id: &Uuid,
        new_utility: f64,
    ) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "UPDATE nodes SET utility = ?1 WHERE id = ?2",
            rusqlite::params![new_utility, node_id.to_string()],
        )
        .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Update node dominance score.
    pub async fn update_node_dominance(
        &self,
        node_id: &Uuid,
        new_dominance: f64,
    ) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "UPDATE nodes SET dominance = ?1 WHERE id = ?2",
            rusqlite::params![new_dominance, node_id.to_string()],
        )
        .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    pub async fn get_nodes_below_heat(
        &self,
        heat: f64,
        before: chrono::DateTime<Utc>,
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let mut stmt = conn.prepare(
            "SELECT id FROM nodes WHERE heat < ?1 AND last_accessed_at < ?2 AND is_recessive = 0 LIMIT 1000"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let rows = stmt.query_map(
            rusqlite::params![heat, before.to_rfc3339()],
            |row| row.get::<_, String>(0),
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let mut ids = Vec::new();
        for row in rows {
            let id_str = row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            if let Ok(uuid) = Uuid::parse_str(&id_str) {
                ids.push(uuid);
            }
        }
        Ok(ids)
    }

    pub async fn get_expired_recessives(
        &self,
        expired_before: chrono::DateTime<Utc>,
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let mut stmt = conn.prepare(
            "SELECT id FROM nodes WHERE is_recessive = 1 AND last_accessed_at < ?1 LIMIT 1000"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let rows = stmt.query_map(
            rusqlite::params![expired_before.to_rfc3339()],
            |row| row.get::<_, String>(0),
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let mut ids = Vec::new();
        for row in rows {
            let id_str = row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            if let Ok(uuid) = Uuid::parse_str(&id_str) {
                ids.push(uuid);
            }
        }
        Ok(ids)
    }

    pub async fn delete_recessive_index(&self, node_id: &Uuid) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "UPDATE nodes SET is_recessive = 2 WHERE id = ?1",
            rusqlite::params![node_id.to_string()],
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    // ── Retrieval (graph traversal) ──────────────────────────────────

    pub async fn skilled_retrieve(
        &self,
        start_ids: &[Uuid],
        beam_width: usize,
        weight_threshold: f64,
        energy_budget: u64,
        max_nodes: usize,
    ) -> Result<(Vec<Uuid>, bool, Option<String>), helix_mind_core::error::MindError> {
        let topo = self.topology.read().await;
        let result = topo.skilled_traverse(start_ids, beam_width, weight_threshold, energy_budget, max_nodes);
        Ok(result)
    }

    pub async fn anchor_retrieve(
        &self,
        start_ids: &[Uuid],
        _query_embedding: Option<Vec<f32>>,
        beam_width: usize,
        weight_threshold: f64,
        energy_budget: u64,
        max_nodes: usize,
    ) -> Result<(Vec<Uuid>, bool, Option<String>), helix_mind_core::error::MindError> {
        let topo = self.topology.read().await;
        let result = topo.anchor_traverse(start_ids, beam_width, weight_threshold, energy_budget, max_nodes);
        Ok(result)
    }

    pub async fn imagination_retrieve(
        &self,
        start_ids: &[Uuid],
        temperature: f64,
        energy_budget: u64,
        max_nodes: usize,
    ) -> Result<(Vec<Uuid>, bool, Option<String>), helix_mind_core::error::MindError> {
        let topo = self.topology.read().await;
        let result = topo.imagination_traverse(start_ids, temperature, energy_budget, max_nodes);
        Ok(result)
    }

    // ── Type queries ─────────────────────────────────────────────────

    pub async fn get_nodes_by_type(
        &self,
        node_type: NodeType,
    ) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let type_str = node_type_str(&node_type);
        let mut stmt = conn.prepare(
            "SELECT id, node_type, content, heat, is_hypothetical, is_recessive,
             sensitivity, generation, created_at, last_accessed_at, access_count,
             initial_impact, corrected_by, notes, dominance, utility, corroborations,
             attribution_ledger, source, high_risk, abstract_provenance, derived_from
             FROM nodes WHERE node_type = ?1 LIMIT 10000"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let rows = stmt.query_map(rusqlite::params![type_str], |row| Ok(row_to_node(row)))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?);
        }
        Ok(nodes)
    }

    pub async fn get_top_l3_for_crystallization(
        &self,
        limit: usize,
    ) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, node_type, content, heat, is_hypothetical, is_recessive,
             sensitivity, generation, created_at, last_accessed_at, access_count,
             initial_impact, corrected_by, notes, dominance, utility, corroborations,
             attribution_ledger, source, high_risk, abstract_provenance, derived_from
             FROM nodes WHERE node_type = 'L3' AND is_recessive = 0
             ORDER BY heat DESC LIMIT ?1"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let rows = stmt.query_map(rusqlite::params![limit as i64], |row| Ok(row_to_node(row)))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?);
        }
        Ok(nodes)
    }

    pub async fn lookup_l2_by_hash(
        &self,
        hash: &str,
    ) -> Result<Option<Node>, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, node_type, content, heat, is_hypothetical, is_recessive,
             sensitivity, generation, created_at, last_accessed_at, access_count,
             initial_impact, corrected_by, notes, dominance, utility, corroborations,
             attribution_ledger, source, high_risk, abstract_provenance, derived_from
             FROM nodes WHERE id = ?1 AND node_type = 'L2'"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let rows = stmt.query_map(rusqlite::params![hash], |row| Ok(row_to_node(row)))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        for row in rows {
            return Ok(Some(row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?));
        }
        Ok(None)
    }

    pub async fn insert_l2_content_index(
        &self,
        _hash: &str,
        _node_id: &Uuid,
    ) -> Result<(), helix_mind_core::error::MindError> {
        Ok(())
    }

    // ── Cognitive dissonance ────────────────────────────────────────

    pub async fn get_unresolved_dissonance(
        &self,
        _older_than: chrono::Duration,
    ) -> Result<Vec<(Uuid, Uuid)>, helix_mind_core::error::MindError> {
        Ok(Vec::new())
    }

    // ── Lifecycle ────────────────────────────────────────────────────

    pub async fn get_elapsed_days(&self) -> Result<u64, helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        let days: f64 = conn.query_row(
            "SELECT COALESCE(julianday('now') - julianday(MIN(created_at)), 0) FROM nodes",
            [],
            |row| row.get(0),
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(days as u64)
    }

    pub async fn archive_past_life(&self, generation: u64) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "UPDATE nodes SET is_recessive = 1 WHERE generation = ?1 AND node_type = 'L3'",
            rusqlite::params![generation],
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    pub async fn reset_self_portrait(&self) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "UPDATE nodes SET is_recessive = 1 WHERE node_type = 'L1'",
            [],
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    pub async fn record_inheritance_crystal_hash(
        &self,
        generation: u64,
        hash: &str,
    ) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.get()?;
        conn.execute(
            "INSERT OR REPLACE INTO life_records (generation, inheritance_crystal_hash)
             VALUES (?1, ?2)",
            rusqlite::params![generation, hash],
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        Ok(())
    }

    // ── Similarity ───────────────────────────────────────────────────

    pub async fn find_similar_node(
        &self,
        _node: &Node,
    ) -> Result<Option<Node>, helix_mind_core::error::MindError> {
        Ok(None)
    }

    // ── Bulk delete ──────────────────────────────────────────────────

    pub async fn delete_all_nodes_by_type(
        &self,
        _node_type: NodeType,
    ) -> Result<(), helix_mind_core::error::MindError> {
        Ok(())
    }

    pub async fn delete_social_graph(&self) -> Result<(), helix_mind_core::error::MindError> {
        Ok(())
    }

    pub async fn delete_user_profile(&self) -> Result<(), helix_mind_core::error::MindError> {
        Err(helix_mind_core::error::MindError::Validation(
            "User profile (CREATOR_IMPRINT) cannot be deleted".into(),
        ))
    }

    // ── SQLite access ────────────────────────────────────────────────

    pub async fn sqlite_get(
        &self,
    ) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>, helix_mind_core::error::MindError> {
        self.sqlite.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))
    }

    // ── Audit ────────────────────────────────────────────────────────

    pub async fn write_audit(
        &self,
        entry: &helix_mind_core::AuditEntry,
    ) -> Result<(), helix_mind_core::error::MindError> {
        let conn = self.sqlite.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        conn.execute(
            "INSERT INTO audit_log (event_id, timestamp, event_type, actor, details)
             VALUES (?1, ?2, ?3, ?4, ?5)",
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

    // ── Generation query ─────────────────────────────────────────────

    pub async fn get_l2_nodes_by_generation(
        &self,
        _generation: u64,
    ) -> Result<Vec<Node>, helix_mind_core::error::MindError> {
        self.get_nodes_by_type(NodeType::L2).await
    }

    // ── Stats ────────────────────────────────────────────────────────

    pub async fn get_stats(&self) -> Result<StorageStats, helix_mind_core::error::MindError> {
        let conn = self.sqlite.pool.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let total_nodes: u64 = conn.query_row("SELECT COUNT(*) FROM nodes", [], |row| row.get(0))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let total_edges: u64 = conn.query_row("SELECT COUNT(*) FROM edges", [], |row| row.get(0))
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let total_interactions: u64 = conn.query_row(
            "SELECT COALESCE(SUM(access_count),0) FROM nodes", [], |row| row.get(0),
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let elapsed_days: u64 = conn.query_row(
            "SELECT (julianday('now') - julianday(MIN(created_at))) FROM nodes", [], |row| row.get(0),
        ).unwrap_or(0) as u64;
        Ok(StorageStats { total_nodes, total_edges, total_interactions, elapsed_days })
    }
}

// ── Helper: row → Node ──────────────────────────────────────────────

fn row_to_node(row: &rusqlite::Row) -> Node {
    let id_str: String = row.get(0).unwrap_or_default();
    let node_type_str: String = row.get(1).unwrap_or_default();
    let content_json: String = row.get(2).unwrap_or_default();
    let ledger_json: String = row.get(17).unwrap_or_default();
    let source_json: String = row.get(18).unwrap_or_default();
    let derived_json: String = row.get(21).unwrap_or_default();

    Node {
        id: Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4()),
        node_type: str_to_node_type(&node_type_str),
        content: serde_json::from_str(&content_json).unwrap_or(NodeContent::Text(String::new())),
        heat: row.get(3).unwrap_or(0.5),
        is_hypothetical: row.get(4).unwrap_or(false),
        is_recessive: row.get(5).unwrap_or(false),
        sensitivity: row.get::<_, Option<String>>(6).unwrap_or(None).map(|s| str_to_sensitivity(&s)),
        generation: row.get(7).unwrap_or(1),
        created_at: parse_datetime(&row.get::<_, String>(8).unwrap_or_default()),
        last_accessed_at: parse_datetime(&row.get::<_, String>(9).unwrap_or_default()),
        access_count: row.get(10).unwrap_or(0),
        initial_impact: row.get(11).unwrap_or(0.5),
        corrected_by: row.get::<_, Option<String>>(12).unwrap_or(None)
            .and_then(|s| Uuid::parse_str(&s).ok()),
        notes: row.get::<_, Option<String>>(13).unwrap_or(None),
        dominance: row.get(14).unwrap_or(0.5),
        utility: row.get(15).unwrap_or(0.5),
        corroborations: row.get(16).unwrap_or(0),
        attribution_ledger: serde_json::from_str(&ledger_json).unwrap_or_default(),
        source: serde_json::from_str(&source_json).unwrap_or(NodeSource::Local),
        high_risk: row.get(19).unwrap_or(false),
        abstract_provenance: row.get::<_, Option<String>>(20).unwrap_or(None)
            .filter(|s| !s.is_empty()),
        derived_from: serde_json::from_str(&derived_json).unwrap_or_default(),
    }
}

fn parse_datetime(s: &str) -> chrono::DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// ── Enum → string helpers ───────────────────────────────────────────

fn node_type_str(nt: &NodeType) -> &'static str {
    match nt {
        NodeType::L0 => "L0",
        NodeType::L1 => "L1",
        NodeType::L2 => "L2",
        NodeType::L3 => "L3",
    }
}

fn str_to_node_type(s: &str) -> NodeType {
    match s {
        "L0" => NodeType::L0,
        "L1" => NodeType::L1,
        "L2" => NodeType::L2,
        _ => NodeType::L3,
    }
}

fn sensitivity_str(s: &Sensitivity) -> &'static str {
    match s {
        Sensitivity::Public => "Public",
        Sensitivity::Private => "Private",
        Sensitivity::Sensitive => "Sensitive",
    }
}

fn str_to_sensitivity(s: &str) -> Sensitivity {
    match s {
        "Public" => Sensitivity::Public,
        "Sensitive" => Sensitivity::Sensitive,
        _ => Sensitivity::Private,
    }
}

fn relation_type_str(rt: &RelationType) -> &'static str {
    match rt {
        RelationType::Causal => "Causal",
        RelationType::Semantic => "Semantic",
        RelationType::Temporal => "Temporal",
        RelationType::CoOccurrence => "CoOccurrence",
        RelationType::Corrects => "Corrects",
        RelationType::Refines => "Refines",
        RelationType::Doubts => "Doubts",
        RelationType::SimilarTo => "SimilarTo",
    }
}

fn str_to_relation_type(s: &str) -> RelationType {
    match s {
        "Causal" => RelationType::Causal,
        "Semantic" => RelationType::Semantic,
        "Temporal" => RelationType::Temporal,
        "CoOccurrence" => RelationType::CoOccurrence,
        "Corrects" => RelationType::Corrects,
        "Refines" => RelationType::Refines,
        "Doubts" => RelationType::Doubts,
        "SimilarTo" => RelationType::SimilarTo,
        _ => RelationType::Semantic,
    }
}