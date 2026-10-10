//! 黑格尔辩证收敛协议（ADR-0021 §5，R1 确定性合题）。
//!
//! 合题是**确定性操作**，不是 LLM 自由发挥：
//! - 正题（Thesis）：提取结构/创造工序的核心主张。
//! - 反题（Antithesis）：提取批判工序的证伪边界与风险。
//! - 合题（Synthesis）：条件约束下的结构化重构——保留正题价值，注入反题的防御约束 → 条件化命题。

use crate::craft::{Process, StepOutput};

/// 确定性合题。输出条件化命题：正题成立 ⇔ 反题所列风险被排除。
pub fn converge_hegelian(query: &str, outputs: &[StepOutput]) -> String {
    // 正题：第一个非批判性工序输出（陈述性核心主张）。
    let thesis = outputs
        .iter()
        .find(|o| !matches!(o.process, Process::Critical))
        .map(|o| o.content.as_str());
    // 反题：第一个批判性工序输出（证伪边界与风险）。
    let antithesis = outputs
        .iter()
        .find(|o| matches!(o.process, Process::Critical))
        .map(|o| o.content.as_str());

    match (thesis, antithesis) {
        (Some(t), Some(a)) => format!(
            "结论（正题）：{}\n反题约束：{}\n条件化命题：{} 成立，当且仅当上述风险被排除。",
            t, a, query
        ),
        (Some(t), None) => format!("结论（正题）：{}", t),
        (None, Some(a)) => format!("反题约束（无正题可合成）：{}", a),
        (None, None) => "（无足够工序输出可供收敛）".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn out(process: Process, content: &str) -> StepOutput {
        StepOutput { process, content: content.to_string() }
    }

    #[test]
    fn thesis_and_antithesis_combined_into_conditional() {
        let outputs = vec![
            out(Process::Structural, "核心实体：A、B、C"),
            out(Process::Critical, "风险：A 依赖未验证"),
        ];
        let s = converge_hegelian("问题X", &outputs);
        assert!(s.contains("核心实体：A、B、C"), "正题保留");
        assert!(s.contains("风险：A 依赖未验证"), "反题注入");
        assert!(s.contains("当且仅当"), "条件化命题");
        assert!(s.contains("问题X"), "query 锚定");
    }

    #[test]
    fn no_antithesis_keeps_thesis_only() {
        let outputs = vec![out(Process::Structural, "结构结论")];
        let s = converge_hegelian("问题Y", &outputs);
        assert!(s.contains("结构结论"));
        assert!(!s.contains("当且仅当"), "无反题则无条件化");
    }

    #[test]
    fn empty_outputs_fallback() {
        let s = converge_hegelian("问题Z", &[]);
        assert!(s.contains("无足够工序输出"));
    }
}
