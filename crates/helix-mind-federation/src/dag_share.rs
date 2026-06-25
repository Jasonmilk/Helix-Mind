//! DAG sharing — export local L2 nodes to the federation (§9.6).
//!
//! Packages public L2 nodes and their edges into JSON format,
//! compresses with Zstandard, and writes to the outgoing directory for
//! Rhizax to pick up and broadcast.

use helix_mind_core::config::FederationConfig;
use helix_mind_core::{graph::NodeType, graph::Sensitivity, MindError};
use helix_mind_storage::StorageEngine;
use serde_json::json;
use sha2::Digest;
use std::sync::Arc;
use uuid::Uuid;

pub struct DagShare {
    config: FederationConfig,
    storage: Arc<StorageEngine>,
}

impl DagShare {
    pub fn new(config: FederationConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    /// Share public L2 nodes to the federation network.
    ///
    /// Returns the SHA256 hash (CID-like) of the generated DAG block.
    pub async fn share(
        &self,
        target_helix_id: Option<String>,
    ) -> Result<String, MindError> {
        // 1. Collect public L2 nodes
        let l2_nodes = self
            .storage
            .get_nodes_by_type(NodeType::L2)
            .await?;
        let public_nodes: Vec<_> = l2_nodes
            .into_iter()
            .filter(|n| {
                n.sensitivity == Some(Sensitivity::Public)
            })
            .collect();

        // 2. Collect edges for public nodes
        let node_ids: Vec<Uuid> = public_nodes.iter().map(|n| n.id).collect();
        let dag_edges = self.storage.get_edges_between(&node_ids).await?;

        // 3. Build DAG JSON structure (replaced IPLD DAG-CBOR)
        let dag = json!({
            "version": "1.0",
            "target_helix_id": target_helix_id,
            "timestamp": chrono::Utc::now().timestamp(),
            "nodes": public_nodes,
            "edges": dag_edges
        });

        // 4. Serialize to JSON (architect-specified serialization)
        let json = serde_json::to_vec(&dag).map_err(|e| {
            MindError::Federation(format!("DAG JSON encode error: {}", e))
        })?;

        // 5. Compress with Zstandard (architect-specified compression)
        let compressed = zstd::encode_all(json.as_slice(), 19).map_err(|e| {
            MindError::Federation(format!("Zstd compress error: {}", e))
        })?;

        // 6. Compute CID-like hash (SHA256 hex) (architect-specified hashing)
        let hash = sha2::Sha256::digest(&compressed);
        let cid = hex::encode(hash);

        // 7. Write to outgoing directory
        let filename = format!("{}/{}.dag.zst", self.config.outgoing_dir, cid);
        tokio::fs::write(&filename, compressed).await.map_err(|e| {
            MindError::Storage(format!(
                "Cannot write outgoing DAG: {}", e
            ))
        })?;

        Ok(cid)
    }
}
