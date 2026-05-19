use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

pub struct Digest {
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
}

impl Digest {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    pub async fn run(&self) -> Result<(), helix_mind_core::error::MindError> {
        // 1. Find similar L3 nodes
        let recent_l3 = self.storage.get_nodes_by_type(helix_mind_core::graph::NodeType::L3).await?;
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
            }
        }

        // 2. Resolve cognitive dissonance
        let unresolved = self.storage.get_unresolved_dissonance(chrono::Duration::hours(24)).await?;
        for node in unresolved {
            self.resolve_dissonance(&node).await?;
        }

        // 3. Cleanup expired recessives
        let expired = self.storage.get_expired_recessives(chrono::Utc::now() - chrono::Duration::days(self.config.resurrection_window_days)).await?;
        for node in &expired {
            self.storage.delete_recessive_index(&node.id).await?;
        }

        info!("Digest completed: merged {} nodes, cleaned {} expired recessives", merged, expired.len());
        Ok(())
    }

    fn compute_similarity(&self, a: &helix_mind_core::graph::Node, b: &helix_mind_core::graph::Node) -> Result<f64, helix_mind_core::error::MindError> {
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

    async fn merge_nodes(&self, node: &helix_mind_core::graph::Node, similar: &helix_mind_core::graph::Node) -> Result<(), helix_mind_core::error::MindError> {
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

    async fn resolve_dissonance(&self, node: &helix_mind_core::graph::Node) -> Result<(), helix_mind_core::error::MindError> {
        // TODO: Resolve cognitive dissonance by creating Corrects edges
        Ok(())
    }
}
