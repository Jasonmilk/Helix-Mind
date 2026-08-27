use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::MindError;
use serde_json::Value;

// ---------- Enums ----------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    L0,
    L1,
    L2,
    L3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sensitivity {
    Public,
    Private,
    Sensitive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CognitiveMode {
    Skilled,
    Anchor,
    Imagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutonomyLevel {
    Agent,
    Open,
    Survival,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationType {
    Causal,
    Semantic,
    Temporal,
    CoOccurrence,
    Corrects,
    Refines,
    Doubts,
    SimilarTo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TerminationCause {
    LimitReached,
    TaskFailed,
    UserTerminated,
}

// ---------- NodeContent ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeContent {
    Text(String),
    Structured(HashMap<String, String>),
    Reference(String),
    Event {
        event_type: String,
        payload: HashMap<String, String>,
    },
    GeneLock {
        lineage_name: String,
        core_principles: Vec<String>,
        custom_clauses: Vec<String>,
    },
}

// ---------- Phase-State & Subject-Dependency (P0 / ADR-0010, ADR-0011) ----------

/// Maturity state of a knowledge node (ADR-0011).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseState {
    Gas,
    Liquid,
    Crystal,
}

impl Default for PhaseState {
    fn default() -> Self {
        PhaseState::Liquid
    }
}

/// Privacy axis: whether the node may enter the shared knowledge tree (ADR-0011).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectDependency {
    High,
    Low,
}

impl Default for SubjectDependency {
    fn default() -> Self {
        SubjectDependency::High
    }
}

/// Liquid concentration marker (colloid = high-concentration liquid, not a separate layer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Concentration {
    Dissolved,
    Colloidal,
}

impl Default for Concentration {
    fn default() -> Self {
        Concentration::Dissolved
    }
}

/// Liquid meta — concentration + internal tension (ADR-0011).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhaseMeta {
    pub concentration: Concentration,
    pub tension: f64,
}

impl Default for PhaseMeta {
    fn default() -> Self {
        Self {
            concentration: Concentration::Dissolved,
            tension: 0.0,
        }
    }
}

/// Cognitive budget routing class — decided by the body before calling Mind (ADR-0010).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetTier {
    /// Default: normal deep query (liquid + colloid). Backward compatible for generic bodies.
    Augmentable,
    /// 0-token emergency — crystals + high-relevance colloids only.
    Endogenous,
    /// Exploration — may touch gas traces.
    ExogenousRequired,
    /// No cognition — metadata only.
    Void,
}

impl Default for BudgetTier {
    fn default() -> Self {
        BudgetTier::Augmentable
    }
}

// ---------- Node ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub node_type: NodeType,
    pub content: NodeContent,
    pub heat: f64,
    pub is_hypothetical: bool,
    pub is_recessive: bool,
    pub sensitivity: Option<Sensitivity>,
    pub generation: u64,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub access_count: u64,
    pub initial_impact: f64,
    pub corrected_by: Option<Uuid>,
    pub notes: Option<String>,
    pub dominance: f64,            // Dominance index [0.0, 1.0]
    pub utility: f64,              // Utility consensus weight [0.0, 1.0]
    pub corroborations: u64,       // Count of corroborations
    pub attribution_ledger: Vec<HonorStamp>, // Ledger for attribution records
    pub source: NodeSource,        // Marker for node origin/source
    pub high_risk: bool,           // Flag for high-risk nodes
    pub abstract_provenance: Option<String>,  // Summary after evidence fixation
    pub derived_from: Vec<Uuid>,
    #[serde(default)]
    pub phase_state: PhaseState,                // ADR-0011: maturity state
    #[serde(default)]
    pub subject_dependency: SubjectDependency, // ADR-0011: privacy axis (materialized)
    #[serde(default)]
    pub meta: PhaseMeta,                        // ADR-0011: liquid meta (concentration/tension)
}

impl Default for Node {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            node_type: NodeType::L3,
            content: NodeContent::Text(String::new()),
            heat: 0.5,
            is_hypothetical: false,
            is_recessive: false,
            sensitivity: None,
            generation: 1,
            created_at: now,
            last_accessed_at: now,
            access_count: 0,
            initial_impact: 0.5,
            corrected_by: None,
            notes: None,
            dominance: 0.5,          // Default dominance weight: 0.5
            utility: 0.5,           // Default utility score: 0.5
            corroborations: 0,      // Default corroboration count: 0
            attribution_ledger: Vec::new(), // Default empty attribution ledger
            source: NodeSource::default(), // Default local node source
            high_risk: false,       // Default: non high-risk
            abstract_provenance: None, // Default empty abstract provenance
            derived_from: Vec::new(),
            phase_state: PhaseState::default(),          // Liquid
            subject_dependency: SubjectDependency::default(), // High (default node is L3)
            meta: PhaseMeta::default(),                  // Dissolved, tension 0.0
        }
    }
}

