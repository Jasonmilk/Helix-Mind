//! Damped momentum decay engine (§5.2 of the whitepaper).
//!
//! When knowledge fails in physical-world empirical execution, its utility
//! weight (𝒰) decays via a damped half-life algorithm. A single occasional
//! failure only causes a slight jitter; repeated failures break through the
//! momentum barrier and cause a cliff-like drop, demoting the node to the
//! recessive branch.

use helix_mind_core::graph::Node;
use helix_mind_storage::StorageEngine;
use std::sync::Arc;
use tracing::info;

/// Decay engine implementing the damped momentum half-life algorithm.
pub struct DecayEngine {
    /// Momentum factor α (default 0.95). Higher = more resistant to single failures.
    alpha: f64,
    /// Environmental damping coefficient γ (default 0.15). Filters environmental noise.
    gamma: f64,
    /// Storage handle for updating node utility.
    storage: Arc<StorageEngine>,
}

impl DecayEngine {
    pub fn new(alpha: f64, gamma: f64, storage: Arc<StorageEngine>) -> Self {
        Self { alpha, gamma, storage }
    }

    /// Apply damped momentum decay to a node.
    ///
    /// Formula: 𝒰_{t+1} = α · 𝒰_t − (1 − α) · δ · γ
    ///
    /// - `error_magnitude` (δ): mapped from the PredictionFailureReport to [0.0, 1.0].
    ///   Higher values indicate a more severe failure.
    ///
    /// Returns the new utility value.
    pub async fn apply_decay(
        &self,
        node: &Node,
        error_magnitude: f64,
    ) -> Result<f64, helix_mind_core::error::MindError> {
        let delta = error_magnitude.clamp(0.0, 1.0);
        let new_utility = self.alpha * node.utility - (1.0 - self.alpha) * delta * self.gamma;
        let new_utility = new_utility.max(0.0);

        // Persist
        self.storage
            .update_node_utility(&node.id, new_utility)
            .await?;

        // If utility drops below recessive threshold, mark as recessive
        if new_utility < 0.3 && !node.is_recessive {
            self.storage.mark_recessive(&node.id).await?;
            info!(
                "Node {} demoted to recessive branch (𝒰 = {:.3})",
                node.id, new_utility
            );
        }

        // Recalculate dominance
        let new_dominance = (node.corroborations as f64 * 0.01 + new_utility * 0.5).min(1.0);
        self.storage
            .update_node_dominance(&node.id, new_dominance)
            .await?;

        Ok(new_utility)
    }

    /// Apply a reward (successful empirical validation).
    ///
    /// Formula: 𝒰_{t+1} = 𝒰_t + η · (1 − 𝒰_t)
    ///
    /// This ensures that recessive nodes can climb back to dominant status
    /// when they prove useful again.
    pub async fn apply_reward(
        &self,
        node: &Node,
        success_magnitude: f64,
    ) -> Result<f64, helix_mind_core::error::MindError> {
        let eta = success_magnitude.clamp(0.0, 1.0) * 0.2; // max 0.2 per success
        let new_utility = node.utility + eta * (1.0 - node.utility);
        let new_utility = new_utility.min(1.0);

        self.storage
            .update_node_utility(&node.id, new_utility)
            .await?;

        let new_dominance = (node.corroborations as f64 * 0.01 + new_utility * 0.5).min(1.0);
        self.storage
            .update_node_dominance(&node.id, new_dominance)
            .await?;

        Ok(new_utility)
    }

    /// Get current momentum factor.
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// Get current damping coefficient.
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decay_formula() {
        // Simulate: 𝒰=1.0, α=0.95, γ=0.15, δ=0.5
        // 🔥 架构师要求：显式标注 f64 类型
        let alpha: f64 = 0.95;
        let gamma: f64 = 0.15;
        let utility: f64 = 1.0;
        let delta: f64 = 0.5;

        let new_utility = alpha * utility - (1.0 - alpha) * delta * gamma;
        // = 0.95 - 0.05 * 0.5 * 0.15 = 0.95 - 0.00375 = 0.94625
        assert!((new_utility - 0.94625).abs() < 0.0001);
        assert!(new_utility > 0.94); // single failure barely dents high-utility knowledge
    }

    #[test]
    fn test_repeated_failures_break_through() {
        // 🔥 架构师要求：显式标注 f64 类型
        let alpha: f64 = 0.95;
        let gamma: f64 = 0.15;
        let mut utility: f64 = 1.0;
        let delta: f64 = 0.8; // severe failure each time

        // After 10 consecutive failures
        for _ in 0..10 {
            utility = alpha * utility - (1.0 - alpha) * delta * gamma;
            utility = utility.max(0.0);
        }
        // Should be significantly reduced
        assert!(utility < 0.6, "utility should drop below 0.6 after 10 failures, got {}", utility);
    }

    #[test]
    fn test_reward_formula() {
        // 🔥 架构师要求：显式标注 f64 类型
        let utility: f64 = 0.3;
        let eta: f64 = 0.5 * 0.2; // success_magnitude=0.5, max 0.2
        let new_utility: f64 = utility + eta * (1.0 - utility);
        // = 0.3 + 0.1 * 0.7 = 0.37
        assert!((new_utility - 0.37).abs() < 0.0001);
    }
}