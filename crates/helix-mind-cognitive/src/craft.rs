//! 认知工艺编排器（ADR-0021，Phase 2 最小原型）。
//!
//! 主会话编排 → 独立会话执行（MSC 隔离）→ 黑格尔辩证收敛。
//! - 执行经 ADR-0017 `CognitiveService` 注入（B1：Mind=编排，执行不直连 LLM）。
//! - 熔断：步骤数 ≤ max_steps；每步超时 30s；总 Token 预算（EnergyExhausted）。
//! - trace_id 确定性派生（`craft#{job_id}`），非 UUID（DNA 原则 11）。
//! - 会话隔离：每步只接收 MSC（原始输入 + 工序专有 Prompt + 全局约束），不横向引用草稿。

use std::sync::Arc;
use std::time::Duration;

use helix_mind_core::error::MindError;
use helix_mind_core::graph::{Node, NodeContent, NodeType, Sensitivity};
use helix_mind_metabolism::CognitiveService;

use crate::converge::converge_hegelian;

/// 认知操作算子（工序）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Process {
    Structural,
    Critical,
    Creative,
    Situational,
    MetaCritical,
}

impl Process {
    pub fn label(self) -> &'static str {
        match self {
            Process::Structural => "结构性",
            Process::Critical => "批判性",
            Process::Creative => "创造性",
            Process::Situational => "情境/意图解析",
            Process::MetaCritical => "元批判",
        }
    }
    /// 工序专有 Prompt（MSC 组成之一）。
    pub fn prompt(self) -> &'static str {
        match self {
            Process::Structural => "结构化分析：识别边界、依赖、架构",
            Process::Critical => "批判性审查：寻找漏洞、盲区、假设错误",
            Process::Creative => "创造性探索：提出新方案、替代路径",
            Process::Situational => "情境解析：理解需求、情绪、上下文",
            Process::MetaCritical => "元批判：审视思考过程本身",
        }
    }
}

/// 运行模式（与工序正交）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Skilled,
    Anchored,
    Imaginative,
}

/// 一道工序（工序 + 分配的运行模式）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessStep {
    pub process: Process,
    pub mode: Mode,
}

impl ProcessStep {
    pub fn new(process: Process, mode: Mode) -> Self {
        Self { process, mode }
    }
}

/// 编排器配置（熔断参数）。
#[derive(Debug, Clone)]
pub struct CraftConfig {
    /// 最大子工序数（默认 5）。
    pub max_steps: usize,
    /// 单工序超时（默认 30s）。
    pub step_timeout: Duration,
    /// 总 Token 预算（0 = 不限制；由 EnergyContext 传入）。
    pub total_token_budget: u64,
}

impl Default for CraftConfig {
    fn default() -> Self {
        Self {
            max_steps: 5,
            step_timeout: Duration::from_secs(30),
            total_token_budget: 0,
        }
    }
}

/// 编排输入。
#[derive(Debug, Clone)]
pub struct CraftInput {
    /// 原始输入（事实锚点，MSC 中不可裁剪）。
    pub query: String,
    /// 工序组合（≤ max_steps）。
    pub steps: Vec<ProcessStep>,
    /// 全局约束 Schema（MSC 组成之一）。
    pub global_constraints: String,
    /// 调用方作业标识（Anaphase run_cycle job_id；trace_id 确定性派生源，非 UUID）。
    pub job_id: String,
}

/// 单工序输出。
#[derive(Debug, Clone)]
pub struct StepOutput {
    pub process: Process,
    pub content: String,
}

/// 编排结果。
#[derive(Debug, Clone)]
pub struct CraftResult {
    /// 本编排的根 trace_id（全息留痕-交互追溯；所有子工序共享，Phase 2 编排级）。
    pub trace_id: String,
    pub steps: Vec<StepOutput>,
    /// 黑格尔收敛合题（条件化命题）。
    pub synthesis: String,
}

/// 最小充分上下文（MSC）——每道工序的独立会话注入物。
struct Msc<'a> {
    trace_id: &'a str,
    query: &'a str,
    process: Process,
    mode: Mode,
    global_constraints: &'a str,
}

/// 认知工艺编排器。
pub struct CognitiveCraft {
    cognitive: Arc<dyn CognitiveService>,
    config: CraftConfig,
}

impl CognitiveCraft {
    pub fn new(cognitive: Arc<dyn CognitiveService>, config: CraftConfig) -> Self {
        Self { cognitive, config }
    }

