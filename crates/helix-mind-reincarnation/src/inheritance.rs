use helix_mind_core::config::LifecycleConfig;
use helix_mind_storage::StorageEngine;
use helix_mind_storage::WritePriority;
use std::sync::Arc;
use zstd::stream::*;
use tracing::info;

pub struct Inheritance {
    config: LifecycleConfig,
    storage: Arc<StorageEngine>,
}

impl Inheritance {
    pub fn new(config: LifecycleConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    /// Create inheritance crystal
    pub async fn create_crystal(&self) -> Result<String, helix_mind_core::error::MindError> {
        let l2_nodes = self.storage.get_l2_nodes_by_generation(1).await?;
        let count = l2_nodes.len(); // 先保存长度
        info!("Creating inheritance crystal with {} L2 nodes", count);

        let content = serde_json::to_vec(&l2_nodes)?;
        let mut compressed = Vec::new();
        let mut encoder = Encoder::new(&mut compressed, 19)?;
        std::io::copy(&mut content.as_slice(), &mut encoder)?;
        encoder.finish()?;

        let hash = helix_mind_core::sha256_digest(&compressed);
        let filename = format!("./inheritance_crystal_{}.zst", hash);
        tokio::fs::write(&filename, &compressed).await?;

        info!("Inheritance crystal created: {}", hash);
        Ok(hash)
    }

    /// Load inheritance crystal
    pub async fn load_crystal(&self, hash: &str) -> Result<(), helix_mind_core::error::MindError> {
        let filename = format!("./inheritance_crystal_{}.zst", hash);
        
        let file = std::fs::File::open(&filename)?; 
        let mut decoder = Decoder::new(file)?;
        let mut content = Vec::new();
        std::io::copy(&mut decoder, &mut content)?;

        let l2_nodes: Vec<helix_mind_core::graph::Node> = serde_json::from_slice(&content)?;
        let count = l2_nodes.len(); // 先保存长度

        for mut node in l2_nodes {
            node.generation = 2;
            self.storage.write_node(node, WritePriority::Critical).await?;
        }

        info!("Loaded inheritance crystal: {} nodes restored", count);
        Ok(())
    }
}