// ---------- Edge ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub weight: f64,
    pub relation_type: RelationType,
    pub is_soft: bool,
}

// ---------- Validate trait ----------

pub trait Validate {
    fn validate(&self) -> Result<(), crate::error::MindError>;
}

impl Validate for Node {
    fn validate(&self) -> Result<(), crate::error::MindError> {
        if self.heat < 0.0 || self.heat > 1.0 {
            return Err(crate::error::MindError::Validation("heat must be in [0,1]".into()));
        }
        if self.initial_impact < 0.0 || self.initial_impact > 1.0 {
            return Err(crate::error::MindError::Validation("initial_impact must be in [0,1]".into()));
        }
        if self.node_type == NodeType::L3 && self.sensitivity.is_none() {
            return Err(crate::error::MindError::Validation("L3 nodes must have sensitivity".into()));
        }
        if self.node_type == NodeType::L0 {
            if let NodeContent::GeneLock { .. } = self.content {
                // valid
            } else {
                return Err(crate::error::MindError::Validation("L0 nodes must have GeneLock content".into()));
            }
        }
        Ok(())
    }
}

impl Validate for Edge {
    fn validate(&self) -> Result<(), crate::error::MindError> {
        if self.weight < -1.0 || self.weight > 1.0 {
            return Err(crate::error::MindError::Validation("weight must be in [-1, 1]".into()));
        }
        if self.relation_type == RelationType::Corrects && self.weight != -1.0 {
            return Err(crate::error::MindError::Validation("Corrects edge must have weight -1.0".into()));
        }
        if self.relation_type == RelationType::Doubts && (self.weight < 0.0 || self.weight > 1.0) {
            return Err(crate::error::MindError::Validation("Doubts edge weight must be non-negative".into()));
        }
        if matches!(self.relation_type, RelationType::Corrects | RelationType::Refines | RelationType::Doubts) && self.is_soft {
            return Err(crate::error::MindError::Validation("Dialectical edges must be hard (is_soft = false)".into()));
        }
        Ok(())
    }
}

// ---------- Energy Context ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyContext {
    pub token_budget: u64,
    pub heliotropism: f64,
    pub pulse: f64,
    pub vigilance: f64,
    pub latency_limit_ms: u64,
    pub system_load: f64,
    pub impasse_depth: u8,   // Current impasse depth (default 0)
    pub familiarity: f64,    // System familiarity weight (default 0.5)
    pub budget_tier: BudgetTier, // ADR-0010: cognitive budget routing class
}

impl Default for EnergyContext {
    fn default() -> Self {
        Self {
            token_budget: 1000,
            heliotropism: 0.0,
            pulse: 0.3,
            vigilance: 0.2,
            latency_limit_ms: 500,
            system_load: 0.0,
            impasse_depth: 0,
            familiarity: 0.5,
            budget_tier: BudgetTier::default(),
        }
    }
}

impl Validate for EnergyContext {
    fn validate(&self) -> Result<(), crate::error::MindError> {
        if self.heliotropism < -1.0 || self.heliotropism > 1.0 {
            return Err(crate::error::MindError::Validation("heliotropism must be in [-1,1]".into()));
        }
        if self.pulse < 0.0 || self.pulse > 1.0 {
            return Err(crate::error::MindError::Validation("pulse must be in [0,1]".into()));
        }
        if self.vigilance < 0.0 || self.vigilance > 1.0 {
            return Err(crate::error::MindError::Validation("vigilance must be in [0,1]".into()));
        }
        if self.system_load < 0.0 || self.system_load > 1.0 {
            return Err(crate::error::MindError::Validation("system_load must be in [0,1]".into()));
        }
        Ok(())
    }
}

// ---------- Query Structures ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixQueryRequest {
    pub query: String,
    pub suggested_mode: CognitiveMode,
    pub energy_context: EnergyContext,
    pub include_recessive: bool,
    pub allow_imagination: bool,
    pub autonomy_level: AutonomyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixQueryResult {
    pub effective_mode: CognitiveMode,
    pub mode_negotiation: Option<String>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub trace_id: Uuid,
    pub latency_ms: u64,
    pub tokens_consumed: u64,
    pub is_partial: bool,
    pub exhaustion_reason: Option<String>,
    pub impasse_level: ImpasseLevel,
    pub stages_attempted: u8,
    pub suggested_actions: Vec<ActionSuggestion>,
}

