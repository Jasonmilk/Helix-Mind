//! 自适应突变（ADR-0021 Phase 3 抗僵化）。
//!
//! **Bounded ε-Greedy + EMA 自适应突变率**：
//! - 突变率下限 3%、上限 20%（Bounded）。
//! - 成功率用 EMA（指数移动平均）平滑，防止震荡。
//! - 成功率信号 = **确定性信号**（任务完成标志/重试率），禁用 LLM 自我评分（R3）。
//!
//! 语义：长期成功（ema_success → 1）→ 降低探索（利用熟练路径，突变率趋 3%）；
//! 长期失败（ema_success → 0）→ 提高探索（探索新工序/模式，突变率趋 20%）。

/// 突变参数（DNA 原则 11：阈值有来源，默认值 = ADR-0021 协议默认锚点）。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MutationConfig {
    /// 突变率下限（默认 0.03）。
    pub min_rate: f64,
    /// 突变率上限（默认 0.20）。
    pub max_rate: f64,
    /// EMA 平滑系数（默认 0.3；越大越敏感）。
    pub alpha: f64,
}

impl Default for MutationConfig {
    fn default() -> Self {
        Self {
            min_rate: 0.03,
            max_rate: 0.20,
            alpha: 0.3,
        }
    }
}

/// 自适应突变状态机。
#[derive(Debug, Clone)]
pub struct AdaptiveMutation {
    /// 当前突变率（钳制在 min_rate..=max_rate）。
    mutation_rate: f64,
    /// EMA 平滑的成功率（0-1）。
    ema_success: f64,
    /// 参数（来源：MutationConfig，调用方按需配置）。
    config: MutationConfig,
}

impl Default for AdaptiveMutation {
    fn default() -> Self {
        Self::new(MutationConfig::default())
    }
}

impl AdaptiveMutation {
    pub fn new(config: MutationConfig) -> Self {
        Self {
            mutation_rate: config.min_rate + 0.02,
            ema_success: 0.5,
            config,
        }
    }

    /// Deterministically rebuild state from persisted values (P10c,
    /// ADR-0031 D3): sleep review restores the mutation rate / EMA across
    /// restarts. Caller validates ranges; values are clamped defensively.
    pub fn restore(mutation_rate: f64, ema_success: f64, config: MutationConfig) -> Self {
        Self {
            mutation_rate: mutation_rate.clamp(config.min_rate, config.max_rate),
            ema_success: ema_success.clamp(0.0, 1.0),
            config,
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
        let c = &self.config;
        self.ema_success = c.alpha * target + (1.0 - c.alpha) * self.ema_success;
        self.mutation_rate = c.max_rate - (c.max_rate - c.min_rate) * self.ema_success;
        self.mutation_rate = self.mutation_rate.clamp(c.min_rate, c.max_rate);
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
        let cfg = MutationConfig::default();
        let mut m = AdaptiveMutation::new(cfg);
        // 连续成功 → 突变率趋近下限，仍钳制在界内。
        for _ in 0..50 {
            m.record_outcome(true);
        }
        assert!(m.mutation_rate() >= cfg.min_rate);
        assert!(m.ema_success() > 0.99, "连续成功 EMA 趋 1");
        // 连续失败 → 突变率趋近上限，仍钳制在界内。
        for _ in 0..50 {
            m.record_outcome(false);
        }
        assert!(m.mutation_rate() <= cfg.max_rate);
        assert!(m.ema_success() < 0.01, "连续失败 EMA 趋 0");
    }

    #[test]
    fn epsilon_greedy_follows_rate() {
        let mut m = AdaptiveMutation::new(MutationConfig::default());
        m.record_outcome(true); // 成功率升高 → 突变率下降 → 更少探索
        // rng 0.0 恒 < rate → 探索；rng 0.5 通常不探索（rate < 0.2）。
        assert!(m.should_explore(0.0));
        assert!(!m.should_explore(0.5), "突变率 < 0.5 时不探索");
    }

    #[test]
    fn success_reduces_exploration_failure_increases() {
        let mut m = AdaptiveMutation::new(MutationConfig::default());
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
