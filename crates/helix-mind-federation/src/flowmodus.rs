use helix_mind_core::config::FederationConfig;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::UnixStream;
use tracing::{info, warn};

pub struct FlowmodusScanner {
    config: FederationConfig,
    storage: Arc<StorageEngine>,
    running: Arc<RwLock<bool>>,
}

impl FlowmodusScanner {
    pub fn new(
        config: FederationConfig,
        storage: Arc<StorageEngine>,
        running: Arc<RwLock<bool>>,
    ) -> Self {
        Self { config, storage, running }
    }

    pub async fn run(self) {
        info!("Flowmodus scanner started");
        loop {
            // Check if running
            let running = *self.running.read().await;
            if !running {
                info!("Flowmodus scanner stopped");
                break;
            }

            // Scan sandbox directory
            if let Err(e) = self.scan_sandbox().await {
                warn!("Sandbox scan failed: {}", e);
            }

            // Scan outgoing directory
            if let Err(e) = self.scan_outgoing().await {
                warn!("Outgoing scan failed: {}", e);
            }

            // Sleep
            tokio::time::sleep(tokio::time::Duration::from_secs(self.config.scan_interval_sec)).await;
        }
    }

    async fn scan_sandbox(&self) -> Result<(), helix_mind_core::error::MindError> {
        let dir = tokio::fs::read_dir(&self.config.sandbox_dir).await?;
        let mut processed = 0;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("zst") {
                continue;
            }
            let filename = entry.file_name();
            let cid = filename.to_str().and_then(|s| s.strip_suffix(".dag.zst"));
            if let Some(cid) = cid {
                // Process incoming
                self.process_incoming(cid).await?;
                // Delete file
                tokio::fs::remove_file(path).await?;
                processed += 1;
            }
        }
        if processed > 0 {
            info!("Processed {} incoming DAGs from sandbox", processed);
        }
        Ok(())
    }

    async fn scan_outgoing(&self) -> Result<(), helix_mind_core::error::MindError> {
        // Try to connect to Flowmodus IPC socket
        match UnixStream::connect(&self.config.flowmodus_ipc_socket).await {
            Ok(_stream) => {
                // TODO: Send outgoing DAGs to Flowmodus
                // For now, just clean up old files
                let dir = tokio::fs::read_dir(&self.config.outgoing_dir).await?;
                let mut deleted = 0;
                for entry in dir {
                    let entry = entry?;
                    let metadata = entry.metadata().await?;
                    let modified = chrono::DateTime::from(metadata.modified()?);
                    if chrono::Utc::now() - modified > chrono::Duration::days(self.config.cremation_years as i64) {
                        tokio::fs::remove_file(entry.path()).await?;
                        deleted += 1;
                    }
                }
                if deleted > 0 {
                    info!("Cleaned up {} old outgoing DAGs", deleted);
                }
            }
            Err(_) => {
                // Flowmodus not running, skip
            }
        }
        Ok(())
    }

    async fn process_incoming(&self, cid: &str) -> Result<(), helix_mind_core::error::MindError> {
        let sandbox = super::sandbox::Sandbox::new(self.config.clone(), self.storage.clone());
        sandbox.process_incoming(cid).await?;
        Ok(())
    }
}
