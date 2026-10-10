//! 确定性问题价值评估（ADR-0021 Phase 3）。
//!
//! 决定"这个问题值不值得深度思考"以及"思考到什么深度"。
//! **确定性信号、0 Token**：规则（长度 + 关键词 + 实体多样性），禁用 LLM 自我评分
//! （信用由他证，自我评价不可靠，R3）。

use crate::craft::Mode;

/// 价值等级。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueGrade {
    /// 低价值：直接熟练模式（不触发认知工艺）。
    Low,
    /// 中价值：锚定模式（适度扩展）。
    Medium,
    /// 高价值：触发认知工艺（深度编排）。
    High,
}

impl ValueGrade {
    /// 建议的思考深度（运行模式）。
    pub fn suggested_mode(self) -> Mode {
        match self {
            ValueGrade::Low => Mode::Skilled,
            ValueGrade::Medium => Mode::Anchored,
            ValueGrade::High => Mode::Imaginative,
        }
    }
}

/// 复杂信号关键词（命中任一提升价值等级）。
const DEPTH_KEYWORDS: &[&str] = &[
    "分析", "审查", "评估", "设计", "权衡", "架构", "风险", "方案", "论证", "推导", "优化",
];

/// 价值评估器（纯函数 + 0 Token）。
#[derive(Debug, Clone, Copy, Default)]
pub struct ValueAssessor;

impl ValueAssessor {
    /// 确定性价值评估。
    ///
    /// 规则：
    /// 1. 基础分 = 长度档（<20 Low / 20-60 Medium / >60 High）。
    /// 2. 复杂关键词命中 → 提升一档。
    /// 3. 空输入 → Low。
    pub fn assess(&self, query: &str) -> ValueGrade {
        let q = query.trim();
        if q.is_empty() {
            return ValueGrade::Low;
        }
        let chars = q.chars().count();
        let base = if chars >= 60 {
            ValueGrade::High
        } else if chars >= 20 {
            ValueGrade::Medium
        } else {
            ValueGrade::Low
        };
        let has_depth = DEPTH_KEYWORDS.iter().any(|k| q.contains(k));
        match (base, has_depth) {
            (ValueGrade::Low, true) => ValueGrade::Medium,
            (ValueGrade::Medium, true) => ValueGrade::High,
            _ => base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_simple_is_low() {
        assert_eq!(ValueAssessor.assess("现在几点"), ValueGrade::Low);
        assert_eq!(ValueAssessor.assess("现在几点").suggested_mode(), Mode::Skilled);
    }

    #[test]
    fn short_with_depth_keyword_medium() {
        assert_eq!(ValueAssessor.assess("帮我分析"), ValueGrade::Medium);
    }

    #[test]
    fn long_with_depth_is_high() {
        let long = "请深入分析这个分布式系统的架构可靠性风险评估权衡方案设计论证推导优化边界";
        assert_eq!(ValueAssessor.assess(long), ValueGrade::High);
        assert_eq!(ValueAssessor.assess(long).suggested_mode(), Mode::Imaginative);
    }

    #[test]
    fn empty_is_low() {
        assert_eq!(ValueAssessor.assess(""), ValueGrade::Low);
        assert_eq!(ValueAssessor.assess("   "), ValueGrade::Low);
    }
}
