use helix_mind_core::graph::{Node, NodeType, Edge, RelationType};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use uuid::Uuid;
use super::sqlite_pool::SqlitePool;

#[derive(Debug, Clone)]
pub struct TopoNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub is_recessive: bool,
}

#[derive(Debug, Clone)]
pub struct TopoEdge {
    pub weight: f64,
    pub relation_type: RelationType,
    pub is_soft: bool,
}

#[derive(PartialEq)]
pub struct ScoredNode {
    pub id: Uuid,
    pub score: f64,
}

impl Eq for ScoredNode {}

impl PartialOrd for ScoredNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct MemoryTopology {
    pub graph: DiGraph<TopoNode, TopoEdge>,
    pub id_to_index: HashMap<Uuid, NodeIndex>,
    pub index_to_id: HashMap<NodeIndex, Uuid>,
}

impl MemoryTopology {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            id_to_index: HashMap::new(),
            index_to_id: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &Node) {
        if self.id_to_index.contains_key(&node.id) {
            return;
        }
        let topo = TopoNode {
            id: node.id,
            node_type: node.node_type.clone(),
            is_recessive: node.is_recessive,
        };
        let idx = self.graph.add_node(topo);
        self.id_to_index.insert(node.id, idx);
        self.index_to_id.insert(idx, node.id);
    }

    pub fn add_edge(
        &mut self,
        source: Uuid,
        target: Uuid,
        edge: &Edge,
    ) -> Result<(), helix_mind_core::error::MindError> {
        let src_idx = *self.id_to_index.get(&source).ok_or_else(|| {
            helix_mind_core::error::MindError::NotFound(format!("Source node not found: {}", source))
        })?;
        let tgt_idx = *self.id_to_index.get(&target).ok_or_else(|| {
            helix_mind_core::error::MindError::NotFound(format!("Target node not found: {}", target))
        })?;
        let topo = TopoEdge {
            weight: edge.weight,
            relation_type: edge.relation_type.clone(),
            is_soft: edge.is_soft,
        };
        self.graph.add_edge(src_idx, tgt_idx, topo);
        Ok(())
    }

    pub fn mark_recessive(&mut self, node_id: &Uuid) {
        if let Some(idx) = self.id_to_index.get(node_id) {
            if let Some(node) = self.graph.node_weight_mut(*idx) {
                node.is_recessive = true;
            }
        }
    }

    pub fn remove_node(&mut self, node_id: &Uuid) {
        if let Some(idx) = self.id_to_index.remove(node_id) {
            self.index_to_id.remove(&idx);
            self.graph.remove_node(idx);
        }
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Cycle detection: would adding edge A' → A create a cycle?
    /// Correct algorithm (v3.3): BFS from `target` (A), check if `source` (A') is reachable.
    pub fn would_create_cycle(&self, source: Uuid, target: Uuid) -> bool {
        let target_idx = match self.id_to_index.get(&target) {
            Some(idx) => *idx,
            None => return false, // target not in graph, no cycle
        };
        let source_idx = match self.id_to_index.get(&source) {
            Some(idx) => *idx,
            None => return false, // source not in graph, no cycle
        };

        // BFS from target → check if source is reachable
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push(target_idx);
        visited.insert(target_idx);

        while let Some(current) = queue.pop() {
            if current == source_idx {
                return true; // cycle detected: target → ... → source already exists
            }
            for edge in self.graph.edges(current) {
                let neighbor = edge.target();
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push(neighbor);
                }
            }
        }
        false
    }

    /// BFS reachable nodes within max_depth hops (only non-recessive, non-soft edges)
    pub fn bfs_reachable(&self, start: Uuid, max_depth: u8) -> Vec<Uuid> {
        let start_idx = match self.id_to_index.get(&start) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue: Vec<(NodeIndex, u8)> = Vec::new();
        queue.push((start_idx, 0));
        visited.insert(start_idx);

        while let Some((current, depth)) = queue.pop() {
            if let Some(id) = self.index_to_id.get(&current) {
                result.push(*id);
            }
            if depth >= max_depth {
                continue;
            }
            for edge in self.graph.edges(current) {
                let neighbor = edge.target();
                if visited.contains(&neighbor) {
                    continue;
                }
                // Skip recessive nodes for dominant traversal
                if let Some(node) = self.graph.node_weight(neighbor) {
                    if node.is_recessive {
                        continue;
                    }
                }
                visited.insert(neighbor);
                queue.push((neighbor, depth + 1));
            }
        }
        result
    }

    // ── Traversal methods ────────────────────────────────────────────

    /// Skilled mode: Beam Search along high-weight edges only (non-soft, weight >= threshold)
    pub fn skilled_traverse(
        &self,
        start_ids: &[Uuid],
        beam_width: usize,
        weight_threshold: f64,
        energy_budget: u64,
        max_nodes: usize,
    ) -> (Vec<Uuid>, bool, Option<String>) {
        let mut visited = HashSet::new();
        let mut result_ids = Vec::new();
        let mut energy_remaining = energy_budget;
        let mut heap = BinaryHeap::new();

        // Seed with start nodes
        for id in start_ids {
            if let Some(idx) = self.id_to_index.get(id) {
                visited.insert(*idx);
                result_ids.push(*id);
                heap.push(ScoredNode { id: *id, score: 1.0 });
            }
        }

        while let Some(current) = heap.pop() {
            if result_ids.len() >= max_nodes || energy_remaining == 0 {
                break;
            }
            let current_idx = match self.id_to_index.get(&current.id) {
                Some(idx) => *idx,
                None => continue,
            };

            let mut candidates: Vec<(Uuid, f64)> = Vec::new();
            for edge in self.graph.edges(current_idx) {
                if edge.weight().is_soft {
                    continue;
                }
                if edge.weight().weight < weight_threshold {
                    continue;
                }
                let neighbor_idx = edge.target();
                if visited.contains(&neighbor_idx) {
                    continue;
                }
                if let Some(node) = self.graph.node_weight(neighbor_idx) {
                    if node.is_recessive {
                        continue;
                    }
                }
                if let Some(neighbor_id) = self.index_to_id.get(&neighbor_idx) {
                    candidates.push((*neighbor_id, edge.weight().weight));
                }
            }

            // Sort by weight descending, keep top beam_width
            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            for (neighbor_id, _weight) in candidates.iter().take(beam_width) {
                if let Some(idx) = self.id_to_index.get(neighbor_id) {
                    visited.insert(*idx);
                }
                result_ids.push(*neighbor_id);
                heap.push(ScoredNode { id: *neighbor_id, score: *_weight });
                if energy_remaining > 0 {
                    energy_remaining -= 1;
                }
            }
        }

        let is_partial = energy_remaining == 0;
        let reason = if is_partial {
            Some("energy budget exhausted".to_string())
        } else {
            None
        };
        (result_ids, is_partial, reason)
    }

    /// Anchor mode: graph diffusion + all edges (including soft, lower threshold)
    pub fn anchor_traverse(
        &self,
        start_ids: &[Uuid],
        beam_width: usize,
        weight_threshold: f64,
        energy_budget: u64,
        max_nodes: usize,
    ) -> (Vec<Uuid>, bool, Option<String>) {
        let mut visited = HashSet::new();
        let mut result_ids = Vec::new();
        let mut energy_remaining = energy_budget;
        let mut heap = BinaryHeap::new();

        for id in start_ids {
            if let Some(idx) = self.id_to_index.get(id) {
                visited.insert(*idx);
                result_ids.push(*id);
                heap.push(ScoredNode { id: *id, score: 1.0 });
            }
        }

        let decay_factor: f64 = 0.8; // soft edge decay

        while let Some(current) = heap.pop() {
            if result_ids.len() >= max_nodes || energy_remaining == 0 {
                break;
            }
            let current_idx = match self.id_to_index.get(&current.id) {
                Some(idx) => *idx,
                None => continue,
            };

            let mut candidates: Vec<(Uuid, f64)> = Vec::new();
            for edge in self.graph.edges(current_idx) {
                let effective_weight = if edge.weight().is_soft {
                    edge.weight().weight * decay_factor
                } else {
                    edge.weight().weight
                };
                if effective_weight < weight_threshold {
                    continue;
                }
                let neighbor_idx = edge.target();
                if visited.contains(&neighbor_idx) {
                    continue;
                }
                if let Some(node) = self.graph.node_weight(neighbor_idx) {
                    if node.is_recessive {
                        continue;
                    }
                }
                if let Some(neighbor_id) = self.index_to_id.get(&neighbor_idx) {
                    candidates.push((*neighbor_id, effective_weight));
                }
            }

            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            for (neighbor_id, _weight) in candidates.iter().take(beam_width) {
                if let Some(idx) = self.id_to_index.get(neighbor_id) {
                    visited.insert(*idx);
                }
                result_ids.push(*neighbor_id);
                heap.push(ScoredNode { id: *neighbor_id, score: *_weight });
                if energy_remaining > 0 {
                    energy_remaining -= 1;
                }
            }
        }

        let is_partial = energy_remaining == 0;
        let reason = if is_partial {
            Some("energy budget exhausted".to_string())
        } else {
            None
        };
        (result_ids, is_partial, reason)
    }

    /// Imagination mode: chaotic walk, no edge filtering, high temperature
    pub fn imagination_traverse(
        &self,
        start_ids: &[Uuid],
        _temperature: f64,
        energy_budget: u64,
        max_nodes: usize,
    ) -> (Vec<Uuid>, bool, Option<String>) {
        let mut visited = HashSet::new();
        let mut result_ids = Vec::new();
        let mut energy_remaining = energy_budget;

        for id in start_ids {
            if let Some(idx) = self.id_to_index.get(id) {
                visited.insert(*idx);
                result_ids.push(*id);
            }
        }

        // For now, simple BFS with no weight filtering (full exploration)
        let mut queue: Vec<NodeIndex> = start_ids
            .iter()
            .filter_map(|id| self.id_to_index.get(id).copied())
            .collect();

        while let Some(current) = queue.pop() {
            if result_ids.len() >= max_nodes || energy_remaining == 0 {
                break;
            }
            for edge in self.graph.edges(current) {
                let neighbor = edge.target();
                if visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                if let Some(id) = self.index_to_id.get(&neighbor) {
                    result_ids.push(*id);
                }
                queue.push(neighbor);
                if energy_remaining > 0 {
                    energy_remaining -= 1;
                }
                if result_ids.len() >= max_nodes || energy_remaining == 0 {
                    break;
                }
            }
        }

        let is_partial = energy_remaining == 0;
        let reason = if is_partial {
            Some("energy budget exhausted".to_string())
        } else {
            None
        };
        (result_ids, is_partial, reason)
    }

    // ── Rebuild from SQLite ──────────────────────────────────────────

    pub fn rebuild_from_sqlite(sqlite: &SqlitePool) -> Result<Self, helix_mind_core::error::MindError> {
        let mut topology = Self::new();
        let conn = sqlite.get()?;

        // Load nodes
        let mut stmt = conn.prepare(
            "SELECT id, node_type, is_recessive FROM nodes"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let node_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
            ))
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        for row in node_iter {
            let (id_str, type_str, is_recessive) = row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let node_type = match type_str.as_str() {
                "L0" => NodeType::L0,
                "L1" => NodeType::L1,
                "L2" => NodeType::L2,
                _ => NodeType::L3,
            };
            let topo = TopoNode { id, node_type, is_recessive };
            let idx = topology.graph.add_node(topo);
            topology.id_to_index.insert(id, idx);
            topology.index_to_id.insert(idx, id);
        }

        // Load edges
        let mut stmt = conn.prepare(
            "SELECT source_id, target_id, weight, relation_type, is_soft FROM edges"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let edge_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, bool>(4)?,
            ))
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        for row in edge_iter {
            let (src_str, tgt_str, weight, rel_str, is_soft) = row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            let source = Uuid::parse_str(&src_str).unwrap_or_else(|_| Uuid::new_v4());
            let target = Uuid::parse_str(&tgt_str).unwrap_or_else(|_| Uuid::new_v4());
            let relation_type = match rel_str.as_str() {
                "Causal" => RelationType::Causal,
                "Semantic" => RelationType::Semantic,
                "Temporal" => RelationType::Temporal,
                "CoOccurrence" => RelationType::CoOccurrence,
                "Corrects" => RelationType::Corrects,
                "Refines" => RelationType::Refines,
                "Doubts" => RelationType::Doubts,
                "SimilarTo" => RelationType::SimilarTo,
                _ => RelationType::Semantic,
            };

            if let (Some(&src_idx), Some(&tgt_idx)) = (topology.id_to_index.get(&source), topology.id_to_index.get(&target)) {
                let topo = TopoEdge { weight, relation_type, is_soft };
                topology.graph.add_edge(src_idx, tgt_idx, topo);
            }
        }

        Ok(topology)
    }
}
