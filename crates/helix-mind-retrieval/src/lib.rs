pub mod mode;

use helix_mind_storage::WritePriority;
use helix_mind_core::graph::*;
use helix_mind_core::config::RetrievalConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct RetrievalEngine {
    config: RetrievalConfig,
    storage: Arc<StorageEngine>,
}

impl RetrievalEngine {
    pub fn new(config: RetrievalConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    /// Main query entry
    pub async fn query(
        &self,
        query: &str,
        suggested_mode: CognitiveMode,
        energy_context: &EnergyContext,
        include_recessive: bool,
        allow_imagination: bool,
        autonomy_level: AutonomyLevel,
    ) -> Result<HelixQueryResult, helix_mind_core::error::MindError> {
        let start = Utc::now();
        let trace_id = Uuid::new_v4();

        // 1. Negotiate mode based on energy context
        let (effective_mode, negotiation_note) = self.negotiate_mode(
            suggested_mode,
            energy_context,
            allow_imagination,
            autonomy_level,
        );

        // 2. Extract start nodes from query
        let start_ids = self.extract_start_nodes(query).await?;
        if start_ids.is_empty() {
            // No start nodes, return empty
            return Ok(HelixQueryResult {
                effective_mode,
                mode_negotiation: Some(negotiation_note),
                nodes: Vec::new(),
                edges: Vec::new(),
                trace_id,
                latency_ms: (Utc::now() - start).num_milliseconds() as u64,
                tokens_consumed: 0,
                is_partial: false,
                exhaustion_reason: None,
            });
        }

        // 3. Run retrieval based on mode
        let (node_ids, is_partial, exhaustion_reason) = match effective_mode {
            CognitiveMode::Skilled => {
                self.storage.skilled_retrieve(
                    &start_ids,
                    self.config.beam_width,
                    self.config.weight_threshold,
                    energy_context.token_budget,
                    self.config.max_nodes_per_query,
                ).await?
            }
            CognitiveMode::Anchor => {
                self.storage.anchor_retrieve(
                    &start_ids,
                    None, // TODO: vector embedding
                    self.config.beam_width,
                    self.config.weight_threshold,
                    energy_context.token_budget,
                    self.config.max_nodes_per_query,
                ).await?
            }
            CognitiveMode::Imagination => {
                if !allow_imagination {
                    return Err(helix_mind_core::error::MindError::Validation("Imagination mode not allowed".into()));
                }
                self.storage.imagination_retrieve(
                    &start_ids,
                    energy_context.pulse,
                    energy_context.token_budget,
                    self.config.max_nodes_per_query,
                ).await?
            }
        };

        // 4. Load full nodes
        let nodes = self.storage.get_nodes_by_ids(&node_ids).await?;
        let edges = self.storage.get_edges_between(&node_ids).await?;

        // 5. Update access counts
        for node in &nodes {
            let mut node_clone = node.clone();
            node_clone.last_accessed_at = Utc::now();
            node_clone.access_count += 1;
            self.storage.write_node(node_clone, WritePriority::Critical).await?;
        }

        let latency_ms = (Utc::now() - start).num_milliseconds() as u64;
        let tokens_consumed = node_ids.len() as u64;

        Ok(HelixQueryResult {
            effective_mode,
            mode_negotiation: Some(negotiation_note),
            nodes,
            edges,
            trace_id,
            latency_ms,
            tokens_consumed,
            is_partial,
            exhaustion_reason,
        })
    }

    /// Negotiate cognitive mode based on energy context
    fn negotiate_mode(
        &self,
        suggested: CognitiveMode,
        energy: &EnergyContext,
        allow_imagination: bool,
        autonomy: AutonomyLevel,
    ) -> (CognitiveMode, String) {
        // If system load is high, degrade to lower mode
        if energy.system_load > 0.9 {
            return (CognitiveMode::Skilled, "Degraded to Skilled due to high system load".into());
        }
        // If latency limit is small, use fastest mode
        if energy.latency_limit_ms < 100 {
            return (CognitiveMode::Skilled, "Degraded to Skilled due to latency constraint".into());
        }
        // If token budget is small, use Skilled
        if energy.token_budget < 100 {
            return (CognitiveMode::Skilled, "Degraded to Skilled due to token budget".into());
        }
        // If survival mode, only Skilled
        if autonomy == AutonomyLevel::Survival {
            return (CognitiveMode::Skilled, "Survival mode: only Skilled available".into());
        }
        // If imagination not allowed, can't use it
        if !allow_imagination && suggested == CognitiveMode::Imagination {
            return (CognitiveMode::Anchor, "Imagination not allowed, falling back to Anchor".into());
        }
        // Otherwise use suggested
        (suggested, "Using suggested mode".into())
    }

    /// Extract start nodes from query text
    async fn extract_start_nodes(&self, _query: &str) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        // TODO: Use NER to extract entities from query, map to node IDs
        // For now, just return empty, will be implemented later
        Ok(Vec::new())
    }
}
