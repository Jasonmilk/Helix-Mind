pub mod digest;
pub mod crystallize;
pub mod hibernate;
pub mod ner;
pub mod decay;
pub mod symbolic;
pub mod cognitive;

pub use cognitive::{
    CognitiveService, build_cognitive_service, DeterministicAdapter, RemoteAdapter, FakeAdapter,
};

use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;

/// Metabolism engine — event-driven, no heartbeat, no polling.
///
/// All metabolism tasks are triggered by explicit API calls or by the
/// main loop's idle detection (`tokio::select!`). There is no background
/// scheduler thread (Iron Law #13: No Heartbeat, Event-Driven).
///
/// `cognitive` is the single LLM-access port (ADR-0017): every LLM-touching
/// capability resolves through it; `crystallize`/`ner` never call an LLM
/// directly.
pub struct MetabolismEngine {
    pub config: MetabolismConfig,
    pub storage: Arc<StorageEngine>,
    pub cognitive: Arc<dyn CognitiveService>,
}

impl MetabolismEngine {
    pub fn new(config: MetabolismConfig, storage: Arc<StorageEngine>) -> Self {
        // Fail-closed: adapter selection by llm_mode (Z2). A misconfiguration
        // never produces an accidental outbound path.
        let cognitive = build_cognitive_service(&config)
            .unwrap_or_else(|_| Arc::new(DeterministicAdapter::new(config.clone())));
        Self {
            config,
            storage,
            cognitive,
        }
    }

    /// Test seam: inject a Fake / Deterministic cognitive service explicitly.
    pub fn with_cognitive(
        config: MetabolismConfig,
        storage: Arc<StorageEngine>,
        cognitive: Arc<dyn CognitiveService>,
    ) -> Self {
        Self {
            config,
            storage,
            cognitive,
        }
    }

    /// Trigger metabolism based on an external event.
    ///
    /// Called by the main loop when:
    /// - L3 write count exceeds threshold → Micro-Sleep (digest)
    /// - System idle + pending backlog → Deep Dream (crystallize)
    /// - Continuous idle > 2 hours → Deep Hibernate
    pub async fn trigger_by_event(
        &self,
        event: MetabolismEvent,
    ) -> Result<MetabolismReport, helix_mind_core::error::MindError> {
        match event {
            MetabolismEvent::MicroSleep => {
                let digest = digest::Digest::new(self.config.clone(), self.storage.clone());
                let merged = digest.run().await?;
                Ok(MetabolismReport {
                    digest_merged: merged,
                    crystallized: 0,
                    hibernated: 0,
                })
            }
            MetabolismEvent::DeepDream => {
                let crystallize =
                    crystallize::Crystallize::new(self.storage.clone(), self.cognitive.clone());
                crystallize.run().await?;
                Ok(MetabolismReport {
                    digest_merged: 0,
                    crystallized: 0,
                    hibernated: 0,
                })
            }
            MetabolismEvent::DeepHibernate => {
                let hibernate =
                    hibernate::Hibernate::new(self.config.clone(), self.storage.clone());
                hibernate.run().await?;
                Ok(MetabolismReport {
                    digest_merged: 0,
                    crystallized: 0,
                    hibernated: 0,
                })
            }
        }
    }

    /// Trigger manual digest (called by API layer).
    pub async fn trigger_digest(&self) -> Result<(), helix_mind_core::error::MindError> {
        let digest = digest::Digest::new(self.config.clone(), self.storage.clone());
        let _merged = digest.run().await?;
        Ok(())
    }

    /// Trigger manual crystallization (called by API layer).
    pub async fn trigger_crystallize(&self) -> Result<(), helix_mind_core::error::MindError> {
        let crystallize =
            crystallize::Crystallize::new(self.storage.clone(), self.cognitive.clone());
        crystallize.run().await
    }

    /// Trigger manual hibernate (called by API layer).
    pub async fn trigger_hibernate(&self) -> Result<(), helix_mind_core::error::MindError> {
        let hibernate = hibernate::Hibernate::new(self.config.clone(), self.storage.clone());
        hibernate.run().await
    }
}

/// Event types that can trigger metabolism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetabolismEvent {
    /// Triggered every N L3 writes or on short idle.
    MicroSleep,
    /// Triggered on longer idle with pending crystallization backlog.
    DeepDream,
    /// Triggered after continuous idle > threshold (default 2 hours).
    DeepHibernate,
}

/// Summary of a metabolism run.
#[derive(Debug, Clone)]
pub struct MetabolismReport {
    pub digest_merged: u64,
    pub crystallized: u64,
    pub hibernated: u64,
}
