//! SA-Core 参数层：α / 软边衰减 / 闸门 / 迭代预算的**单一来源**（ADR-0042）。
//!
//! # 为什么闸门必须是**相对**量（D0）
//!
//! `topology.rs` 对**每个**种子注入 `1.0`（`a_0[flat_idx] = 1.0`），所以激活总质量的
//! 量纲是「种子个数 k」，不是 1。因此**绝对**阈值不是尺度不变的：同一个 `0.8` 对
//! 1 个种子的查询（k=1）和 30 个种子的查询（k=30）含义完全不同。
//!
//! 更糟的是在当前取值下它**恒为致命**：单种子、单位权边时，第 1 跳邻居的激活上界
//! 是 `α`（skilled 0.5 / anchor 0.7），**永远够不到 0.8** ⇒ 非种子节点全被清零 ⇒
//! 扩散退化成「只返回种子」，也就是关键词命中。`imagination`（α=0.9）之所以还能
//! 多跳，只是因为它另传了一个 ~0.001 的阈值绕开了这个门。
//!
//! 前沿做法是相对阈值：Andersen–Chung–Lang 的局部 PPR 用 `r(v) > ε·d(v)` 判停，
//! 阈值相对**度数**（Thorup 讲义给出同一推式 `r(w) += (1−α)·r(v)/d(v)`）。
//! 本仓已经抄了行归一化那一半（等价于除以度数），**却把阈值留在绝对刻度上**——
//! 两半只抄了一半。D0 补上另一半：`θ = τ · max_j |a_j|`，即相对**当轮峰值激活**。
//!
//! 为什么锚在**峰值**而不是总质量（这一点是写回归测试时才暴露的）：
//! 总质量随种子数**线性增长**，而峰值不随种子数变化（各条独立链的取值本来就相同）。
//! 于是「相对总质量」的闸门会让多种子查询比单种子查询**探索得更浅**——
//! 同一个 τ 对不同查询不等价，尺度不变性只修好了一半。锚在峰值则两种查询的深度
//! **完全一致**，`τ` 的含义真正与查询规模无关。
//!
//! # 为什么 α 是「模式基准 + 调制」而不是纯映射（D1）
//!
//! 纯映射（`heliotropism ∈ [−1,1] → α ∈ [0.2, 0.8]`）在 `h = 0` 处把三个模式的 α
//! 全部压成 `0.5`，**静默抹平** anchor(0.7) 与 imagination(0.9) 的差异——那是功能
//! 退化，不是中性改造。调制式在 `h = 0` 处**精确复原**各模式基准值（行为中性），
//! 只在 `h ≠ 0` 时改变扩散半径。
//!
//! 一个可核对的闭环：README 公告的「Optimistic 0.8 / Defensive 0.2」在
//! `gain = 0.3`、skilled 基准 `0.5` 时**恰好是 skilled 模式的两个角**（0.5 ± 0.3），
//! 所以 README 的那两个数没有被推翻，而是被收编为 skilled 的特例。
//!
//! # 为什么没有 `max_hops` 与 `max_iterations` 两个旋钮（D2，我上一版 ADR 的更正）
//!
//! 同步幂迭代下**一次迭代 = 一跳**，二者是**同一个量纲**。我上一版 ADR 写成
//! 「迭代次数与跳数混用是两个不同的量」是错的——凭空造两个数值相同的旋钮属于
//! 假精度。真正缺的是**收敛判据**：现在唯一的停止规则是硬截断，于是跑出来的是
//! **截断 PPR** 而非不动点，而这个事实被「跳数」之名掩盖了。D2 因此只做两件事：
//! 补相对 ℓ1 收敛判据，并把既有的那个唯一旋钮（[`RetrievalConfig::max_hops`]）
//! 正名为**算力预算**——`max_hops` 因此从「被闸门架空的摆设」变成真正生效的约束。

use serde::Deserialize;

