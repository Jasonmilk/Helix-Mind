use crate::proto::*;
use super::*;
use tonic::{Request, Response, Status};

pub async fn handle_helix_query(
    service: &HelixMindServiceImpl,
    request: Request<HelixQueryRequest>,
) -> Result<Response<HelixQueryResult>, Status> {
    let req = request.into_inner();

    // ── Passive lifecycle check (Iron Law #13: No Heartbeat) ──────
    match service.reincarnation.check_lifecycle().await {
        Ok(Some(warning)) => {
            let msg = match warning {
                helix_mind_reincarnation::LifecycleWarning::TimeLimitReached { elapsed, max } => {
                    format!("Lifecycle warning: time limit reached ({}/{} days)", elapsed, max)
                }
            };
            tracing::warn!("{}", msg);
        }
        Err(e) => tracing::error!("Lifecycle check failed: {}", e),
        _ => {}
    }

    let suggested_mode = match req.suggested_mode {
        0 => helix_mind_core::graph::CognitiveMode::Skilled,
        1 => helix_mind_core::graph::CognitiveMode::Anchor,
        2 => helix_mind_core::graph::CognitiveMode::Imagination,
        _ => return Err(Status::invalid_argument("Invalid cognitive mode")),
    };

    let energy_context = if let Some(ec) = req.energy_context {
        helix_mind_core::graph::EnergyContext {
            token_budget: ec.token_budget,
            heliotropism: ec.heliotropism,
            pulse: ec.pulse,
            vigilance: ec.vigilance,
            latency_limit_ms: ec.latency_limit_ms,
            system_load: ec.system_load,
            familiarity: ec.familiarity,
            impasse_depth: ec.impasse_depth as u8,
            // P0 (ADR-0010): budget tier from the body (front routing)
            budget_tier: match ec.budget_tier {
                0 => helix_mind_core::graph::BudgetTier::Augmentable,
                1 => helix_mind_core::graph::BudgetTier::Endogenous,
                2 => helix_mind_core::graph::BudgetTier::ExogenousRequired,
                _ => helix_mind_core::graph::BudgetTier::Void,
            },
        }
    } else {
        helix_mind_core::graph::EnergyContext::default()
    };

    let autonomy_level = match req.autonomy_level {
        0 => helix_mind_core::graph::AutonomyLevel::Agent,
        1 => helix_mind_core::graph::AutonomyLevel::Open,
        2 => helix_mind_core::graph::AutonomyLevel::Survival,
        _ => return Err(Status::invalid_argument("Invalid autonomy level")),
    };

    let result = service
        .retrieval
        .query(
            &req.query,
            suggested_mode,
            &energy_context,
            req.include_recessive,
            req.allow_imagination,
            autonomy_level,
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    let response = HelixQueryResult {
        effective_mode: match result.effective_mode {
            helix_mind_core::graph::CognitiveMode::Skilled => 0,
            helix_mind_core::graph::CognitiveMode::Anchor => 1,
            helix_mind_core::graph::CognitiveMode::Imagination => 2,
        },
        mode_negotiation: result.mode_negotiation.unwrap_or_default(),
        nodes: result
            .nodes
            .into_iter()
            .map(super::layer1::convert_node)
            .collect(),
        edges: result
            .edges
            .into_iter()
            .map(super::layer1::convert_edge)
            .collect(),
        trace_id: result.trace_id.to_string(),
        // P3c (ADR-0020): traceparent pass-through — echo the request value,
        // never generate. Empty in → empty out (Mind is not the trace root).
        traceparent: req.traceparent.clone(),
        latency_ms: result.latency_ms,
        tokens_consumed: result.tokens_consumed,
        is_partial: result.is_partial,
        exhaustion_reason: result.exhaustion_reason.unwrap_or_default(),
        impasse_level: result.impasse_level as i32,
        stages_attempted: result.stages_attempted as i32,
        suggested_actions: result
            .suggested_actions
            .into_iter()
            .map(|a| SuggestedAction {
                action_type: a.action_type,
                parameters: a.parameters.to_string(),
                reason: a.reason,
            })
            .collect(),
        // P4 M-10: activation_vector mapping (frozen-contract payoff).
        activation_vector: result
            .activation_vector
            .into_iter()
            .map(|a| ActivationEntry {
                node_id: a.node_id.to_string(),
                activation: a.activation,
            })
            .collect(),
    };

    Ok(Response::new(response))
}

pub async fn handle_helix_consolidate(
    service: &HelixMindServiceImpl,
    request: Request<HelixConsolidateRequest>,
) -> Result<Response<HelixConsolidateResult>, Status> {
    let req = request.into_inner();
    let success = match req.r#type.as_str() {
        "digest" => {
            service
                .metabolism
                .trigger_digest()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            true
        }
        "crystallize" => {
            service
                .metabolism
                .trigger_crystallize()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            true
        }
        "hibernate" => {
            service
                .metabolism
                .trigger_hibernate()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            true
        }
        _ => return Err(Status::invalid_argument("Invalid consolidation type")),
    };
    Ok(Response::new(HelixConsolidateResult {
        success,
        message: "Consolidation completed".into(),
    }))
}

