//! 自适应突变（ADR-0021 Phase 3 抗僵化）。
//!
//! **Bounded ε-Greedy + EMA 自适应突变率**：
//! - 突变率下限 3%、上限 20%（Bounded）。
//! - 成功率用 EMA（指数移动平均）平滑，防止震荡。
//! - 成功率信号 = **确定性信号**（任务完成标志/重试率），禁用 LLM 自我评分（R3）。
//!
//! 语义：长期成功（ema_success → 1）→ 降低探索（利用熟练路径，突变率趋 3%）；
//! 长期失败（ema_success → 0）→ 提高探索（探索新工序/模式，突变率趋 20%）。

/// 自适应突变状态机。
#[derive(Debug, Clone)]
pub struct AdaptiveMutation {
    /// 当前突变率（钳制在 3%-20%）。
    mutation_rate: f64,
    /// EMA 平滑的成功率（0-1）。
    ema_success: f64,
    /// EMA 平滑系数（0-1；越大越敏感）。
    alpha: f64,
}

const MIN_RATE: f64 = 0.03;
const MAX_RATE: f64 = 0.20;
const DEFAULT_ALPHA: f64 = 0.3;

impl Default for AdaptiveMutation {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveMutation {
    pub fn new() -> Self {
        Self {
            mutation_rate: 0.05,
            ema_success: 0.5,
            alpha: DEFAULT_ALPHA,
        }
    }

    /// ε-Greedy 决策：给定均匀随机数 rng ∈ [0,1)，是否探索（尝试非熟练路径）。
    ///
    /// 调用方传入确定性/伪随机数（测试可注入，生产可用固定步长避免非确定性）。
    pub fn should_explore(&self, rng: f64) -> bool {
        rng < self.mutation_rate
    }

    /// 记录一次确定性结果（成功/失败）并自适应调整突变率。
    ///
    /// - EMA 更新：`ema = alpha * target + (1 - alpha) * ema`，target = 成功 ? 1 : 0。
    /// - 突变率线性映射：`rate = MAX - (MAX - MIN) * ema`，钳制。
    pub fn record_outcome(&mut self, success: bool) {
        let target = if success { 1.0 } else { 0.0 };
        self.ema_success = self.alpha * target + (1.0 - self.alpha) * self.ema_success;
        self.mutation_rate = MAX_RATE - (MAX_RATE - MIN_RATE) * self.ema_success;
        self.mutation_rate = self.mutation_rate.clamp(MIN_RATE, MAX_RATE);
    }

    /// 连续记录一组确定性结果（如一个阶段的多次任务）。
    pub fn record_batch(&mut self, outcomes: &[bool]) {
        for &o in outcomes {
            self.record_outcome(o);
        }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn ema_success(&self) -> f64 {
        self.ema_success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_are_respected() {
        let mut m = AdaptiveMutation::new();
        // 连续成功 → 突变率趋近下限 3%，仍钳制在界内。
        for _ in 0..50 {
            m.record_outcome(true);
        }
        assert!(m.mutation_rate() >= MIN_RATE);
        assert!(m.ema_success() > 0.99, "连续成功 EMA 趋 1");
        // 连续失败 → 突变率趋近上限 20%，仍钳制在界内。
        for _ in 0..50 {
            m.record_outcome(false);
        }
        assert!(m.mutation_rate() <= MAX_RATE);
        assert!(m.ema_success() < 0.01, "连续失败 EMA 趋 0");
    }

    #[test]
    fn epsilon_greedy_follows_rate() {
        let mut m = AdaptiveMutation::new();
        m.record_outcome(true); // 成功率升高 → 突变率下降 → 更少探索
        // rng 0.0 恒 < rate → 探索；rng 0.5 通常不探索（rate < 0.2）。
        assert!(m.should_explore(0.0));
        assert!(!m.should_explore(0.5), "突变率 < 0.5 时不探索");
    }

    #[test]
    fn success_reduces_exploration_failure_increases() {
        let mut m = AdaptiveMutation::new();
        for _ in 0..30 {
            m.record_outcome(true);
        }
        let after_success = m.mutation_rate();
        for _ in 0..30 {
            m.record_outcome(false);
        }
        let after_failure = m.mutation_rate();
        assert!(
            after_failure > after_success,
            "失败后探索率应高于成功后 ({} > {})",
            after_failure,
            after_success
        );
    }
}
