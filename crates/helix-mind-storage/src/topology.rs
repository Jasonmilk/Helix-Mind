use helix_mind_core::graph::{Node, NodeType, Edge, RelationType};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
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

    pub fn add_edge(&mut self, source: Uuid, target: Uuid, edge: &Edge) -> Result<(), helix_mind_core::error::MindError> {
        let src_idx = *self.id_to_index.get(&source)
            .ok_or_else(|| helix_mind_core::error::MindError::NotFound(format!("Source node not found: {}", source)))?;
        let tgt_idx = *self.id_to_index.get(&target)
            .ok_or_else(|| helix_mind_core::error::MindError::NotFound(format!("Target node not found: {}", target)))?;
        let topo = TopoEdge {
            weight: edge.weight,
            relation_type: edge.relation_type.clone(),
            is_soft: edge.is_soft,
        };
        self.graph.add_edge(src_idx, tgt_idx, topo);
        Ok(())
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

    /// Rebuild topology from SQLite
    pub fn rebuild_from_sqlite(sqlite: &SqlitePool) -> Result<Self, helix_mind_core::error::MindError> {
        let mut topology = Self::new();
        
        // 获取连接，转换错误
        let conn = sqlite.get()
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        // 1. Load nodes
        // prepare 转换错误
        let mut stmt = conn.prepare("SELECT id, node_type, is_recessive FROM nodes")
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        
        // query_map 转换错误
        let node_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
            ))
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        for node in node_iter {
            // 内部迭代错误转换
            let (id_str, node_type_str, is_recessive): (String, String, bool) = node
                .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            
            let id = Uuid::parse_str(&id_str)
                .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            
            let node_type = match node_type_str.as_str() {
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

        // 2. Load edges
        let mut stmt = conn.prepare("SELECT source_id, target_id, weight, relation_type, is_soft FROM edges")
            .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
        
        let edge_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, bool>(4)?,
            ))
        }).map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;

        for edge in edge_iter {
            let (src_str, tgt_str, weight, rel_str, is_soft): (String, String, f64, String, bool) = edge
                .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            
            let source = Uuid::parse_str(&src_str)
                .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            let target = Uuid::parse_str(&tgt_str)
                .map_err(|e| helix_mind_core::error::MindError::Storage(e.to_string()))?;
            
            let relation_type = match rel_str.as_str() {
                "Causal" => RelationType::Causal,
                "Semantic" => RelationType::Semantic,
                "Temporal" => RelationType::Temporal,
                "CoOccurrence" => RelationType::CoOccurrence,
                "Corrects" => RelationType::Corrects,
                "Refines" => RelationType::Refines,
                "Doubts" => RelationType::Doubts,
                "SimilarTo" => RelationType::SimilarTo,
                _ => return Err(helix_mind_core::error::MindError::Storage("Unknown relation type".into())),
            };

            if let (Some(&src_idx), Some(&tgt_idx)) = (topology.id_to_index.get(&source), topology.id_to_index.get(&target)) {
                topology.graph.add_edge(src_idx, tgt_idx, TopoEdge {
                    weight,
                    relation_type,
                    is_soft,
                });
            }
        }

        Ok(topology)
    }
}