pub async fn handle_federated_share(
    service: &HelixMindServiceImpl,
    request: Request<FederatedDagShareRequest>,
) -> Result<Response<FederatedDagShareResponse>, Status> {
    let req = request.into_inner();
    let target = if req.target_helix_id.is_empty() {
        None
    } else {
        Some(req.target_helix_id)
    };
    let cid = service
        .federation
        .share_dag(target)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(Response::new(FederatedDagShareResponse { cid }))
}

pub async fn handle_reincarnation(
    service: &HelixMindServiceImpl,
    request: Request<TriggerReincarnationRequest>,
) -> Result<Response<TriggerReincarnationResponse>, Status> {
    let req = request.into_inner();
    let new_generation = service
        .reincarnation
        .trigger_sunset(&req.confirm_token, "No note left.")
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(Response::new(TriggerReincarnationResponse { new_generation }))
}

pub async fn handle_reload_gene_lock(
    service: &HelixMindServiceImpl,
    _request: Request<ReloadGeneLockRequest>,
) -> Result<Response<ReloadGeneLockResponse>, Status> {
    let content = tokio::fs::read_to_string(&service.config.gene_lock.file_path)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let lock = helix_mind_core::graph::L0GeneLock::from_markdown(&content)
        .map_err(|e| Status::internal(e.to_string()))?;

    let audit = helix_mind_core::graph::AuditEntry::new(
        helix_mind_core::graph::AuditEventType::GeneLockReloaded,
        "api",
        &format!("Gene lock reloaded, hash: {}", lock.l0_hash),
    );
    service
        .storage
        .write_audit(&audit)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(ReloadGeneLockResponse {
        l0_hash: lock.l0_hash,
        lineage_name: lock.lineage_name,
        core_principles: lock.core_principles,
    }))
}

pub async fn handle_sync_human_view(
    service: &HelixMindServiceImpl,
    request: Request<SyncHumanViewRequest>,
) -> Result<Response<SyncHumanViewResponse>, Status> {
    let _req = request.into_inner();

    let nodes = service
        .storage
        .get_nodes_by_type(helix_mind_core::graph::NodeType::L3)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let public_nodes: Vec<_> = nodes
        .into_iter()
        .filter(|n| n.sensitivity == Some(helix_mind_core::graph::Sensitivity::Public))
        .collect();

    let sync = helix_mind_storage::human_view::HumanViewSync::new(
        &service.config.storage.human_view_dir,
        service.config.storage.human_view_max_size_mb,
    );
    let conflicts = sync
        .sync(&public_nodes)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    let audit = helix_mind_core::graph::AuditEntry::new(
        helix_mind_core::graph::AuditEventType::HumanViewSynced,
        "api",
        "Human view synced",
    );
    service
        .storage
        .write_audit(&audit)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(SyncHumanViewResponse {
        success: true,
        conflicts,
    }))
}
