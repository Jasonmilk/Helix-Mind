//! The only LLM/cognitive-access port in the metabolism crate (P2b M-09,
//! ADR-0017).
//!
//! Iron-law guarantee: `cognitive.rs` is the *only* place in `helix-mind-metabolism`
//! that may hold a `reqwest`/HTTP outbound call. `crystallize`/`ner` never call
//! an LLM directly. Adapter selection is driven by `llm_mode`:
//! - `disabled` (production, locked) → `DeterministicAdapter` (no LLM).
//! - `debug_direct` (test/debug only) → `RemoteAdapter` (HTTP gateways).
//! - anything else is treated as `disabled` (fail-closed).
//! `RemoteAdapter::new` refuses construction outside `debug_direct`, so an
//! accidental outbound path is physically impossible (Z2 ruling).

use crate::symbolic::{self, LogicAssertion};
use async_trait::async_trait;
use helix_mind_core::config::MetabolismConfig;
use helix_mind_core::error::MindError;
use helix_mind_core::graph::{Node, NodeContent};
use std::collections::HashMap;
use std::sync::Arc;

/// Cognitive capability port. Every LLM-touching capability of Helix-Mind goes
/// through this trait.
#[async_trait]
pub trait CognitiveService: Send + Sync {
    /// L3 observations → an empirical principle (summary).
    async fn summarize(&self, nodes: &[Node]) -> Result<String, MindError>;
    /// Text → entity list (NER).
    async fn extract_entities(&self, text: &str) -> Result<Vec<String>, MindError>;
    /// Node → structured logic assertions (for SymbolicSolver arbitration).
    async fn translate_assertions(&self, node: &Node) -> Result<Vec<LogicAssertion>, MindError>;
}

// ── DeterministicAdapter (production default) ───────────────────────────

/// No-LLM adapter: keyword-frequency summary, whitespace tokenization for
/// entities, structured-content extraction for assertions. Deterministic,
/// zero external dependencies, zero outbound traffic.
pub struct DeterministicAdapter {
    #[allow(dead_code)]
    config: MetabolismConfig,
}

