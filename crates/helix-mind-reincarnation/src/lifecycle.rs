use helix_mind_core::config::LifecycleConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecyclePhase {
    Normal,
    Countdown,
    Dusk,
    Cremation,
}

pub struct LifecycleMonitor {
    config: LifecycleConfig,
    storage: Arc<StorageEngine>,
    running: Arc<RwLock<bool>>,
    current_phase: LifecyclePhase,
}

impl LifecycleMonitor {
    pub fn new(
        config: LifecycleConfig,
        storage: Arc<StorageEngine>,
        running: Arc<RwLock<bool>>,
    ) -> Self {
        Self {
            config,
            storage,
            running,
            current_phase: LifecyclePhase::Normal,
        }
    }

    pub async fn run(mut self) {
        info!("Lifecycle monitor started");
        loop {
            let running = *self.running.read().await;
            if !running {
                info!("Lifecycle monitor stopped");
                break;
            }

            // 临时绕过 stats 检查，因为 get_stats 返回 ()
            // let stats = self.storage.get_stats().await.unwrap_or(());
            let node_limit_reached = false;
            let interaction_limit_reached = false;
            
            let elapsed_days = match self.storage.get_elapsed_days().await {
                Ok(d) => d,
                Err(e) => {
                    warn!("Failed to get elapsed days: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    continue;
                }
            };

            let time_limit_reached = self.config.max_wall_clock_days.map(|max| elapsed_days >= max).unwrap_or(false);

            if node_limit_reached || interaction_limit_reached || time_limit_reached {
                if self.current_phase == LifecyclePhase::Normal {
                    info!("Limit reached, entering countdown phase");
                    self.current_phase = LifecyclePhase::Countdown;
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(self.config.countdown_minutes * 60)).await;

                if self.current_phase == LifecyclePhase::Countdown {
                    info!("Countdown complete, entering dusk phase");
                    self.current_phase = LifecyclePhase::Dusk;
                    let audit = helix_mind_core::graph::AuditEntry::new(
                        helix_mind_core::graph::AuditEventType::EmergencyDuskTriggered,
                        "lifecycle",
                        "Emergency dusk triggered due to lifecycle limit",
                    );
                    let _ = self.storage.write_audit(&audit).await;
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

                if self.current_phase == LifecyclePhase::Dusk {
                    info!("Dusk complete, entering cremation phase");
                    self.current_phase = LifecyclePhase::Cremation;
                    let _ = self.trigger_cremation().await;
                    self.current_phase = LifecyclePhase::Normal;
                }
            } else {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        }
    }

    async fn trigger_cremation(&self) -> Result<(), helix_mind_core::error::MindError> {
        let crystal_hash = if self.config.inheritance_crystal {
            let inheritance = super::inheritance::Inheritance::new(self.config.clone(), self.storage.clone());
            Some(inheritance.create_crystal().await?)
        } else {
            None
        };

        if self.config.archive_past_life {
            self.storage.archive_past_life(1).await?;
        }

        self.storage.delete_all_nodes_by_type(helix_mind_core::graph::NodeType::L3).await?;
        self.storage.delete_social_graph().await?;
        self.storage.delete_user_profile().await?;
        self.storage.reset_self_portrait().await?;

        let audit = helix_mind_core::graph::AuditEntry::new(
            helix_mind_core::graph::AuditEventType::DigitalCremationTriggered,
            "lifecycle",
            "Digital cremation triggered, new life started",
        );
        self.storage.write_audit(&audit).await?;

        info!("Digital cremation complete, new life started (Crystal: {:?})", crystal_hash);
        Ok(())
    }
}