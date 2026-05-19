use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

pub struct Hibernate {
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
}

impl Hibernate {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    pub async fn run(&self) -> Result<(), helix_mind_core::error::MindError> {
        let cold_nodes = self.storage.get_nodes_below_heat(0.1, chrono::Utc::now() - chrono::Duration::days(30)).await?;
        if cold_nodes.is_empty() {
            return Ok(());
        }

        let mut archived = 0;
        for node in cold_nodes {
            if node.node_type != helix_mind_core::graph::NodeType::L3 {
                continue;
            }
            
            // 暂时注释掉 deep_cold 逻辑，因为该模块尚未实现
            // let deep_cold = super::deep_cold::DeepColdStore::new(&self.config.deep_cold_dir);
            // let _stub = deep_cold.archive_node(&node, 365 * 100).await?;
            
            self.storage.mark_recessive(&node.id).await?;
            archived += 1;
        }

        info!("Hibernate completed: marked {} cold nodes as recessive", archived);
        Ok(())
    }
}