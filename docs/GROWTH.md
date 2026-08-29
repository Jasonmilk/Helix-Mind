# 生长日志（变异 + 阶段完成记录）

> **所属方法论**：DNA 自生长方法论 v2.0
> **对齐知识本体**：v4.1

规则：只保留最近 3 条。第 4 条写入时，最旧的归档到 `docs/archive/growth/`（已版本化，永不删除）。

---
## [2026-08-29] 完成：CI-144 v2.0 前置必填项锁定（11 项设计定案）

### 触发条件
CI-144 v2.0 协议规范升级计划通过完整审查（原 8 项前置必填 + 补丁 A/B/C 共 11 项设计锁定），DNA 方法论闭环完成（PLAN/GROWTH/ADR），准备进入 v2.0-alpha 编码阶段。

### 变更性质
- **`docs/vision/ci-144-v2.0-upgrade.md` v2.0-alpha.0**：CI-144 v2.0 协议规范升级计划完整版——PAL 扩展至 24 字节（容纳 64-bit PAH-Signature）、BIND-19 新增 16-bit Seq-Counter（防重放）、PAH 双层安全架构（64-bit 快速校验 + 512-bit 完整验证）、Replay-Enable=0 强化约束（强制 MEDIUM + 强制 PAH 验证 + 审计标记）
- **补丁 A：Replay-Enable=0 强化约束**：三条硬约束缺一不可——风险降级（强制 MEDIUM，代码语义修正：min() → 直接 RiskLevel::MEDIUM）+ 强化验证（强制 PAH 签名）+ 审计强制标记（REPLAY_DISABLED + 硬件时钟）
- **补丁 B：PAH 双层安全架构**：第一层 64-bit 快速校验（Tuck 亚毫秒级拦截决策）+ 第二层 512-bit 完整验证（载荷解密后，Helix-Mind/云端审计）；实时拦截 + 事后追责闭环
- **补丁 C：Seq-Counter 回绕与密钥轮换**：≥65534 触发密钥轮换（预留 2 帧缓冲）；KEY_ROTATION 控制帧（主密钥加密保护）；未收到轮换帧而回绕视为异常重放攻击；AtomicU16 + SeqCst 原子递增
- **6 个 ADR 占位符（ADR-0022~0027）**：Alpha 实现期间需逐个锁定的 6 个实现细节（B-1/B-2/B-3/C-1/C-2/C-3）
- **方法论闭环**：PLAN.md 任务更新 + GROWTH.md 记录 + 6 个 ADR 占位符创建

### 兼容性
纯文档阶段，零代码变更；CI-144 v1.0 契约零变更（向后兼容：BIND-19 新增 PAL-Present 标志位，v1.0 接收端忽略）；v2.0-alpha 分支待创建后开始编码。

### 验收
11/11 项设计约束全部评审通过；ci-144-v2.0-upgrade.md 版本 v2.0-alpha.0 状态 READY_FOR_ALPHA；6 个 ADR 占位符创建；PLAN ≤150 行；GROWTH ≤3 条（P7 已归档）。

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
