# 归档：P1 检索闭环

> 原载 `docs/GROWTH.md`，2026-08-28 P9 收尾时归档（GROWTH 超 3 条）。永不删除。

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
