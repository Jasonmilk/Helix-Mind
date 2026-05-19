use helix_mind_core::graph::{Node, DeepColdStub};
use chrono::{Utc, Duration};
use zstd::stream::*;

pub struct DeepColdStore {
    dir: String,
}

impl DeepColdStore {
    pub fn new(dir: &str) -> Self {
        std::fs::create_dir_all(dir).ok();
        Self { dir: dir.to_string() }
    }

    /// Archive node to deep cold storage
    pub async fn archive_node(&self, node: &Node, retention_days: i64) -> Result<DeepColdStub, helix_mind_core::error::MindError> {
        let node_id = node.id;
        let filename = format!("{}/{}.zst", self.dir, node_id);
        let content = serde_json::to_vec(node)?;
        // Compress with zstd
        let mut file = std::fs::File::create(&filename)?;
        let mut encoder = Encoder::new(&mut file, 19)?;
        std::io::copy(&mut content.as_slice(), &mut encoder)?;
        encoder.finish()?;

        let compressed_size = std::fs::metadata(&filename)?.len();
        let created_at = Utc::now();
        let expired_at = created_at + Duration::days(retention_days);

        let stub = DeepColdStub {
            node_id,
            status: "archived".into(),
            compressed_location: filename,
            compressed_size_bytes: compressed_size,
            created_at,
            expired_at,
            original_type: node.node_type.clone(),
        };

        Ok(stub)
    }

    /// Restore node from deep cold
    pub async fn restore_node(&self, stub: &DeepColdStub) -> Result<Node, helix_mind_core::error::MindError> {
        let file = std::fs::File::open(&stub.compressed_location)?;
        let mut decoder = Decoder::new(file)?;
        let mut content = Vec::new();
        std::io::copy(&mut decoder, &mut content)?;
        let node: Node = serde_json::from_slice(&content)?;
        Ok(node)
    }

    /// Delete expired stubs
    pub async fn cleanup_expired(&self) -> Result<usize, helix_mind_core::error::MindError> {
        let now = Utc::now();
        let mut deleted = 0;
        let dir = std::fs::read_dir(&self.dir)?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("zst") {
                continue;
            }
            // Check modified time
            let metadata = entry.metadata()?;
            let modified = chrono::DateTime::from(metadata.modified()?);
            if now - modified > Duration::days(365 * 100) { // 100 years
                std::fs::remove_file(path)?;
                deleted += 1;
            }
        }
        Ok(deleted)
    }
}
