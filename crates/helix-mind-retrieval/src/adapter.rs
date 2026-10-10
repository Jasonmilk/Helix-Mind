use std::collections::HashMap;
use uuid::Uuid;

/// Start-node extractor — maps a natural-language query to candidate node IDs.
///
/// This is the seam where the retrieval pipeline's Stage-1 "extract start
/// nodes" plugs into a semantic source.
/// - P0.5: `FakeAdapter` provides a deterministic test double so the whole
///   retrieval pipeline (Stage 1..5) can be exercised without a live LLM/NER.
/// - P1 (M-01): the real FTS5-trigram implementation lands behind the same
///   trait, consuming this seam unchanged.
pub trait StartNodeExtractor: Send + Sync {
    fn extract_start_nodes(&self, query: &str) -> Vec<Uuid>;
}

/// Honest no-op extractor — the default until P1.
///
/// Returns nothing, which is the truthful baseline while the real extraction
/// path is unimplemented (prevents false-positive retrieval success).
#[derive(Debug, Default)]
pub struct EmptyExtractor;

impl StartNodeExtractor for EmptyExtractor {
    fn extract_start_nodes(&self, _query: &str) -> Vec<Uuid> {
        Vec::new()
    }
}

/// Deterministic LLM/NER simulation for tests and dev fixtures (P0.5).
///
/// Maps an exact query string to a fixed set of start-node IDs. No randomness,
/// no network, fully reproducible — usable by P1 tests before the real
/// FTS5 extractor lands.
#[derive(Debug, Default)]
pub struct FakeAdapter {
    map: HashMap<String, Vec<Uuid>>,
}

impl FakeAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a deterministic query → start-nodes mapping.
    pub fn add(&mut self, query: &str, ids: Vec<Uuid>) -> &mut Self {
        self.map.insert(query.to_string(), ids);
        self
    }
}

impl StartNodeExtractor for FakeAdapter {
    fn extract_start_nodes(&self, query: &str) -> Vec<Uuid> {
        self.map.get(query).cloned().unwrap_or_default()
    }
}
