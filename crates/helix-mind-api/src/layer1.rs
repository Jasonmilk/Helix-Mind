use super::*;
use tonic::{Request, Response, Status};

pub async fn handle_query(
    service: &HelixMindServiceImpl,
    request: Request<QueryRequest>,
) -> Result<Response<QueryResponse>, Status> {
    let req = request.into_inner();
    let energy_context = helix_mind_core::graph::EnergyContext::default();

    let result = service.retrieval.query(
        &req.query,
        helix_mind_core::graph::CognitiveMode::Skilled,
        &energy_context,
        false,
        false,
        helix_mind_core::graph::AutonomyLevel::Agent,
    ).await.map_err(|e| Status::internal(e.to_string()))?;

    let response = QueryResponse {
        nodes: result.nodes.into_iter().map(convert_node).collect(),
        edges: result.edges.into_iter().map(convert_edge).collect(),
        trace_id: result.trace_id.to_string(),
        latency_ms: result.latency_ms,
        is_partial: result.is_partial,
        exhaustion_reason: result.exhaustion_reason.unwrap_or_default(),
    };

    Ok(Response::new(response))
}

pub async fn handle_remember(
    service: &HelixMindServiceImpl,
    request: Request<RememberRequest>,
) -> Result<Response<RememberResponse>, Status> {
    let req = request.into_inner();
    let mut node = helix_mind_core::graph::Node::default();
    node.content = helix_mind_core::graph::NodeContent::Text(req.content);
    node.sensitivity = Some(helix_mind_core::graph::Sensitivity::Private);

    service.storage.write_node(node, helix_mind_storage::WritePriority::Critical).await
    .map_err(|e| Status::internal(e.to_string()))?;.map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(RememberResponse {
        node_id: node.id.to_string(),
    }))
}

pub async fn handle_forget(
    service: &HelixMindServiceImpl,
    request: Request<ForgetRequest>,
) -> Result<Response<ForgetResponse>, Status> {
    let req = request.into_inner();
    let node_id = uuid::Uuid::parse_str(&req.node_id).map_err(|e| Status::invalid_argument(e.to_string()))?;

    service.storage.mark_recessive(&node_id).await.map_err(|e| Status::internal(e.to_string()))?;

    Ok(Response::new(ForgetResponse {
        success: true,
    }))
}

// Convert core Node to proto Node
pub(crate) fn convert_node(node: helix_mind_core::graph::Node) -> Node {
    Node {
        id: node.id.to_string(),
        node_type: format!("{:?}", node.node_type),
        content_json: serde_json::to_string(&node.content).unwrap_or_default(),
        heat: node.heat,
        is_hypothetical: node.is_hypothetical,
        is_recessive: node.is_recessive,
        sensitivity: node.sensitivity.map(|s| format!("{:?}", s)).unwrap_or_default(),
        generation: node.generation,
        created_at: Some(prost_types::Timestamp::from(
    std::time::SystemTime::from(node.created_at),
)),
        last_accessed_at: Some(prost_types::Timestamp::from(
    std::time::SystemTime::from(node.last_accessed_at),
)),
        access_count: node.access_count,
        initial_impact: node.initial_impact,
        corrected_by: node.corrected_by.map(|u| u.to_string()).unwrap_or_default(),
        notes: node.notes.unwrap_or_default(),
        derived_from: node.derived_from.into_iter().map(|u| u.to_string()).collect(),
    }
}

// Convert core Edge to proto Edge
pub(crate) fn convert_edge(edge: helix_mind_core::graph::Edge) -> Edge {
    Edge {
        source_id: edge.source_id.to_string(),
        target_id: edge.target_id.to_string(),
        weight: edge.weight,
        relation_type: format!("{:?}", edge.relation_type),
        is_soft: edge.is_soft,
    }
}
