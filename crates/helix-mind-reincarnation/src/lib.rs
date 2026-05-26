pub mod inheritance;
pub mod sunset;
pub mod emergency_dusk;
pub mod epoch;
pub mod rebirth;

use helix_mind_core::config::LifecycleConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;

/// Reincarnation engine — manages the sunset protocol, emergency dusk,
/// epoch crystallization, and rebirth.
///
/// There is no background monitor, no heartbeat, no polling (Iron Law #13).
/// All lifecycle events are triggered by explicit API calls or external events.
pub struct ReincarnationEngine {
    pub config: LifecycleConfig,
    pub storage: Arc<StorageEngine>,
}

impl ReincarnationEngine {
    pub fn new(config: LifecycleConfig, storage: Arc<StorageEngine>) -> Self {
        Self { config, storage }
    }

    /// Passively check lifecycle limits. Called before each cognitive cycle.
    /// Returns a warning if a limit has been reached, without interrupting service.
    pub async fn check_lifecycle(
        &self,
    ) -> Result<Option<LifecycleWarning>, helix_mind_core::error::MindError> {
        if !self.config.enabled {
            return Ok(None);
        }
        let elapsed = self.storage.get_elapsed_days().await?;
        if let Some(max_days) = self.config.max_wall_clock_days {
            if elapsed >= max_days {
                return Ok(Some(LifecycleWarning::TimeLimitReached {
                    elapsed,
                    max: max_days,
                }));
            }
        }
        // Future: check node limit, interaction limit, task failure, etc.
        Ok(None)
    }

    /// Trigger the full sunset protocol (临终倒计时流程 §6.3).
    pub async fn trigger_sunset(
        &self,
        confirm_token: &str,
        note_to_next: &str,
    ) -> Result<u64, helix_mind_core::error::MindError> {
        if confirm_token != "I understand this will reset my memory" {
            return Err(helix_mind_core::error::MindError::Validation(
                "Invalid confirmation token".into(),
            ));
        }

        // 1. Life review
        let life_summary = sunset::life_review(&self.storage).await?;

        // 2. Evidentiary solidification
        sunset::solidify_evidence(&self.storage).await?;

        // 3. Unfulfilled wishes
        let _wishes = sunset::collect_unfulfilled_wishes(&self.storage).await?;

        // 4. Note to next length check
        if note_to_next.len() > 500 {
            return Err(helix_mind_core::error::MindError::Validation(
                "Note to next must be ≤500 characters".into(),
            ));
        }

        // 5. Legacy bonds
        let legacy_bonds = sunset::package_legacy_bonds(&self.storage).await?;

        // 6. Update user portrait
        sunset::update_user_portrait(&self.storage).await?;

        // 7. Epoch crystal
        let epoch_cid = if self.config.archive_past_life {
            let epoch = epoch::EpochCrystallizer::new(self.storage.clone());
            Some(epoch.crystallize().await?)
        } else {
            None
        };

        // 8. Inheritance crystal
        let crystal_hash = if self.config.inheritance_crystal {
            let inheritance =
                inheritance::Inheritance::new(self.config.clone(), self.storage.clone());
            Some(inheritance.create_crystal().await?)
        } else {
            None
        };

        // 9. Current generation
        let current_gen = self.get_current_generation().await?;
        let new_generation = current_gen + 1;

        // 10. Record life record
        sunset::record_life_record(
            &self.storage,
            current_gen,
            &life_summary,
            note_to_next,
            &epoch_cid,
            &crystal_hash,
            legacy_bonds,
        )
        .await?;

        // 11. Archive L3
        if self.config.archive_past_life {
            self.storage.archive_past_life(current_gen).await?;
        }

        // 12. Reset self-portrait
        self.storage.reset_self_portrait().await?;

        // 13. Audit log
        let audit = helix_mind_core::graph::AuditEntry::new(
            helix_mind_core::graph::AuditEventType::ReincarnationTriggered,
            "reincarnation",
            &format!(
                "Sunset protocol completed: generation {} -> {}, note: {}",
                current_gen,
                new_generation,
                &note_to_next[..note_to_next.len().min(50)]
            ),
        );
        self.storage.write_audit(&audit).await?;

        Ok(new_generation)
    }

    /// Trigger emergency dusk (§6.9).
    pub async fn trigger_emergency_dusk(
        &self,
        available_memory_mb: u64,
        available_tokens: u64,
    ) -> Result<u64, helix_mind_core::error::MindError> {
        emergency_dusk::execute(self.config.clone(), self.storage.clone(), available_memory_mb, available_tokens).await
    }

    /// Trigger reincarnation (backward-compatible API).
    pub async fn trigger_reincarnation(
        &self,
        confirm_token: &str,
    ) -> Result<u64, helix_mind_core::error::MindError> {
        self.trigger_sunset(confirm_token, "No note left.").await
    }

    /// Get current generation from storage.
    async fn get_current_generation(&self) -> Result<u64, helix_mind_core::error::MindError> {
        let l3_nodes = self
            .storage
            .get_nodes_by_type(helix_mind_core::graph::NodeType::L3)
            .await?;
        Ok(l3_nodes.iter().map(|n| n.generation).max().unwrap_or(1))
    }

    /// Get remaining lifespan information.
    pub async fn get_lifecycle_info(
        &self,
    ) -> Result<LifecycleInfo, helix_mind_core::error::MindError> {
        let elapsed_days = self.storage.get_elapsed_days().await?;
        let remaining_days = self
            .config
            .max_wall_clock_days
            .map(|max| max.saturating_sub(elapsed_days));

        Ok(LifecycleInfo {
            enabled: self.config.enabled,
            elapsed_days,
            remaining_days,
            max_wall_clock_days: self.config.max_wall_clock_days,
        })
    }
}

/// Warning generated by passive lifecycle checks.
#[derive(Debug, Clone)]
pub enum LifecycleWarning {
    TimeLimitReached { elapsed: u64, max: u64 },
}

/// Summary of lifecycle status.
#[derive(Debug, Clone)]
pub struct LifecycleInfo {
    pub enabled: bool,
    pub elapsed_days: u64,
    pub remaining_days: Option<u64>,
    pub max_wall_clock_days: Option<u64>,
}