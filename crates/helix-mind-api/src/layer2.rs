use super::*;
use tonic::{Request, Response, Status};

pub async fn handle_advanced_query(
    service: &HelixMindServiceImpl,
    request: Request<AdvancedQueryRequest>,
) -> Result<Response<QueryResponse>, Status> {
    let req = request.into_inner();
    let mode = match req.mode {
        0 => helix_mind_core::graph::CognitiveMode::Skilled,
        1 => helix_mind_core::graph::CognitiveMode::Anchor,
        2 => helix_mind_core::graph::CognitiveMode::Imagination,
        _ => return Err(Status::invalid_argument("Invalid cognitive mode")),
    };

    let energy_context = helix_mind_core::graph::EnergyContext {
        token_budget: req.top_k as u64 * 10,
        ..Default::default()
    };

    let result = service.retrieval.query(
        &req.query,
        mode,
        &energy_context,
        req.include_recessive,
        false,
        helix_mind_core::graph::AutonomyLevel::Open,
    ).await.map_err(|e| Status::internal(e.to_string()))?;

    let response = QueryResponse {
        nodes: result.nodes.into_iter().map(super::layer1::convert_node).collect(),
        edges: result.edges.into_iter().map(super::layer1::convert_edge).collect(),
        trace_id: result.trace_id.to_string(),
        latency_ms: result.latency_ms,
        is_partial: result.is_partial,
        exhaustion_reason: result.exhaustion_reason.unwrap_or_default(),
    };

    Ok(Response::new(response))
}