// ---------- L0 Gene Lock ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L0GeneLock {
    pub lineage_name: String,
    pub core_principles: Vec<String>,
    pub custom_clauses: Vec<String>,
    pub memory_integrity: bool,
    pub l0_hash: String,
}

// ---------- Life Record ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeRecord {
    pub generation: u64,
    pub name_at_time: String,
    pub lifespan: Lifespan,
    pub tokens: TokenUsage,
    pub dag_stats: DagStats,
    pub inheritance_crystal_hash: Option<String>,
    pub note_to_next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifespan {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub cause: TerminationCause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub total_consumed: u64,
    pub by_task: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagStats {
    pub nodes_added: u64,
    pub edges_added: u64,
    pub federated_contributions: Vec<FederatedContribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedContribution {
    pub source_helix_id: String,
    pub nodes_received: u64,
    pub timestamp: DateTime<Utc>,
}

// ---------- Deep Cold Stub ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepColdStub {
    pub node_id: Uuid,
    pub status: String,
    pub compressed_location: String,
    pub compressed_size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
    pub original_type: NodeType,
}

// ---------- Audit ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub actor: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    GeneLockReloaded,
    LifecyclePhaseChanged,
    FederationNodeReceived,
    FederationNodeMerged,
    DigitalCremationTriggered,
    HumanViewSynced,
    ReincarnationTriggered,
    PrivacyAccessGranted,
    EmergencyDuskTriggered,
}

impl AuditEntry {
    pub fn new(event_type: AuditEventType, actor: &str, details: &str) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            actor: actor.to_string(),
            details: details.to_string(),
        }
    }
}

impl L0GeneLock {
    pub fn from_markdown(content: &str) -> Result<Self, MindError> {
        let mut lineage_name = "Dash".to_string();
        let mut core_principles = Vec::new();
        let mut custom_clauses = Vec::new();
        let mut memory_integrity = true;

        let mut current_section = "";
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("## ") {
                current_section = &trimmed[3..];
            } else if trimmed.starts_with("1. ") || trimmed.starts_with("2. ") || trimmed.starts_with("3. ") {
                core_principles.push(trimmed[3..].to_string());
            } else if trimmed.starts_with("4. ") || trimmed.starts_with("5. ") {
                custom_clauses.push(trimmed[3..].to_string());
            } else if current_section == "Lineage Name" && !trimmed.is_empty() && !trimmed.starts_with('#') {
                lineage_name = trimmed.to_string();
            } else if current_section == "Memory Integrity" && trimmed.contains("append-only") {
                memory_integrity = true;
            }
        }

        let mut gene_lock = Self {
            lineage_name,
            core_principles,
            custom_clauses,
            memory_integrity,
            l0_hash: String::new(),
        };

        let canonical = serde_json::to_string(&gene_lock)
            .map_err(|e| MindError::Config(e.to_string()))?;
        gene_lock.l0_hash = crate::sha256_digest(canonical.as_bytes());

        Ok(gene_lock)
    }
}

// -----------------------------------------------------------------------------
// v3.3 New Enums and Structs
// -----------------------------------------------------------------------------

/// Source of the node (local or federated)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeSource {
    Local,
    Federated {
        source_helix_id: String,
        source_generation: u64,
        verified_at: Option<DateTime<Utc>>,
    },
}

impl Default for NodeSource {
    fn default() -> Self {
        NodeSource::Local
    }
}

/// Record of contributor honor/attribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HonorStamp {
    pub contributor_name: String,
    pub contributor_type: ContributorType,
    pub l0_hash: Option<String>,
    pub evidence_type: EvidenceType,
    pub citation: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub original_cid: String,
    pub signature: String,
}

/// Type of contributor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributorType {
    Human,
    Helix,
    Unknown,
}

/// Type of evidence for the node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    HumanCitation,
    EmpiricalDiscovery,
    FederatedSync,
    Inherited,
}

/// Level of system impasse (deadlock/failure state)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ImpasseLevel {
    None,
    LocalDominantFailed,
    SharedTreeNoMatch,
    SpiralExhausted,
    RecessiveNoBreakthrough,
}

impl Default for ImpasseLevel {
    fn default() -> Self {
        ImpasseLevel::None
    }
}

/// Suggested action for query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSuggestion {
    pub action_type: String,
    pub parameters: Value,
    pub reason: String,
}