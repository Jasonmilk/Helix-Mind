use crate::proto::*;
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
    // Layer routing: 0..=3 maps to L0..L3; absent or -1 keeps the protocol
    // default (L3 episodic). The knowledge layer (L2) is the only non-default
    // target used today — rails/facts land there.
    if (0..=3).contains(&req.node_type) {
        node.node_type = match req.node_type {
            0 => helix_mind_core::graph::NodeType::L0,
            1 => helix_mind_core::graph::NodeType::L1,
            2 => helix_mind_core::graph::NodeType::L2,
            _ => helix_mind_core::graph::NodeType::L3,
        };
    }

    // ADR-0043: `parent_ids` is INTENT-7 §3.2 `WRITE_NODE`'s parent set — the
    // nodes this one derives from. It becomes the node's `derived_from` (an
    // existing field that crystallize already writes) and, after the write, one
    // edge per parent. Direction is derived -> origin, matching crystallize.
    //
    // Unparseable ids and self-references are dropped rather than fatal: a bad
    // parent must not cost the caller the node itself. Absent `parent_ids` yields
    // no edges, which is exactly the behaviour before ADR-0043 (tolerant
    // degradation, ADR-0043 D6).
    let parents: Vec<uuid::Uuid> = req.parent_ids
        .iter()
        .filter_map(|raw| uuid::Uuid::parse_str(raw).ok())
        .filter(|id| *id != node.id)
        .collect();
    node.derived_from = parents.clone();
    // Capture the resolved layer before `node` is moved into the write.
    let layer = node.node_type.clone();

    let node_id = node.id; // Save UUID before moving node
    service.storage.write_node(node, 
    helix_mind_storage::WritePriority::Critical).await
        .map_err(|e| Status::internal(e.to_string()))?;

    for parent in &parents {
        match service.storage.add_edge(&derived_edge(node_id, *parent, &layer)).await {
            Ok(()) => {}
            // A parent link that would close a HARD-edge cycle is skipped, not
            // fatal. The node is already valid and written; letting a topology
            // constraint veto a memory write would be backwards, and the DAG
            // requirement still holds for hard edges (ADR-0043 D8).
            Err(helix_mind_core::error::MindError::CycleDetected { .. }) => {
                tracing::warn!(
                    node = %node_id,
                    parent = %parent,
                    "skipped a parent link that would close a hard-edge cycle (ADR-0043 D8)"
                );
            }
            Err(e) => return Err(Status::internal(e.to_string())),
        }
    }

    Ok(Response::new(RememberResponse { node_id: node_id.to_string() }))
}

/// ADR-0043 D7: the relation a `parent_ids` entry becomes.
///
/// `TEMPORAL` for L3 episodic succession. The relation table in
/// `docs/spec/data-contract.md` permits `TEMPORAL` between L2/L3 and restricts
/// `REFINES` to L2 -> L2, so `REFINES` was never a legal choice for an L3 parent.
/// L2 keeps `REFINES`, which is crystallize's existing convention.
///
/// Weight 0.8 reuses crystallize's existing value; ADR-0043 deliberately does not
/// re-calibrate weights in the same batch that first creates edges, otherwise a
/// recall change could not be attributed to either cause.
///
/// Hard edge (`is_soft = false`) and direction derived -> origin.
pub fn derived_edge(source: uuid::Uuid, parent: uuid::Uuid, source_layer: &helix_mind_core::graph::NodeType) -> helix_mind_core::graph::Edge {
    use helix_mind_core::graph::{Edge, NodeType, RelationType};
    let relation_type = match source_layer {
        NodeType::L2 => RelationType::Refines,
        _ => RelationType::Temporal,
    };
    Edge {
        source_id: source,
        target_id: parent,
        weight: 0.8,
        relation_type,
        is_soft: false,
    }
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
        // Text content is the node's physical text — hand it over raw, not
        // JSON-wrapped (P10): consumers (Anaphase prompt injection) read it
        // verbatim; wrapping buried the actual memory in `{"Text":"..."}`.
        // Non-text shapes keep the JSON form for structural fidelity.
        content_json: match &node.content {
            helix_mind_core::graph::NodeContent::Text(t) => t.clone(),
            other => serde_json::to_string(other).unwrap_or_default(),
        },
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
        // P0 (ADR-0011): phase-state & subject-dependency
        phase_state: format!("{:?}", node.phase_state).to_lowercase(),
        subject_dependency: format!("{:?}", node.subject_dependency).to_lowercase(),
        concentration: format!("{:?}", node.meta.concentration).to_lowercase(),
        tension: node.meta.tension,
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


#[cfg(test)]
mod derived_edge_tests {
    use super::derived_edge;
    use helix_mind_core::graph::{NodeType, RelationType};
    use uuid::Uuid;

    /// L3 episodic succession must be TEMPORAL, never REFINES: the spec's relation
    /// table restricts REFINES to L2 -> L2, so it is not a legal L3 parent relation.
    #[test]
    fn l3_succession_is_temporal() {
        let e = derived_edge(Uuid::new_v4(), Uuid::new_v4(), &NodeType::L3);
        assert_eq!(e.relation_type, RelationType::Temporal);
        assert!(!e.is_soft, "provenance edges are hard edges");
    }

    /// L2 keeps crystallize's existing REFINES convention.
    #[test]
    fn l2_abstraction_is_refines() {
        let e = derived_edge(Uuid::new_v4(), Uuid::new_v4(), &NodeType::L2);
        assert_eq!(e.relation_type, RelationType::Refines);
    }

    /// Direction is derived -> origin, as crystallize established. Reversing it
    /// would make diffusion flow backwards in time without any error.
    #[test]
    fn direction_is_derived_to_origin() {
        let derived = Uuid::new_v4();
        let origin = Uuid::new_v4();
        let e = derived_edge(derived, origin, &NodeType::L3);
        assert_eq!(e.source_id, derived);
        assert_eq!(e.target_id, origin);
    }
}
