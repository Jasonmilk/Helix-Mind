use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub retrieval: RetrievalConfig,
    #[serde(default)]
    pub storage: StorageConfig,
    #[serde(default)]
    pub metabolism: MetabolismConfig,
    #[serde(default)]
    pub lifecycle: LifecycleConfig,
    #[serde(default)]
    pub federation: FederationConfig,
    #[serde(default)]
    pub gene_lock: GeneLockConfig,
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub mind: MindSystemConfig,
}

// ---------- RetrievalConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RetrievalConfig {
    #[serde(default = "default_max_hops")]
    pub max_hops: usize,
    #[serde(default = "default_beam_width")]
    pub beam_width: usize,
    #[serde(default = "default_weight_threshold")]
    pub weight_threshold: f64,
    #[serde(default = "default_soft_edge_decay")]
    pub soft_edge_decay_factor: f64,
    #[serde(default = "default_soft_edge_min_weight")]
    pub soft_edge_min_weight: f64,
    #[serde(default = "default_max_nodes_per_query")]
    pub max_nodes_per_query: usize,
    #[serde(default = "default_dead_end_penalty")]
    pub dead_end_penalty_factor: f64,
    #[serde(default = "default_tentative_edge_weight")]
    pub tentative_edge_weight: f64,
}

// ---------- StorageConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct StorageConfig {
    #[serde(default = "default_sqlite_path")]
    pub sqlite_path: String,
    #[serde(default = "default_parquet_dir")]
    pub parquet_dir: String,
    #[serde(default = "default_deep_cold_dir")]
    pub deep_cold_dir: String,
    #[serde(default = "default_human_view_dir")]
    pub human_view_dir: String,
    #[serde(default = "default_human_view_max_size_mb")]
    pub human_view_max_size_mb: u64,
    #[serde(default = "default_topology_max_nodes")]
    pub topology_max_nodes: usize,
    #[serde(default = "default_l3_merge_similarity")]
    pub l3_merge_similarity_threshold: f64,
    #[serde(default = "default_vector_similarity")]
    pub vector_similarity_threshold: f64,
    #[serde(default = "default_node_cache_capacity")]
    pub node_cache_capacity: u64,
    #[serde(default = "default_deferred_write_interval_sec")]
    pub deferred_write_interval_sec: u64,
}

// ---------- MetabolismConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MetabolismConfig {
    #[serde(default = "default_deep_cold_dir")]
    pub deep_cold_dir: String,
    #[serde(default = "default_micro_sleep_interval")]
    pub digest_interval_sec: u64,
    #[serde(default = "default_merge_similarity")]
    pub merge_similarity_threshold: f64,
    #[serde(default = "default_crystallize_idle_timeout")]
    pub crystallize_idle_timeout_sec: u64,
    #[serde(default = "default_resurrection_window")]
    pub resurrection_window_days: i64,
    #[serde(default = "default_llm_gateway_url")]
    pub llm_gateway_url: String,
    /// LLM access mode: "disabled" (production, locked) | "debug_direct" (test/debug only).
    #[serde(default = "default_llm_mode")]
    pub llm_mode: String,
    #[serde(default = "default_ner_mode")]
    pub ner_mode: String,
    #[serde(default = "default_ner_gateway_url")]
    pub ner_gateway_url: String,
    #[serde(default = "default_ner_model_path")]
    pub ner_model_path: String,
    #[serde(default = "default_dedup_mode")]
    pub dedup_mode: String,
    #[serde(default = "default_semantic_model_path")]
    pub semantic_model_path: String,
}

// ---------- LifecycleConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct LifecycleConfig {
    #[serde(default = "default_lifecycle_enabled")]
    pub enabled: bool,
    #[serde(default = "default_max_nodes")]
    pub max_nodes: Option<u64>,
    #[serde(default = "default_max_interactions")]
    pub max_interactions: Option<u64>,
    #[serde(default = "default_max_wall_clock_days")]
    pub max_wall_clock_days: Option<u64>,
    #[serde(default = "default_countdown_minutes")]
    pub countdown_minutes: u64,
    #[serde(default = "default_inheritance_crystal")]
    pub inheritance_crystal: bool,
    #[serde(default = "default_archive_past_life")]
    pub archive_past_life: bool,
    /// Emergency dusk: minimum free memory (MB) before deterministic fallback.
    #[serde(default = "default_emergency_dusk_min_memory_mb")]
    pub emergency_dusk_min_memory_mb: u64,
    /// Emergency dusk: minimum token balance before deterministic fallback.
    #[serde(default = "default_emergency_dusk_min_tokens")]
    pub emergency_dusk_min_tokens: u64,
}

// ---------- FederationConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FederationConfig {
    #[serde(default = "default_outgoing_dir")]
    pub outgoing_dir: String,
    #[serde(default = "default_sandbox_dir")]
    pub sandbox_dir: String,
    #[serde(default = "default_cremation_years")]
    pub cremation_years: u64,
    #[serde(default = "default_scan_interval_sec")]
    pub scan_interval_sec: u64,
}

// ---------- GeneLockConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GeneLockConfig {
    #[serde(default = "default_gene_lock_path")]
    pub file_path: String,
}

