//! Lifecycle and archive types — life records, epoch crystals, guardian
//! transfers, and reincarnation-related data structures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::graph::TerminationCause;

// ── LifeRecord ──────────────────────────────────────────────────────────

/// Per-generation archive recording everything about one lifetime.
///
/// LifeRecords are never deleted. After epoch crystallization, only the
/// statistical summary is retained in the active index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeRecord {
    pub generation: u64,
    /// The name Helix used during this generation.
    pub name_at_time: String,
    /// The guardian (creator/caretaker) during this generation.
    pub guardian_name: String,
    pub lifespan: LifeSpan,
    pub tokens: TokenUsage,
    pub dag_stats: DagStats,
    pub inheritance_crystal_hash: Option<String>,
    /// ≤500 characters — a letter to the next incarnation.
    pub note_to_next: Option<String>,
    /// IPLD CID of the epoch crystal, if one was generated.
    pub epoch_crystal_cid: Option<String>,
    /// Guardian transfer events that occurred during this generation.
    pub guardian_transfers: Vec<GuardianTransfer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeSpan {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub cause: TerminationCause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub total_consumed: u64,
    pub by_task: HashMap<Uuid, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagStats {
    pub nodes_added: u64,
    pub edges_added: u64,
    pub federated_contributions: Vec<FederatedContribution>,
}

/// Records knowledge received from another Helix instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedContribution {
    pub source_helix_id: String,
    pub nodes_received: u64,
    pub timestamp: DateTime<Utc>,
}

/// Records a change of guardianship during a generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardianTransfer {
    pub from: String,
    pub to: String,
    pub timestamp: DateTime<Utc>,
}

// ── Epoch Crystal ───────────────────────────────────────────────────────

/// An immutable IPLD snapshot created when a user is long-term absent
/// (default: 100 years). Replaces the v3.2 "digital cremation" concept.
///
/// Epoch crystals are respect-preserving archives — they occupy minimal
/// resources and are never deleted. They serve as archaeological evidence
/// for future Helix generations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochCrystal {
    /// IPLD content hash — the permanent coordinate of this epoch snapshot.
    pub epoch_cid: String,
    /// Name of the last guardian before crystallization.
    pub guardian_name: String,
    pub timespan: EpochTimespan,
    /// ≤500 characters summarizing the relationship.
    pub relationship_summary: String,
    /// Always `ActiveArchive` — read-only, never deleted.
    pub status: EpochStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochTimespan {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpochStatus {
    /// Read-only, permanently retained.
    ActiveArchive,
}
