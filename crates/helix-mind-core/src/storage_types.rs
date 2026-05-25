//! Storage-layer types shared across crates.
//!
//! These types are defined in `core` so that `storage`, `retrieval`, and
//! `metabolism` crates can use them without circular dependencies.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graph::NodeType;

/// A minimal stub left in the active index when an L3 node is moved to
/// deep cold storage. Prevents foreign-key crashes (dangling references)
/// while keeping the archived memory accessible via explicit thaw.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepColdStub {
    pub node_id: Uuid,
    /// Always "deep_cold".
    pub status: String,
    /// Filesystem path to the compressed archive.
    pub compressed_location: String,
    /// Size of the compressed archive in bytes.
    pub compressed_size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
    pub original_type: NodeType,
}
