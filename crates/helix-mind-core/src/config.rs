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

impl Default for Config {
    fn default() -> Self {
        Self {
            retrieval: RetrievalConfig::default(),
            storage: StorageConfig::default(),
            metabolism: MetabolismConfig::default(),
            lifecycle: LifecycleConfig::default(),
            federation: FederationConfig::default(),
            gene_lock: GeneLockConfig::default(),
            api: ApiConfig::default(),
            mind: MindSystemConfig::default(),
        }
    }
}

// ---------- RetrievalConfig ----------
#[derive(Debug, Clone, Deserialize)]
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

// Manual Default (P0 debt fix): derive(Default) ignored serde default fns and
// produced empty/zero values when a whole section (e.g. `[api]`) was absent.
impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            max_hops: default_max_hops(),
            beam_width: default_beam_width(),
            weight_threshold: default_weight_threshold(),
            soft_edge_decay_factor: default_soft_edge_decay(),
            soft_edge_min_weight: default_soft_edge_min_weight(),
            max_nodes_per_query: default_max_nodes_per_query(),
            dead_end_penalty_factor: default_dead_end_penalty(),
            tentative_edge_weight: default_tentative_edge_weight(),
        }
    }
}

// ---------- StorageConfig ----------
#[derive(Debug, Clone, Deserialize)]
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
    /// P6 (ADR-0015): 是否启用 WAL 事实来源（默认 true）。
    /// `:memory:` 数据库自动禁用（内存库无持久伙伴）。
    #[serde(default = "default_wal_enabled")]
    pub wal_enabled: bool,
    /// P6 (ADR-0015): WAL 段文件目录（默认 ./data/wal）。
    #[serde(default = "default_wal_dir")]
    pub wal_dir: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            sqlite_path: default_sqlite_path(),
            parquet_dir: default_parquet_dir(),
            deep_cold_dir: default_deep_cold_dir(),
            human_view_dir: default_human_view_dir(),
            human_view_max_size_mb: default_human_view_max_size_mb(),
            topology_max_nodes: default_topology_max_nodes(),
            l3_merge_similarity_threshold: default_l3_merge_similarity(),
            vector_similarity_threshold: default_vector_similarity(),
            node_cache_capacity: default_node_cache_capacity(),
            deferred_write_interval_sec: default_deferred_write_interval_sec(),
            wal_enabled: default_wal_enabled(),
            wal_dir: default_wal_dir(),
        }
    }
}

// ---------- MetabolismConfig ----------
#[derive(Debug, Clone, Deserialize)]
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
    /// How old a Conflicts edge must be before it becomes eligible for
    /// deterministic arbitration (cooling window). Digest calls
    /// `get_unresolved_dissonance` with this window.
    #[serde(default = "default_dissonance_window")]
    pub dissonance_window_hours: u64,
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

impl Default for MetabolismConfig {
    fn default() -> Self {
        Self {
            deep_cold_dir: default_deep_cold_dir(),
            digest_interval_sec: default_micro_sleep_interval(),
            merge_similarity_threshold: default_merge_similarity(),
            crystallize_idle_timeout_sec: default_crystallize_idle_timeout(),
            resurrection_window_days: default_resurrection_window(),
            dissonance_window_hours: default_dissonance_window(),
            llm_gateway_url: default_llm_gateway_url(),
            llm_mode: default_llm_mode(),
            ner_mode: default_ner_mode(),
            ner_gateway_url: default_ner_gateway_url(),
            ner_model_path: default_ner_model_path(),
            dedup_mode: default_dedup_mode(),
            semantic_model_path: default_semantic_model_path(),
        }
    }
}

// ---------- LifecycleConfig ----------
#[derive(Debug, Clone, Deserialize)]
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

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            enabled: default_lifecycle_enabled(),
            max_nodes: default_max_nodes(),
            max_interactions: default_max_interactions(),
            max_wall_clock_days: default_max_wall_clock_days(),
            countdown_minutes: default_countdown_minutes(),
            inheritance_crystal: default_inheritance_crystal(),
            archive_past_life: default_archive_past_life(),
            emergency_dusk_min_memory_mb: default_emergency_dusk_min_memory_mb(),
            emergency_dusk_min_tokens: default_emergency_dusk_min_tokens(),
        }
    }
}

// ---------- FederationConfig ----------
#[derive(Debug, Clone, Deserialize)]
pub struct FederationConfig {
    /// 出站门控（ADR-0018 P3a）：能力未就绪 = 功能不存在。
    /// 默认 false，联邦出站/入站处理在未显式启用时一律拒绝。
    #[serde(default = "default_federation_enabled")]
    pub enabled: bool,
    #[serde(default = "default_outgoing_dir")]
    pub outgoing_dir: String,
    #[serde(default = "default_sandbox_dir")]
    pub sandbox_dir: String,
    #[serde(default = "default_cremation_years")]
    pub cremation_years: u64,
    #[serde(default = "default_scan_interval_sec")]
    pub scan_interval_sec: u64,
}