// ---------- ApiConfig ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ApiConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    #[serde(default = "default_layer1_enabled")]
    pub layer1_enabled: bool,
    #[serde(default = "default_layer2_enabled")]
    pub layer2_enabled: bool,
}

// ---------- MindSystemConfig (v3.3 New Core Config) ----------
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MindSystemConfig {
    /// Dominance threshold for node priority (架构师指定默认值 0.8)
    #[serde(default = "default_dominance_threshold")]
    pub dominance_threshold: f64,
    /// Utility consensus weight threshold
    #[serde(default = "default_utility_threshold")]
    pub utility_threshold: f64,
    /// Minimum required corroborations for valid node
    #[serde(default = "default_corroboration_min")]
    pub corroboration_min_required: u64,
    /// Enable validation for high-risk nodes
    #[serde(default = "default_high_risk_validation")]
    pub high_risk_validation_enabled: bool,
    /// Max retry attempts when impasse occurs
    #[serde(default = "default_impasse_retry_limit")]
    pub impasse_retry_limit: u8,
}

// ---------- Default Functions ----------
fn default_max_hops() -> usize { 3 }
fn default_beam_width() -> usize { 3 }
fn default_weight_threshold() -> f64 { 0.8 }
fn default_soft_edge_decay() -> f64 { 0.8 }
fn default_soft_edge_min_weight() -> f64 { 0.1 }
fn default_max_nodes_per_query() -> usize { 20 }
fn default_dead_end_penalty() -> f64 { 0.8 }
fn default_tentative_edge_weight() -> f64 { 0.3 }

fn default_sqlite_path() -> String { "./data/helix_mind.db".into() }
fn default_parquet_dir() -> String { "./data/parquet".into() }
fn default_deep_cold_dir() -> String { "./data/deep_cold".into() }
fn default_human_view_dir() -> String { "./data/human_views".into() }
fn default_human_view_max_size_mb() -> u64 { 10 }
fn default_topology_max_nodes() -> usize { 500_000 }
fn default_l3_merge_similarity() -> f64 { 0.95 }
fn default_vector_similarity() -> f64 { 0.85 }
fn default_node_cache_capacity() -> u64 { 10000 }
fn default_deferred_write_interval_sec() -> u64 { 5 }

fn default_micro_sleep_interval() -> u64 { 300 }
fn default_merge_similarity() -> f64 { 0.95 }
fn default_crystallize_idle_timeout() -> u64 { 600 }
fn default_resurrection_window() -> i64 { 30 }
fn default_llm_gateway_url() -> String { "http://localhost:11434/api/generate".into() }
fn default_llm_mode() -> String { "disabled".into() }
fn default_ner_mode() -> String { "local".into() }
fn default_ner_gateway_url() -> String { String::new() }
fn default_ner_model_path() -> String { "./models/ner.onnx".into() }
fn default_dedup_mode() -> String { "lexical".into() }
fn default_semantic_model_path() -> String { "./models/all-MiniLM-L6-v2.onnx".into() }

fn default_lifecycle_enabled() -> bool { false }
fn default_max_nodes() -> Option<u64> { Some(100_000) }
fn default_max_interactions() -> Option<u64> { Some(50_000) }
fn default_max_wall_clock_days() -> Option<u64> { Some(3650) }
fn default_countdown_minutes() -> u64 { 15 }
fn default_inheritance_crystal() -> bool { true }
fn default_archive_past_life() -> bool { true }

fn default_outgoing_dir() -> String { "./federation/outgoing".into() }
fn default_sandbox_dir() -> String { "./federation/sandbox".into() }
fn default_cremation_years() -> u64 { 100 }
fn default_scan_interval_sec() -> u64 { 60 }

fn default_gene_lock_path() -> String { "./gene_lock.md".into() }
fn default_listen_addr() -> String { "127.0.0.1:50051".into() }
fn default_layer1_enabled() -> bool { true }
fn default_layer2_enabled() -> bool { true }

// ---------- v3.3 MindSystem Default Functions ----------
fn default_dominance_threshold() -> f64 { 0.8 }
fn default_utility_threshold() -> f64 { 0.7 }
fn default_corroboration_min() -> u64 { 1 }
fn default_high_risk_validation() -> bool { true }
fn default_impasse_retry_limit() -> u8 { 3 }
fn default_emergency_dusk_min_memory_mb() -> u64 { 50 }
fn default_emergency_dusk_min_tokens() -> u64 { 100 }

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config_path = std::env::var("HELIX_MIND_CONFIG")
            .unwrap_or_else(|_| "config.toml".to_string());

        let config = config::Config::builder()
            .add_source(config::File::with_name(&config_path).required(false))
            .add_source(
                config::Environment::with_prefix("HELIX_MIND")
                    .separator("__")
            )
            .build()?;

        config.try_deserialize()
    }

    pub fn compute_core_hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let core_data = format!(
            "{:?}{:?}{:?}{:?}",
            self.lifecycle.enabled,
            self.metabolism.digest_interval_sec,
            self.storage.sqlite_path,
            self.gene_lock.file_path
        );
        let mut hasher = Sha256::new();
        hasher.update(core_data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
