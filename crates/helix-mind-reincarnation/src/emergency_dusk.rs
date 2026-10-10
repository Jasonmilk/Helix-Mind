//! Emergency dusk — lightweight end-of-life protocol (§6.9 of the whitepaper).
//!
//! Triggered when external annihilation events (token exhaustion, VPS expiry,
//! disk full) are detected. Uses deterministic fallback when resources are
//! critically low — no LLM calls, pure algorithmic compression.

use helix_mind_core::config::LifecycleConfig;
use helix_mind_storage::StorageEngine;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Execute the emergency dusk protocol.
///
/// Returns the new generation number.
pub async fn execute(
    config: LifecycleConfig,
    storage: Arc<StorageEngine>,
    available_memory_mb: u64,
    available_tokens: u64,
) -> Result<u64, helix_mind_core::error::MindError> {
    // Assess resource state
    let resource_critical = available_memory_mb <= config.emergency_dusk_min_memory_mb
        || available_tokens <= config.emergency_dusk_min_tokens;

    if resource_critical {
        // Pure algorithm fallback — no LLM calls
        info!("Emergency dusk: resource critical, using deterministic fallback");
        execute_deterministic_fallback(&storage).await?;
    } else {
        // Standard compression — light LLM semantic summarization
        info!("Emergency dusk: resources sufficient, using standard compression");
        execute_standard_compression(&storage).await?;
    }

    // Write audit log
    let audit = helix_mind_core::graph::AuditEntry::new(
        helix_mind_core::graph::AuditEventType::EmergencyDuskTriggered,
        "emergency_dusk",
        &format!(
            "Emergency dusk executed (critical={}, mem={}MB, tokens={})",
            resource_critical, available_memory_mb, available_tokens
        ),
    );
    storage.write_audit(&audit).await?;

    // Generate inheritance crystal
    let crystal_hash = if config.inheritance_crystal {
        let inheritance =
            super::inheritance::Inheritance::new(config.clone(), storage.clone());
        Some(inheritance.create_crystal().await?)
    } else {
        None
    };

    // 真实世代：从 storage 的 L3 节点最大 generation 获取（P6-3 修复，不再硬编码 1）。
    let current_gen = storage
        .get_nodes_by_type(helix_mind_core::graph::NodeType::L3)
        .await?
        .iter()
        .map(|n| n.generation)
        .max()
        .unwrap_or(1);
    let new_generation = current_gen + 1;
    if let Some(hash) = &crystal_hash {
        storage
            .record_inheritance_crystal_hash(new_generation, hash)
            .await?;
    }

    Ok(new_generation)
}

/// Deterministic fallback: no LLM calls, pure algorithmic compression (§6.9.1).
async fn execute_deterministic_fallback(
    storage: &Arc<StorageEngine>,
) -> Result<(), helix_mind_core::error::MindError> {
    // 1. Topological sort extraction — get high-heat L2 nodes
    let l2_nodes = storage
        .get_nodes_by_type(helix_mind_core::graph::NodeType::L2)
        .await?;
    let hot_nodes: Vec<_> = l2_nodes
        .iter()
        .filter(|n| n.heat > 0.3)
        .collect();

    // 2. Static keyword extraction using TF-IDF (pure algorithm)
    let mut tf: HashMap<String, u64> = HashMap::new();
    for node in &hot_nodes {
        let text = node_content_to_string(&node.content);
        for word in text.split_whitespace() {
            let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if cleaned.len() > 2 {
                *tf.entry(cleaned).or_insert(0) += 1;
            }
        }
    }

    // Top-10 keywords
    let mut keywords: Vec<(String, u64)> = tf.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1));
    let top_keywords: Vec<String> = keywords
        .iter()
        .take(10)
        .map(|(k, _)| k.clone())
        .collect();

    // 3. Binary archive — serialize and compress
    let archive_data = serde_json::json!({
        "compression_method": "deterministic_fallback",
        "node_count": hot_nodes.len(),
        "top_keywords": top_keywords,
        "node_ids": hot_nodes.iter().map(|n| n.id.to_string()).collect::<Vec<_>>(),
    });

    let json = serde_json::to_vec(&archive_data).map_err(|e| {
        helix_mind_core::error::MindError::Storage(format!("JSON encode error: {}", e))
    })?;
    let compressed = zstd::encode_all(json.as_slice(), 19).map_err(|e| {
        helix_mind_core::error::MindError::Storage(format!("Zstd compress error: {}", e))
    })?;

    // 4. Write to inheritance crystal location
    let hash = helix_mind_core::sha256_digest(&compressed);
    let filename = format!("./inheritance_crystal_emergency_{}.zst", hash);
    tokio::fs::write(&filename, &compressed).await.map_err(|e| {
        helix_mind_core::error::MindError::Storage(format!("Cannot write emergency crystal: {}", e))
    })?;

    info!(
        "Deterministic fallback complete: {} nodes archived with {} keywords (hash: {})",
        hot_nodes.len(),
        top_keywords.len(),
        hash,
    );
    Ok(())
}

/// Standard compression with light LLM semantic summarization.
async fn execute_standard_compression(
    storage: &Arc<StorageEngine>,
) -> Result<(), helix_mind_core::error::MindError> {
    // Use the normal inheritance crystal creation
    let inheritance = super::inheritance::Inheritance::new(
        helix_mind_core::config::LifecycleConfig::default(),
        storage.clone(),
    );
    let hash = inheritance.create_crystal().await?;
    info!("Standard compression complete: crystal hash={}", hash);
    Ok(())
}

/// Extract plain text from node content for keyword extraction.
fn node_content_to_string(content: &helix_mind_core::graph::NodeContent) -> String {
    match content {
        helix_mind_core::graph::NodeContent::Text(t) => t.clone(),
        helix_mind_core::graph::NodeContent::Structured(map) => {
            map.values()
            .map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(" ")
        }
        _ => String::new(),
    }
}