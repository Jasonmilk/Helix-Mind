use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

pub struct Hibernate {
    #[allow(dead_code)]
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
}

impl Hibernate {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    pub async fn run(&self) -> Result<(), helix_mind_core::error::MindError> {
        let cold_node_ids = self
            .storage
            .get_nodes_below_heat(
                0.1,
                chrono::Utc::now() - chrono::Duration::days(30),
            )
            .await?;

        if cold_node_ids.is_empty() {
            return Ok(());
        }

        // Fetch full node objects for the cold IDs
        let cold_nodes = self.storage.get_nodes_by_ids(&cold_node_ids).await?;

        let mut archived = 0;
        for node in &cold_nodes {
            if node.node_type != helix_mind_core::graph::NodeType::L3 {
                continue;
            }
            // Mark as recessive; deep cold archiving will be implemented later
            self.storage.mark_recessive(&node.id).await?;
            archived += 1;
        }

        info!(
            "Hibernate completed: marked {} cold nodes as recessive",
            archived
        );
        Ok(())
    }
}
