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
            // P10c (ADR-0031 D3): Deep Dream chain — forget cold L3 first,
            // then review L1 strategy coverage and adapt the mutation rate
            // (deterministic, 0 tokens). Review failure degrades the pass,
            // never the forget path (physical facts first).
            service
                .metabolism
                .trigger_hibernate()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            let report = super::sleep_review::run_sleep_review(&service.storage)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            tracing::info!(
                "[consolidate:hibernate] sleep review: compared={} legacy={} review={} verdict={:?} rate={:.4}",
                report.compared,
                report.legacy_coverage,
                report.review_coverage,
                report.verdict,
                report.mutation_rate,
            );
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

/// P10 (ADR-0031): cognitive craft — Anaphase-triggered orchestration.
///
/// Retrieval (helix_query) and orchestration (helix_craft) stay decoupled:
/// this handler maps the wire request onto the cognitive crate's
/// `CognitiveCraft` and returns the orchestration result. Deterministic
/// trace_id (`craft#{job_id}`) comes from the caller's job id, never a UUID.
pub async fn handle_helix_craft(
    service: &HelixMindServiceImpl,
    request: Request<HelixCraftRequest>,
) -> Result<Response<HelixCraftResult>, Status> {
    use helix_mind_cognitive::{CraftInput, Process, ProcessStep, Mode};

    let req = request.into_inner();

    // Parse process+mode pairs; unknown values rejected (fail-closed).
    let mut steps = Vec::with_capacity(req.steps.len());
    for s in &req.steps {
        let process = match s.process.as_str() {
            "structural" => Process::Structural,
            "critical" => Process::Critical,
            "creative" => Process::Creative,
            "situational" => Process::Situational,
            "meta_critical" => Process::MetaCritical,
            other => return Err(Status::invalid_argument(format!("Unknown process: {other}"))),
        };
        let mode = match s.mode.as_str() {
            "skilled" => Mode::Skilled,
            "anchored" => Mode::Anchored,
            "imaginative" => Mode::Imaginative,
            other => return Err(Status::invalid_argument(format!("Unknown mode: {other}"))),
        };
        steps.push(ProcessStep::new(process, mode));
    }

    // P10b provenance derived BEFORE the input move (deterministic,
    // same job_id → same craft#{job_id} → idempotent replay).
    let provenance = format!("craft#{}", req.job_id);
    let input = CraftInput {
        query: req.query,
        steps,
        global_constraints: req.global_constraints,
        job_id: req.job_id.clone(),
    };

    let result = service
        .cognitive
        .orchestrate(input)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    // P10b (ADR-0031 D2): synthesis → L1 strategy node. Append-only via the
    // existing storage pipeline (WAL + SQLite + FTS — no new store), idempotent
    // by provenance (see above). Retrieval reuse (T3) is automatic: L1 nodes
    // enter the same FTS index as any other node — retrieval filters no type.
    let grade = helix_mind_cognitive::ValueAssessor.assess(&result.synthesis);
    let existing = service
        .storage
        .get_nodes_by_type(helix_mind_core::graph::NodeType::L1)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let already = existing
        .iter()
        .any(|n| n.abstract_provenance.as_deref() == Some(provenance.as_str()));
    if !already {
        // Deterministic node id (name-based, DNA principle 11): same provenance
        // derives the same id — replay writes the SAME node, never a twin.
        let node_id = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, provenance.as_bytes());
        let node = helix_mind_core::graph::Node {
            id: node_id,
            node_type: helix_mind_core::graph::NodeType::L1,
            content: helix_mind_core::graph::NodeContent::Text(result.synthesis.clone()),
            abstract_provenance: Some(provenance.clone()),
            notes: Some(format!("value_grade={grade:?}")),
            high_risk: false,
            ..Default::default()
        };
        service
            .storage
            .write_node(node, helix_mind_storage::WritePriority::Deferred)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
    }

    let response = HelixCraftResult {
        trace_id: result.trace_id,
        steps: result
            .steps
            .into_iter()
            .map(|o| StepOutput {
                process: format!("{:?}", o.process).to_ascii_lowercase(),
                content: o.content,
            })
            .collect(),
        synthesis: result.synthesis,
        // P10b (ADR-0031 D3): value grade from the deterministic assessor,
        // echoed back and persisted as node metadata.
        value_grade: format!("{grade:?}"),
        tokens_consumed: 0,
        // P3c (ADR-0020): traceparent pass-through — echo the request value,
        // never generate. Empty in → empty out (Mind is not the trace root).
        traceparent: req.traceparent.clone(),
    };

    Ok(Response::new(response))
}

// P10d (ADR-0032): ana_wakeup ack — close or renew a claimed alarm.
pub async fn handle_ana_wakeup_ack(
    service: &HelixMindServiceImpl,
    request: Request<AnaWakeupAckRequest>,
) -> Result<Response<AnaWakeupAckResult>, Status> {
    let req = request.into_inner();
    let ok = super::alarm::ack_alarm(&service.storage, &req.claim_id, &req.status)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(Response::new(AnaWakeupAckResult {
        success: ok,
        message: if ok {
            "alarm acknowledged".into()
        } else {
            "unknown claim or status".into()
        },
    }))
}

// P10d (ADR-0032): ana_wakeup — list due alarms (atomically claimed).
pub async fn handle_ana_wakeup(
    service: &HelixMindServiceImpl,
    request: Request<AnaWakeupRequest>,
) -> Result<Response<AnaWakeupResult>, Status> {
    let req = request.into_inner();
    let alarms = super::alarm::list_due_alarms(&service.storage, req.jitter_minutes)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let count = alarms.len() as u32;
    let due = alarms
        .into_iter()
        .map(|a| AlarmDue {
            job_id: a.job_id,
            action: a.action,
            due_at: a.due_at,
            mode: a.mode,
            claim_id: a.claim_id,
        })
        .collect();
    Ok(Response::new(AnaWakeupResult {
        alarms: due,
        claimed: count,
    }))
}
