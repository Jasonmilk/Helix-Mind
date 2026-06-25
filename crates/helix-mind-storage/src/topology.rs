use helix_mind_core::graph::{Node, NodeType, Edge, RelationType, NodeContent};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use uuid::Uuid;
use super::sqlite_pool::SqlitePool;

#[derive(Debug, Clone)]
pub struct TopoNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub is_recessive: bool,
    pub domain: Option<String>,
    pub k_core: usize,
}

#[derive(Debug, Clone)]
pub struct TopoEdge {
    pub weight: f64,
    pub relation_type: RelationType,
    pub is_soft: bool,
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
        
        // Dynamically extract the domain metadata from the structured NodeContent (HashMap<String, String>)
        let domain = match &node.content {
            NodeContent::Structured(map) => {
                map.get("domain").cloned()
            }
            _ => None,
        };

        let topo = TopoNode {
            id: node.id,
            node_type: node.node_type.clone(),
            is_recessive: node.is_recessive,
            domain,
            k_core: 0,
        };
        let idx = self.graph.add_node(topo);
        self.id_to_index.insert(node.id, idx);
        self.index_to_id.insert(idx, node.id);

        // Incrementally recalculate k-core metrics for all nodes to maintain consistency
        let cores = self.compute_k_cores();
        for node_idx in self.graph.node_indices() {
            if let Some(n) = self.graph.node_weight_mut(node_idx) {
                n.k_core = *cores.get(&n.id).unwrap_or(&0);
            }
        }
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
    /// BFS from `target` (A), check if `source` (A') is reachable.
    pub fn would_create_cycle(&self, source: Uuid, target: Uuid) -> bool {
        let target_idx = match self.id_to_index.get(&target) {
            Some(idx) => *idx,
            None => return false,
        };
        let source_idx = match self.id_to_index.get(&source) {
            Some(idx) => *idx,
            None => return false,
        };

        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push(target_idx);
        visited.insert(target_idx);

        while let Some(current) = queue.pop() {
            if current == source_idx {
                return true;
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

    // ── O(V+E) Bucket Sort K-Core Decomposition ──────────────────────

    /// Computes the exact core number for each node in linear time.
    /// Completely deterministic and optimized for memory-constrained hardware.
    pub fn compute_k_cores(&self) -> HashMap<Uuid, usize> {
        let mut degrees: HashMap<Uuid, usize> = HashMap::new();
        let mut adj: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let mut max_deg = 0;

        for node_idx in self.graph.node_indices() {
            if let Some(node) = self.graph.node_weight(node_idx) {
                degrees.insert(node.id, 0);
                adj.insert(node.id, Vec::new());
            }
        }

        // Build undirected representations for core calculation
        for edge_idx in self.graph.edge_indices() {
            if let Some((src_idx, tgt_idx)) = self.graph.edge_endpoints(edge_idx) {
                if let (Some(&src_id), Some(&tgt_id)) = (self.index_to_id.get(&src_idx), self.index_to_id.get(&tgt_idx)) {
                    adj.entry(src_id).or_default().push(tgt_id);
                    adj.entry(tgt_id).or_default().push(src_id);
                    *degrees.entry(src_id).or_insert(0) += 1;
                    *degrees.entry(tgt_id).or_insert(0) += 1;
                }
            }
        }

        for &deg in degrees.values() {
            if deg > max_deg {
                max_deg = deg;
            }
        }

        // Initialize sorting buckets
        let mut buckets = vec![HashSet::new(); max_deg + 1];
        for (&node_id, &deg) in &degrees {
            buckets[deg].insert(node_id);
        }

        let mut core = HashMap::new();
        let mut current_degrees = degrees.clone();
        let mut active_nodes = degrees.keys().cloned().collect::<HashSet<Uuid>>();

        for k in 0..=max_deg {
            while !buckets[k].is_empty() {
                let node_id = *buckets[k].iter().next().unwrap();
                buckets[k].remove(&node_id);
                active_nodes.remove(&node_id);
                core.insert(node_id, k);

                if let Some(neighbors) = adj.get(&node_id) {
                    for &neighbor in neighbors {
                        if active_nodes.contains(&neighbor) {
                            let neighbor_deg = current_degrees.get(&neighbor).unwrap();
                            if *neighbor_deg > k {
                                // Shift neighbor to a lower bucket
                                buckets[*neighbor_deg].remove(&neighbor);
                                let new_deg = neighbor_deg - 1;
                                buckets[new_deg].insert(neighbor);
                                *current_degrees.get_mut(&neighbor).unwrap() = new_deg;
                            }
                        }
                    }
                }
            }
        }

        core
    }

    // ── SA-Core Spreading Activation Diffusion Engine ──────────────────

    fn sa_core_diffusion(
        &self,
        start_ids: &[Uuid],
        alpha: f64,
        decay_factor: f64,
        weight_threshold: f64,
        max_hops: usize,
        max_nodes: usize,
        target_domain: Option<String>,
        min_k_core: usize,
    ) -> (Vec<Uuid>, Vec<(Uuid, f64)>) {
        // 1. Gather active nodes filtered by domain and k-core thresholding
        let active_nodes: Vec<NodeIndex> = self.graph.node_indices()
            .filter(|&idx| {
                if let Some(node) = self.graph.node_weight(idx) {
                    if node.is_recessive {
                        return false;
                    }
                    // Prune by k-core threshold to eliminate low-cohesion noise
                    if node.k_core < min_k_core {
                        return false;
                    }
                    // Prune by domain if target domain is specified
                    if let Some(ref target) = target_domain {
                        if let Some(ref d) = node.domain {
                            // Keep node if domain matches, or if it is a starting seed (cross-domain bridge)
                            if d != target && !start_ids.contains(&node.id) {
                                return false;
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            })
            .collect();

        let n = active_nodes.len();
        if n == 0 {
            return (Vec::new(), Vec::new());
        }

        let mut idx_to_flat = HashMap::with_capacity(n);
        for (flat_idx, &node_idx) in active_nodes.iter().enumerate() {
            idx_to_flat.insert(node_idx, flat_idx);
        }

        // 2. Explicitly initialize as f64 to eliminate type inference ambiguity
        let mut a_0: Vec<f64> = vec![0.0; n];
        for id in start_ids {
            if let Some(&node_idx) = self.id_to_index.get(id) {
                if let Some(&flat_idx) = idx_to_flat.get(&node_idx) {
                    a_0[flat_idx] = 1.0;
                }
            }
        }

        let mut a_current = a_0.clone();

        // Map edge properties to algebraic variables
        let get_raw_weight = |edge: &TopoEdge| -> f64 {
            let base_weight = match edge.relation_type {
                RelationType::Corrects => -1.0, // Inhibitory postsynaptic IPSP signal
                RelationType::Doubts => 0.3,
                _ => edge.weight,
            };
            if edge.is_soft {
                base_weight * decay_factor
            } else {
                base_weight
            }
        };

        // 3. Spreading activation loop
        for _ in 0..max_hops {
            let mut a_next: Vec<f64> = vec![0.0; n];

            for (i, &src_idx) in active_nodes.iter().enumerate() {
                let energy_i = a_current[i];
                if energy_i.abs() < 1e-9 {
                    continue;
                }

                let mut sum_abs = 0.0;
                let mut active_edges = Vec::new();

                for edge_ref in self.graph.edges(src_idx) {
                    let target_idx = edge_ref.target();
                    if let Some(&j) = idx_to_flat.get(&target_idx) {
                        let w = get_raw_weight(edge_ref.weight());
                        sum_abs += w.abs();
                        active_edges.push((j, w));
                    }
                }

                // Row-Normalization to preserve mathematical convergence bounds
                if sum_abs > 0.0 {
                    for (j, w) in active_edges {
                        let normalized_w = w / sum_abs;
                        a_next[j] += energy_i * normalized_w;
                    }
                }
            }

            // Attenuation and initial focus injection
            for j in 0..n {
                let val = alpha * a_next[j] + (1.0 - alpha) * a_0[j];
                a_current[j] = if val < weight_threshold { 0.0 } else { val };
            }
        }

        // 4. Map back flat indices to Uuids and sort by final energy descending
        let mut energy_nodes: Vec<(Uuid, f64)> = a_current.iter().enumerate()
            .filter(|&(_, &energy)| energy > 0.0)
            .filter_map(|(idx, &energy)| {
                let node_idx = active_nodes[idx];
                self.index_to_id.get(&node_idx).map(|&uuid| (uuid, energy))
            })
            .collect();

        energy_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let result_ids: Vec<Uuid> = energy_nodes.iter()
            .take(max_nodes)
            .map(|x| x.0)
            .collect();

        (result_ids, energy_nodes)
    }

    // ── Unified SA-Core Traverse API ──────────────────────────────

    /// Exposes a unified SA-Core mathematical search interface supporting domain and k-core constraints.
    pub fn sa_core_traverse(
        &self,
        start_ids: &[Uuid],
        alpha: f64,
        decay_factor: f64,
        weight_threshold: f64,
        max_hops: usize,
        max_nodes: usize,
        target_domain: Option<String>,
        min_k_core: usize,
    ) -> (Vec<Uuid>, Vec<(Uuid, f64)>) {
        self.sa_core_diffusion(
            start_ids,
            alpha,
            decay_factor,
            weight_threshold,
            max_hops,
            max_nodes,
            target_domain,
            min_k_core,
        )
    }

    // ── Legacy Traversal Mapped to SA-Core ───────────────────────────

    /// Skilled mode: only diffuse along non-soft edges
    pub fn skilled_traverse(
        &self,
        start_ids: &[Uuid],
        beam_width: usize,
        weight_threshold: f64,
        _energy_budget: u64,
        max_nodes: usize,
    ) -> (Vec<Uuid>, bool, Option<String>) {
        let max_hops = beam_width.max(3);
        let alpha = 0.5;

        let (result_ids, _) = self.sa_core_diffusion(
            start_ids,
            alpha,
            0.0,
            weight_threshold,
            max_hops,
            max_nodes,
            None,
            0,
        );

        (result_ids, false, None)
    }

    /// Anchor mode: introduce soft edge decay penalty
    pub fn anchor_traverse(
        &self,
        start_ids: &[Uuid],
        beam_width: usize,
        weight_threshold: f64,
        _energy_budget: u64,
        max_nodes: usize,
    ) -> (Vec<Uuid>, bool, Option<String>) {
        let max_hops = beam_width.max(3);
        let alpha = 0.7;
        let decay_factor = 0.8;

        let (result_ids, _) = self.sa_core_diffusion(
            start_ids,
            alpha,
            decay_factor,
            weight_threshold,
            max_hops,
            max_nodes,
            None,
            0,
        );

        (result_ids, false, None)
    }

    /// Imagination mode: chaotic walk
    pub fn imagination_traverse(
        &self,
        start_ids: &[Uuid],
        temperature: f64,
        _energy_budget: u64,
        max_nodes: usize,
    ) -> (Vec<Uuid>, bool, Option<String>) {
        let max_hops = 5;
        let alpha = 0.9;
        let decay_factor = 0.95;
        let threshold = (0.01 * (1.0 - temperature)).max(0.001);

        let (result_ids, _) = self.sa_core_diffusion(
            start_ids,
            alpha,
            decay_factor,
            threshold,
            max_hops,
            max_nodes,
            None,
            0,
        );

        (result_ids, false, None)
    }

    // ── Rebuild from SQLite ──────────────────────────────────────────

    pub fn rebuild_from_sqlite(sqlite: &SqlitePool) -> Result<Self, helix_mind_core::error::MindError> {
        let mut topology = Self::new();
        let conn = sqlite.get()?;

        // Load nodes (loading content to dynamically extract domain attributes)
        let mut stmt = conn.prepare(
            "SELECT id, node_type, is_recessive, content FROM nodes"
        ).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        let node_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
                row.get::<_, String>(3)?,
            ))
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        for row in node_iter {
            let (id_str, type_str, is_recessive, content_json) = row.map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let node_type = match type_str.as_str() {
                "L0" => NodeType::L0,
                "L1" => NodeType::L1,
                "L2" => NodeType::L2,
                _ => NodeType::L3,
            };

            // Dynamically parse domain fields from the structured JSON metadata
            let domain = if let Ok(serde_json::Value::Object(map)) = serde_json::from_str(&content_json) {
                map.get("domain").and_then(|v| v.as_str().map(|s| s.to_string()))
            } else {
                None
            };

            let topo = TopoNode { id, node_type, is_recessive, domain, k_core: 0 };
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

        // Calculate and load core levels for all nodes to complete Milestone 2 initialization
        let cores = topology.compute_k_cores();
        for node_idx in topology.graph.node_indices() {
            if let Some(n) = topology.graph.node_weight_mut(node_idx) {
                n.k_core = *cores.get(&n.id).unwrap_or(&0);
            }
        }

        Ok(topology)
    }
}
