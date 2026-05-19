pub mod lifecycle;
pub mod inheritance;

use helix_mind_core::config::LifecycleConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReincarnationEngine {
    config: LifecycleConfig,
    storage: Arc<StorageEngine>,
    running: Arc<RwLock<bool>>,
}

impl ReincarnationEngine {
    pub fn new(config: LifecycleConfig, storage: Arc<StorageEngine>) -> Self {
        Self {
            config,
            storage,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start lifecycle monitor
    pub async fn start(&self) -> Result<(), helix_mind_core::error::MindError> {
        if !self.config.enabled {
            return Ok(());
        }

        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        let monitor = self::lifecycle::LifecycleMonitor::new(
            self.config.clone(),
            self.storage.clone(),
            self.running.clone(),
        );
        tokio::spawn(monitor.run());

        Ok(())
    }

    /// Stop lifecycle monitor
    pub async fn stop(&self) -> Result<(), helix_mind_core::error::MindError> {
        let mut running = self.running.write().await;
        *running = false;
        Ok(())
    }

    /// Trigger manual reincarnation
    pub async fn trigger_reincarnation(&self, confirm_token: &str) -> Result<u64, helix_mind_core::error::MindError> {
        // Verify confirm token
        if confirm_token != "I understand this will reset my memory" {
            return Err(helix_mind_core::error::MindError::Validation("Invalid confirmation token".into()));
        }

        // 1. Create inheritance crystal
        let crystal_hash = if self.config.inheritance_crystal {
            let inheritance = self::inheritance::Inheritance::new(self.config.clone(), self.storage.clone());
            Some(inheritance.create_crystal().await?)
        } else {
            None
        };

        // 2. Archive past life
        if self.config.archive_past_life {
            self.storage.archive_past_life(1).await?;
        }

        // 3. Reset storage
        self.storage.delete_all_nodes_by_type(helix_mind_core::graph::NodeType::L3).await?;
        self.storage.delete_social_graph().await?;
        self.storage.delete_user_profile().await?;
        self.storage.reset_self_portrait().await?;

        // 4. Increment generation
        let new_generation = 2; // TODO: Get current generation from storage
        self.storage.record_inheritance_crystal_hash(new_generation, &crystal_hash.unwrap_or_default()).await?;

        // 5. Write audit log
        let audit = helix_mind_core::graph::AuditEntry::new(
            helix_mind_core::graph::AuditEventType::ReincarnationTriggered,
            "user",
            &format!("Reincarnation triggered, new generation: {}", new_generation),
        );
        self.storage.write_audit(&audit).await?;

        Ok(new_generation)
    }

    /// Get current lifecycle phase
    pub async fn get_phase(&self) -> Result<String, helix_mind_core::error::MindError> {
        // TODO: Get from storage
        Ok("Normal".into())
    }

    /// Get remaining countdown
    pub async fn get_countdown_remaining(&self) -> Result<Option<u64>, helix_mind_core::error::MindError> {
        // TODO: Get from storage
        Ok(None)
    }
}
