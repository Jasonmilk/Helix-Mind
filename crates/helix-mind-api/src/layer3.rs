use super::*;
use tonic::{Request, Response, Status};

pub async fn handle_helix_query(
    service: &HelixMindServiceImpl,
    request: Request<HelixQueryRequest>,
) -> Result<Response<HelixQueryResult>, Status> {
    let req = request.into_inner();
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

    let result = service.retrieval.query(
        &req.query,
        suggested_mode,
        &energy_context,
        req.include_recessive,
        req.allow_imagination,
        autonomy_level,
    ).await.map_err(|e| Status::internal(e.to_string()))?;

    let response = HelixQueryResult {
        effective_mode: match result.effective_mode {
            helix_mind_core::graph::CognitiveMode::Skilled => 0,
            helix_mind_core::graph::CognitiveMode::Anchor => 1,
            helix_mind_core::graph::CognitiveMode::Imagination => 2,
        },
        mode_negotiation: result.mode_negotiation,
        nodes: result.nodes.into_iter().map(super::layer1::convert_node).collect(),
        edges: result.edges.into_iter().map(super::layer1::convert_edge).collect(),
        trace_id: result.trace_id.to_string(),
        latency_ms: result.latency_ms,
        tokens_consumed: result.tokens_consumed,
        is_partial: result.is_partial,
        exhaustion_reason: result.exhaustion_reason,
    };

    Ok(Response::new(response))
}

pub async fn handle_helix_consolidate(
    service: &HelixMindServiceImpl,
    request: Request<HelixConsolidateRequest>,
) -> Result<Response<HelixConsolidateResult>, Status> {
    let req = request.into_inner();
    let success = match req.type_.as_str() {
        "digest" => {
            service.metabolism.trigger_digest().await.map_err(|e| Status::internal(e.to_string()))?;
            true
        }
        "crystallize" => {
            service.metabolism.trigger_crystallize().await.map_err(|e| Status::internal(e.to_string()))?;
            true
        }
        "hibernate" => {
            service.metabolism.trigger_hibernate().await.map_err(|e| Status::internal(e.to_string()))?;
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
    request: Request<FederatedDAGShareRequest>,
) -> Result<Response<FederatedDAGShareResponse>, Status> {
    let req = request.into_inner();
    let target = if req.target_helix_id.is_empty() { None } else { Some(req.target_helix_id) };
    let cid = service.federation.share_dag(target).await.map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(FederatedDAGShareResponse {
        cid,
    }))
}

pub async fn handle_reincarnation(
    service: &HelixMindServiceImpl,
    request: Request<TriggerReincarnationRequest>,
) -> Result<Response<TriggerReincarnationResponse>, Status> {
    let req = request.into_inner();
    let new_generation = service.reincarnation.trigger_reincarnation(&req.confirm_token).await.map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(TriggerReincarnationResponse {
        new_generation,
    }))
}

pub async fn handle_reload_gene_lock(
    service: &HelixMindServiceImpl,
    _request: Request<ReloadGeneLockRequest>,
) -> Result<Response<ReloadGeneLockResponse>, Status> {
    // Reload gene lock from file
    let content = tokio::fs::read_to_string(&service.config.gene_lock.file_path).await.map_err(|e| Status::internal(e.to_string()))?;
    let lock = helix_mind_core::graph::L0GeneLock::from_markdown(&content).map_err(|e| Status::internal(e.to_string()))?;

    // Write audit log
    let audit = helix_mind_core::audit::AuditEntry::new(
        helix_mind_core::audit::AuditEventType::GeneLockReloaded,
        "api",
        &format!("Gene lock reloaded, hash: {}", lock.l0_hash),
    );
    service.storage.write_audit(&audit).await.map_err(|e| Status::internal(e.to_string()))?;

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
    // Get all public nodes
    let nodes = service.storage.get_nodes_by_type(helix_mind_core::graph::NodeType::L3).await.map_err(|e| Status::internal(e.to_string()))?;
    let public_nodes: Vec<_> = nodes.into_iter()
        .filter(|n| n.sensitivity == Some(helix_mind_core::graph::Sensitivity::Public))
        .collect();

    // Sync
    let sync = helix_mind_storage::human_view::HumanViewSync::new(
        &service.config.storage.human_view_dir,
        service.config.storage.human_view_max_size_mb,
    );
    let conflicts = sync.sync(&public_nodes).await.map_err(|e| Status::internal(e.to_string()))?;

    // Write audit log
    let audit = helix_mind_core::audit::AuditEntry::new(
        helix_mind_core::audit::AuditEventType::HumanViewSynced,
        "api",
        "Human view synced",
    );
    service.storage.write_audit(&audit).await.map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(SyncHumanViewResponse {
        success: true,
        conflicts,
    }))
}
