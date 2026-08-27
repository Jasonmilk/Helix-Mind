use helix_mind_core::config::MetabolismConfig;
use helix_mind_core::error::MindError;
use std::sync::Arc;
use crate::cognitive::CognitiveService;

pub struct NerEngine {
    #[allow(dead_code)]
    config: MetabolismConfig,
    /// Single LLM-access port (ADR-0017) — no direct reqwest here.
    cognitive: Arc<dyn CognitiveService>,
}

impl NerEngine {
    pub fn new(config: MetabolismConfig, cognitive: Arc<dyn CognitiveService>) -> Self {
        Self { config, cognitive }
    }

    /// Extract entities via the cognitive port (P2c, ADR-0017). Backend is
    /// chosen by the adapter: Deterministic = local whitespace tokenization,
    /// Remote = HTTP NER gateway (debug_direct only). `ner_mode` no longer
    /// opens a direct outbound path.
    pub async fn extract_entities(&self, text: &str) -> Result<Vec<String>, MindError> {
        self.cognitive.extract_entities(text).await
    }
}
