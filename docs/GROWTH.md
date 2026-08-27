# 生长日志（变异 + 阶段完成记录）

> **所属方法论**：DNA 自生长方法论 v2.0
> **对齐知识本体**：v4.1

规则：只保留最近 3 条。第 4 条写入时，最旧的归档到 `docs/archive/growth/`（已版本化，永不删除）。

---

## [2026-08-28] 完成：P2 代谢闭环（a/b/c）

### 触发条件
审查③ O10（`find_similar_node` 度量）/ S7（LLM 越界）→ P2 计划（ADR-0014/0017）审查通过，用户授权编码。

### 变更性质
- **ADR 冻结**：ADR-0014（P2a 确定性代谢）、ADR-0017（P2b CognitiveService 端口）
- **数据契约（Append-Only）**：`RelationType::Conflicts` 追加（失调声明）；edges 表加 `created_at`（`migrate_edges_created_at` 幂等迁移）；`MetabolismConfig.dissonance_window_hours`（冷却窗口可配置，消魔法数 24）
- **P2a 确定性代谢**：`find_similar_node` 真实实现（FTS5 候选 top-1 + 跳过 recessive 防环）；`get_unresolved_dissonance`（Conflicts 边查询 + 排除已 `corrected_by`）；`update_corrected_by` 原语；`Digest::resolve_dissonance` 用 SymbolicSolver 仲裁（Corrects edge -1.0 + corrected_by trace）；Text-only 失调诚实保留（P2b 边界）；`Digest::run` 返回真实 merged 数（修 report 硬编码 0 的虚假数据）
- **P2b CognitiveService 端口**（`cognitive.rs`）：trait（summarize/extract_entities/translate_assertions）+ 三适配器（Deterministic 生产默认零 LLM / Remote debug_direct 门控 / Fake 测试）+ 工厂 fail-closed（未知 llm_mode 视为 disabled）
- **P2c 越界清零**：crystallize/ner 删除直接 reqwest，改走 cognitive 端口；metabolism 唯一 HTTP 出网点收敛到 `cognitive.rs`
- **M-04/M-05 集成测试**（3 个）：merge 闭环（双胞胎合并 → 1 recessive + SimilarTo edge）/ 结构化失调闭环（Corrects + corrected_by + 不再出现在 unresolved）/ Text-only 失调不臆造 Corrects

### 兼容性
- 硬冻结契约无变更；`RelationType::Conflicts` 为 Append-Only 枚举追加（ADR-0012）
- edges created_at 对旧库幂等迁移（默认 1970 回填，仅影响新边写入）
- `Digest::run` 返回类型 `Result<u64>`（内部语义，api 消费者零改动）

### 验收
`cargo test --workspace` 全绿（新增 metabolism 单测 15 + 集成 3）｜ 0 warning ｜ `grep reqwest` 仅 cognitive.rs ｜ Z2（llm_mode 门控）物理生效

### 状态
🧬 待提交（用户确认后 push）

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

## [2026-08-27] 变异：知识本体 v4.1 — 相态范式入根 + VISION.md 建立

### 触发条件
2026-07-29 新共识（认知相态的河流与催化器）产生后未文档化；VISION.md 初版方案被指出版本漂移（沿用 v3.4 海马体范式）与相态概念缺位；L2 共享语义出现"主动/自动"歧义。

### 变更性质
- **知识本体**：v4.0 → v4.1（硬冻结契约无变更，核心定位深化：记忆中枢 → 认知相态的河流与催化器）
- **新增概念**：四层相态（气/液/胶/晶）、认知预算前置路由、Append-Only Schema Evolution
- **决策冻结**：ADR-0010（认知预算前置路由）、ADR-0011（相态模型与主体依赖轴）、ADR-0012（Append-Only Schema Evolution）、ADR-0016（L2 自动共享语义，宪法级锁定）
- **根索引建立**：VISION.md v1.1（10 条原子原则 + 叙事/决策映射 + 宪法澄清）
- **宪法不变**：`ECOSYSTEM.md §7` / `docs/spec/federation.md` / `docs/spec/data-contract.md` 的"公开 DAG 自动共享"措辞保留；ADR-0016 作为执行澄清而非修正

### 兼容性
- 所有硬冻结契约无变更（外部组件 Anaphase rs / CI-144 无需修改）
- 数据契约追加 `phase_state`/`subject_dependency`/`concentration`/`tension`（Append-Only 字段 16-19，SQLite 幂等迁移 + 一次性物化回填）
- `HelixQueryResult` 预留 `reserved 13`（activation_vector，P4 M-10 落地）

### 状态
🧬 已合并（提交 1289507）
