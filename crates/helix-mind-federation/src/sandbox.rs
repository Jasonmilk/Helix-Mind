use helix_mind_core::config::FederationConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use dag_cbor::from_slice;
use libipld::Ipld;
use zstd::stream::*;
use tracing::info;

pub struct Sandbox {
    config: FederationConfig,
    storage: Arc<StorageEngine>,
}

impl Sandbox {
    pub fn new(config: FederationConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    pub async fn process_incoming(&self, cid: &str) -> Result<(), helix_mind_core::error::MindError> {
        // 1. Read file from sandbox
        let filename = format!("{}/{}.dag.zst", self.config.sandbox_dir, cid);
        let file = tokio::fs::File::open(filename).await?;
        let mut decoder = Decoder::new(file)?;
        let mut cbor = Vec::new();
        std::io::copy(&mut decoder, &mut cbor)?;

        // 2. Decode DAG-CBOR
        let dag: Ipld = from_slice(&cbor)?;

        // 3. Validate DAG
        self.validate_dag(&dag)?;

        // 4. Extract nodes and edges
        let nodes = self.extract_nodes(&dag)?;
        let edges = self.extract_edges(&dag)?;

        // 5. Merge into local storage
        let mut merged = 0;
        for node in nodes {
            // Check if node already exists
            if self.storage.get_nodes_by_ids(&[node.id]).await?.is_empty() {
                self.storage.upsert_node(&node).await?;
                merged += 1;
            }
        }
        for edge in edges {
            self.storage.add_edge(&edge).await?;
        }

        // 6. Write audit log
        let audit = helix_mind_core::audit::AuditEntry::new(
            helix_mind_core::audit::AuditEventType::FederationNodeMerged,
            "federation",
            &format!("Merged {} nodes from CID {}", merged, cid),
        );
        self.storage.write_audit(&audit).await?;

        info!("Processed incoming DAG: merged {} nodes", merged);
        Ok(())
    }

    fn validate_dag(&self, dag: &Ipld) -> Result<(), helix_mind_core::error::MindError> {
        // Check version
        if let Ipld::Map(map) = dag {
            if let Some(Ipld::String(version)) = map.get("version") {
                if version != "1.0" {
                    return Err(helix_mind_core::error::MindError::Federation("Invalid DAG version".into()));
                }
            } else {
                return Err(helix_mind_core::error::MindError::Federation("Missing version".into()));
            }
            Ok(())
        } else {
            Err(helix_mind_core::error::MindError::Federation("Invalid DAG format".into()))
        }
    }

    fn extract_nodes(&self, dag: &Ipld) -> Result<Vec<helix_mind_core::graph::Node>, helix_mind_core::error::MindError> {
        if let Ipld::Map(map) = dag {
            if let Some(Ipld::List(nodes_ipld)) = map.get("nodes") {
                let mut nodes = Vec::new();
                for node_ipld in nodes_ipld {
                    let node_json = serde_json::to_value(node_ipld)?;
                    let node: helix_mind_core::graph::Node = serde_json::from_value(node_json)?;
                    nodes.push(node);
                }
                Ok(nodes)
            } else {
                Err(helix_mind_core::error::MindError::Federation("Missing nodes".into()))
            }
        } else {
            Err(helix_mind_core::error::MindError::Federation("Invalid DAG format".into()))
        }
    }

    fn extract_edges(&self, dag: &Ipld) -> Result<Vec<helix_mind_core::graph::Edge>, helix_mind_core::error::MindError> {
        if let Ipld::Map(map) = dag {
            if let Some(Ipld::List(edges_ipld)) = map.get("edges") {
                let mut edges = Vec::new();
                for edge_ipld in edges_ipld {
                    let edge_json = serde_json::to_value(edge_ipld)?;
                    let edge: helix_mind_core::graph::Edge = serde_json::from_value(edge_json)?;
                    edges.push(edge);
                }
                Ok(edges)
            } else {
                Err(helix_mind_core::error::MindError::Federation("Missing edges".into()))
            }
        } else {
            Err(helix_mind_core::error::MindError::Federation("Invalid DAG format".into()))
        }
    }
}
