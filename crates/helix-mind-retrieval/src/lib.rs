pub mod adapter;
pub mod fts_extractor;

pub use adapter::{StartNodeExtractor, FakeAdapter, EmptyExtractor};
pub use fts_extractor::{FtsExtractor, sanitize_query, escape_fts};

use helix_mind_core::graph::*;
use helix_mind_core::config::RetrievalConfig;
use helix_mind_core::sa_core::SaCoreParams;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct RetrievalEngine {
    config: RetrievalConfig,
    storage: Arc<StorageEngine>,
    /// Start-node extraction seam (P0.5: FakeAdapter in tests; P1: FTS5).
    extractor: Arc<dyn StartNodeExtractor>,
    /// Track the current impasse depth across queries.
    impasse_depth: tokio::sync::RwLock<u8>,
    /// Track recent query embeddings for familiarity estimation.
    #[allow(dead_code)]
    recent_embeddings: tokio::sync::RwLock<Vec<Vec<f32>>>,
}

/// Whether energy constraints force the cognitive mode down to Skilled.
///
/// Named pure function: thresholds are read from `RetrievalConfig`
/// (config-overridable, zero hardcoding) and the predicate carries no state,
/// so it is testable without constructing a `RetrievalEngine`.
pub fn energy_degraded(energy: &EnergyContext, cfg: &RetrievalConfig) -> bool {
    energy.system_load > cfg.high_system_load
        || energy.latency_limit_ms < cfg.min_latency_limit_ms
        || energy.token_budget < cfg.min_token_budget
}


/// Map SA-Core activations into the protocol's `ActivationEntry` list.
///
/// Pure and order-preserving: the diffusion returns each node with its final
/// activation for THIS cycle, and the white-box contract is "what SA-Core
/// actually chose" — so the order it produced is the order reported.
fn activation_entries(pairs: &[(Uuid, f64)]) -> Vec<ActivationEntry> {
    pairs
        .iter()
        .map(|(node_id, activation)| ActivationEntry {
            node_id: *node_id,
            activation: *activation,
        })
        .collect()
}

impl RetrievalEngine {
    pub fn new(config: RetrievalConfig, storage: Arc<StorageEngine>) -> Self {
        // P1 (M-01): the real FTS5-trigram extractor is the production default.
        // Tests override via `with_extractor` (FakeAdapter / EmptyExtractor).
        let extractor = Arc::new(FtsExtractor::new(
            &storage,
            config.max_nodes_per_query,
            config.stopwords.clone(),
        ));
        Self::with_extractor(config, storage, extractor)
    }

