//! First-order logic clash checker — deterministic symbolic solver (§5.1).
//!
//! The LLM is only used as a "translator" to extract structured logic assertions
//! from natural language node content. The actual arbitration (conflict detection,
//! one-vote veto) is performed by this deterministic symbolic solver running in
//! Rust memory. This eliminates the "verifying hallucinations with hallucinations"
//! pitfall warned about in the whitepaper.

use helix_mind_core::graph::Node;
use std::collections::HashSet;
use tracing::info;

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
    ) -> Result<(), helix_mind_core::error::MindError> {
        // 1. Check against L0 constitution — hard one-vote veto
        for new_assertion in new_assertions {
            for l0_assertion in l0_assertions {
                if self.is_direct_contradiction(new_assertion, l0_assertion) {
                    return Err(helix_mind_core::error::MindError::SandboxRejected {
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
                    return Err(helix_mind_core::error::MindError::SandboxRejected {
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
}
