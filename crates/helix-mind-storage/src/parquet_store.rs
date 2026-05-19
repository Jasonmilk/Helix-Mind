use std::path::Path;
use helix_mind_core::graph::Node;
use helix_mind_core::error::MindError;
use uuid::Uuid;

pub struct ParquetStore;

impl ParquetStore {
    pub fn save_nodes<P: AsRef<Path>>(path: P, nodes: &[Node]) -> Result<(), MindError> {
        let mut ids = Vec::new();
        let mut node_types = Vec::new();
        let mut contents = Vec::new();
        let mut heats = Vec::new();
        let mut created_ats = Vec::new();

        for node in nodes {
            ids.push(node.id.to_string());
            node_types.push(format!("{:?}", node.node_type));
            contents.push(serde_json::to_string(&node.content)
                .map_err(|e| MindError::Storage(e.to_string()))?);
            heats.push(node.heat);
            created_ats.push(node.created_at.timestamp());
        }

        // Simplified: just serialize to JSON for now to avoid polars compatibility issues
        let json = serde_json::json!({
            "ids": ids,
            "node_types": node_types,
            "contents": contents,
            "heats": heats,
            "created_ats": created_ats,
        });

        let content = serde_json::to_string_pretty(&json)
            .map_err(|e| MindError::Storage(e.to_string()))?;
        
        std::fs::write(path.as_ref(), content)
            .map_err(|e| MindError::Io(e))?;

        Ok(())
    }

    pub fn load_nodes<P: AsRef<Path>>(path: P) -> Result<Vec<Node>, MindError> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| MindError::Io(e))?;
        
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| MindError::Storage(e.to_string()))?;

        let mut nodes = Vec::new();
        
        if let Some(arr) = json.get("ids").and_then(|v| v.as_array()) {
            for (i, id_val) in arr.iter().enumerate() {
                let id_str = id_val.as_str().ok_or_else(|| {
                    MindError::Storage("Invalid id format".into())
                })?;
                
                let id = Uuid::parse_str(id_str)
                    .map_err(|e| MindError::Uuid(e))?;

                let heat = json.get("heats")
                    .and_then(|v| v.as_array())
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);

                let content_str = json.get("contents")
                    .and_then(|v| v.as_array())
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| MindError::Storage("Missing content".into()))?;

                let content = serde_json::from_str(content_str)
                    .map_err(|e| MindError::Storage(e.to_string()))?;

                let node = Node {
                    id,
                    heat,
                    content,
                    ..Default::default()
                };
                nodes.push(node);
            }
        }

        Ok(nodes)
    }
}
