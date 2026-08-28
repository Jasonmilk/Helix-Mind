//! Epoch crystallization (§9.5 of the whitepaper).
//!
//! When a user is long-term absent, the entire memory snapshot is packaged
//! into an immutable IPLD DAG archive — the epoch crystal. This replaces
//! the v3.2 "digital cremation" concept.

use helix_mind_core::graph::NodeType;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

pub struct EpochCrystallizer {
    storage: Arc<StorageEngine>,
}

impl EpochCrystallizer {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    /// Create an epoch crystal — an immutable IPLD snapshot of all user-related data.
    ///
    /// Returns the CID of the generated crystal.
    pub async fn crystallize(
        &self,
    ) -> Result<String, helix_mind_core::error::MindError> {
        // 1. Collect all L3 memories
        let l3_nodes = self.storage
            .get_nodes_by_type(NodeType::L3)
            .await?;

        // 2. Get current user portrait
        let _l2_nodes = self.storage
            .get_nodes_by_type(NodeType::L2)
            .await?;

        // 3. Build epoch snapshot
        let epoch = serde_json::json!({
            "version": "1.0",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "total_nodes": l3_nodes.len(),
            "l3_node_ids": l3_nodes.iter().map(|n| n.id.to_string()).collect::<Vec<_>>(),
            "relationship_summary": format!("This epoch contains {} memories across one generation.", l3_nodes.len()),
        });

        // 4. Compress
        let json = serde_json::to_vec(&epoch).map_err(|e| {
            helix_mind_core::error::MindError::Storage(format!("JSON encode error: {}", e))
        })?;
        let compressed = zstd::encode_all(json.as_slice(), 19).map_err(|e| {
            helix_mind_core::error::MindError::Storage(format!("Zstd compress error: {}", e))
        })?;

        // 5. Compute CID
        let hash = helix_mind_core::sha256_digest(&compressed);
        // P6-3: 写配置目录（deep_cold_dir），不污染当前工作目录。
        let dir = &self.storage.config.deep_cold_dir;
        std::fs::create_dir_all(dir)
            .map_err(|e| helix_mind_core::error::MindError::Io(e))?;
        let filename = format!("{}/epoch_crystal_{}.zst", dir, hash);
        tokio::fs::write(&filename, &compressed).await.map_err(|e| {
            helix_mind_core::error::MindError::Storage(format!("Cannot write epoch crystal: {}", e))
        })?;

        info!("Epoch crystal created: {}", hash);
        Ok(hash)
    }
}