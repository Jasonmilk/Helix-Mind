use helix_mind_core::config::MetabolismConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

// 显式导入子模块结构体，避免路径错误
use crate::digest::Digest;
use crate::crystallize::Crystallize;
use crate::hibernate::Hibernate;

pub struct Scheduler {
    config: MetabolismConfig,
    storage: Arc<StorageEngine>,
    running: Arc<RwLock<bool>>,
}

impl Scheduler {
    pub fn new(
        config: MetabolismConfig,
        storage: Arc<StorageEngine>,
        running: Arc<RwLock<bool>>,
    ) -> Self {
        Self { config, storage, running }
    }

    pub async fn run(self) {
        info!("Metabolism scheduler started");
        let mut last_digest = chrono::Utc::now();
        let mut last_crystallize = chrono::Utc::now();
        let mut last_hibernate = chrono::Utc::now();

        loop {
            // Check if running
            let running = *self.running.read().await;
            if !running {
                info!("Metabolism scheduler stopped");
                break;
            }

            let now = chrono::Utc::now();

            // Check digest interval
            if now - last_digest > chrono::Duration::seconds(self.config.digest_interval_sec as i64) {
                info!("Running digest...");
                // 修复：直接使用导入的结构体
                if let Err(e) = Digest::new(self.config.clone(), self.storage.clone()).run().await {
                    warn!("Digest failed: {}", e);
                }
                last_digest = now;
            }

            // Check crystallize interval
            if now - last_crystallize > chrono::Duration::seconds(self.config.crystallize_idle_timeout_sec as i64) {
                info!("Running crystallization...");
                // 修复：直接使用导入的结构体
                if let Err(e) = Crystallize::new(self.config.clone(), self.storage.clone()).run().await {
                    warn!("Crystallization failed: {}", e);
                }
                last_crystallize = now;
            }

            // Check hibernate interval (daily)
            if now - last_hibernate > chrono::Duration::days(1) {
                info!("Running hibernate...");
                // 修复：直接使用导入的结构体
                if let Err(e) = Hibernate::new(self.config.clone(), self.storage.clone()).run().await {
                    warn!("Hibernate failed: {}", e);
                }
                last_hibernate = now;
            }

            // Sleep
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}