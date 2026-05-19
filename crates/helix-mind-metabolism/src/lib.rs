pub mod scheduler;
pub mod digest;
pub mod crystallize;
pub mod hibernate;
pub mod ner;

use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MetabolismEngine {
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
    running: Arc<RwLock<bool>>,
}

impl MetabolismEngine {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        Self {
            config,
            storage,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start background scheduler
    pub async fn start(&self) -> Result<(), helix_mind_core::error::MindError> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        let scheduler = scheduler::Scheduler::new(
            self.config.clone(),
            self.storage.clone(),
            self.running.clone(),
        );
        tokio::spawn(scheduler.run());

        Ok(())
    }

    /// Stop background scheduler
    pub async fn stop(&self) -> Result<(), helix_mind_core::error::MindError> {
        let mut running = self.running.write().await;
        *running = false;
        Ok(())
    }

    /// Trigger manual digest
    pub async fn trigger_digest(&self) -> Result<(), helix_mind_core::error::MindError> {
        let digest = digest::Digest::new(self.config.clone(), self.storage.clone());
        digest.run().await?;
        Ok(())
    }

    /// Trigger manual crystallization
    pub async fn trigger_crystallize(&self) -> Result<(), helix_mind_core::error::MindError> {
        let crystallize = crystallize::Crystallize::new(self.config.clone(), self.storage.clone());
        crystallize.run().await?;
        Ok(())
    }

    /// Trigger manual hibernate
    pub async fn trigger_hibernate(&self) -> Result<(), helix_mind_core::error::MindError> {
        let hibernate = hibernate::Hibernate::new(self.config.clone(), self.storage.clone());
        hibernate.run().await?;
        Ok(())
    }
}
