//! System 0 启发式门控（ADR-0021 §B2）。
//!
//! 认知工艺启动前的轻量门控判断，避免"为了判断是否简单，先花 500 Token 编排"的递归悖论。
//! 全部 0 Token：规则匹配（长度 + 关键词）+ 用户意图标签 + **确定性问题价值评估** +
//! **FTS5 bm25 相似度信号**（Phase 3）。
//!
//! bm25 相似度信号来源：上层用 `storage::fts::fts_search` 对 query 匹配已知简单问题池，
//! 取最高 bm25 命中归一化为 `simple_similarity ∈ [0,1]` 注入（cognitive 保持纯逻辑，
//! 零向量依赖、零新依赖，符合极致节能）。

use crate::value::ValueGrade;

/// 门控决策。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateDecision {
    /// 低价值/简单 → 熟练模式直接输出（不触发认知工艺）。
    DirectSkilled,
    /// 高价值/复杂 → 触发认知工艺。
    TriggerCraft,
}

/// 门控信号输入。
#[derive(Debug, Clone)]
pub struct GateSignals {
    /// 用户问题原文。
    pub query: String,
    /// 用户显式意图标签（"帮我深入分析"等 → 跳过门控直接触发）。
    pub explicit_deep: bool,
}

/// bm25 简单相似度阈值（≥ 视为与已知简单问题高度相似 → 直接熟练模式）。
const SIMPLE_SIMILARITY_THRESHOLD: f64 = 0.7;

/// 触发关键词（规则匹配）。命中任一即视为复杂。
const TRIGGER_KEYWORDS: &[&str] = &[
    "分析", "审查", "评估", "设计", "权衡", "深入", "为什么", "如何", "架构", "风险",
    "方案", "比较", "论证", "推导", "优化", "权衡",
];

/// 简单问题长度阈值（字符）。超过视为可能复杂。
const SIMPLE_LENGTH_THRESHOLD: usize = 40;

impl GateSignals {
    pub fn new(query: impl Into<String>, explicit_deep: bool) -> Self {
        Self { query: query.into(), explicit_deep }
    }
}

/// 基础门控：规则 + 用户意图标签（0 Token）。
///
/// 顺序（B2）：
/// 1. 用户显式"深入分析"标签 → TriggerCraft（跳过门控）。
/// 2. 关键词命中 → TriggerCraft。
/// 3. 长度超阈值 → TriggerCraft。
/// 4. 否则 → DirectSkilled。
pub fn system0_gate(signals: &GateSignals) -> GateDecision {
    if signals.explicit_deep {
        return GateDecision::TriggerCraft;
    }
    let q = signals.query.trim();
    if q.is_empty() {
        return GateDecision::DirectSkilled;
    }
    if TRIGGER_KEYWORDS.iter().any(|k| q.contains(k)) {
        return GateDecision::TriggerCraft;
    }
    if q.chars().count() >= SIMPLE_LENGTH_THRESHOLD {
        return GateDecision::TriggerCraft;
    }
    GateDecision::DirectSkilled
}

/// 增强门控（Phase 3）：价值评估 + bm25 相似度信号 + 规则/标签。
///
/// 顺序：
/// 1. 用户显式"深入分析"标签 → TriggerCraft（跳过门控）。
/// 2. bm25 简单相似度 ≥ 阈值 → DirectSkilled（与已知简单问题高度相似）。
/// 3. 确定性问题价值评估 → Low = DirectSkilled；Medium/High = TriggerCraft。
/// 4. 规则兜底（关键词/长度）。
pub fn system0_gate_enhanced(
    signals: &GateSignals,
    value: ValueGrade,
    simple_similarity: Option<f64>,
) -> GateDecision {
    if signals.explicit_deep {
        return GateDecision::TriggerCraft;
    }
    if let Some(sim) = simple_similarity {
        if sim >= SIMPLE_SIMILARITY_THRESHOLD {
            return GateDecision::DirectSkilled;
        }
    }
    match value {
        ValueGrade::Low => GateDecision::DirectSkilled,
        ValueGrade::Medium | ValueGrade::High => GateDecision::TriggerCraft,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_question_direct_skilled() {
        assert_eq!(
            system0_gate(&GateSignals::new("现在几点", false)),
            GateDecision::DirectSkilled
        );
    }

    #[test]
    fn keyword_triggers_craft() {
        assert_eq!(
            system0_gate(&GateSignals::new("帮我分析这个方案的架构风险", false)),
            GateDecision::TriggerCraft
        );
    }

    #[test]
    fn long_question_triggers_craft() {
        let long = "这个问题需要深入探讨多个层面包括第一层面的定义第二层面的边界以及第三层面的实现细节";
        assert_eq!(
            system0_gate(&GateSignals::new(long, false)),
            GateDecision::TriggerCraft
        );
    }

    #[test]
    fn explicit_label_skips_gate() {
        // 短问题 + 显式标签 → 触发（用户意图优先）。
        assert_eq!(
            system0_gate(&GateSignals::new("帮我深入分析", true)),
            GateDecision::TriggerCraft
        );
    }

    #[test]
    fn empty_query_direct() {
        assert_eq!(
            system0_gate(&GateSignals::new("", false)),
            GateDecision::DirectSkilled
        );
    }

    // ── 增强门控（Phase 3）──

    #[test]
    fn enhanced_bm25_similarity_routes_simple() {
        // 短问题 + 高简单相似度（0.9）→ 直接熟练，即使长度触发基础规则。
        let sig = GateSignals::new("现在几点", false);
        assert_eq!(
            system0_gate_enhanced(&sig, ValueGrade::Medium, Some(0.9)),
            GateDecision::DirectSkilled,
            "bm25 高相似优先于价值评估"
        );
    }

    #[test]
    fn enhanced_low_value_direct_low_similarity_craft_by_rule() {
        // 低价值 + 低相似度 → DirectSkilled（价值主导）。
        let sig = GateSignals::new("简单问题", false);
        assert_eq!(
            system0_gate_enhanced(&sig, ValueGrade::Low, Some(0.1)),
            GateDecision::DirectSkilled
        );
        // 高价值 + 低相似度 → TriggerCraft。
        let sig2 = GateSignals::new("简单问题", false);
        assert_eq!(
            system0_gate_enhanced(&sig2, ValueGrade::High, Some(0.1)),
            GateDecision::TriggerCraft
        );
    }

    #[test]
    fn enhanced_explicit_label_always_triggers() {
        let sig = GateSignals::new("你好", true);
        assert_eq!(
            system0_gate_enhanced(&sig, ValueGrade::Low, Some(0.99)),
            GateDecision::TriggerCraft,
            "用户显式意图优先于一切门控信号"
        );
    }
}

