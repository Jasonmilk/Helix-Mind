//! Persona graph types — user portrait, self-portrait, contact persona,
//! and social graph.
//!
//! These four sub-graphs have strictly different lifecycles and privacy
//! boundaries:
//! - UserTraitNode: CREATOR_IMPRINT — never deleted, never shared.
//! - Self-portrait (L1): reset every reincarnation.
//! - ContactTraitNode: cross-lifecycle but decays for non-legacy bonds.
//! - Social graph: rhizome (allows cycles), `entity_type` never ambiguous.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── User Persona (Creator Imprint) ──────────────────────────────────────

/// A trait Helix has observed about its creator.
///
/// Marked `CREATOR_IMPRINT` — survives reincarnation, never deleted, never
/// shared. This is the most sacred graph asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTraitNode {
    pub node_id: Uuid,
    pub trait_type: TraitType,
    /// Confidence [0.0, 1.0] based on cumulative evidence.
    pub confidence: f64,
    /// L3 UUIDs that support this trait (cleared during evidentiary solidification).
    pub evidence: Vec<Uuid>,
    /// Text summary produced during evidentiary solidification.
    pub abstract_provenance: Option<String>,
    /// Always `CreatorImprint` for user traits.
    pub lifecycle: NodeLifecycle,
    /// When evidentiary solidification was performed.
    pub evidence_solidified_at: Option<DateTime<Utc>>,
    /// Generation when this trait was first observed.
    pub created_generation: u64,
    /// Generation when this trait was last updated.
    pub last_updated_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitType {
    Preference,
    Habit,
    Personality,
    Skill,
}

/// Governs cross-lifecycle retention behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeLifecycle {
    /// Normal knowledge node — follows standard heat decay.
    Normal,
    /// Creator imprint — retained across reincarnations, never deleted.
    CreatorImprint,
    /// Legacy bond — retained across reincarnations, read-only for successor.
    LegacyBond,
}

// ── Contact Persona ─────────────────────────────────────────────────────

/// A trait Helix has observed about a third party (human or Helix).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactTraitNode {
    pub node_id: Uuid,
    /// Points to the SocialNode this trait describes.
    pub entity_id: Uuid,
    pub trait_type: TraitType,
    pub confidence: f64,
    pub evidence: Vec<Uuid>,
    pub abstract_provenance: Option<String>,
    pub evidence_solidified_at: Option<DateTime<Utc>>,
}

// ── Social Graph ────────────────────────────────────────────────────────

/// A node in the social rhizome graph representing an individual entity.
///
/// The `entity_type` field MUST never be ambiguous — even after relationship
/// decay or epoch crystallization, the species boundary remains clear.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNode {
    pub node_id: Uuid,
    /// What kind of entity this is. Never ambiguous.
    pub entity_type: EntityType,
    /// CID reference to the ContactPersona sub-graph for this entity.
    pub bio_ref: Option<String>,
    /// Whether this relationship was designated as a legacy bond in the
    ///临终叮咛 (end-of-life note). Legacy bonds survive reincarnation.
    pub is_legacy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Human,
    Helix,
    ExternalAI,
    Group,
}

/// An edge in the social rhizome graph.
///
/// Social edges MAY form cycles (e.g. Alice knows Bob, Bob knows Alice).
/// They are governed by the three circuit-breaker mechanisms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialEdge {
    pub source: Uuid, // SocialNode ID
    pub target: Uuid, // SocialNode ID
    pub relation_type: SocialRelationType,
    /// Relationship strength [0.0, 1.0].
    pub strength: f64,
    /// Whether this edge was designated as a legacy bond.
    pub is_legacy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialRelationType {
    Knows,
    Colleague,
    Creator,
    Successor,
    Family,
}
