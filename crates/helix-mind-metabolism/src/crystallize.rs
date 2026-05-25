use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use helix_mind_storage::WritePriority;
use std::sync::Arc;
use tracing::info;

pub struct Crystallize {
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
}

impl Crystallize {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    pub async fn run(&self) -> Result<(), helix_mind_core::error::MindError> {
        let idle_l3 = self.storage.get_top_l3_for_crystallization(100).await?;
        if idle_l3.is_empty() {
            return Ok(());
        }

        let summary = self.summarize_with_llm(&idle_l3).await?;

        let mut l2_node = helix_mind_core::graph::Node::default();
        l2_node.node_type = helix_mind_core::graph::NodeType::L2;
        l2_node.content = helix_mind_core::graph::NodeContent::Structured(
            std::collections::HashMap::from_iter(vec![
                ("summary".into(), summary.clone()),
            ]),
        );
        l2_node.heat = 0.5;
        l2_node.sensitivity = Some(helix_mind_core::graph::Sensitivity::Public);
        l2_node.derived_from = idle_l3.iter().map(|n| n.id).collect();

        // Check if an L2 with this content already exists
        let content_hash = helix_mind_core::sha256_digest(summary.as_bytes());
        if let Some(existing) = self.storage.lookup_l2_by_hash(&content_hash).await? {
            // Link L3 nodes to existing L2
            for l3_node in &idle_l3 {
                let edge = helix_mind_core::graph::Edge {
                    source_id: existing.id,
                    target_id: l3_node.id,
                    weight: 0.8,
                    relation_type: helix_mind_core::graph::RelationType::Refines,
                    is_soft: false,
                };
                self.storage.add_edge(&edge).await?;
            }
        } else {
            // Save L2 node ID before moving ownership
            let l2_node_id = l2_node.id;
            // Transfer ownership
            self.storage
                .write_node(l2_node, WritePriority::Critical)
                .await?;
            // Use saved ID
            self.storage
                .insert_l2_content_index(&content_hash, &l2_node_id)
                .await?;

            for l3_node in &idle_l3 {
                let edge = helix_mind_core::graph::Edge {
                    source_id: l2_node_id,
                    target_id: l3_node.id,
                    weight: 0.8,
                    relation_type: helix_mind_core::graph::RelationType::Refines,
                    is_soft: false,
                };
                self.storage.add_edge(&edge).await?;
            }
        }

        info!("Crystallized {} L3 nodes into L2 principle", idle_l3.len());
        Ok(())
    }

    async fn summarize_with_llm(
        &self,
        nodes: &[helix_mind_core::graph::Node],
    ) -> Result<String, helix_mind_core::error::MindError> {
        let mut prompt =
            "Summarize the following observations into a single empirical principle:\n".to_string();
        for node in nodes {
            if let helix_mind_core::graph::NodeContent::Text(t) = &node.content {
                prompt.push_str("- ");
                prompt.push_str(t);
                prompt.push_str("\n");
            }
        }
        prompt.push_str("\nPrinciple:");

        let client = reqwest::Client::new();
        let resp: serde_json::Value = client
            .post(&self.config.llm_gateway_url)
            .json(&serde_json::json!({
                "model": "llama3",
                "prompt": prompt,
                "stream": false,
            }))
            .send()
            .await
            .map_err(|e| {
                helix_mind_core::error::MindError::Metabolism(format!(
                    "LLM request failed: {}",
                    e
                ))
            })?
            .json()
            .await
            .map_err(|e| {
                helix_mind_core::error::MindError::Metabolism(format!(
                    "LLM response parse failed: {}",
                    e
                ))
            })?;

        Ok(resp["response"]
            .as_str()
            .unwrap_or("No summary generated")
            .to_string())
    }
}
