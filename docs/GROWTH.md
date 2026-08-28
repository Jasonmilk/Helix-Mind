# 生长日志（变异 + 阶段完成记录）

> **所属方法论**：DNA 自生长方法论 v2.0
> **对齐知识本体**：v4.1

规则：只保留最近 3 条。第 4 条写入时，最旧的归档到 `docs/archive/growth/`（已版本化，永不删除）。

---
## [2026-08-28] 完成：P1 检索闭环

### 触发条件
审查③ S1（主干断路）与 R2（中文 FTS5 召回为零）后，P1 三裁决（2026-08-28 用户定案）授权落地 M-01/02/03。

### 变更性质
- **ADR-0013**：FTS5 trigram 起始节点检索（索引载体 / 异步队列 / 注入防御 / 相态加权 / 生产默认接线）
- **storage 索引层**（`fts.rs`，新建）：`nodes_fts` trigram 外部内容表 + `rebuild_fts` 启动全量投影 + `run_fts_worker` 异步批量冲刷（`FTS_BATCH_SIZE=100` / 防抖 100ms / Flush 屏障）+ `fts_search`（`-bm25`×相态权重）+ `fts_like_search`（1-2 字符 LIKE 兜底）+ `audit_sanitized`
- **StorageEngine 集成**：`fts_tx` 发送端 + `write_node` 入队 Upsert + `flush_fts_index` 屏障 + `ensure_schema` 建 FTS 表
- **retrieval**（`fts_extractor.rs`，新建）：`FtsExtractor` 只读消费者 + `sanitize_query` 白名单清洗 + `escape_fts` 双引号转义；`RetrievalEngine::new` 默认注入 FtsExtractor
- **M-03 测试**（5 个）：中文召回 / 1-2 字符 LIKE fallback / 相态加权（Crystal 优先）/ 注入清洗+审计+索引存活 / 端到端 FTS 默认
- **关键修正**：FTS5 `bm25()` 为**负数**，先 `-bm25` 转正再乘相态权重，否则 Crystal 高权重被 DESC 反向拉低

### 兼容性
- 契约零变更（复用 ADR-0011 `phase_state` 列 + ADR-0012 Append-Only）；`get_nodes_by_phase` 预留供 P2 预算路由
- 测试基建复用 FakeAdapter（`with_extractor` 注入不变）；生产默认行为变化：`RetrievalEngine::new` 从 EmptyExtractor → FtsExtractor

### 验收
`cargo test --workspace` 全绿（cli 5 / core 8 / retrieval lib 3 + retrieval_test 2 + fts_extractor_test 5 / storage 4 = 27，0 warning）｜ FTS5 中文 trigram 端到端召回 ✅ ｜ 注入攻击面收敛（清洗+转义+审计）✅ ｜ 查询写放大消除（F3，见下）

### 状态
🧬 已合并（P1 代码 + F3 前置修复，提交待用户确认后推送）

### 未决→已裁决（2026-08-28）
PLAN §1.5 验收项「无查询写放大」（F3）：裁决为选项 (a) 单事务批量更新，作为 **P2a 前置修复**已落地——`StorageEngine::bump_access_counts`（单事务、SQL 级原子自增，去读-改-写竞争、去逐节点 fsync）+ `update_access_counts` 批量调用 + 测试。选项 (b) 内存计数 + 代谢期回刷归 **P4+**（等 Micro-Sleep 就绪）。

---
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
