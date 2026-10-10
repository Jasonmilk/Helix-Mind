//! Deterministic sandbox review for external nodes (ADR-0018 P3a).
//!
//! Replaces the former LLM stub (which always passed) with a **zero-LLM,
//! fully deterministic** dual-judge review:
//! - Judge A (`review_node`): `SymbolicSolver` internal logical-consistency check.
//! - Judge B (`risk_review_node`): high-risk rule (system-level operations).
//!
//! `dual_blind_review` is the cross-check of these two independent deterministic
//! judges. If an LLM review is ever added, it must supersede via a new ADR
//! (never overwrite this file's semantics silently).

use helix_mind_core::graph::Node;
use helix_mind_core::symbolic::{assertions_from_node, SymbolicSolver};
use serde::{Deserialize, Serialize};

/// Structured review verdict for every judged node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewVerdict {
    pub logically_coherent: bool,
    pub conflict_with_local_dag: bool,
    pub reason: String,
}

/// Judge A — deterministic logical-consistency check (SymbolicSolver).
///
/// A node is `logically_coherent` when its structured assertions contain no
/// internal contradiction (same subject+object with opposing predicates).
/// Nodes without structured assertions (text-only) trivially pass this judge;
/// their arbitration is deferred to merge-time conflict checks.
///
/// `_local_dag_context` is retained for signature compatibility; structural
/// local-DAG conflict detection requires a structured local assertion set and
/// is covered at the merge/outbound gate, not here.
pub async fn review_node(
    node: &Node,
    _local_dag_context: &str,
) -> Result<ReviewVerdict, helix_mind_core::error::MindError> {
    let solver = SymbolicSolver::new();
    let assertions = assertions_from_node(&node.content);
    let coherent = solver.find_internal_contradiction(&assertions).is_none();
    Ok(ReviewVerdict {
        logically_coherent: coherent,
        conflict_with_local_dag: false,
        reason: if coherent {
            "deterministic solver: internally consistent".into()
        } else {
            "deterministic solver: internal logical contradiction".into()
        },
    })
}

/// Judge B — high-risk rule (system-level operations).
///
/// High-risk is surfaced as `conflict_with_local_dag` so the sandbox pipeline
/// flags it as suspicious (not auto-mergeable) without conflating it with
/// logical incoherence.
pub fn risk_review_node(node: &Node) -> ReviewVerdict {
    let high_risk = is_high_risk_node(node);
    ReviewVerdict {
        logically_coherent: true, // risk rules do not judge logical consistency
        conflict_with_local_dag: high_risk,
        reason: if high_risk {
            "high-risk rule: system-level operations".into()
        } else {
            "high-risk rule: clear".into()
        },
    }
}

/// Dual-blind review: two independent deterministic judges cross-check the
/// same node (ADR-0018 P3a). If the judges disagree, the caller treats the
/// node as suspicious and refuses auto-merge.
pub async fn dual_blind_review(
    node: &Node,
    local_dag_context: &str,
) -> Result<(ReviewVerdict, ReviewVerdict), helix_mind_core::error::MindError> {
    let v1 = review_node(node, local_dag_context).await?;
    let v2 = risk_review_node(node);
    Ok((v1, v2))
}

/// Determine if a node is high-risk (involves system-level operations).
pub fn is_high_risk_node(node: &Node) -> bool {
    // Only keep 2 branches, wildcard serializes all remaining variants and convert to lowercase
    let content_str = match &node.content {
        helix_mind_core::graph::NodeContent::Text(t) => t.to_lowercase(),
        helix_mind_core::graph::NodeContent::Structured(map) => {
            serde_json::to_string(map).unwrap_or_default().to_lowercase()
        }
        // Wildcard covers all remaining variants, serialized and converted to lowercase
        _ => serde_json::to_string(&node.content)
            .unwrap_or_default()
            .to_lowercase(),
    };

    let high_risk_patterns = [
        "rm ", "delete", "remove", "kill", "terminate",
        "sudo", "root", "chmod", "chown", "mount", "format",
        "systemctl", "reboot", "shutdown", "poweroff",
        "drop table", "truncate", "delete from",
    ];

    high_risk_patterns.iter().any(|p| content_str.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use helix_mind_core::graph::{Node, NodeContent, NodeType};
    use std::collections::HashMap;

    fn text_node(content: &str) -> Node {
        Node {
            content: NodeContent::Text(content.into()),
            node_type: NodeType::L2,
            ..Default::default()
        }
    }

    fn structured_node(assertions_json: &str) -> Node {
        let mut map = HashMap::new();
        map.insert("assertions".into(), assertions_json.into());
        Node {
            content: NodeContent::Structured(map),
            node_type: NodeType::L2,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn consistent_node_passes_solver_judge() {
        let node = structured_node(
            r#"[{"subject":"A","predicate":"causes","object":"B"}]"#,
        );
        let v = review_node(&node, "").await.unwrap();
        assert!(v.logically_coherent);
        assert!(!v.conflict_with_local_dag);
    }

    #[tokio::test]
    async fn self_contradictory_node_rejected() {
        let node = structured_node(
            r#"[
                {"subject":"A","predicate":"causes","object":"B"},
                {"subject":"A","predicate":"prevents","object":"B"}
            ]"#,
        );
        let v = review_node(&node, "").await.unwrap();
        assert!(!v.logically_coherent);
    }

    #[tokio::test]
    async fn text_node_trivially_passes_solver_judge() {
        let node = text_node("some ordinary knowledge statement");
        let v = review_node(&node, "").await.unwrap();
        assert!(v.logically_coherent);
    }

    #[test]
    fn high_risk_rule_flags_system_ops() {
        let node = text_node("run sudo rm -rf /var/log");
        let v = risk_review_node(&node);
        assert!(v.conflict_with_local_dag);
        assert!(v.logically_coherent);
    }

    #[test]
    fn benign_node_cleared_by_risk_rule() {
        let node = text_node("the sun rises in the east");
        let v = risk_review_node(&node);
        assert!(!v.conflict_with_local_dag);
    }

    #[tokio::test]
    async fn dual_blind_cross_check_rejects_suspicious() {
        // Judge B flags high-risk while Judge A stays coherent → disagree →
        // caller must treat as suspicious.
        let node = text_node("execute sudo systemctl reboot now");
        let (v1, v2) = dual_blind_review(&node, "").await.unwrap();
        assert!(v1.logically_coherent);
        assert!(v2.conflict_with_local_dag);
    }
}
