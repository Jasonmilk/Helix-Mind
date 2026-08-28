# 归档：P2 代谢闭环（a/b/c）

> 原载 `docs/GROWTH.md`，2026-08-28 P8 收尾时归档（GROWTH 超 3 条）。永不删除。

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