impl DeterministicAdapter {
    pub fn new(config: MetabolismConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl CognitiveService for DeterministicAdapter {
    async fn summarize(&self, nodes: &[Node]) -> Result<String, MindError> {
        Ok(deterministic_summarize(nodes))
    }

    async fn extract_entities(&self, text: &str) -> Result<Vec<String>, MindError> {
        Ok(text.split_whitespace().map(|s| s.to_string()).collect())
    }

    async fn translate_assertions(&self, node: &Node) -> Result<Vec<LogicAssertion>, MindError> {
        Ok(symbolic::assertions_from_node(&node.content))
    }
}

// ── RemoteAdapter (debug_direct only) ───────────────────────────────────

/// HTTP adapter to the LLM / NER gateways. Construction is refused unless
/// `llm_mode == "debug_direct"` (Z2: production is locked to Deterministic).
pub struct RemoteAdapter {
    llm_gateway_url: String,
    ner_gateway_url: String,
}

impl RemoteAdapter {
    pub fn new(config: &MetabolismConfig) -> Result<Self, MindError> {
        if config.llm_mode != "debug_direct" {
            return Err(MindError::Metabolism(format!(
                "RemoteAdapter requires llm_mode='debug_direct' (got '{}'); \
                 production is locked to DeterministicAdapter (Z2)",
                config.llm_mode
            )));
        }
        Ok(Self {
            llm_gateway_url: config.llm_gateway_url.clone(),
            ner_gateway_url: config.ner_gateway_url.clone(),
        })
    }
}

#[async_trait]
impl CognitiveService for RemoteAdapter {
    async fn summarize(&self, nodes: &[Node]) -> Result<String, MindError> {
        let mut prompt =
            "Summarize the following observations into a single empirical principle:\n"
                .to_string();
        for node in nodes {
            if let NodeContent::Text(t) = &node.content {
                prompt.push_str("- ");
                prompt.push_str(t);
                prompt.push_str("\n");
            }
        }
        prompt.push_str("\nPrinciple:");

        let client = reqwest::Client::new();
        let resp: serde_json::Value = client
            .post(&self.llm_gateway_url)
            .json(&serde_json::json!({
                "model": "llama3",
                "prompt": prompt,
                "stream": false,
            }))
            .send()
            .await
            .map_err(|e| MindError::Metabolism(format!("LLM request failed: {}", e)))?
            .json()
            .await
            .map_err(|e| MindError::Metabolism(format!("LLM response parse failed: {}", e)))?;
        Ok(resp["response"]
            .as_str()
            .unwrap_or("No summary generated")
            .to_string())
    }

    async fn extract_entities(&self, text: &str) -> Result<Vec<String>, MindError> {
        if self.ner_gateway_url.is_empty() {
            return Err(MindError::Metabolism(
                "NER gateway URL is empty (ner_gateway_url)".into(),
            ));
        }
        let client = reqwest::Client::new();
        let resp: serde_json::Value = client
            .post(&self.ner_gateway_url)
            .json(&serde_json::json!({ "text": text }))
            .send()
            .await
            .map_err(|e| MindError::Metabolism(format!("NER request failed: {}", e)))?
            .json()
            .await
            .map_err(|e| MindError::Metabolism(format!("NER parse failed: {}", e)))?;
        let entities = resp["entities"]
            .as_array()
            .ok_or_else(|| MindError::Metabolism("Invalid NER response".into()))?
            .iter()
            .filter_map(|e| e.as_str().map(|s| s.to_string()))
            .collect();
        Ok(entities)
    }

    async fn translate_assertions(&self, node: &Node) -> Result<Vec<LogicAssertion>, MindError> {
        // Structured assertions are deterministic regardless of backend;
        // Text-only translation is a P2c+ follow-up (LLM translator role).
        Ok(symbolic::assertions_from_node(&node.content))
    }
}

// ── FakeAdapter (tests) ─────────────────────────────────────────────────

/// Deterministic fixed outputs for tests (replaces the old FakeAdapter seam
/// without touching the retrieval crate).
pub struct FakeAdapter {
    summary: String,
    entities: Vec<String>,
}

impl FakeAdapter {
    pub fn new(summary: &str, entities: Vec<String>) -> Self {
        Self {
            summary: summary.to_string(),
            entities,
        }
    }
}

#[async_trait]
impl CognitiveService for FakeAdapter {
    async fn summarize(&self, _nodes: &[Node]) -> Result<String, MindError> {
        Ok(self.summary.clone())
    }
    async fn extract_entities(&self, _text: &str) -> Result<Vec<String>, MindError> {
        Ok(self.entities.clone())
    }
    async fn translate_assertions(&self, node: &Node) -> Result<Vec<LogicAssertion>, MindError> {
        Ok(symbolic::assertions_from_node(&node.content))
    }
}

// ── Factory ─────────────────────────────────────────────────────────────

/// Build the cognitive service for the given config. `llm_mode`:
/// `debug_direct` → RemoteAdapter; everything else → DeterministicAdapter
/// (fail-closed; unknown values are treated as `disabled`).
pub fn build_cognitive_service(
    config: &MetabolismConfig,
) -> Result<Arc<dyn CognitiveService>, MindError> {
    if config.llm_mode == "debug_direct" {
        Ok(Arc::new(RemoteAdapter::new(config)?))
    } else {
        Ok(Arc::new(DeterministicAdapter::new(config.clone())))
    }
}

// ── Deterministic summary heuristic ─────────────────────────────────────

/// No-LLM empirical-principle summary: aggregate the observation texts, count
/// whitespace tokens + CJK bigrams, and emit a principle anchored on the top
/// keywords. Honest heuristic — quality is knowingly below an LLM summary;
/// `debug_direct` switches to the remote backend for real synthesis.
pub fn deterministic_summarize(nodes: &[Node]) -> String {
    let texts: Vec<String> = nodes
        .iter()
        .filter_map(|n| match &n.content {
            NodeContent::Text(t) => Some(t.clone()),
            _ => None,
        })
        .collect();
    if texts.is_empty() {
        return "无文本内容可摘要".to_string();
    }

    let mut freq: HashMap<String, usize> = HashMap::new();
    for t in &texts {
        for token in tokenize(t) {
            *freq.entry(token).or_insert(0) += 1;
        }
    }
    let mut ranked: Vec<(String, usize)> = freq.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let keywords: Vec<String> = ranked.iter().take(5).map(|(k, _)| k.clone()).collect();
    format!("经验原则：基于 {} 条观察归纳，核心关键词：{}。", texts.len(), keywords.join("、"))
}

/// Whitespace tokens + CJK bigrams (deterministic, no external dictionary).
fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    // CJK bigrams
    for i in 0..chars.len().saturating_sub(1) {
        if is_cjk(chars[i]) && is_cjk(chars[i + 1]) {
            tokens.push(format!("{}{}", chars[i], chars[i + 1]));
        }
    }
    // Whitespace-delimited tokens (drop pure-CJK run fragments)
    for tok in text.split_whitespace() {
        if tok.chars().any(|c| !is_cjk(c)) {
            tokens.push(tok.to_string());
        }
    }
    tokens
}

fn is_cjk(c: char) -> bool {
    matches!(c as u32, 0x4E00..=0x9FFF | 0x3400..=0x4DBF | 0x20000..=0x2A6DF)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_node(s: &str) -> Node {
        Node {
            content: NodeContent::Text(s.to_string()),
            ..Node::default()
        }
    }

    #[tokio::test]
    async fn deterministic_adapter_is_llm_free() {
        let adapter = DeterministicAdapter::new(MetabolismConfig::default());
        let nodes = vec![
            text_node("过热会导致系统崩溃"),
            text_node("过热会导致性能下降"),
        ];
        let s = adapter.summarize(&nodes).await.unwrap();
        assert!(s.contains("过热"), "keyword anchor present: {s}");
        assert!(s.contains("2 条观察"));
    }

    #[tokio::test]
    async fn remote_adapter_refuses_construction_outside_debug_direct() {
        let config = MetabolismConfig::default(); // llm_mode = "disabled"
        assert!(RemoteAdapter::new(&config).is_err());
        let mut config = config;
        config.llm_mode = "debug_direct".into();
        assert!(RemoteAdapter::new(&config).is_ok());
    }

    #[tokio::test]
    async fn fake_adapter_returns_fixed_outputs() {
        let adapter = FakeAdapter::new("原则X", vec!["A".into(), "B".into()]);
        let s = adapter.summarize(&[]).await.unwrap();
        let e = adapter.extract_entities("anything").await.unwrap();
        assert_eq!(s, "原则X");
        assert_eq!(e, vec!["A", "B"]);
    }

    #[tokio::test]
    async fn factory_fail_closed_for_unknown_mode() {
        let mut config = MetabolismConfig::default();
        config.llm_mode = "bogus".into();
        let svc = build_cognitive_service(&config).unwrap();
        // Unknown mode → Deterministic (fail-closed).
        let s = svc.summarize(&[]).await.unwrap();
        assert!(s.contains("无文本内容可摘要"));
    }
}
