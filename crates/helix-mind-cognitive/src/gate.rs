//! System 0 启发式门控（ADR-0021 §B2）。
//!
//! 认知工艺启动前的轻量门控判断，避免"为了判断是否简单，先花 500 Token 编排"的递归悖论。
//! 全部 0 Token：规则匹配（长度 + 关键词）+ 用户意图标签。
//!
//! FTS5 bm25 相关度信号（Phase 3 增强）：通过 `GateSignal` trait 预留，当前实现为纯规则
//! （零向量依赖、零新依赖，符合极致节能）。

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
}
