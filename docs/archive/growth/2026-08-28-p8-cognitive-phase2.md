## [2026-08-28] 完成：P8 认知工艺 Phase 2 — 编排器最小原型

### 触发条件
P7 认知工艺 Phase 1 定稿（ADR-0021 Active）后，用户授权 Phase 2 编码（编排器最小原型，DeterministicAdapter 闭环验证编排逻辑）。

### 变更性质
- **新 crate `helix-mind-cognitive`**（认知工艺编排器，Mind 核心域）
- **System 0 门控**（`gate.rs`，B2）：规则匹配（触发关键词 + 长度阈值）+ 用户意图标签（显式深入跳过门控），0 Token；FTS5 bm25 相关度留 Phase 3（GateSignal 接口预留）
- **编排器**（`craft.rs`，B1）：`CognitiveCraft`——熔断（步骤数 1..=5 / 单步超时 30s / Token 预算 EnergyExhausted）→ 独立会话执行（每步仅注入 MSC=原始输入+工序Prompt+全局约束，不横向引用草稿 → 会话隔离）→ 根 trace_id 生成（全息留痕，子工序共享）→ 黑格尔确定性收敛
- **工序 × 模式**：5 工序（结构性/批判性/创造性/情境/元批判）× 3 模式（熟练/锚定/想象力）；工序执行经 ADR-0017 CognitiveService 注入（生产默认 DeterministicAdapter，0 Token；Remote 仅 debug_direct）
- **收敛**（`converge.rs`，R1）：确定性合题——正题（非批判输出）+ 反题（批判输出）→ 条件化命题"当且仅当风险被排除"
- **执行边界锁死**：编排器零 LLM 直连，全部经 CognitiveService trait（Z2 延续）

### 兼容性
- 新 crate，零既有代码改动；硬冻结契约零变更
- 依赖 helix-mind-metabolism（CognitiveService trait 消费方，无循环依赖）

### 验收
`cargo test --workspace` 全绿（新增 cognitive 单元 12 + 集成 4，workspace 总 86，0 warning）｜ 门控：简单跳过/复杂触发/显式标签优先 ✅ ｜ 编排闭环：结构+批判+创造三工序收敛出条件化命题 ✅ ｜ 熔断：步骤超限 / 超时(慢执行器) / 预算耗尽 三路全断 ✅ ｜ 会话隔离：批判会话不引用结构草稿 ✅

### 状态
🧬 待提交（用户确认后 push）
---
