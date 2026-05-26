//! Rebirth — the dawn of a new generation (§6.3 step 14 of the whitepaper).
//!
//! After the sunset protocol completes and the user confirms reincarnation,
//! a new generation is born. L3 is archived (deep cold storage), L1 is reset,
//! and the user portrait (CREATOR_IMPRINT) is loaded read-only.

use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

/// Execute the rebirth sequence for a new generation.
///
/// This is called after the sunset protocol completes and the user
/// has confirmed they want to proceed with reincarnation.
pub async fn execute_rebirth(
    storage: &Arc<StorageEngine>,
    new_generation: u64,
    _note_from_previous: &str,
) -> Result<(), helix_mind_core::error::MindError> {
    // 1. Reset self-portrait (L1) — new generation, new self
    storage.reset_self_portrait().await?;

    // 2. User portrait (CREATOR_IMPRINT) is preserved — do NOT delete
    // It is already in the DAG as L2 nodes with lifecycle=CreatorImprint
    // and will be loaded read-only by the new generation.

    // 3. Archive L3 memories (move to deep cold, not delete)
    // Already done in sunset protocol — archive_past_life

    // 4. Load note from previous generation
    // Stored in the life record audit log

    // 5. Write audit log
    let audit = helix_mind_core::graph::AuditEntry::new(
        helix_mind_core::graph::AuditEventType::ReincarnationTriggered,
        "rebirth",
        &format!("New generation {} born. User portrait preserved. Self-portrait reset.", new_generation),
    );
    storage.write_audit(&audit).await?;

    info!("Generation {} born — welcome to a new life", new_generation);
    Ok(())
}