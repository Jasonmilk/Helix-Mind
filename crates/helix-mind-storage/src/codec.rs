use helix_mind_core::graph::{Node, NodeType, NodeContent, NodeSource, Sensitivity, RelationType};
use uuid::Uuid;
use chrono::Utc;

/// Converts a database row to a standard Node instance.
pub fn row_to_node(row: &rusqlite::Row) -> Node {
    let id_str: String = row.get(0).unwrap_or_default();
    let node_type_str: String = row.get(1).unwrap_or_default();
    let content_json: String = row.get(2).unwrap_or_default();
    let ledger_json: String = row.get(17).unwrap_or_default();
    let source_json: String = row.get(18).unwrap_or_default();
    let derived_json: String = row.get(21).unwrap_or_default();

    Node {
        id: Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4()),
        node_type: str_to_node_type(&node_type_str),
        content: serde_json::from_str(&content_json).unwrap_or(NodeContent::Text(String::new())),
        heat: row.get(3).unwrap_or(0.5),
        is_hypothetical: row.get(4).unwrap_or(false),
        is_recessive: row.get(5).unwrap_or(false),
        sensitivity: row.get::<_, Option<String>>(6).unwrap_or(None).map(|s| str_to_sensitivity(&s)),
        generation: row.get(7).unwrap_or(1),
        created_at: parse_datetime(&row.get::<_, String>(8).unwrap_or_default()),
        last_accessed_at: parse_datetime(&row.get::<_, String>(9).unwrap_or_default()),
        access_count: row.get(10).unwrap_or(0),
        initial_impact: row.get(11).unwrap_or(0.5),
        corrected_by: row.get::<_, Option<String>>(12).unwrap_or(None)
            .and_then(|s| Uuid::parse_str(&s).ok()),
        notes: row.get::<_, Option<String>>(13).unwrap_or(None),
        dominance: row.get(14).unwrap_or(0.5),
        utility: row.get(15).unwrap_or(0.5),
        corroborations: row.get(16).unwrap_or(0),
        attribution_ledger: serde_json::from_str(&ledger_json).unwrap_or_default(),
        source: serde_json::from_str(&source_json).unwrap_or(NodeSource::Local),
        high_risk: row.get(19).unwrap_or(false),
        abstract_provenance: row.get::<_, Option<String>>(20).unwrap_or(None)
            .filter(|s| !s.is_empty()),
        derived_from: serde_json::from_str(&derived_json).unwrap_or_default(),
    }
}

pub fn parse_datetime(s: &str) -> chrono::DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

pub fn node_type_str(nt: &NodeType) -> &'static str {
    match nt {
        NodeType::L0 => "L0",
        NodeType::L1 => "L1",
        NodeType::L2 => "L2",
        NodeType::L3 => "L3",
    }
}

pub fn str_to_node_type(s: &str) -> NodeType {
    match s {
        "L0" => NodeType::L0,
        "L1" => NodeType::L1,
        "L2" => NodeType::L2,
        _ => NodeType::L3,
    }
}

pub fn sensitivity_str(s: &Sensitivity) -> &'static str {
    match s {
        Sensitivity::Public => "Public",
        Sensitivity::Private => "Private",
        Sensitivity::Sensitive => "Sensitive",
    }
}

pub fn str_to_sensitivity(s: &str) -> Sensitivity {
    match s {
        "Public" => Sensitivity::Public,
        "Sensitive" => Sensitivity::Sensitive,
        _ => Sensitivity::Private,
    }
}

pub fn relation_type_str(rt: &RelationType) -> &'static str {
    match rt {
        RelationType::Causal => "Causal",
        RelationType::Semantic => "Semantic",
        RelationType::Temporal => "Temporal",
        RelationType::CoOccurrence => "CoOccurrence",
        RelationType::Corrects => "Corrects",
        RelationType::Refines => "Refines",
        RelationType::Doubts => "Doubts",
        RelationType::SimilarTo => "SimilarTo",
    }
}

pub fn str_to_relation_type(s: &str) -> RelationType {
    match s {
        "Causal" => RelationType::Causal,
        "Semantic" => RelationType::Semantic,
        "Temporal" => RelationType::Temporal,
        "CoOccurrence" => RelationType::CoOccurrence,
        "Corrects" => RelationType::Corrects,
        "Refines" => RelationType::Refines,
        "Doubts" => RelationType::Doubts,
        "SimilarTo" => RelationType::SimilarTo,
        _ => RelationType::Semantic,
    }
}
