use helix_mind_core::config::FederationConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use cid::Cid;
use dag_cbor::to_vec;
use libipld::Ipld;
use uuid::Uuid;
use chrono::Utc;
use zstd::stream::*;

pub struct DagShare {
    config: FederationConfig,
    storage: Arc<StorageEngine>,
}

impl DagShare {
    pub fn new(config: FederationConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    pub async fn share(&self, target_helix_id: Option<String>) -> Result<String, helix_mind_core::error::MindError> {
        // 1. Collect public L2 nodes
        let l2_nodes = self.storage.get_nodes_by_type(helix_mind_core::graph::NodeType::L2).await?;
        let public_nodes: Vec<_> = l2_nodes.into_iter()
            .filter(|n| n.sensitivity == Some(helix_mind_core::graph::Sensitivity::Public))
            .collect();

        // 2. Build DAG
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for node in &public_nodes {
            nodes.push(serde_json::to_value(node)?);
        }
        let dag_edges = self.storage.get_edges_between(&public_nodes.iter().map(|n| n.id).collect::<Vec<_>>()).await?;
        for edge in dag_edges {
            edges.push(serde_json::to_value(edge)?);
        }

        let dag = Ipld::Map(std::collections::HashMap::from_iter(vec![
            ("version".into(), Ipld::String("1.0".into())),
            ("target_helix_id".into(), target_helix_id.map(|s| Ipld::String(s)).unwrap_or(Ipld::Null)),
            ("timestamp".into(), Ipld::Integer(Utc::now().timestamp() as i128)),
            ("nodes".into(), Ipld::List(nodes.into_iter().map(|v| Ipld::from(v)).collect())),
            ("edges".into(), Ipld::List(edges.into_iter().map(|v| Ipld::from(v)).collect())),
        ]));

        // 3. Encode DAG-CBOR
        let cbor = to_vec(&dag)?;
        // Compress
        let mut compressed = Vec::new();
        let mut encoder = Encoder::new(&mut compressed, 19)?;
        std::io::copy(&mut cbor.as_slice(), &mut encoder)?;
        encoder.finish()?;

        // 4. Compute CIDv1
        let cid = Cid::new_v1(0x70, cid::multihash::Multihash::wrap(0x12, &sha2::digest::digest::<sha2::Sha256>(&compressed)))?;

        // 5. Write to outgoing directory
        let filename = format!("{}/{}.dag.zst", self.config.outgoing_dir, cid);
        tokio::fs::write(filename, compressed).await?;

        Ok(cid.to_string())
    }
}
