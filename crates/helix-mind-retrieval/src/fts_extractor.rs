//! Real P1 (M-01 / ADR-0013) start-node extractor backed by the FTS5 trigram
//! projection maintained by the storage engine.
//!
//! Read-only consumer: queries the index through its own pooled connection.
//! Input handling (ruling 2026-08-28):
//! - Whitelist sanitize: keep only Han / alphanumeric / whitespace.
//! - FTS5 literal-phrase escaping: double quotes around the query, inner quotes
//!   doubled — user input is treated as a literal phrase, never as syntax.
//! - Audit: every sanitized input is recorded to the audit log (Mind owns
//!   "what was said is harmless"; CI-144 owns transport/identity, Tuck owns
//!   audit/sanitization at the gateway — this is the application layer).

use crate::adapter::StartNodeExtractor;
use helix_mind_storage::fts::{audit_sanitized, fts_like_search, fts_search};
use helix_mind_storage::sqlite_pool::SqlitePool;
use helix_mind_storage::StorageEngine;
use uuid::Uuid;

/// Minimum query length that goes through FTS5; shorter queries use LIKE.
const FTS_MIN_CHARS: usize = 3;

pub struct FtsExtractor {
    pool: SqlitePool,
    max_results: usize,
}

impl FtsExtractor {
    /// Build from a running storage engine (clones its SQLite pool).
    pub fn new(storage: &StorageEngine, max_results: usize) -> Self {
        Self {
            pool: storage.sqlite.clone(),
            max_results,
        }
    }

    /// Build directly from a pool (for tests / decoupled wiring).
    pub fn from_pool(pool: SqlitePool, max_results: usize) -> Self {
        Self { pool, max_results }
    }

    fn fts_search(&self, escaped_match: &str) -> Vec<Uuid> {
        fts_search(&self.pool, escaped_match, self.max_results)
            .map(|hits| hits.into_iter().map(|h| h.node_id).collect())
            .unwrap_or_default()
    }

    fn like_search(&self, cleaned: &str) -> Vec<Uuid> {
        // Sanitization already stripped %/_ (non-alphanumeric), so the LIKE
        // pattern is safe; escape defensively anyway.
        let escaped = cleaned.replace('%', "\\%").replace('_', "\\_");
        let pattern = format!("%{escaped}%");
        fts_like_search(&self.pool, &pattern, self.max_results)
            .map(|hits| hits.into_iter().map(|h| h.node_id).collect())
            .unwrap_or_default()
    }
}

impl StartNodeExtractor for FtsExtractor {
    fn extract_start_nodes(&self, query: &str) -> Vec<Uuid> {
        let (cleaned, removed) = sanitize_query(query);
        audit_sanitized(&self.pool, query, &removed);
        let trimmed = cleaned.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }
        if trimmed.chars().count() >= FTS_MIN_CHARS {
            self.fts_search(&escape_fts(trimmed))
        } else {
            self.like_search(trimmed)
        }
    }
}

/// Whitelist sanitization — keeps only Han/alphanumeric/whitespace (ruling
/// 2026-08-28). Returns (cleaned, removed_chars).
pub fn sanitize_query(query: &str) -> (String, String) {
    let mut cleaned = String::new();
    let mut removed = String::new();
    for c in query.chars() {
        if c.is_alphanumeric() || c.is_whitespace() {
            cleaned.push(c);
        } else {
            removed.push(c);
        }
    }
    (cleaned, removed)
}

/// FTS5 literal-phrase escaping lives with the index (storage layer, ADR-0013);
/// re-exported here so the retrieval crate exposes a single escaping authority.
pub use helix_mind_storage::fts::escape_fts;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_keeps_han_alnum_space_only() {
        let (cleaned, removed) = sanitize_query("认知相态 & 河流; DROP TABLE; \"q\"");
        assert_eq!(cleaned, "认知相态  河流 DROP TABLE q");
        assert_eq!(removed, "&;;\"\"");
    }

    #[test]
    fn route_threshold_uses_fts_for_3plus_like_for_shorter() {
        // Threshold is by char count of the trimmed, sanitized query.
        assert_eq!("认知相态".chars().count() >= FTS_MIN_CHARS, true);
        assert_eq!("ab".chars().count() >= FTS_MIN_CHARS, false);
    }
}
