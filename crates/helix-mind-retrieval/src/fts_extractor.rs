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
    /// Query tokens dropped before retrieval (P10 recall). Deterministic,
    /// from RetrievalConfig.stopwords (protocol defaults, config-overridable).
    stopwords: Vec<String>,
}

impl FtsExtractor {
    /// Build from a running storage engine (clones its SQLite pool).
    pub fn new(storage: &StorageEngine, max_results: usize, stopwords: Vec<String>) -> Self {
        Self {
            pool: storage.sqlite.clone(),
            max_results,
            stopwords,
        }
    }

    /// Build directly from a pool (for tests / decoupled wiring).
    pub fn from_pool(pool: SqlitePool, max_results: usize, stopwords: Vec<String>) -> Self {
        Self { pool, max_results, stopwords }
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
        // P10 recall (2026-09-07): tokenized retrieval. One FTS5 phrase of a
        // natural-language question almost never matches stored text
        // ("我叫Jason你记得我吗" vs stored "你好,我是Jason" — zero hits).
        // Split into tokens (ascii words + Han runs, stopwords stripped) and
        // OR the per-token hits. Deterministic, zero new dependencies.
        let tokens = tokenize_query(trimmed, &self.stopwords);
        let mut seen: std::collections::HashSet<Uuid> = std::collections::HashSet::new();
        let mut ids: Vec<Uuid> = Vec::new();
        for tok in &tokens {
            let hits = if tok.chars().count() >= FTS_MIN_CHARS {
                self.fts_search(&escape_fts(tok))
            } else {
                self.like_search(tok)
            };
            for hit in hits {
                if seen.insert(hit) {
                    ids.push(hit);
                    if ids.len() >= self.max_results {
                        return ids;
                    }
                }
            }
        }
        // Fallback 1: bigram windows of long Han tokens (content words that
        // never match verbatim, e.g. "你好" inside "你好你还"). LIKE per
        // window, merged, deduped — deterministic, bounded by max_results.
        if ids.is_empty() {
            for tok in &tokens {
                for bg in bigram_candidates(tok) {
                    for hit in self.like_search(&bg) {
                        if seen.insert(hit) {
                            ids.push(hit);
                            if ids.len() >= self.max_results {
                                return ids;
                            }
                        }
                    }
                }
            }
        }
        // Fallback 2: nothing at all — raw sanitized phrase as before, so
        // behaviour never regresses.
        if ids.is_empty() {
            if trimmed.chars().count() >= FTS_MIN_CHARS {
                ids = self.fts_search(&escape_fts(trimmed));
            } else {
                ids = self.like_search(trimmed);
            }
        }
        ids
    }
}

/// Bigram fallback candidates for long Han tokens (P10 recall). A token
/// like "你好你还" is rarely stored verbatim; its 2-char windows ("你好",
/// "好你", "你还", "你记") hit stored content via LIKE. Deterministic.
fn bigram_candidates(token: &str) -> Vec<String> {
    let chars: Vec<char> = token.chars().collect();
    if chars.len() < 4 {
        return Vec::new();
    }
    chars.windows(2).map(|w| w.iter().collect()).collect()
}

/// Deterministic lightweight tokenizer (P10 recall). Splits the sanitized
/// query into: ascii words kept whole; Han runs split on stopword substrings
/// (the first stopword found cuts the run, both sides recurse). Non-empty,
/// non-stopword tokens win. No external crates — character scan only.
pub fn tokenize_query(cleaned: &str, stopwords: &[String]) -> Vec<String> {
    // Han-only stopwords (ascii ones apply to whole ascii tokens below).
    let han_stop: Vec<&str> = stopwords
        .iter()
        .map(|s| s.as_str())
        .filter(|s| !s.is_empty() && s.chars().all(|c| !c.is_ascii()))
        .collect();

    let mut tokens: Vec<String> = Vec::new();
    let mut ascii_buf = String::new();
    let mut han_buf = String::new();
    let flush_ascii = |ascii_buf: &mut String, tokens: &mut Vec<String>| {
        if !ascii_buf.is_empty() {
            let t = ascii_buf.to_ascii_lowercase();
            if !stopwords.iter().any(|s| s.eq_ignore_ascii_case(&t)) {
                tokens.push(t);
            }
            ascii_buf.clear();
        }
    };
    let flush_han = |han_buf: &mut String, tokens: &mut Vec<String>| {
        if !han_buf.is_empty() {
            split_han_run(han_buf, &han_stop, tokens);
            han_buf.clear();
        }
    };
    for c in cleaned.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            flush_han(&mut han_buf, &mut tokens);
            ascii_buf.push(c);
        } else if !c.is_ascii() {
            flush_ascii(&mut ascii_buf, &mut tokens);
            han_buf.push(c);
        } else {
            // ascii whitespace / other: boundary between tokens.
            flush_ascii(&mut ascii_buf, &mut tokens);
            flush_han(&mut han_buf, &mut tokens);
        }
    }
    flush_ascii(&mut ascii_buf, &mut tokens);
    flush_han(&mut han_buf, &mut tokens);
    tokens
}

