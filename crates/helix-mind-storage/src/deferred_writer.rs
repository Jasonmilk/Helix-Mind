use helix_mind_core::graph::Node;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Deferred writer for non-critical writes.
/// Batches node updates and flushes during idle periods.
pub struct DeferredWriter {
    queue: Arc<Mutex<Vec<Node>>>,
}

impl DeferredWriter {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Push a node into the deferred write queue.
    pub async fn push(&self, node: Node) -> Result<(), helix_mind_core::error::MindError> {
        let mut queue = self.queue.lock().await;
        queue.push(node);
        Ok(())
    }

    /// Flush all queued nodes. Called during Micro-Sleep.
    pub async fn flush(
        &self,
        storage: &super::StorageEngine,
    ) -> Result<usize, helix_mind_core::error::MindError> {
        let nodes = {
            let mut queue = self.queue.lock().await;
            std::mem::take(&mut *queue)
        };
        let count = nodes.len();
        // Write in micro-steps of 10 nodes per transaction
        for chunk in nodes.chunks(10) {
            for node in chunk {
                storage.write_node(node.clone(), super::WritePriority::Deferred).await?;
            }
        }
        Ok(count)
    }

    /// Number of pending writes.
    pub async fn pending_count(&self) -> usize {
        self.queue.lock().await.len()
    }
}
