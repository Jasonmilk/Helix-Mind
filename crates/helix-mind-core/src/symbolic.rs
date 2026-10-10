//! First-order logic clash checker — deterministic symbolic solver (§5.1).
//!
//! The LLM is only used as a "translator" to extract structured logic assertions
//! from natural language node content. The actual arbitration (conflict detection,
//! one-vote veto) is performed by this deterministic symbolic solver running in
//! Rust memory. This eliminates the "verifying hallucinations with hallucinations"
//! pitfall warned about in the whitepaper.

use std::collections::HashSet;
use crate::graph::NodeContent;

/// A structured logic assertion extracted from a knowledge node.
///
/// Format: `subject PREDICATE object`
/// Example: "hardware.cpu.overheating CAUSES system.crash"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicAssertion {
    /// The subject concept (e.g. "hardware.cpu.overheating").
    pub subject: String,
    /// The predicate (e.g. "causes", "prevents", "implies", "increases").
    pub predicate: Predicate,
    /// The object concept (e.g. "system.crash").
    pub object: String,
}

/// Supported predicates for first-order logic assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Predicate {
    Causes,
    Prevents,
    Implies,
    Increases,
    Decreases,
    Requires,
    Conflicts,
}

impl Predicate {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "causes" => Some(Predicate::Causes),
            "prevents" => Some(Predicate::Prevents),
            "implies" => Some(Predicate::Implies),
            "increases" => Some(Predicate::Increases),
            "decreases" => Some(Predicate::Decreases),
            "requires" => Some(Predicate::Requires),
            "conflicts" => Some(Predicate::Conflicts),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Predicate::Causes => "causes",
            Predicate::Prevents => "prevents",
            Predicate::Implies => "implies",
            Predicate::Increases => "increases",
            Predicate::Decreases => "decreases",
            Predicate::Requires => "requires",
            Predicate::Conflicts => "conflicts",
        }
    }
}

/// Deterministic first-order logic clash checker.
///
/// This solver operates on structured `LogicAssertion` lists extracted from
/// knowledge nodes. It does NOT use any LLM for arbitration — the LLM's role
/// is strictly limited to translating natural language node content into
/// `LogicAssertion` structs (via the `LLMTranslator` trait).
pub struct SymbolicSolver;

impl SymbolicSolver {
    pub fn new() -> Self {
        Self
    }