impl Default for FederationConfig {
    fn default() -> Self {
        Self {
            enabled: default_federation_enabled(),
            outgoing_dir: default_outgoing_dir(),
            sandbox_dir: default_sandbox_dir(),
            cremation_years: default_cremation_years(),
            scan_interval_sec: default_scan_interval_sec(),
        }
    }
}

// ---------- GeneLockConfig ----------
#[derive(Debug, Clone, Deserialize)]
pub struct GeneLockConfig {
    #[serde(default = "default_gene_lock_path")]
    pub file_path: String,
}

impl Default for GeneLockConfig {
    fn default() -> Self {
        Self { file_path: default_gene_lock_path() }
    }
}

// ---------- ApiConfig ----------
/// gRPC 传输模式（ADR-0019 P3b）
/// - `Tcp`：远程部署，mTLS 预留（P3 后实现）
/// - `Unix`：本地 UDS，SO_PEERCRED 白名单鉴权（fail-closed）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Transport {
    Tcp,
    Unix,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    #[serde(default = "default_transport")]
    pub transport: Transport,
    #[serde(default = "default_trusted_uids")]
    pub trusted_uids: Vec<u32>,
    /// 负载检查阈值（ValidationLayer，ADR-0019 P3b 收尾）。
    /// 超过该值返回 Unavailable；默认 1.0 表示无负载源时不拒绝。
    #[serde(default = "default_max_system_load")]
    pub max_system_load: f64,
    #[serde(default = "default_layer1_enabled")]
    pub layer1_enabled: bool,
    #[serde(default = "default_layer2_enabled")]
    pub layer2_enabled: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            listen_addr: default_listen_addr(),
            transport: default_transport(),
            trusted_uids: default_trusted_uids(),
            max_system_load: default_max_system_load(),
            layer1_enabled: default_layer1_enabled(),
            layer2_enabled: default_layer2_enabled(),
        }
    }
}

// ---------- MindSystemConfig (v3.3 New Core Config) ----------
#[derive(Debug, Clone, Deserialize)]
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

impl Default for MindSystemConfig {
    fn default() -> Self {
        Self {
            dominance_threshold: default_dominance_threshold(),
            utility_threshold: default_utility_threshold(),
            corroboration_min_required: default_corroboration_min(),
            high_risk_validation_enabled: default_high_risk_validation(),
            impasse_retry_limit: default_impasse_retry_limit(),
        }
    }
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
fn default_wal_enabled() -> bool { true }
fn default_wal_dir() -> String { "./data/wal".into() }
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
fn default_dissonance_window() -> u64 { 24 }
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
fn default_federation_enabled() -> bool { false }

fn default_gene_lock_path() -> String { "./gene_lock.md".into() }
fn default_listen_addr() -> String { "127.0.0.1:50051".into() }
fn default_transport() -> Transport { Transport::Tcp }
fn default_trusted_uids() -> Vec<u32> { Vec::new() }
fn default_max_system_load() -> f64 { 1.0 }
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

    /// P0 debt fix: create all declared parent directories at startup so the
    /// process does not fail with ENOENT on first run (e.g. `./data` missing).
    /// Never touches the SQLite file itself — only parent directories.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        let mut dirs = Vec::new();

        // Storage parents.
        push_parent(&mut dirs, &self.storage.sqlite_path);
        dirs.push(std::path::Path::new(&self.storage.parquet_dir).to_path_buf());
        dirs.push(std::path::Path::new(&self.storage.deep_cold_dir).to_path_buf());
        dirs.push(std::path::Path::new(&self.storage.human_view_dir).to_path_buf());

        // Federation parents.
        dirs.push(std::path::Path::new(&self.federation.outgoing_dir).to_path_buf());
        dirs.push(std::path::Path::new(&self.federation.sandbox_dir).to_path_buf());

        // Model file parents.
        push_parent(&mut dirs, &self.metabolism.ner_model_path);
        push_parent(&mut dirs, &self.metabolism.semantic_model_path);

        // Gene lock parent (usually cwd — skip empty).
        push_parent(&mut dirs, &self.gene_lock.file_path);

        for dir in dirs {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(())
    }
}

/// Push the parent directory of `p` onto `dirs` unless it is empty.
fn push_parent(dirs: &mut Vec<std::path::PathBuf>, p: &str) {
    use std::path::Path;
    if !p.is_empty() {
        if let Some(parent) = Path::new(p).parent() {
            if !parent.as_os_str().is_empty() {
                dirs.push(parent.to_path_buf());
            }
        }
    }
}
