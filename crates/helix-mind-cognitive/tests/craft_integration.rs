//! P8 认知工艺 Phase 2 编排器集成测试。
//!
//! 验证编排闭环（DeterministicAdapter 0 Token）+ 熔断（超时 / Token 预算 / 步骤数）。

use std::sync::Arc;
use std::time::Duration;

use helix_mind_cognitive::{
    CognitiveCraft, CraftConfig, CraftInput, CraftResult, Mode, Process, ProcessStep,
};
use helix_mind_core::error::MindError;
use helix_mind_core::graph::Node;
use helix_mind_core::symbolic::LogicAssertion;
use helix_mind_metabolism::CognitiveService;

fn plan3() -> Vec<ProcessStep> {
    vec![
        ProcessStep::new(Process::Structural, Mode::Anchored),
        ProcessStep::new(Process::Critical, Mode::Anchored),
        ProcessStep::new(Process::Creative, Mode::Imaginative),
    ]
}

fn deterministic() -> Arc<dyn CognitiveService> {
    Arc::new(helix_mind_metabolism::DeterministicAdapter::new(
        helix_mind_core::config::MetabolismConfig::default(),
    ))
}

/// 慢执行器：模拟独立会话耗时，用于超时熔断测试。
struct SlowAdapter {
    delay: Duration,
}

#[async_trait::async_trait]
impl CognitiveService for SlowAdapter {
    async fn summarize(&self, _nodes: &[Node]) -> Result<String, MindError> {
        tokio::time::sleep(self.delay).await;
        Ok("slow summary".into())
    }
    async fn extract_entities(&self, _text: &str) -> Result<Vec<String>, MindError> {
        tokio::time::sleep(self.delay).await;
        Ok(vec!["slow".into()])
    }
    async fn translate_assertions(&self, _node: &Node) -> Result<Vec<LogicAssertion>, MindError> {
        tokio::time::sleep(self.delay).await;
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn full_closed_loop_with_deterministic_three_steps() {
    let craft = CognitiveCraft::new(deterministic(), CraftConfig::default());
    let result: CraftResult = craft
        .orchestrate(CraftInput {
            query: "帮我分析分布式系统的架构可靠性风险".into(),
            steps: plan3(),
            global_constraints: "确定性分析，不引入外部调用".into(),
        job_id: "test-job".to_string(),
})
        .await
        .unwrap();

    assert_eq!(result.steps.len(), 3);
    // 收敛合成：正题（结构/创造）+ 反题（批判）→ 条件化命题。
    assert!(result.synthesis.contains("当且仅当"), "条件化合题");
}

#[tokio::test]
async fn timeout_breaker_aborts_slow_step() {
    // 慢执行器 100ms 超 10ms 配置 → 第一步即超时熔断。
    let slow: Arc<dyn CognitiveService> = Arc::new(SlowAdapter {
        delay: Duration::from_millis(100),
    });
    let craft = CognitiveCraft::new(
        slow,
        CraftConfig {
            step_timeout: Duration::from_millis(10),
            ..Default::default()
        },
    );
    let err = craft
        .orchestrate(CraftInput {
            query: "分析架构风险".into(),
            steps: plan3(),
            global_constraints: String::new(),
        job_id: "test-job".to_string(),
})
        .await
        .unwrap_err();
    assert!(err.to_string().contains("timed out"), "超时熔断生效: {}", err);
}

#[tokio::test]
async fn token_budget_breaker_exhausts_energy() {
    let craft = CognitiveCraft::new(
        deterministic(),
        CraftConfig {
            total_token_budget: 1, // 预算过小（<10）→ 直接 EnergyExhausted
            ..Default::default()
        },
    );
    let err = craft
        .orchestrate(CraftInput {
            query: "分析".into(),
            steps: plan3(),
            global_constraints: String::new(),
        job_id: "test-job".to_string(),
})
        .await
        .unwrap_err();
    assert!(
        matches!(err, MindError::EnergyExhausted),
        "预算熔断返回 EnergyExhausted: {}",
        err
    );
}

#[tokio::test]
async fn empty_steps_rejected() {
    let craft = CognitiveCraft::new(deterministic(), CraftConfig::default());
    let err = craft
        .orchestrate(CraftInput {
            query: "q".into(),
            steps: vec![],
            global_constraints: String::new(),
        job_id: "test-job".to_string(),
})
        .await
        .unwrap_err();
    assert!(err.to_string().contains("steps must be 1..=5"), "空工序拒绝");
}