    /// Check whether a new node's assertions clash with the existing L0 constitution
    /// or with existing high-consensus L2 nodes.
    ///
    /// Returns `Ok(())` if no hard clash is found.
    /// Returns `Err(SandboxRejected)` if a clash is detected (one-vote veto).
    pub fn check_clash(
        &self,
        new_assertions: &[LogicAssertion],
        l0_assertions: &[LogicAssertion],
        existing_l2_assertions: &[LogicAssertion],
    ) -> Result<(), crate::error::MindError> {
        // 1. Check against L0 constitution — hard one-vote veto
        for new_assertion in new_assertions {
            for l0_assertion in l0_assertions {
                if self.is_direct_contradiction(new_assertion, l0_assertion) {
                    return Err(crate::error::MindError::SandboxRejected {
                        reason: format!(
                            "Node assertion '{} {} {}' contradicts L0 constitution '{} {} {}'",
                            new_assertion.subject,
                            new_assertion.predicate.as_str(),
                            new_assertion.object,
                            l0_assertion.subject,
                            l0_assertion.predicate.as_str(),
                            l0_assertion.object,
                        ),
                    });
                }
            }
        }

        // 2. Check against existing high-consensus L2 nodes
        for new_assertion in new_assertions {
            for l2_assertion in existing_l2_assertions {
                if self.is_direct_contradiction(new_assertion, l2_assertion) {
                    return Err(crate::error::MindError::SandboxRejected {
                        reason: format!(
                            "Node assertion '{} {} {}' contradicts existing L2 knowledge '{} {} {}'",
                            new_assertion.subject,
                            new_assertion.predicate.as_str(),
                            new_assertion.object,
                            l2_assertion.subject,
                            l2_assertion.predicate.as_str(),
                            l2_assertion.object,
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Detect direct logical contradiction between two assertions.
    ///
    /// Two assertions contradict each other when:
    /// - They share the same subject and object, AND
    /// - One asserts a positive relationship while the other asserts its negation
    ///   (e.g. "causes" vs "prevents", "increases" vs "decreases").
    fn is_direct_contradiction(&self, a: &LogicAssertion, b: &LogicAssertion) -> bool {
        if a.subject != b.subject || a.object != b.object {
            return false;
        }

        // Contradiction pairs
        matches!(
            (a.predicate, b.predicate),
            (Predicate::Causes, Predicate::Prevents)
                | (Predicate::Prevents, Predicate::Causes)
                | (Predicate::Increases, Predicate::Decreases)
                | (Predicate::Decreases, Predicate::Increases)
                | (Predicate::Implies, Predicate::Conflicts)
                | (Predicate::Conflicts, Predicate::Implies)
        )
    }

    /// Find the first internally contradictory assertion pair (if any).
    ///
    /// Two assertions contradict when they share subject+object and carry
    /// opposing predicates. Returns `Some((i, j))` with `i < j` on the first
    /// contradiction found, `None` when the set is internally consistent.
    /// Used by the deterministic federation review (ADR-0018 P3a).
    pub fn find_internal_contradiction(
        &self,
        assertions: &[LogicAssertion],
    ) -> Option<(usize, usize)> {
        for i in 0..assertions.len() {
            for j in (i + 1)..assertions.len() {
                if self.is_direct_contradiction(&assertions[i], &assertions[j]) {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// Extract unique concept IDs from a set of assertions.
    pub fn extract_concepts(assertions: &[LogicAssertion]) -> HashSet<String> {
        let mut concepts = HashSet::new();
        for a in assertions {
            concepts.insert(a.subject.clone());
            concepts.insert(a.object.clone());
        }
        concepts
    }
}

impl Default for SymbolicSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministically extract assertions from a node's content (P2a, ADR-0014).
///
/// Convention: `NodeContent::Structured` map containing an `assertions` key
/// whose value is a JSON array of `{"subject": s, "predicate": p, "object": o}`
/// (produced by the crystallizer / future LLM translator). Text-only or other
/// shapes return an empty list — those nodes defer dissonance arbitration to
/// P2b (LLM translation), the storage layer already filters them out of
/// `get_unresolved_dissonance`.
pub fn assertions_from_node(content: &NodeContent) -> Vec<LogicAssertion> {
    let NodeContent::Structured(map) = content else {
        return Vec::new();
    };
    let Some(raw) = map.get("assertions") else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<AssertionJson>>(raw)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|a| {
            let predicate = Predicate::from_str(&a.predicate)?;
            Some(LogicAssertion {
                subject: a.subject,
                predicate,
                object: a.object,
            })
        })
        .collect()
}

#[derive(serde::Deserialize)]
struct AssertionJson {
    subject: String,
    predicate: String,
    object: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_assertion(subject: &str, predicate: Predicate, object: &str) -> LogicAssertion {
        LogicAssertion {
            subject: subject.to_string(),
            predicate,
            object: object.to_string(),
        }
    }

    #[test]
    fn test_no_contradiction() {
        let solver = SymbolicSolver::new();
        let new = vec![make_assertion("A", Predicate::Causes, "B")];
        let existing = vec![make_assertion("X", Predicate::Causes, "Y")];
        assert!(solver.check_clash(&new, &[], &existing).is_ok());
    }

    #[test]
    fn test_direct_contradiction() {
        let solver = SymbolicSolver::new();
        let new = vec![make_assertion("A", Predicate::Causes, "B")];
        let l0 = vec![make_assertion("A", Predicate::Prevents, "B")];
        assert!(solver.check_clash(&new, &l0, &[]).is_err());
    }

    #[test]
    fn test_increase_decrease_contradiction() {
        let solver = SymbolicSolver::new();
        let new = vec![make_assertion("temp", Predicate::Increases, "stability")];
        let existing = vec![make_assertion("temp", Predicate::Decreases, "stability")];
        assert!(solver.check_clash(&new, &[], &existing).is_err());
    }

    #[test]
    fn test_no_contradiction_different_objects() {
        let solver = SymbolicSolver::new();
        let new = vec![make_assertion("A", Predicate::Causes, "B")];
        let existing = vec![make_assertion("A", Predicate::Prevents, "C")];
        assert!(solver.check_clash(&new, &[], &existing).is_ok());
    }

    #[test]
    fn test_multiple_assertions_one_clash() {
        let solver = SymbolicSolver::new();
        let new = vec![
            make_assertion("A", Predicate::Causes, "B"),
            make_assertion("X", Predicate::Increases, "Y"),
        ];
        let l0 = vec![make_assertion("X", Predicate::Decreases, "Y")];
        assert!(solver.check_clash(&new, &l0, &[]).is_err());
    }

    #[test]
    fn assertions_from_node_parses_structured_assertions() {
        use crate::graph::NodeContent;
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(
            "assertions".into(),
            r#"[{"subject":"A","predicate":"causes","object":"B"}]"#.into(),
        );
        let content = NodeContent::Structured(map);
        let assertions = assertions_from_node(&content);
        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].subject, "A");
        assert_eq!(assertions[0].predicate, Predicate::Causes);
        assert_eq!(assertions[0].object, "B");
    }

    #[test]
    fn assertions_from_node_returns_empty_for_text() {
        let content = NodeContent::Text("plain text".into());
        assert!(assertions_from_node(&content).is_empty());
    }

    #[test]
    fn assertions_from_node_ignores_unknown_predicates() {
        use crate::graph::NodeContent;
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(
            "assertions".into(),
            r#"[{"subject":"A","predicate":"teleports","object":"B"}]"#.into(),
        );
        let content = NodeContent::Structured(map);
        assert!(assertions_from_node(&content).is_empty());
    }
}