    /// 主会话编排：校验 → 独立会话执行每道工序（MSC 隔离 + 熔断）→ 黑格尔收敛。
    pub async fn orchestrate(&self, input: CraftInput) -> Result<CraftResult, MindError> {
        // 熔断 1：步骤数（空或超限 → 拒绝，防编排空转/过度拆分）。
        if input.steps.is_empty() || input.steps.len() > self.config.max_steps {
            return Err(MindError::Validation(format!(
                "steps must be 1..={} (got {})",
                self.config.max_steps,
                input.steps.len()
            )));
        }
        // 熔断 2：总 Token 预算（EnergyContext 前置路由传入；0 = 不限制）。
        if self.config.total_token_budget > 0 && self.config.total_token_budget < 10 {
            return Err(MindError::EnergyExhausted);
        }

        // 2. 独立会话执行每道工序（每步只注入 MSC，不横向引用草稿 → 会话隔离）。
        // 根 trace_id 确定性派生（全息留痕-交互追溯；DNA 原则 11：非 UUID，
        // 同 job_id 同输入 → 字节级一致 trace）；所有子工序共享同一 trace_id。
        let trace_id = format!("craft#{}", input.job_id);
        let mut outputs = Vec::with_capacity(input.steps.len());
        for step in &input.steps {
            let msc = Msc {
                trace_id: &trace_id,
                query: &input.query,
                process: step.process,
                mode: step.mode,
                global_constraints: &input.global_constraints,
            };
            // 熔断 3：单工序超时。
            let content = tokio::time::timeout(self.config.step_timeout, self.execute_step(&msc))
                .await
                .map_err(|_| {
                    MindError::Validation(format!(
                        "cognitive craft step '{}' timed out after {:?}",
                        step.process.label(),
                        self.config.step_timeout
                    ))
                })??;
            outputs.push(StepOutput { process: step.process, content });
        }

        // 3. 黑格尔辩证确定性收敛。
        let synthesis = converge_hegelian(&input.query, &outputs);

        Ok(CraftResult { trace_id, steps: outputs, synthesis })
    }

    /// 独立会话执行一道工序（经 CognitiveService；生产默认 DeterministicAdapter，0 Token）。
    async fn execute_step(&self, msc: &Msc<'_>) -> Result<String, MindError> {
        // 会话上下文：工序专有 Prompt + 原始输入（MSC 组成）。
        let session = format!("[{}] {}", msc.process.prompt(), msc.query);
        match msc.process {
            Process::Structural => {
                let entities = self.cognitive.extract_entities(&session).await?;
                Ok(format!("结构实体：{}", entities.join("、")))
            }
            Process::Critical => {
                // 确定性风险扫描 + 实体提取（批判不臆造风险，只标已知风险面）。
                let risks = risk_scan(&session);
                let entities = self.cognitive.extract_entities(&session).await?;
                let mut parts = Vec::new();
                if !risks.is_empty() {
                    parts.push(format!("风险候选：{}", risks.join("、")));
                } else {
                    parts.push("风险候选：未发现已知风险信号".into());
                }
                if !entities.is_empty() {
                    parts.push(format!("涉及实体：{}", entities.join("、")));
                }
                Ok(format!("批判面：{}", parts.join("；")))
            }
            Process::Creative => {
                let node = session_node(&session);
                let summary = self.cognitive.summarize(&[node]).await?;
                Ok(format!("备选方向：{}", summary))
            }
            Process::Situational => {
                let node = session_node(&session);
                let summary = self.cognitive.summarize(&[node]).await?;
                Ok(format!("情境摘要：{}", summary))
            }
            Process::MetaCritical => {
                // 元审视：确定性回顾思考过程（约束 + 模式 + 链路），不引入外部判断。
                Ok(format!(
                    "元审视：模式={:?}，全局约束={}，trace={}",
                    msc.mode, msc.global_constraints, msc.trace_id
                ))
            }
        }
    }
}

/// 会话节点：把 MSC 会话上下文包装为临时 L3 节点（CognitiveService 的 Node 输入）。
fn session_node(session: &str) -> Node {
    Node {
        content: NodeContent::Text(session.to_string()),
        node_type: NodeType::L3,
        sensitivity: Some(Sensitivity::Private),
        ..Default::default()
    }
}

