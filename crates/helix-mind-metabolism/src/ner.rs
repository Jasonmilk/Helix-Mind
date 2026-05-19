use helix_mind_core::config::MetabolismConfig;
use helix_mind_core::error::MindError;

pub struct NerEngine {
    config: MetabolismConfig,
    // 移除了 local_session: Option<Session>，不再依赖 ort crate
}

impl NerEngine {
    pub fn new(config: MetabolismConfig) -> Result<Self, MindError> {
        // 不再加载 ONNX 模型，直接返回引擎实例
        Ok(Self { config })
    }

    pub async fn extract_entities(&self, text: &str) -> Result<Vec<String>, MindError> {
        match self.config.ner_mode.as_str() {
            "local" => self.local_extract(text).await,
            "remote" => self.remote_extract(text).await,
            _ => Ok(Vec::new()),
        }
    }

    async fn local_extract(&self, text: &str) -> Result<Vec<String>, MindError> {
        // 临时模拟实现：按空格分词
        // 未来如果需要真正的本地 NER，可以集成轻量级 crate (如 rust-bert) 而不依赖 ort
        Ok(text.split_whitespace().map(|s| s.to_string()).collect())
    }

    async fn remote_extract(&self, text: &str) -> Result<Vec<String>, MindError> {
        if self.config.ner_gateway_url.is_empty() {
            return Err(MindError::Metabolism("NER gateway URL is empty".into()));
        }

        let client = reqwest::Client::new();
        let resp: serde_json::Value = client.post(&self.config.ner_gateway_url)
            .json(&serde_json::json!({ "text": text }))
            .send()
            .await
            .map_err(|e| MindError::Metabolism(format!("NER request failed: {}", e)))?
            .json()
            .await
            .map_err(|e| MindError::Metabolism(format!("NER parse failed: {}", e)))?;

        let entities = resp["entities"].as_array()
            .ok_or_else(|| MindError::Metabolism("Invalid NER response".into()))?
            .iter()
            .filter_map(|e| e.as_str().map(|s| s.to_string()))
            .collect();

        Ok(entities)
    }
}