use crate::config::RetrievalConfig;
use crate::graph::CognitiveMode;

/// Skilled 模式**忽略软边**的语义常量。
///
/// 这里的 `0.0` 不是可调参数，而是「软边在本模式下不参与传播」这一**语义开关**
/// （`is_soft` 边的权重乘 0 即被排除）。因此它不放进 config：它没有可调的自由度。
pub const SOFT_EDGES_DISABLED: f64 = 0.0;

/// SA-Core 的全部可调参数（config `[retrieval.sa_core]`）。
///
/// 默认值均为**行为中性**或**已论证**的取值，见各字段注释。
#[derive(Debug, Clone, Deserialize)]
pub struct SaCoreConfig {
    // ---- α 的模式基准（heliotropism = 0 时精确取这些值）----
    /// Skilled 基准。0.5 ⇒ 有效扩散半径 `1/(1−α) = 2` 跳。
    #[serde(default = "default_alpha_skilled")]
    pub alpha_skilled: f64,
    /// Anchor 基准。0.7 ⇒ 半径 ≈ 3.3 跳。
    #[serde(default = "default_alpha_anchor")]
    pub alpha_anchor: f64,
    /// Imagination 基准。0.9 ⇒ 半径 10 跳（混沌漫游，本就该广）。
    #[serde(default = "default_alpha_imagination")]
    pub alpha_imagination: f64,

    /// Imagination 的软边衰减。Skilled 恒为 0（`SOFT_EDGES_DISABLED`）、
    /// Anchor 取 `soft_edge_decay_factor`，故此处只需一个字段。
    #[serde(default = "default_decay_imagination")]
    pub decay_imagination: f64,

    // ---- α 的 heliotropism 调制 ----
    /// 每个单位的 heliotropism 给 α 的增量。0.3 使 skilled 的 ±1 两角
    /// 正好落在 README 已公告的 0.8 / 0.2 上。
    #[serde(default = "default_heliotropism_gain")]
    pub heliotropism_gain: f64,
    /// α 下夹（防御侧极限）。skilled 在 h=−1 时正好取到 0.2。
    #[serde(default = "default_alpha_floor")]
    pub alpha_floor: f64,
    /// α 上夹。**必须 < 1.0**：`ρ(αW) ≤ α`，α→1 时收敛判据所需迭代数 →∞。
    #[serde(default = "default_alpha_ceiling")]
    pub alpha_ceiling: f64,

    // ---- 闸门（D0）----
    /// 相对闸门系数 τ：节点存活需持有至少 `τ × 当轮峰值激活`。
    ///
    /// 锚在**峰值**（而不是总质量）是为了对种子数完全尺度不变：总质量随种子数
    /// 线性增长，峰值不变，所以相对总质量会让多种子查询探索得更浅。
    ///
    /// 默认 0.02 在 α=0.5 的链上给出约 5 跳有效深度（`(1−α)α^t ≥ τ·α`），
    /// 而旧的绝对 0.8 给出 **0 跳**。
    #[serde(default = "default_gate_relative_tau")]
    pub gate_relative_tau: f64,

    // ---- 迭代预算（D2）----
    /// 相对 ℓ1 收敛判据：`Σ|a_new − a_old| < ε × Σ|a_new|` 即停。
    /// 用**相对**形式是因为激活总质量的量纲是「种子个数」。
    ///
    /// 注意**没有** `max_iterations` 字段：同步幂迭代下一次迭代就是一跳，
    /// 迭代上限与跳数上限是**同一个旋钮**，即 [`RetrievalConfig::max_hops`]。
    /// 再造一个数值相同的字段属于假精度（见本模块头部 D2 说明）。
    #[serde(default = "default_convergence_epsilon")]
    pub convergence_epsilon: f64,
}

