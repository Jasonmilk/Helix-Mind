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
    ///
    /// Outbound gate (ADR-0018 P3a): capability not ready = feature does not
    /// exist. When `FederationConfig.enabled` is false (default), outbound
    /// sharing is refused outright. Every public node must pass the
    /// deterministic dual-judge review (logical consistency + high-risk rule)
    /// before packaging — fail-closed, no un-reviewed node leaves the mind.
    pub async fn share(
        &self,
        target_helix_id: Option<String>,
    ) -> Result<String, MindError> {
        // ADR-0018: outbound gate — default off.
        if !self.config.enabled {
            return Err(MindError::Federation(
                "federation disabled: outbound capability not ready (ADR-0018)".into(),
            ));
        }

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

        // 2. Deterministic outbound review (fail-closed) — no un-reviewed node leaves.
        for node in &public_nodes {
            let (v1, v2) = crate::review::dual_blind_review(node, "").await?;
            if !v1.logically_coherent || v2.conflict_with_local_dag {
                return Err(MindError::Federation(format!(
                    "outbound review failed (fail-closed): node {} — v1.coherent={}, v2.risk={} ({} | {})",
                    node.id,
                    v1.logically_coherent,
                    v2.conflict_with_local_dag,
                    v1.reason,
                    v2.reason,
                )));
            }
        }

        // 3. Collect edges for public nodes
        let node_ids: Vec<Uuid> = public_nodes.iter().map(|n| n.id).collect();
        let dag_edges = self.storage.get_edges_between(&node_ids).await?;

        // 4. Build DAG JSON structure (replaced IPLD DAG-CBOR)
        let dag = json!({
            "version": "1.0",
            "target_helix_id": target_helix_id,
            "timestamp": chrono::Utc::now().timestamp(),
            "nodes": public_nodes,
            "edges": dag_edges
        });

        // 5. Serialize to JSON (architect-specified serialization)
        let json = serde_json::to_vec(&dag).map_err(|e| {
            MindError::Federation(format!("DAG JSON encode error: {}", e))
        })?;

        // 6. Compress with Zstandard (architect-specified compression)
        let compressed = zstd::encode_all(json.as_slice(), 19).map_err(|e| {
            MindError::Federation(format!("Zstd compress error: {}", e))
        })?;

        // 7. Compute CID-like hash (SHA256 hex) (architect-specified hashing)
        let hash = sha2::Sha256::digest(&compressed);
        let cid = hex::encode(hash);

        // 8. Write to outgoing directory
        let filename = format!("{}/{}.dag.zst", self.config.outgoing_dir, cid);
        tokio::fs::write(&filename, compressed).await.map_err(|e| {
            MindError::Storage(format!(
                "Cannot write outgoing DAG: {}", e
            ))
        })?;

        Ok(cid)
    }
}
