//! 睡眠复盘（ADR-0021 Phase 3 抗僵化）。
//!
//! 在 Deep Dream 代谢窗口内执行"非熟练模式检视"：用锚定/想象力模式检视高权重旧路径
//! （熟练模式策略），验证旧路径是否仍有效。**复盘强制使用非熟练模式，防止"假复盘"**（R4）。
//!
//! 确定性判定（0 Token，禁用 LLM 自我评分）：以"检视输出的实体覆盖度"相对"旧路径实体覆盖度"
//! 的增量作为僵化信号——检视发现显著更多实体 → 旧路径已僵化（Stale，建议降权/变异）；
//! 否则旧路径仍有效（Viable）。
//!
//! 接入点：由代谢 Deep Dream 事件调用（代谢窗口内执行，不阻塞主认知循环）。

/// 复盘裁决。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewVerdict {
    /// 旧路径仍有效（保留熟练权重）。
    Viable,
    /// 旧路径已僵化（检视发现显著更丰富的替代路径 → 建议降权/触发变异）。
    Stale,
}

/// 睡眠复盘参数（DNA 原则 11：阈值有来源，默认值 = ADR-0021 协议默认锚点）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewConfig {
    /// 检视输出相对旧路径实体覆盖度的增量阈值（≥ 视为僵化；默认 2）。
    pub stale_entity_delta: usize,
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self { stale_entity_delta: 2 }
    }
}

/// 睡眠复盘器（纯函数）。
#[derive(Debug, Clone, Copy, Default)]
pub struct SleepReview {
    /// 参数（来源：ReviewConfig，调用方按需配置）。
    pub config: ReviewConfig,
}

impl SleepReview {
    /// 非熟练模式检视：
    /// - `legacy_entity_count`：旧路径（熟练模式）工序输出的实体覆盖数。
    /// - `review_entity_count`：用锚定/想象力模式重跑同一工序得到的实体覆盖数。
    ///
    /// 检视增量 ≥ config.stale_entity_delta → 旧路径僵化（Stale）；否则仍有效（Viable）。
    pub fn review_path(&self, legacy_entity_count: usize, review_entity_count: usize) -> ReviewVerdict {
        if review_entity_count.saturating_sub(legacy_entity_count) >= self.config.stale_entity_delta {
            ReviewVerdict::Stale
        } else {
            ReviewVerdict::Viable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_when_review_finds_much_more() {
        // 旧路径只覆盖 1 个实体，检视发现 4 个 → 僵化。
        assert_eq!(SleepReview::default().review_path(1, 4), ReviewVerdict::Stale);
        // 恰好在阈值（差 2）→ 僵化。
        assert_eq!(SleepReview::default().review_path(1, 3), ReviewVerdict::Stale);
    }

    #[test]
    fn viable_when_review_finds_similar_or_less() {
        assert_eq!(SleepReview::default().review_path(4, 4), ReviewVerdict::Viable);
        assert_eq!(SleepReview::default().review_path(4, 5), ReviewVerdict::Viable, "差 1 不僵化");
        assert_eq!(SleepReview::default().review_path(5, 2), ReviewVerdict::Viable, "更少则旧路径更优");
    }
}
