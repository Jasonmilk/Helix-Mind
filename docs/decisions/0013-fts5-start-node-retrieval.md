- **决策日期**：2026-08-28
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：审查③ S1（主干断路）/ R2（中文 FTS5 召回为零）→ P1 三裁决（2026-08-28 用户定案）
- **状态**：采纳

# ADR-0013: FTS5 trigram 起始节点检索（P1 M-01 / M-02 / M-03）

## 状态

采纳（2026-08-28，P1 检索闭环入口）

## 问题

审查③ S1：`extract_start_nodes` 为空实现，SA-Core 完全不可达，既有测试为假阴性。审查③ R2：SQLite FTS5 默认 `unicode61` tokenizer 对中文连续文本视为单 token，子串召回为零。P1 必须落地"检索闭环"——从 schema 到索引到检索算法的全栈真实实现。

## 决策

### 1. 索引载体：FTS5 trigram 影子表（外部内容表）
- 建 `nodes_fts` 虚拟表：`node_id UNINDEXED, content, phase_state`，`tokenize='trigram'`，content 外键引用 `nodes(id)` 并 `content=''`（external content 语义：索引与源分离，源为真理源）。
- 零依赖：bundled SQLite 已编译 `ENABLE_FTS5`（探针测试 `fts5_trigram_is_available_in_bundled_sqlite` 已验证）。拒绝 unicode61（中文召回为零）、拒绝 jieba（增加词典依赖，过度工程）。
- 重建：`rebuild_fts`（DELETE + INSERT SELECT 从 `nodes` 全量投影），启动时执行；启动探针缺失 FTS5 即 panic 阻断（Z6 契约，Fossil SCM 晶体实践，不做回退）。

### 2. 索引同步：异步写入队列 + 批量合并（P1 核心）
- `write_node` 成功后向 `mpsc::UnboundedSender<FtsCommand>` 入队 `Upsert`，**不阻塞节点写事务**。
- worker（`run_fts_worker`，`tokio::select!`）三种触发：`FTS_BATCH_SIZE=100` 批量 / `FTS_DEBOUNCE_MS=100` 防抖 / `Flush` 屏障（oneshot ack，测试与 read-your-writes 用）。
- `StorageEngine::flush_fts_index()`：发送 Flush 屏障并 await ack，确定性完成。
- 租户级可配置延迟 / LIKE 一致性兜底 = P1 后续优化（本轮不做）。

### 3. MATCH 注入防御（无妥协空间）
三层纵深：CI-144（传输/身份：谁在说、怎么传）+ Tuck（审计/脱敏：记了什么、路由到哪）+ Mind（应用层：内容是否无害）。Mind 层：
- **白名单清洗**：只保留中文/英文/数字/空白，其余字符移除（`sanitize_query`）。
- **FTS5 转义**：清洗后结果用双引号包裹、内部引号加倍（`escape_fts`），作为**字面短语**传入 MATCH——用户输入永不作为 FTS5 语法进入。
- **审计记录**：每次清洗写入 `audit_log`（`fts_input_sanitized`），用于攻击模式分析。
- 查询路由：≥3 字符走 FTS5，1-2 字符走 `LIKE %..%`（ESCAPE `\`）兜底。

### 4. 相态加权排序（M-01 排序层）
- `ORDER BY (-bm25(nodes_fts)) * CASE phase_state WHEN 'Crystal' THEN 1.5 WHEN 'Liquid' THEN 1.0 ELSE 0.5 END DESC`。
- **关键修正**：FTS5 `bm25()` 返回**负数**（越负越相关），必须先 `-bm25` 转正再乘正权重，否则 Crystal 的高权重会被 DESC 推向更低排名（实现期发现，已修正）。
- 预算路由（ADR-0010 `get_nodes_by_phase` 范围过滤）= P2 落地，P1 只做排序加权 + 接口预留。

### 5. 生产默认接线
- `RetrievalEngine::new` 默认注入 `FtsExtractor`（持 `SqlitePool`，clone 自 StorageEngine）；测试经 `with_extractor` 覆盖（FakeAdapter / EmptyExtractor）。
- `FtsExtractor` 为只读消费者，不持有写句柄；审计写入走 storage 的 `audit_log`。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 中文 3 字以上召回完美（trigram 子串匹配） | 索引异步存在 1-5s 级延迟窗口（可接受，兜底 LIKE） |
| 零依赖、零外部服务 | 影子表投影需自行维护（增删改入队 + 批量冲刷） |
| 注入攻击面收敛为字面短语 | 白名单会剥离部分语义字符（如标点），短查询依赖 LIKE 兜底 |
| 查询路径零额外 fsync（索引异步） | worker 生命周期随 StorageEngine，需在测试中显式 flush |

## 回滚阈值

- 若 FTS5 在目标部署环境缺失：启动探针 panic 阻断（Z6 契约），不静默回退——这是"能力未就绪 = 功能不存在"的物理表达。
- 若异步索引延迟在真实负载下不可接受：放宽 `FTS_BATCH_SIZE` / `FTS_DEBOUNCE_MS`，或引入外部内容表 + 显式同步钩子；LIKE 兜底已就位保证正确性不回退。
- 若 bm25 排序质量不达预期：可切换 matchinfo + 自定义重排（P2a M-04 方向），不改契约字段。

## 关联

- 前置：ADR-0011（相态模型，phase_state 列）、ADR-0012（Append-Only 契约）、ADR-0010（预算路由，P2 消费）
- 后续：P2a M-04 `find_similar_node`（FTS5 matchinfo + 编辑距离）、P2b M-09 CognitiveService
- 未决（已关闭 2026-08-28）：PLAN §1.5 验收项「无查询写放大」（F3）裁决为选项 (a) 单事务批量更新，作为 P2a 前置修复已落地（`bump_access_counts`，单事务原子自增）；选项 (b) 内存计数 + 代谢期回刷归 P4+。
