use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;
use helix_mind_core::symbolic;

pub struct Digest {
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
}

impl Digest {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    /// Returns the number of nodes merged in this digest pass.
    pub async fn run(&self) -> Result<u64, helix_mind_core::error::MindError> {
        // 1. Find similar L3 nodes
        let recent_l3 = self
            .storage
            .get_nodes_by_type(helix_mind_core::graph::NodeType::L3)
            .await?;
        let mut merged = 0;

        for node in &recent_l3 {
            if node.is_recessive {
                continue;
            }
            // Find similar node
            if let Some(similar) = self.storage.find_similar_node(node).await? {
                // Check similarity threshold
                let similarity = self.compute_similarity(node, &similar)?;
                if similarity >= self.config.merge_similarity_threshold {
                    // Merge them
                    self.merge_nodes(node, &similar).await?;
                    merged += 1;
                }
            } else {
            }
        }

        // 2. Resolve cognitive dissonance (cooling window from config)
        let unresolved = self
            .storage
            .get_unresolved_dissonance(chrono::Duration::hours(
                self.config.dissonance_window_hours as i64,
            ))
            .await?;

        for (node_a_id, node_b_id) in &unresolved {
            // Fetch the actual nodes for dissonance resolution
            let nodes = self.storage.get_nodes_by_ids(&[*node_a_id, *node_b_id]).await?;
            if nodes.len() >= 2 {
                self.resolve_dissonance(&nodes[0], &nodes[1]).await?;
            }
        }

        // 3. Cleanup expired recessives
        let expired = self
            .storage
            .get_expired_recessives(
                chrono::Utc::now()
                    - chrono::Duration::days(self.config.resurrection_window_days),
            )
            .await?;

        for node_id in &expired {
            self.storage.delete_recessive_index(node_id).await?;
        }

        info!(
            "Digest completed: merged {} nodes, cleaned {} expired recessives",
            merged,
            expired.len()
        );
        Ok(merged)
    }

    fn compute_similarity(
        &self,
        a: &helix_mind_core::graph::Node,
        b: &helix_mind_core::graph::Node,
    ) -> Result<f64, helix_mind_core::error::MindError> {
        // TODO: Use vector embedding similarity
        // For now, simple string comparison
        let content_a = match &a.content {
            helix_mind_core::graph::NodeContent::Text(t) => t,
            _ => return Ok(0.0),
        };
        let content_b = match &b.content {
            helix_mind_core::graph::NodeContent::Text(t) => t,
            _ => return Ok(0.0),
        };

        // Levenshtein similarity
        let distance = levenshtein::levenshtein(content_a, content_b);
        let max_len = content_a.len().max(content_b.len()) as f64;
        if max_len == 0.0 {
            return Ok(1.0);
        }
        Ok(1.0 - (distance as f64) / max_len)
    }

    async fn merge_nodes(
        &self,
        node: &helix_mind_core::graph::Node,
        similar: &helix_mind_core::graph::Node,
    ) -> Result<(), helix_mind_core::error::MindError> {
        // Mark old node as recessive
        self.storage.mark_recessive(&node.id).await?;

        // Add edge from new to old
        let edge = helix_mind_core::graph::Edge {
            source_id: similar.id,
            target_id: node.id,
            weight: 0.9,
            relation_type: helix_mind_core::graph::RelationType::SimilarTo,
            is_soft: false,
        };
        self.storage.add_edge(&edge).await?;
        Ok(())
    }

    async fn resolve_dissonance(
        &self,
        node_a: &helix_mind_core::graph::Node,
        node_b: &helix_mind_core::graph::Node,
    ) -> Result<(), helix_mind_core::error::MindError> {
        // P2a (ADR-0014): deterministic arbitration via SymbolicSolver.
        // get_unresolved_dissonance already filtered to pairs whose nodes both
        // carry structured assertions.
        let a_assertions = symbolic::assertions_from_node(&node_a.content);
        let b_assertions = symbolic::assertions_from_node(&node_b.content);
        let solver = symbolic::SymbolicSolver::new();

        // Confirm a genuine logical contradiction before creating a Corrects
        // edge. If no direct contradiction, the Conflicts edge is a stale/false
        // signal — record nothing (do not fabricate a correction).
        if solver.check_clash(&a_assertions, &[], &b_assertions).is_ok() {
            return Ok(());
        }

        // Deterministic arbitration: the higher-heat node is the retained
        // truth; it corrects the lower-heat node (Corrects edge weight -1.0,
        // contract-validated in core).
        let (corrector, corrected) = if node_a.heat >= node_b.heat {
            (node_a, node_b)
        } else {
            (node_b, node_a)
        };
        let edge = helix_mind_core::graph::Edge {
            source_id: corrector.id,
            target_id: corrected.id,
            weight: -1.0,
            relation_type: helix_mind_core::graph::RelationType::Corrects,
            is_soft: false,
        };
        self.storage.add_edge(&edge).await?;
        self.storage
            .update_corrected_by(&corrected.id, &corrector.id)
            .await?;
        Ok(())
    }
}
