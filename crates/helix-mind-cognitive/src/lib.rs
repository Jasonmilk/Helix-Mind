//! 认知工艺（Cognitive Craft）——Helix 的"如何思考"系统（ADR-0021，Phase 2 最小原型）。
//!
//! - `gate`：System 0 启发式门控（B2：规则 + 用户意图标签，0 Token）。
//! - `craft`：编排器（工序编排 + 独立会话隔离 + 熔断）。
//! - `converge`：黑格尔辩证确定性收敛（R1）。
//!
//! 执行边界（B1）：Mind=编排（0 Token 纯逻辑），执行经 ADR-0017 `CognitiveService` 注入
//! （生产默认 DeterministicAdapter，0 Token；Remote 仅 debug_direct 可插拔）。

pub mod converge;
pub mod craft;
pub mod gate;
pub mod mutation;
pub mod review;
pub mod value;

pub use converge::converge_hegelian;
pub use craft::{CognitiveCraft, CraftConfig, CraftInput, CraftResult, Mode, Process, ProcessStep};
pub use gate::{system0_gate, system0_gate_enhanced, GateDecision, GateSignals};
pub use mutation::{AdaptiveMutation, MutationConfig};
pub use review::{ReviewConfig, ReviewVerdict, SleepReview};
pub use value::{ValueAssessor, ValueGrade};