impl Default for SaCoreConfig {
    fn default() -> Self {
        Self {
            alpha_skilled: default_alpha_skilled(),
            alpha_anchor: default_alpha_anchor(),
            alpha_imagination: default_alpha_imagination(),
            decay_imagination: default_decay_imagination(),
            heliotropism_gain: default_heliotropism_gain(),
            alpha_floor: default_alpha_floor(),
            alpha_ceiling: default_alpha_ceiling(),
            gate_relative_tau: default_gate_relative_tau(),
            convergence_epsilon: default_convergence_epsilon(),
        }
    }
}

/// 一次检索**实际使用**的 SA-Core 参数（已按模式与 heliotropism 求值）。
///
/// `Copy` 且纯数据：storage 层因此不需要知道「模式」或「heliotropism」，
/// 只需按给定参数跑图。这保持了 `topology.rs` 是**算法**、参数来源是**策略**的分层。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaCoreParams {
    /// 阻尼/重启系数（PPR 的 `damping`，**不是** decay）。
    pub alpha: f64,
    /// 软边权重衰减（仅作用于 `is_soft` 边）。
    pub decay_factor: f64,
    /// 相对闸门系数 τ。
    pub gate_relative_tau: f64,
    /// 相对 ℓ1 收敛阈值。
    pub convergence_epsilon: f64,
}

/// α = clamp(base + gain · heliotropism, floor, ceiling)。
///
/// `heliotropism` 已在 `EnergyContext::validate` 处校验范围，这里再夹一次是
/// **防御性**的：α 越界会直接破坏 `ρ(αW) < 1` 的收敛前提，代价不可接受。
pub fn alpha_from_heliotropism(
    base: f64,
    heliotropism: f64,
    gain: f64,
    floor: f64,
    ceiling: f64,
) -> f64 {
    let h = if heliotropism.is_finite() {
        heliotropism.clamp(-1.0, 1.0)
    } else {
        0.0
    };
    (base + gain * h).clamp(floor, ceiling)
}

impl SaCoreParams {
    /// 按认知模式取基准参数，再由 heliotropism 调制 α。
    ///
    /// 这是**唯一**的模式参数来源：`retrieval/src/mode.rs` 曾有一份数值矛盾的
    /// 副本（Skilled `weight_threshold=0.9`、Imagination `0.3`）且零调用者，
    /// 已删除而非复活——两套参数并存正是 ADR-0042 §4.4 要消除的病。
    pub fn for_mode(mode: CognitiveMode, heliotropism: f64, cfg: &RetrievalConfig) -> Self {
        let sa = &cfg.sa_core;
        let (base, decay_factor) = match mode {
            CognitiveMode::Skilled => (sa.alpha_skilled, SOFT_EDGES_DISABLED),
            // Anchor 的软边衰减沿用既有的 `soft_edge_decay_factor`：那个名字
            // 描述的就是这件事，不另立第二个数。
            CognitiveMode::Anchor => (sa.alpha_anchor, cfg.soft_edge_decay_factor),
            CognitiveMode::Imagination => (sa.alpha_imagination, sa.decay_imagination),
        };
        Self {
            alpha: alpha_from_heliotropism(
                base,
                heliotropism,
                sa.heliotropism_gain,
                sa.alpha_floor,
                sa.alpha_ceiling,
            ),
            decay_factor,
            gate_relative_tau: sa.gate_relative_tau,
            convergence_epsilon: sa.convergence_epsilon,
        }
    }
}

