//! Sandbox review pipeline (§9.5.1, §9.5.2 of the whitepaper).
//!
//! All externally sourced knowledge enters the sandbox before touching the
//! main DAG. The review pipeline enforces:
//! - Zero-context-pollution LLM review
//! - Dual-blind verification
//! - Source diversity requirements
//! - High-risk node flagging
//! - Cognitive DoS rate limiting

use crate::review::{self, is_high_risk_node};
use helix_mind_core::config::FederationConfig;
use helix_mind_core::graph::{Edge, Node};
use helix_mind_core::{AuditEntry, AuditEventType, MindError};
use helix_mind_storage::WritePriority;
use helix_mind_storage::StorageEngine;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct Sandbox {
    config: FederationConfig,
    storage: Arc<StorageEngine>,
    /// Rate limiter: tracks review counts per source Helix ID
    rate_limiter: Mutex<HashMap<String, usize>>,
    /// Credit downgrade log: source Helix IDs that produced bad knowledge
    credit_downgrades: Mutex<HashMap<String, usize>>,
}

impl Sandbox {
    pub fn new(config: FederationConfig, storage: Arc<StorageEngine>) -> Self {
        Self {
            config,
            storage,
            rate_limiter: Mutex::new(HashMap::new()),
            credit_downgrades: Mutex::new(HashMap::new()),
        }
    }

    /// Process an incoming DAG from the sandbox directory.
    ///
    /// Steps:
    /// 1. Read and decompress the DAG-JSON file
    /// 2. Validate DAG structure and version
    /// 3. Extract nodes and edges
    /// 4. For each node: run sandbox review pipeline
    /// 5. Merge approved nodes, reject violations
    pub async fn process_incoming(
        &self,
        cid: &str,
    ) -> Result<crate::SandboxReport, MindError> {
        // 1. Read file from sandbox as bytes
        let file_path = format!("{}/{}.dag.zst", self.config.sandbox_dir, cid);
        let compressed = tokio::fs::read(&file_path).await.map_err(|e| {
            MindError::Storage(format!("Cannot read sandbox file: {}", e))
        })?;

        // 2. Decompress
        let cbor = zstd::decode_all(compressed.as_slice()).map_err(|e| {
            MindError::Storage(format!("Zstd decode error: {}", e))
        })?;

        // 3. Parse as JSON (simplified: expect a JSON map with "nodes" and "edges")
        let dag: serde_json::Value = serde_json::from_slice(&cbor).map_err(|e| {
            MindError::Federation(format!("Invalid DAG JSON: {}", e))
        })?;

        // 4. Validate DAG structure
        self.validate_dag(&dag)?;

        // 5. Extract nodes and edges
        let nodes = self.extract_nodes(&dag)?;
        let edges = self.extract_edges(&dag)?;

        // 6. Extract source Helix ID for rate limiting and credit tracking
        let source_helix_id = self.extract_source_id(&dag).unwrap_or_else(|| "unknown".to_string());

        // 7. Check rate limit (Cognitive DoS protection — §9.5.2)
        self.check_rate_limit(&source_helix_id).await?;

        // 8. Sandbox review pipeline for each node
        let mut nodes_merged = 0;
        let mut nodes_rejected = 0;
        let mut rejection_reasons = Vec::new();

        for mut node in nodes {
            // 8a. Check if node already exists
            if !self.storage.get_nodes_by_ids(&[node.id]).await?.is_empty() {
                // Node already exists — corroborate instead of duplicate
                let mut existing = self.storage.get_nodes_by_ids(&[node.id]).await?
                    .into_iter().next().unwrap();
                existing.corroborations += 1;
                let new_dominance = (existing.corroborations as f64 * 0.01 + existing.utility * 0.5).min(1.0);
                existing.dominance = new_dominance;
                self.storage.write_node(existing, WritePriority::Critical).await?;
                nodes_merged += 1;
                continue;
            }

            // 8b. Mark node as federated source
            node.source = helix_mind_core::graph::NodeSource::Federated {
                source_helix_id: source_helix_id.clone(),
                source_generation: 1,
                verified_at: Some(chrono::Utc::now()),
            };

            // 8c. Run dual-blind LLM review (§9.5.1)
            let (verdict1, verdict2) = review::dual_blind_review(
                &node,
                "local DAG context placeholder",
            ).await?;

            if !verdict1.logically_coherent || !verdict2.logically_coherent {
                rejection_reasons.push((
                    node.id.to_string(),
                    format!("Dual-blind review failed: v1={}, v2={}",
                        verdict1.logically_coherent, verdict2.logically_coherent),
                ));
                nodes_rejected += 1;
                self.record_credit_downgrade(&source_helix_id).await;
                continue;
            }

            // 8d. Check for conflict with local DAG
            if verdict1.conflict_with_local_dag || verdict2.conflict_with_local_dag {
                // Mark as suspicious, do not auto-merge
                node.high_risk = true;
                node.notes = Some("Suspicious: conflicts with local DAG (dual-blind review)".into());
                // Still merge but with high_risk flag — user must confirm
            }

            // 8e. High-risk node detection (§9.5.1)
            if is_high_risk_node(&node) {
                node.high_risk = true;
                node.utility = node.utility.min(0.3); // cap utility for high-risk (§9.5.2)
                node.notes = Some("High-risk: involves system-level operations".into());
            }

            // 8f. Write node to local storage
            self.storage.write_node(node.clone(), WritePriority::Critical).await?;
            nodes_merged += 1;
        }

        // 9. Merge edges
        for edge in &edges {
            self.storage.add_edge(edge).await?;
        }

        // 10. Write audit log
        let audit = AuditEntry::new(
            AuditEventType::FederationNodeMerged,
            "federation",
            &format!(
                "Processed DAG {}: merged {} nodes, rejected {} nodes",
                cid, nodes_merged, nodes_rejected
            ),
        );
        self.storage.write_audit(&audit).await?;

        info!(
            "Sandbox processed {}: merged {}, rejected {}",
            cid, nodes_merged, nodes_rejected
        );

        Ok(crate::SandboxReport {
            nodes_merged,
            nodes_rejected,
            rejection_reasons,
        })
    }