    /// Construct with an explicit start-node extractor.
    /// P0.5: inject `FakeAdapter` in tests; P1: inject the real FTS5 extractor.
    pub fn with_extractor(
        config: RetrievalConfig,
        storage: Arc<StorageEngine>,
        extractor: Arc<dyn StartNodeExtractor>,
    ) -> Self {
        Self {
            config,
            storage,
            extractor,
            impasse_depth: tokio::sync::RwLock::new(0),
            recent_embeddings: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    /// Main query entry — implements the five-stage impasse escalation model.
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
        let mut stages_attempted = 1_u8;

        // Read current impasse depth
        let current_impasse = *self.impasse_depth.read().await;

        // Decision before instrument: the negotiated mode selects WHICH SA-Core
        // parameter set the retrieval runs with, so it must be resolved before
        // retrieval begins. It used to be negotiated last while each stage
        // hardcoded its own mode, so a query could report
        // `effective_mode: Imagination` having actually diffused with Skilled's
        // α (0.5, soft edges disabled). The reported mode and the executed mode
        // disagreed, and the reported one was the lie.
        //
        // Pure reordering: `negotiate_mode` reads `current_impasse`, which is read
        // on the line above from the *previous* cycle, so lifting the call leaves
        // every existing result identical.
        let (effective_mode, negotiation_note) = self.negotiate_mode(
            suggested_mode,
            energy_context,
            allow_imagination,
            autonomy_level,
            current_impasse,
        );

        // 1. Extract start nodes from query
        let start_ids = self.extract_start_nodes(query).await?;
        if start_ids.is_empty() {
            // No entry point at all. Report the mode that was really negotiated:
            // this branch used to hardcode `Anchor`, telling the body "Mind chose
            // Anchor" when Mind had chosen something else. A degenerate result
            // must describe the real decision, not invent one.
            return Ok(HelixQueryResult {
                effective_mode,
                mode_negotiation: Some(format!(
                    "{negotiation_note}; no start nodes found for this query"
                )),
                nodes: Vec::new(),
                edges: Vec::new(),
                trace_id,
                latency_ms: (Utc::now() - start).num_milliseconds() as u64,
                tokens_consumed: 0,
                is_partial: false,
                exhaustion_reason: None,
                impasse_level: ImpasseLevel::None,
                stages_attempted: 0,
                suggested_actions: Vec::new(),
                // No start nodes -> nothing to seed, so genuinely empty. (The
                // other sites used to say the same thing and were wrong: the
                // algorithm existed, the call was missing.)
                activation_vector: Vec::new(),
            });
        }

        // 2. Stage 1: Local Dominant Retrieval
        // SA-Core (2026-09-17): the activation the diffusion already computes now
        // travels with the stage's node ids instead of being dropped one layer
        // down. Every result used to carry `activation_vector: Vec::new()` under a
        // comment claiming the algorithm was "not yet implemented" — it WAS
        // implemented (storage::topology::sa_core_diffusion), and every layer above
        // already carried it: the storage API returned it, proto field 13 reserved
        // a seat for it, and helix-mind-api already mapped it. The retrieval call
        // was the one layer missing.
        let (node_ids, activations, is_partial, exhaustion_reason) = self.stage_local_dominant(
            &start_ids,
            energy_context,
            effective_mode,
        ).await?;

        // If satisfied (got enough results), return directly
        if !node_ids.is_empty() && !self.is_impasse_triggered(&node_ids, query).await? {
            let nodes = self.storage.get_nodes_by_ids(&node_ids).await?;
            let edges = self.storage.get_edges_between(&node_ids).await?;
            self.update_access_counts(&nodes).await?;
            // Decrease impasse depth on success
            self.adjust_impasse(-1).await;
            let latency_ms = (Utc::now() - start).num_milliseconds() as u64;
            return Ok(HelixQueryResult {
                effective_mode,
                mode_negotiation: Some(negotiation_note),
                nodes,
                edges,
                trace_id,
                latency_ms,
                tokens_consumed: node_ids.len() as u64,
                is_partial,
                exhaustion_reason,
                impasse_level: ImpasseLevel::None,
                stages_attempted,
                suggested_actions: Vec::new(),
                activation_vector: activation_entries(&activations),
            });
        }

        // 3. Stage 2: Shared Knowledge Tree Query (placeholder for Phase 4)
        stages_attempted = 2;
        let mut all_node_ids = node_ids.clone();
        let shared_ids = self.stage_shared_tree(query, energy_context).await?;
        if !shared_ids.is_empty() {
            all_node_ids.extend(shared_ids);
            if !self.is_impasse_triggered(&all_node_ids, query).await? {
                let nodes = self.storage.get_nodes_by_ids(&all_node_ids).await?;
                let edges = self.storage.get_edges_between(&all_node_ids).await?;
                self.update_access_counts(&nodes).await?;
                // Decrease impasse depth on success
                self.adjust_impasse(-1).await;
                let latency_ms = (Utc::now() - start).num_milliseconds() as u64;
                return Ok(HelixQueryResult {
                    effective_mode,
                    mode_negotiation: Some(negotiation_note),
                    nodes,
                    edges,
                    trace_id,
                    latency_ms,
                    tokens_consumed: all_node_ids.len() as u64,
                    is_partial: false,
                    exhaustion_reason: None,
                    impasse_level: ImpasseLevel::None,
                    stages_attempted,
                    suggested_actions: Vec::new(),
                    activation_vector: activation_entries(&activations),
                });
            }
        }

        // 4. Stage 3: Spiral Re-examination (dialectical edges)
        stages_attempted = 3;
        let spiral_ids = self.stage_spiral_re_examination(&all_node_ids).await?;
        if !spiral_ids.is_empty() {
            all_node_ids.extend(spiral_ids);
        }

        // 5. Stage 4: Recessive Breakthrough (include recessive)
        let recessive_ids = if include_recessive || current_impasse >= 3 {
            stages_attempted = 4;
            self.stage_recessive_breakthrough(
                &start_ids,
                energy_context,
            ).await?
        } else {
            Vec::new()
        };
        if !recessive_ids.is_empty() {
            all_node_ids.extend(recessive_ids);
        }

        // 6. Stage 5: Imagination Leap (if allowed and still stuck)
        if allow_imagination && (current_impasse >= 4 || all_node_ids.is_empty()) {
            stages_attempted = 5;
            let imaginative_ids = self.stage_imagination_leap(
                &start_ids,
                energy_context,
            ).await?;
            if !imaginative_ids.is_empty() {
                all_node_ids.extend(imaginative_ids);
            }
        }

        // Determine final impasse level
        let impasse_level = if all_node_ids.is_empty() {
            // Escalate impasse
            self.adjust_impasse(1).await;
            if current_impasse >= 4 {
                ImpasseLevel::RecessiveNoBreakthrough
            } else if current_impasse >= 3 {
                ImpasseLevel::SpiralExhausted
            } else if current_impasse >= 2 {
                ImpasseLevel::SharedTreeNoMatch
            } else {
                ImpasseLevel::LocalDominantFailed
            }
        } else {
            // Success — reduce impasse
            self.adjust_impasse(-1).await;
            ImpasseLevel::None
        };

        let nodes = self.storage.get_nodes_by_ids(&all_node_ids).await?;
        let edges = self.storage.get_edges_between(&all_node_ids).await?;
        self.update_access_counts(&nodes).await?;

        let latency_ms = (Utc::now() - start).num_milliseconds() as u64;
        Ok(HelixQueryResult {
            effective_mode,
            mode_negotiation: Some(negotiation_note),
            nodes,
            edges,
            trace_id,
            latency_ms,
            tokens_consumed: all_node_ids.len() as u64,
            is_partial: false,
            exhaustion_reason: None,
            impasse_level,
            stages_attempted,
            suggested_actions: Vec::new(),
            activation_vector: activation_entries(&activations),
        })
    }

    // ── Stage 1: Local Dominant ─────────────────────────────────────
    async fn stage_local_dominant(
        &self,
        start_ids: &[Uuid],
        energy: &EnergyContext,
        mode: CognitiveMode,
    ) -> Result<(Vec<Uuid>, Vec<(Uuid, f64)>, bool, Option<String>), helix_mind_core::error::MindError> {
        // The *negotiated* mode selects the parameter set — this is where the
        // mode stops being a label and becomes an instrument:
        //   Skilled     α 0.5, soft edges disabled (SOFT_EDGES_DISABLED = 0.0)
        //               — only already-converged hard edges, no exploration
        //   Anchor      α 0.7, soft edges × 0.8  — 接驳: bounded off-rail spread
        //   Imagination α 0.9, soft edges × 0.95 — 漫游: wide, soft-edge-led
        // `for_mode` always had all three arms (sa_core.rs §for_mode). The
        // retrieval path only ever reached the Skilled one because the mode was
        // negotiated *after* this stage, which left `alpha_anchor`,
        // `alpha_imagination` and `decay_imagination` as config knobs with no
        // consumer here — the same "declared but no entry point" defect the
        // project keeps hitting. Now the mode that gets reported is the mode
        // that runs.
        let params = SaCoreParams::for_mode(mode, energy.heliotropism, &self.config);
        // Dispatch to the mode's *own* instrument. This is the shape the whole
        // seam was built for: storage exposes `skilled_retrieve` /
        // `anchor_retrieve` / `imagination_retrieve`, each naming a traverse,
        // and only `skilled_retrieve` was ever reachable from a query.
        //
        // `skilled_traverse` and `anchor_traverse` are byte-identical today
        // (both are `sa_core_diffusion(.., None)`), so routing Anchor through
        // `anchor_retrieve` is behaviour-preserving *today*. It is still the
        // honest routing: it stops being a silent lie the day the two diverge,
        // and it is what makes "the mode picks the instrument" a fact instead of
        // an aspiration. `imagination_traverse` is the one that genuinely
        // differs — it relaxes the relative gate by `temperature`.
        let (ids, activations, partial, reason) = match mode {
            CognitiveMode::Skilled => {
                self.storage.skilled_retrieve(
                    start_ids,
                    &params,
                    self.config.max_hops,
                    energy.token_budget,
                    self.config.max_nodes_per_query,
                ).await?
            }
            CognitiveMode::Anchor => {
                self.storage.anchor_retrieve(
                    start_ids,
                    None,
                    &params,
                    self.config.max_hops,
                    energy.token_budget,
                    self.config.max_nodes_per_query,
                ).await?
            }
            CognitiveMode::Imagination => {
                self.storage.imagination_retrieve(
                    start_ids,
                    energy.pulse,
                    &params,
                    self.config.max_hops,
                    energy.token_budget,
                    self.config.max_nodes_per_query,
                ).await?
            }
        };
        Ok((ids, activations, partial, reason))
    }

    // ── Stage 2: Shared Knowledge Tree ──────────────────────────────
    async fn stage_shared_tree(
        &self,
        _query: &str,
        _energy: &EnergyContext,
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        // TODO: query Rhizax network for shared knowledge tree nodes
        // Will be implemented in Phase 4 when federation is connected
        Ok(Vec::new())
    }

    // ── Stage 3: Spiral Re-examination ───────────────────────────────
    async fn stage_spiral_re_examination(
        &self,
        current_ids: &[Uuid],
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        // Traverse dialectical edges backwards (Corrects, Refines, Doubts)
        // to find previously overthrown knowledge_base nodes
        let mut spiral_ids = Vec::new();
        for id in current_ids {
            let edges = self.storage.get_edges_between(&[*id]).await?;
            for edge in &edges {
                if edge.target_id == *id {
                    match edge.relation_type {
                        RelationType::Corrects
                        | RelationType::Refines
                        | RelationType::Doubts => {
                            spiral_ids.push(edge.source_id);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(spiral_ids)
    }

    // ── Stage 4: Recessive Breakthrough ─────────────────────────────
    async fn stage_recessive_breakthrough(
        &self,
        start_ids: &[Uuid],
        energy: &EnergyContext,
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        // Historical note: this stage used to pass an absolute `0.3` threshold
        // with the comment "lower threshold to catch recessive". That was doubly
        // wrong — (a) an absolute threshold is not scale-invariant (ADR-0042 D0),
        // and (b) it could never include a recessive node anyway, because
        // `sa_core_diffusion` hard-filters `is_recessive` at collection time.
        // Recessive inclusion is a protocol-level decision (`include_recessive`
        // on the request), not a threshold side effect.
        let params = SaCoreParams::for_mode(CognitiveMode::Anchor, energy.heliotropism, &self.config);
        let (ids, _, _, _) = self.storage.anchor_retrieve(
            start_ids,
            None,
            &params,
            self.config.max_hops,
            energy.token_budget / 2,
            self.config.max_nodes_per_query / 2,
        ).await?;
        Ok(ids)
    }

    // ── Stage 5: Imagination Leap ───────────────────────────────────
    async fn stage_imagination_leap(
        &self,
        start_ids: &[Uuid],
        energy: &EnergyContext,
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        // High-temperature, unfiltered exploration. `energy.pulse` modulates the
        // RELATIVE gate (higher temperature admits weaker activations) instead of
        // the old absolute `(0.01 * (1 - temperature)).max(0.001)`.
        let params =
            SaCoreParams::for_mode(CognitiveMode::Imagination, energy.heliotropism, &self.config);
        let (ids, _, _, _) = self.storage.imagination_retrieve(
            start_ids,
            energy.pulse,
            &params,
            self.config.max_hops,
            energy.token_budget / 2,
            self.config.max_nodes_per_query / 2,
        ).await?;
        Ok(ids)
    }

    // ── Impasse Depth Management ───────────────────────────────────
    async fn adjust_impasse(&self, delta: i8) {
        let mut depth = self.impasse_depth.write().await;
        if delta > 0 {
            *depth = (*depth + delta as u8).min(5);
        } else {
            *depth = depth.saturating_sub((-delta) as u8);
        }
    }

    async fn is_impasse_triggered(
        &self,
        node_ids: &[Uuid],
        _query: &str,
    ) -> Result<bool, helix_mind_core::error::MindError> {
        if node_ids.is_empty() {
            return Ok(true); // no results = impasse
        }
        // If results are few and query seems complex, trigger impasse
        if node_ids.len() < 3 {
            // Check if results are highly relevant (simplistic for now)
            return Ok(true);
        }
        // TODO: more sophisticated impasse detection based on result quality
        Ok(false)
    }

    // ── Negotiate mode ───────────────────────────────────────────────
    fn negotiate_mode(
        &self,
        suggested: CognitiveMode,
        energy: &EnergyContext,
        allow_imagination: bool,
        autonomy: AutonomyLevel,
        impasse_depth: u8,
    ) -> (CognitiveMode, String) {
        // If in deep impasse, consider escalating to Imagination
        if impasse_depth >= 4 && allow_imagination {
            return (CognitiveMode::Imagination, "Deep impasse: escalating to Imagination".into());
        }

        // If in moderate impasse, use Anchor for broader search
        if impasse_depth >= 2 {
            return (CognitiveMode::Anchor, "Moderate impasse: using Anchor mode".into());
        }

        // Standard negotiation based on energy. Thresholds come from
        // RetrievalConfig (zero hardcoding); the predicate is a named pure
        // function so it can be tested without building an engine.
        if energy_degraded(energy, &self.config) {
            return (CognitiveMode::Skilled, "Degraded to Skilled due to energy constraints".into());
        }

        if autonomy == AutonomyLevel::Survival {
            return (CognitiveMode::Skilled, "Survival mode: only Skilled available".into());
        }

        if !allow_imagination && suggested == CognitiveMode::Imagination {
            return (CognitiveMode::Anchor, "Imagination not allowed, falling back to Anchor".into());
        }

        (suggested, "Using suggested mode".into())
    }

    // ── Extract start nodes from query ──────────────────────────────
    async fn extract_start_nodes(
        &self,
        query: &str,
    ) -> Result<Vec<Uuid>, helix_mind_core::error::MindError> {
        // P0.5: the extractor seam replaces the former empty stub. Default
        // `EmptyExtractor` keeps the honest empty baseline; tests inject the
        // deterministic `FakeAdapter`. P1 (M-01) lands FTS5-trigram here.
        Ok(self.extractor.extract_start_nodes(query))
    }

    // ── Update access counts for retrieved nodes ────────────────────
    /// F3 (P2a 前置修复): batch the access-count bump into a single transaction
    /// (atomic SQL increment) instead of a per-node Critical write. Soft signal,
    /// not the truth source — durability deliberately relaxed.
    async fn update_access_counts(
        &self,
        nodes: &[Node],
    ) -> Result<(), helix_mind_core::error::MindError> {
        let ids: Vec<Uuid> = nodes.iter().map(|n| n.id).collect();
        self.storage.bump_access_counts(&ids).await
    }
}
