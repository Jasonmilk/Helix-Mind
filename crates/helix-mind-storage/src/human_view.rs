use helix_mind_core::graph::Node;
use std::collections::HashSet;

pub struct HumanViewSync {
    dir: String,
    max_size_mb: u64,
}

impl HumanViewSync {
    pub fn new(dir: &str, max_size_mb: u64) -> Self {
        std::fs::create_dir_all(dir).ok();
        Self {
            dir: dir.to_string(),
            max_size_mb,
        }
    }

    /// Sync memory to human-readable view
    pub async fn sync(&self, nodes: &[Node]) -> Result<Vec<String>, helix_mind_core::error::MindError> {
        let mut conflicts = Vec::new();
        let mut seen_ids = HashSet::new();
        let mut md_content = String::new();
        md_content.push_str("# Helix-Mind Human View\n");
        md_content.push_str(&format!("Generated at: {}\n\n", chrono::Utc::now()));

        for node in nodes {
            if seen_ids.contains(&node.id) {
                conflicts.push(format!("Duplicate node: {}", node.id));
                continue;
            }
            seen_ids.insert(node.id);
            // Only include public nodes
            if node.sensitivity != Some(helix_mind_core::graph::Sensitivity::Public) {
                continue;
            }
            // Add to markdown
            let content = match &node.content {
                helix_mind_core::graph::NodeContent::Text(t) => t,
                _ => continue,
            };
            md_content.push_str(&format!("## {}\n", node.created_at.format("%Y-%m-%d %H:%M")));
            md_content.push_str(content);
            md_content.push_str("\n\n");
        }

        // Check size
        let size_bytes = md_content.len() as u64;
        if size_bytes > self.max_size_mb * 1024 * 1024 {
            return Err(helix_mind_core::error::MindError::Storage("Human view exceeds max size".into()));
        }

        // Write to file
        let filename = format!("{}/view_{}.md", self.dir, chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        tokio::fs::write(filename, md_content).await?;

        Ok(conflicts)
    }
}