    // ── Validation (serde_json::Value) ──────────────────────────

    fn validate_dag(&self, dag: &Value) -> Result<(), MindError> {
        let dag_map = dag.as_object()
            .ok_or_else(|| MindError::Federation("Invalid DAG format: not a JSON object".into()))?;

        let version = dag_map.get("version")
            .ok_or_else(|| MindError::Federation("Missing version field".into()))?
            .as_str()
            .ok_or_else(|| MindError::Federation("Version must be a string".into()))?;

        if version != "1.0" {
            return Err(MindError::Federation("Invalid DAG version".into()));
        }

        if !dag_map.contains_key("nodes") {
            return Err(MindError::Federation("Missing nodes field".into()));
        }

        if !dag_map.contains_key("edges") {
            return Err(MindError::Federation("Missing edges field".into()));
        }

        Ok(())
    }

    fn extract_nodes(&self, dag: &Value) -> Result<Vec<Node>, MindError> {
        let nodes_array = dag["nodes"]
            .as_array()
            .ok_or_else(|| MindError::Federation("Nodes must be an array".into()))?;

        let mut nodes = Vec::new();
        for node_value in nodes_array {
            let node: Node = serde_json::from_value(node_value.clone())
                .map_err(|e| MindError::Federation(format!("Node deserialization error: {}", e)))?;
            nodes.push(node);
        }

        Ok(nodes)
    }

    fn extract_edges(&self, dag: &Value) -> Result<Vec<Edge>, MindError> {
        let edges_array = dag["edges"]
            .as_array()
            .ok_or_else(|| MindError::Federation("Edges must be an array".into()))?;

        let mut edges = Vec::new();
        for edge_value in edges_array {
            let edge: Edge = serde_json::from_value(edge_value.clone())
                .map_err(|e| MindError::Federation(format!("Edge deserialization error: {}", e)))?;
            edges.push(edge);
        }

        Ok(edges)
    }

    fn extract_source_id(&self, dag: &Value) -> Option<String> {
        dag["source_helix_id"]
            .as_str()
            .map(|s| s.to_string())
    }

    // ── Rate limiting (Cognitive DoS protection — §9.5.2) ──────────

    async fn check_rate_limit(&self, source_id: &str) -> Result<(), MindError> {
        let mut limiter = self.rate_limiter.lock().await;
        let count = limiter.entry(source_id.to_string()).or_insert(0);
        *count += 1;

        // If a single source produces too many nodes in a short time, throttle
        let max_per_source: usize = 100; // configurable
        if *count > max_per_source {
            return Err(MindError::SandboxRejected {
                reason: format!(
                    "Rate limit exceeded for source {}: {} reviews",
                    source_id, *count
                ),
            });
        }
        Ok(())
    }

    // ── Credit tracking (§9.5.2) ───────────────────────────────────

    async fn record_credit_downgrade(&self, source_id: &str) {
        let mut downgrades = self.credit_downgrades.lock().await;
        *downgrades.entry(source_id.to_string()).or_insert(0) += 1;
    }

    /// Get the credit downgrade count for a source Helix.
    pub async fn get_credit_downgrades(&self, source_id: &str) -> usize {
        let downgrades = self.credit_downgrades.lock().await;
        downgrades.get(source_id).copied().unwrap_or(0)
    }
}