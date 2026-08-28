# 生长日志（变异 + 阶段完成记录）

> **所属方法论**：DNA 自生长方法论 v2.0
> **对齐知识本体**：v4.1

规则：只保留最近 3 条。第 4 条写入时，最旧的归档到 `docs/archive/growth/`（已版本化，永不删除）。

---
## [2026-08-28] 完成：P7 生态文档同步 + 认知工艺 Phase 1

### 触发条件
P0-P6 全部完成（PLAN 阶段表修正滞后）；用户批准认知工艺 v1.1（吸收 Gemini 审查补丁 7 项 + 豆包审查 B1/B2/B3 三项修正）。

### 变更性质
- **ADR-0021（Active）**：认知工艺决策——①执行边界：Mind=编排（0 Token 纯逻辑），执行经 ADR-0017 CognitiveService 注入（B1）；②System 0 门控：规则 + FTS5 bm25 相关度阈值，非 embedding（B2，零向量依赖）；③与 ADR-0010 划界：预算路由=外部前置定扫描范围，System 0=Mind 内定思考深度（B3）
- **`docs/spec/cognitive-craft.md` v1.1**：认知工艺完整规范（工序×模式、MSC 最小充分上下文、黑格尔辩证确定性合题、Bounded ε-Greedy+EMA 抗僵化、Token 熔断+30s 超时）
- **`docs/ecosystem-alignment.md`**：ECOSYSTEM v1.6 对齐核对——Mind↔Anaphase/Callosum 契约逐项核对无断链；CI-144 动词映射 5/5 一致（FETCH/WRITE_NODE/TENTACLE/FINISH/CANCEL vs INTENT-7 spec §3）；新增 budget_tier/traceparent 契约字段建议
- **文档生态同步**：PLAN 阶段表 P3-P7 全部 ✅ + P8 预览（认知工艺 Phase 2）；GROWTH 超 3 条归档最旧记录（v4.1 变异 → docs/archive/growth/，已版本化）

### 兼容性
纯文档阶段，零代码变更；硬冻结契约零变更；cargo test 全绿（回归确认 70 通过）

### 验收
ADR-0021 Active 且含 B1/B2/B3；cognitive-craft.md 落位；对齐核对 5/5 动词一致、零断链；PLAN ≤150 行

### 状态
🧬 待提交（用户确认后 push）
---

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

## [2026-08-28] 完成：P9 认知工艺 Phase 3 — 完整引擎（价值/突变/复盘/bm25）

### 触发条件
P8 编排器闭环验证通过（DeterministicAdapter 0 Token），用户授权 Phase 3 编码（价值评估、自适应突变、睡眠复盘、bm25 门控增强）。

### 变更性质
- **价值评估**（`value.rs`）：`ValueAssessor` 确定性评估（长度档 + 复杂关键词提升）→ `ValueGrade{Low,Medium,High}` + 建议思考深度（Skilled/Anchored/Imaginative）。0 Token，禁用 LLM 自我评分（R3，信用由他证）
- **自适应突变**（`mutation.rs`）：`AdaptiveMutation`——Bounded ε-Greedy（突变率钳制 3%-20%）+ EMA 成功率平滑（α=0.3）；`record_outcome` 确定性信号更新：成功→降探索（利用熟练路径），失败→升探索（探索新工序/模式）
- **睡眠复盘**（`review.rs`）：`SleepReview`——非熟练模式检视（强制锚定/想象力，防假复盘 R4）：检视输出实体覆盖度相对旧路径增量 ≥2 → Stale（旧路径僵化，建议降权）；否则 Viable。接入点：Deep Dream 代谢窗口（不阻塞主循环）
- **bm25 门控增强**（`gate.rs`）：`system0_gate_enhanced`——用户显式标签 > bm25 简单相似度（≥0.7 → DirectSkilled）> 价值评估（Low→Direct）> 规则兜底。相似度信号由上层经 `storage::fts::fts_search` 计算注入（cognitive 保持纯逻辑，零向量依赖）

### 兼容性
- 全为 cognitive crate 内新模块，零既有代码改动；硬冻结契约零变更；无新依赖

### 验收
`cargo test --workspace` 全绿（cognitive 单元 12→24 + 集成 4，workspace 总 98，0 warning）｜ 价值：短简 Low/带关键词 Medium/长+关键词 High ✅ ｜ 突变：界内钳制、成功降探索失败升探索、EMA 平滑 ✅ ｜ 复盘：增量≥2 Stale / 否则 Viable ✅ ｜ 门控：bm25 高相似优先、显式标签最高优先 ✅

### 状态
🧬 待提交（用户确认后 push）