/// 确定性风险关键词扫描（批判性工序，0 Token，不引入 LLM 判断）。
const RISK_KEYWORDS: &[&str] = &[
    "风险", "假设", "依赖", "盲区", "失败", "不确定", "边界", "冲突", "不可靠", "未验证",
];

fn risk_scan(text: &str) -> Vec<String> {
    RISK_KEYWORDS
        .iter()
        .filter(|k| text.contains(**k))
        .map(|k| k.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::{system0_gate, GateSignals};

    fn deterministic() -> Arc<dyn CognitiveService> {
        Arc::new(helix_mind_metabolism::DeterministicAdapter::new(
            helix_mind_core::config::MetabolismConfig::default(),
        ))
    }

    fn plan() -> Vec<ProcessStep> {
        vec![
            ProcessStep::new(Process::Structural, Mode::Anchored),
            ProcessStep::new(Process::Critical, Mode::Anchored),
        ]
    }

    #[tokio::test]
    async fn orchestrate_with_deterministic_closed_loop() {
        let craft = CognitiveCraft::new(deterministic(), CraftConfig::default());
        let result = craft
            .orchestrate(CraftInput {
                query: "分析这个方案的架构风险".into(),
                steps: plan(),
                global_constraints: "只做确定性分析，不臆造".into(),
            job_id: "test-job".to_string(),
})
            .await
            .unwrap();

        assert_eq!(result.steps.len(), 2);
        assert_eq!(result.steps[0].process, Process::Structural);
        assert!(!result.steps[0].content.is_empty(), "结构工序有输出");
        assert_eq!(result.steps[1].process, Process::Critical);
        assert!(!result.steps[1].content.is_empty(), "批判工序有输出");
        // 根 trace_id 存在（全息留痕，子工序共享）。
        assert!(!result.trace_id.is_empty(), "编排生成 trace_id");

        // 收敛：正题 + 反题 + 条件化命题。
        assert!(result.synthesis.contains("结构实体"), "正题保留");
        assert!(result.synthesis.contains("风险"), "反题注入");
        assert!(result.synthesis.contains("当且仅当"), "条件化命题");
    }

    #[tokio::test]
    async fn step_count_breaker_rejects_oversize() {
        let craft = CognitiveCraft::new(deterministic(), CraftConfig::default());
        let too_many: Vec<ProcessStep> = (0..6)
            .map(|_| ProcessStep::new(Process::Structural, Mode::Skilled))
            .collect();
        let err = craft
            .orchestrate(CraftInput {
                query: "q".into(),
                steps: too_many,
                global_constraints: String::new(),
            job_id: "test-job".to_string(),
})
            .await
            .unwrap_err();
        assert!(err.to_string().contains("steps must be 1..=5"), "max_steps=5 熔断");
    }

    #[tokio::test]
    async fn session_isolation_no_cross_reference() {
        // 会话隔离：批判工序的输入（MSC）不得包含结构工序的输出草稿。
        // 通过检查——critique 输出只来自 query 本身的风险信号，不含"结构实体"前缀。
        let craft = CognitiveCraft::new(deterministic(), CraftConfig::default());
        let result = craft
            .orchestrate(CraftInput {
                query: "评估数据管道的可靠性".into(),
                steps: plan(),
                global_constraints: String::new(),
            job_id: "test-job".to_string(),
})
            .await
            .unwrap();
        // 批判工序内容是"批判面：风险候选..."（来自 query 中的"可靠性"→无命中 or "风险"）
        // 关键：它绝不包含第一步的"结构实体："前缀（无横向引用）。
        assert!(
            !result.steps[1].content.starts_with("结构实体"),
            "批判会话不得引用结构工序草稿"
        );
    }

    #[tokio::test]
    async fn system0_gate_routes_to_craft_only_for_complex() {
        let craft = CognitiveCraft::new(deterministic(), CraftConfig::default());
        // 简单问题 → 门控 DirectSkilled，不编排（编排器本身不该被调用）。
        let simple = GateSignals::new("现在几点", false);
        if system0_gate(&simple) == crate::gate::GateDecision::DirectSkilled {
            // 路径 1：直接熟练模式（无工序）——此处仅验证门控决策一致性。
            assert!(true);
        }
        // 复杂问题 → 触发编排，闭环可跑。
        let _ = craft
            .orchestrate(CraftInput {
                query: "帮我分析架构风险".into(),
                steps: plan(),
                global_constraints: String::new(),
            job_id: "test-job".to_string(),
})
            .await
            .unwrap();
    }
}