// ---------- Default Functions ----------
fn default_alpha_skilled() -> f64 { 0.5 }
fn default_alpha_anchor() -> f64 { 0.7 }
fn default_alpha_imagination() -> f64 { 0.9 }
fn default_decay_imagination() -> f64 { 0.95 }
fn default_heliotropism_gain() -> f64 { 0.3 }
fn default_alpha_floor() -> f64 { 0.2 }
fn default_alpha_ceiling() -> f64 { 0.95 }
fn default_gate_relative_tau() -> f64 { 0.02 }
fn default_convergence_epsilon() -> f64 { 1e-6 }

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> RetrievalConfig {
        RetrievalConfig::default()
    }

    /// D1 的行为中性保证：heliotropism = 0.0 必须**精确**复原三个模式的历史基准值。
    /// 这条断言是「调制式替换纯映射」的正当性来源——否则就是静默改变召回。
    #[test]
    fn heliotropism_zero_reproduces_every_mode_base_alpha() {
        let c = cfg();
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Skilled, 0.0, &c).alpha,
            0.5
        );
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Anchor, 0.0, &c).alpha,
            0.7
        );
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Imagination, 0.0, &c).alpha,
            0.9
        );
    }

    /// README 公告的 Optimistic 0.8 / Defensive 0.2 必须是 skilled 的两个角。
    /// 这条断言把文档里的两个数**钉住**，使 D6 的文档更正有可核对的依据。
    #[test]
    fn readme_published_endpoints_are_the_skilled_corners() {
        let c = cfg();
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Skilled, 1.0, &c).alpha,
            0.8
        );
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Skilled, -1.0, &c).alpha,
            0.2
        );
    }

    /// 收敛前提：任何输入下 α 都必须严格小于 1（否则 `ρ(αW) < 1` 不成立）。
    #[test]
    fn alpha_never_reaches_one_even_at_the_optimistic_extreme() {
        let c = cfg();
        for mode in [
            CognitiveMode::Skilled,
            CognitiveMode::Anchor,
            CognitiveMode::Imagination,
        ] {
            for h in [-1.0, -0.5, 0.0, 0.5, 1.0, 7.0, -7.0] {
                let a = SaCoreParams::for_mode(mode.clone(), h, &c).alpha;
                assert!(a < 1.0, "alpha={a} mode={mode:?} h={h}");
                assert!(a >= c.sa_core.alpha_floor, "alpha={a} below floor");
            }
        }
    }

    /// 方向性：向阳度越高，扩散越广（α 越大）。这是 README 承诺的语义本身。
    #[test]
    fn optimistic_heliotropism_widens_diffusion() {
        assert!(
            alpha_from_heliotropism(0.5, 1.0, 0.3, 0.2, 0.95)
                > alpha_from_heliotropism(0.5, -1.0, 0.3, 0.2, 0.95)
        );
    }

    /// 非有限输入不得污染 α（`f64::NAN.clamp(..)` 返回 NaN，而 NaN 的每次大小
    /// 比较都为假，会静默破坏闸门与收敛判据里的全部判断）。
    ///
    /// 约定是**退化为中性 0.0**（即基准 α），而不是夹到某一端：±∞ 与 NaN 都不是
    /// 「极度乐观/极度保守」的有效读数，凭它们推测意图属于编造数据。
    #[test]
    fn non_finite_heliotropism_degrades_to_neutral() {
        let neutral = alpha_from_heliotropism(0.5, 0.0, 0.3, 0.2, 0.95);
        assert_eq!(neutral, 0.5);
        assert_eq!(alpha_from_heliotropism(0.5, f64::NAN, 0.3, 0.2, 0.95), neutral);
        assert_eq!(
            alpha_from_heliotropism(0.5, f64::INFINITY, 0.3, 0.2, 0.95),
            neutral
        );
        assert_eq!(
            alpha_from_heliotropism(0.5, f64::NEG_INFINITY, 0.3, 0.2, 0.95),
            neutral
        );
    }

    /// Skilled 必须屏蔽软边（衰减 0），Anchor 必须用 config 的软边衰减——
    /// 这两条一起锁住「软边在本仓只有一个来源」。
    #[test]
    fn soft_edge_decay_comes_from_a_single_source() {
        let c = cfg();
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Skilled, 0.0, &c).decay_factor,
            0.0
        );
        assert_eq!(
            SaCoreParams::for_mode(CognitiveMode::Anchor, 0.0, &c).decay_factor,
            c.soft_edge_decay_factor
        );
    }
}
