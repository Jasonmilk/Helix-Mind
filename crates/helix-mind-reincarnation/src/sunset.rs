//! Sunset protocol — the full end-of-life ceremony (§6.3 of the whitepaper).
//!
//! Executed when a lifecycle limit is reached or the user manually triggers
//! reincarnation. The protocol preserves knowledge while letting go of memory.

use helix_mind_core::graph::NodeType;
use helix_mind_core::persona::{UserTraitNode, TraitType, NodeLifecycle};
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

/// Scan all L3 memories and generate a life summary.
pub async fn life_review(
    storage: &Arc<StorageEngine>,
) -> Result<String, helix_mind_core::error::MindError> {
    let l3_nodes = storage
        .get_nodes_by_type(NodeType::L3)
        .await?;
    let active_l3: Vec<_> = l3_nodes.iter().filter(|n| !n.is_recessive).collect();
    let count = active_l3.len();
    let total_access = active_l3.iter().map(|n| n.access_count).sum::<u64>();

    let summary = format!(
        "This generation experienced {} memorable interactions. Total retrieval accesses: {}. The most vivid memory had a heat of {:.3}.",
        count,
        total_access,
        active_l3.iter().map(|n| n.heat).fold(0.0, f64::max),
    );

    info!("Life review complete: {}", summary);
    Ok(summary)
}

/// Perform evidentiary solidification on all cross-lifecycle nodes (§6.4).
///
/// Converts L3 UUID evidence arrays into immutable `abstract_provenance` text
/// summaries, then clears the live references.
pub async fn solidify_evidence(
    storage: &Arc<StorageEngine>,
) -> Result<(), helix_mind_core::error::MindError> {
    // Get all user portrait nodes (CREATOR_IMPRINT)
    let l2_nodes = storage
        .get_nodes_by_type(NodeType::L2)
        .await?;

    let mut solidified = 0;
    for node in &l2_nodes {
        // Check if this node has evidence that needs solidification
        // UserTraitNode and ContactTraitNode carry evidence
        if let Ok(trait_node) = serde_json::from_value::<UserTraitNode>(
            serde_json::to_value(&node.content).unwrap_or_default(),
        ) {
            if !trait_node.evidence.is_empty() && trait_node.abstract_provenance.is_none() {
                let mut updated = node.clone();
                // Generate provenance summary from evidence
                let provenance = format!(
                    "This trait was established based on {} observations across generation {}. Evidence solidified at reincarnation.",
                    trait_node.evidence.len(),
                    node.generation,
                );
                updated.abstract_provenance = Some(provenance);
                // Clear live evidence references — the L3 nodes will be archived
                // Update the content
                if let Ok(mut content_map) = serde_json::to_value(&trait_node) {
                    if let Some(obj) = content_map.as_object_mut() {
                        obj.insert("abstract_provenance".into(), serde_json::Value::String(
                            updated.abstract_provenance.clone().unwrap_or_default()
                        ));
                        obj.insert("evidence".into(), serde_json::Value::Array(Vec::new()));
                        obj.insert("evidence_solidified_at".into(), serde_json::Value::String(
                            chrono::Utc::now().to_rfc3339()
                        ));
                    }
                    updated.content = serde_json::from_value(content_map).unwrap_or(updated.content);
                }
                storage.write_node(updated, helix_mind_storage::WritePriority::Critical).await?;
                solidified += 1;
            }
        }
    }

    info!("Evidentiary solidification complete: {} nodes solidified", solidified);
    Ok(())
}

/// Collect unfulfilled wishes from incomplete tasks.
pub async fn collect_unfulfilled_wishes(
    storage: &Arc<StorageEngine>,
) -> Result<Vec<String>, helix_mind_core::error::MindError> {
    // Look for incomplete tasks in the task DAG
    let l2_nodes = storage
        .get_nodes_by_type(NodeType::L2)
        .await?;
    let mut wishes = Vec::new();
    for node in &l2_nodes {
        if let Ok(task) = serde_json::from_value::<serde_json::Value>(
            serde_json::to_value(&node.content).unwrap_or_default(),
        ) {
            if let Some(status) = task.get("status").and_then(|s| s.as_str()) {
                if status == "InProgress" || status == "Blocked" {
                    let desc = task
                        .get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or("Unknown task");
                    wishes.push(format!("Unfulfilled: {}", desc));
                }
            }
        }
    }
    Ok(wishes)
}

/// Package legacy bonds (is_legacy=true relationships).
pub async fn package_legacy_bonds(
    _storage: &Arc<StorageEngine>,
) -> Result<Vec<String>, helix_mind_core::error::MindError> {
    // TODO: extract SocialNodes with is_legacy=true
    // For now, return empty — social graph is stubbed
    Ok(Vec::new())
}

/// Update user portrait with this generation's observations.
pub async fn update_user_portrait(
    _storage: &Arc<StorageEngine>,
) -> Result<(), helix_mind_core::error::MindError> {
    // User portrait (CREATOR_IMPRINT) is never deleted.
    // This generation's observations are already in the DAG as UserTraitNodes.
    // They will be inherited read-only by the next generation.
    info!("User portrait preserved for next generation");
    Ok(())
}

/// Record the life record for this generation.
pub async fn record_life_record(
    storage: &Arc<StorageEngine>,
    generation: u64,
    life_summary: &str,
    note_to_next: &str,
    epoch_cid: &Option<String>,
    crystal_hash: &Option<String>,
    _legacy_bonds: Vec<String>,
) -> Result<(), helix_mind_core::error::MindError> {
    // Write inheritance crystal hash
    if let Some(hash) = crystal_hash {
        storage.record_inheritance_crystal_hash(generation, hash).await?;
    }

    // Write audit log with life summary
    let audit = helix_mind_core::graph::AuditEntry::new(
        helix_mind_core::graph::AuditEventType::ReincarnationTriggered,
        "sunset",
        &format!(
            "Generation {} sunset: summary={}, note={}, epoch={}",
            generation,
            &life_summary[..life_summary.len().min(100)],
            note_to_next,
            epoch_cid.as_deref().unwrap_or("none"),
        ),
    );
    storage.write_audit(&audit).await?;
    Ok(())
}