/// Cut a Han run on the earliest stopword occurrence; recurse both sides.
/// Keeps only non-empty non-stopword segments (single chars included — the
/// LIKE fallback covers them).
fn split_han_run(run: &str, stopwords: &[&str], out: &mut Vec<String>) {
    let mut rest = run;
    loop {
        let mut cut: Option<(usize, &str)> = None;
        for sw in stopwords {
            if sw.is_empty() {
                continue;
            }
            if let Some(pos) = rest.find(sw) {
                // Earliest position wins; on a tie the LONGER stopword wins
                // ("我们" over "我"), so compound words are never split
                // from inside by their single-char member.
                let better = match cut {
                    None => true,
                    Some((p, ps)) => pos < p || (pos == p && sw.len() > ps.len()),
                };
                if better {
                    cut = Some((pos, sw));
                }
            }
        }
        match cut {
            Some((pos, sw)) => {
                let (left, right) = rest.split_at(pos);
                // Single Han chars are noise under LIKE — drop them (bigram
                // fallback covers content windows instead).
                if left.chars().count() >= 2 {
                    out.push(left.to_string());
                }
                rest = &right[sw.len()..];
            }
            None => {
                // The remaining run may itself be a stopword — drop it too.
                if rest.chars().count() >= 2 && !stopwords.contains(&rest) {
                    out.push(rest.to_string());
                }
                break;
            }
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
    fn tokenize_keeps_ascii_word_drops_chinese_function_words() {
        let sw = helix_mind_core::config::RetrievalConfig::default().stopwords;
        let toks = tokenize_query("我叫Jason你记得我吗", &sw);
        // "Jason" survives whole; single Han chars ("我","你") are noise and
        // dropped; compound "我叫" survives for LIKE.
        assert!(toks.contains(&"jason".to_string()), "got: {:?}", toks);
        assert!(!toks.iter().any(|t| t.chars().count() == 1), "got: {:?}", toks);
        assert!(!toks.iter().any(|t| t == "吗"), "got: {:?}", toks);
    }

    #[test]
    fn tokenize_keeps_content_han_segments() {
        let sw = helix_mind_core::config::RetrievalConfig::default().stopwords;
        let toks = tokenize_query("你好你还记得我们之前聊过什么吗", &sw);
        // Content segments survive ("你好" inside "你好你还", "聊过");
        // function words dropped.
        assert!(toks.iter().any(|t| t.contains("你好")), "got: {:?}", toks);
        assert!(toks.contains(&"聊过".to_string()), "got: {:?}", toks);
        assert!(!toks.iter().any(|t| t == "我们" || t == "什么" || t == "吗"), "got: {:?}", toks);
    }

    #[test]
    fn tokenize_english_stopwords_dropped() {
        let sw = helix_mind_core::config::RetrievalConfig::default().stopwords;
        let toks = tokenize_query("How are you Jason", &sw);
        assert_eq!(toks, vec!["jason"]);
    }

    #[test]
    fn route_threshold_uses_fts_for_3plus_like_for_shorter() {
        // Threshold is by char count of the trimmed, sanitized query.
        assert_eq!("认知相态".chars().count() >= FTS_MIN_CHARS, true);
        assert_eq!("ab".chars().count() >= FTS_MIN_CHARS, false);
    }
}
