# ADR-0031: P10 认知工艺生态深度集成（触发链路 / L1 策略持久化 / Deep Dream 挂载）

- **状态**: Accepted（2026-09-06 用户确认）
- **日期**: 2026-09-06
- **决策范围**: Helix-Mind（认知工艺 crate + API crate + 代谢 crate）/ Anaphase（触发链路）
- **关联**: ADR-0021（认知工艺）、ADR-0022（O-4 Mind-craft 参数）、ADR-0016（确定性优先）、ADR-0010（EnergyContext / budget_tier）、ADR-0006（领域 WAL）

## 1. 背景与问题

P9 已完成认知工艺 Phase 3（编排器 + 价值评估 + 自适应突变 + 睡眠复盘 + bm25 门控），
但认知工艺是**独立 crate，未接入生态**：

- Q1: Anaphase 如何触发认知工艺？（helix_query 现在是纯检索端点，无编排端点）
- Q2: 认知工艺决策（synthesis / 价值分级 / 突变状态）如何固化为可复用策略？（当前一次性）
- Q3: 睡眠复盘如何深度挂载进代谢循环？（SleepReview 已有实现，未接线）

物理事实核验（2026-09-06）：
- `helix_query`（layer3.rs）→ `retrieval.query`，返回 nodes/edges/activation_vector/suggested_actions——无编排能力
- `CognitiveCraft::orchestrate` 已可运行（5 工序 + MSC 隔离 + 黑格尔收敛 + 熔断），生产默认 0 Token（DeterministicAdapter）
- `SleepReview` / `AdaptiveMutation` / `ValueAssessor` / `system0_gate` 已实现（确定性，0 Token）
- 代谢 crate 已有 `trigger_by_event` / `trigger_digest` / `trigger_crystallize` / `trigger_hibernate`
- 存量问题：craft.rs `trace_id = uuid::new_v4()` 违反"确定性派生 id"；mutation.rs 硬编码 `0.03/0.20/0.3`、review.rs 硬编码 `STALE_ENTITY_DELTA=2` 违反 0 硬编码（DNA 原则 11）

## 2. 决策

### D1: P10a 触发形态——新 RPC `helix_craft`，不扩展 helix_query

| 方案 | 裁定 |
|---|---|
| 扩展 helix_query 加 craft 字段 | ❌ 检索与编排耦合：查询语义被污染，Anaphase 侧判断复杂化 |
| **新 RPC `helix_craft`** | ✅ 极致解耦：helix_query 保持检索语义（向后兼容，Anaphase 195 测试不破）；编排独立端点，参数独立演进 |

- 触发时机：**Anaphase 侧按需驱动**（无心跳，Iron Law #13：Mind 不主动推）。Anaphase 在 run_cycle 需要认知工艺时调用 helix_craft。
- 请求：`HelixCraftRequest { query, steps, mode, energy_context, autonomy_level, traceparent }`
- 响应：`HelixCraftResult { trace_id, steps, synthesis, value_grade, tokens_consumed, traceparent }`

### D2: P10b L1 策略持久化——复用现有 storage，不新建

- 认知工艺产物落 **DAG L1（策略层）**：`CraftResult.synthesis` → L1 策略节点
- provenance = `craft#{trace_id}`（确定性派生，非 UUID）
- `ValueAssessor` 分级写入策略节点元数据（`value_grade` 字段）
- 检索时：helix_query 命中策略节点 → 策略复用（零重算）
- **极致复用**：走现有 storage crate（领域 WAL 投影器复用），不新建存储

### D3: P10c Deep Dream 挂载——代谢事件驱动，复用现有 review/mutation

- 代谢 `trigger_hibernate` 事件 → `SleepReview.review_path`（非熟练模式检视）→ 裁决 Viable/Stale
- Stale → `AdaptiveMutation` 应用（Bounded ε-Greedy + EMA）→ 更新策略节点权重
- 全链路确定性 0 Token（复用 P9 已实现，不引入 LLM 自我评分——R3）
- 挂载点：代谢 crate 事件处理内调用 cognitive crate（依赖方向：代谢 → 认知，单向）

