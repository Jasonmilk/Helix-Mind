// Core module declarations
pub mod config;
pub mod error;
pub mod graph;
pub mod tracing;
pub mod audit;

// === v3.3 New Modules (Ordered per architect guidance)
pub mod envelope;
pub mod router;
pub mod tasks;
pub mod persona;
pub mod lifecycle;
pub mod storage_types;
pub mod symbolic;

// Re-export core types for external crates
pub use config::Config;
pub use error::MindError;
pub use graph::*;

// Re-export key types for convenience (per architect guidance)
pub use envelope::{IntentEnvelope, IntentResponse};
pub use router::{IntentHandler, IntentRouter};
pub use tasks::{
    AgendaCreator, AgendaNode, NoteNode, ReminderNode, TaskContextSnapshot,
    TaskNode, TaskStatus, TaskType,
};
pub use persona::{
    ContactTraitNode, EntityType, NodeLifecycle, SocialEdge, SocialNode,
    SocialRelationType, TraitType, UserTraitNode,
};
pub use lifecycle::{
    DagStats, EpochCrystal, EpochStatus, EpochTimespan, FederatedContribution,
    GuardianTransfer, LifeRecord, LifeSpan, TokenUsage,
};
pub use storage_types::DeepColdStub;

// Shared SHA256 utility function
use sha2::{Sha256, Digest};

/// Compute SHA256 digest for input data
pub fn sha256_digest(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}