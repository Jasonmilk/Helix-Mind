//! Federation module — sandbox review + DAG sharing (§9 of the whitepaper).
//!
//! The FederationEngine handles all external knowledge exchange:
//! - Incoming: sandbox isolation → LLM review → merge or reject
//! - Outgoing: IPLD DAG-CBOR packaging → Rhizax shared directory
//!
//! There is no background scheduler, no heartbeat, no polling (Iron Law #13).

pub mod sandbox;
pub mod dag_share;
pub mod review;

use helix_mind_core::config::FederationConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;

pub struct FederationEngine {
    pub config: FederationConfig,
    pub storage: Arc<StorageEngine>,
}

impl FederationEngine {
    pub fn new(config: FederationConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    /// Share local public L2 nodes to the federation network.
    ///
    /// Collects all public L2 nodes, packages them as an IPLD DAG-CBOR block,
    /// compresses with Zstandard, and writes to the outgoing directory for
    /// Rhizax to pick up.
    pub async fn share_dag(
        &self,
        target_helix_id: Option<String>,
    ) -> Result<String, helix_mind_core::error::MindError> {
        let share = dag_share::DagShare::new(self.config.clone(), self.storage.clone());
        share.share(target_helix_id).await
    }

    /// Process an incoming DAG from the sandbox directory.
    ///
    /// Reads the compressed DAG-CBOR file, validates its structure, extracts
    /// nodes and edges, then passes them through the sandbox review pipeline
    /// before merging.
    pub async fn process_incoming(
        &self,
        cid: &str,
    ) -> Result<SandboxReport, helix_mind_core::error::MindError> {
        let sb = sandbox::Sandbox::new(self.config.clone(), self.storage.clone());
        sb.process_incoming(cid).await
    }

    /// Query the shared knowledge tree for nodes semantically similar to
    /// the given query embedding. Called by retrieval Stage 2.
    pub async fn query_shared_tree(
        &self,
        _query_embedding: &[f32],
        _max_results: u8,
    ) -> Result<Vec<(helix_mind_core::graph::Node, Vec<helix_mind_core::graph::Edge>)>, helix_mind_core::error::MindError>
    {
        // TODO: query Rhizax for semantic matches
        // For now, return empty — will be connected in Phase 4.1
        Ok(Vec::new())
    }
}

/// Summary report from a sandbox review run.
#[derive(Debug, Clone)]
pub struct SandboxReport {
    /// Number of nodes merged into local DAG.
    pub nodes_merged: usize,
    /// Number of nodes rejected by the review pipeline.
    pub nodes_rejected: usize,
    /// Rejection reasons keyed by node ID.
    pub rejection_reasons: Vec<(String, String)>,
}