### D4: 零硬编码收口（P10 前置，DNA 原则 11 / ADR-0002）

| 字面量 | 现状 | 改为 |
|---|---|---|
| `trace_id = uuid::new_v4()` | craft.rs | 确定性派生：`craft#{job_id}#{index}`（调用方传入 job_id） |
| `MIN_RATE 0.03 / MAX_RATE 0.20 / DEFAULT_ALPHA 0.3` | mutation.rs | `CraftConfig` 扩展字段（默认值 = ADR-0021 协议默认，来源可配） |
| `STALE_ENTITY_DELTA = 2` | review.rs | `CraftConfig.stale_entity_delta`（默认 2，来源可配） |

### D5: 边界（禁止清单）

- ❌ 认知工艺不直连 LLM 编排（B1 保持：Mind=编排，执行经 CognitiveService 注入）
- ❌ 不做自动路由（M3 边界：触发与否由 Anaphase 判断）
- ❌ 不引入新存储 / 新 crate（极致复用 + 如无必要勿增实体）
- ❌ 不破坏 helix_query 现有语义（向后兼容）

## 3. 任务分解（按依赖序）

| # | 任务 | 验收 |
|---|---|---|
| P10-0 | 零硬编码收口：trace_id 确定性化 + mutation/review 阈值进配置 | 无 uuid 依赖、无硬编码字面量；认知 4 测试 + 新增全绿 |
| P10a-T1 | proto + server：新增 `helix_craft` RPC（请求/响应/转换） | gRPC 编译通过；单测覆盖 |
| P10a-T2 | helix_craft 接线 CognitiveCraft（CraftConfig 注入） | 真实编排调用返回 synthesis |
| P10a-T3 | Anaphase mind adapter 新增 craft 方法 + run_cycle 触发点（按需） | Anaphase→Mind 真实 gRPC 编排闭环 |
| P10b-T1 | 策略节点 schema（L1 + provenance `craft#`） | 类型落地 |
| P10b-T2 | synthesis → L1 持久化（storage 复用） | 落盘 + 回读一致 |
| P10b-T3 | ValueAssessor 分级写元数据 + 检索策略复用 | helix_query 命中策略节点 |
| P10c-T1 | trigger_hibernate → SleepReview 挂载 | 事件驱动复盘执行 |
| P10c-T2 | AdaptiveMutation 状态持久化 + Stale 应用 | 权重更新落盘 |
| P10-T | 测试 + 文档：集成测试全绿；ADR-0031 + PLAN v6.2 + GROWTH + README + ECOSYSTEM | 全绿 + 文档对齐 |

## 4. 后果

**正面**：
- 认知工艺从"独立可运行"变为"生态可触发"——Anaphase 编排闭环打通（P10a）
- 认知工艺决策复利化：每次编排的 synthesis 固化为 L1 策略，检索可复用（P10b）
- 睡眠复盘进入代谢循环：抗僵化成为生态机制而非孤立模块（P10c）
- 0 硬编码 + 确定性 id：哲学自洽（P10-0）

**负面/代价**：
- 新 RPC = proto 变更（向后兼容：新增端点不影响现有端点）
- L1 策略节点增长需代谢衰减管理（Heat 衰减已存在，复用）

**风险与对策**：
- 策略节点膨胀 → 复用现有 Heat 衰减 + 归档机制（不新建）
- 编排死循环 → CraftConfig 熔断已存在（max_steps/step_timeout/token_budget）

## 5. 一句话总结

> P10 让认知工艺从"器官"成为"循环"——Anaphase 按需触发编排（helix_craft），
> 编排产物固化为 L1 策略（复用存储），睡眠复盘深度挂载代谢（复用 review/mutation）。
> 全部复用既有实现，零新 crate，0 硬编码收口。
