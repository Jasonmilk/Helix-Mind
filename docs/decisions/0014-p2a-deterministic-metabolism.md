- **决策日期**：2026-08-28
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：审查③ O10（`find_similar_node` 度量需明确）/ S7（LLM 越界）→ P2a 计划（2026-08-28 起草，待审查）
- **状态**：Active（2026-08-28 审查通过，P2 编码完成）

# ADR-0014: P2a 确定性代谢——find_similar_node 度量 + 失调消解基础

## 状态

Active（2026-08-28 审查通过，P2 编码完成）

## 问题

P2a 两个 M 任务建立在空实现上：
1. `StorageEngine::find_similar_node`（engine.rs）返回 `Ok(None)`——digest 的去重合并永远不触发。
2. `get_unresolved_dissonance` 返回空 Vec + `Digest::resolve_dissonance` 是 TODO——失调消解流程从未运行。
3. 度量方式未定：审查③ O10 建议"FTS5 matchinfo + 编辑距离，SQLite 层"，需冻结为决策。

## 决策

### 1. `find_similar_node` 度量：FTS5 bm25 候选 + 编辑距离阈值（SQLite 层，非 Rust 全表扫描）
- **候选阶段（SQLite 层，复用 P1 FTS5 trigram 索引）**：用待合并节点 content 的 FTS5 MATCH 检索 top-1 候选（排除自身 id，`-bm25` 排序，与 ADR-0013 一致）。
- **阈值阶段（Rust 层，复用既有 `Digest::compute_similarity`）**：Levenshtein 相似度 ≥ `merge_similarity_threshold` 才合并。两个阶段职责分离：FTS5 提供"候选召回"，Levenshtein 提供"精确阈值裁决"。
- **中文局限（诚实标注）**：Levenshtein 按字节计算，中文语义近义召回有限——这是既有限制，**不引入 embedding**（避免新依赖与模型下载，P4+ 可换）。FTS5 trigram 已保证中文子串召回，候选质量足够。
- **空 content / 非 Text / 长度 <3**：返回 `None`（无候选）。
- **API**：`find_similar_node(&self, node: &Node) -> Result<Option<Node>, MindError>`（签名不变，digest 调用方零改动）。

### 2. M-05 失调消解：不新增实体，复用 `Conflicts` edge + 现有 `Corrects` 契约
- **数据来源 = `edges` 表中的 `RelationType::Conflicts`**（schema 已存在，`corrected_by`/`Corrects` 契约已冻结）——**不新建 dissonance 表**（如无必要勿增实体）。
- `get_unresolved_dissonance(older_than)`：查询 Conflicts edge 对应的节点对，排除已被 `corrected_by` 解析的，返回待消解对。
- `resolve_dissonance(node_a, node_b)`：
  - 确定性部分：若节点为 `NodeContent::Structured`，从结构化字段提取 `LogicAssertion`，用既有 `SymbolicSolver::check_clash` 仲裁；确认直接矛盾 → 创建 `Corrects` edge（weight=-1.0，契约验证已冻结）+ 更新 `corrected_by`。
  - **Text 节点（无结构化断言）**：P2a 无 LLM 无法确定性提取断言——**不臆测**，创建 `Doubts`/跳过并记录待 P2b `CognitiveService.translate_assertions` 接入（LLM 翻译器角色，symbolic.rs 注释已预留）。
- **诚实边界**：P2a 只落地"可确定性处理"的部分；LLM 翻译路径明确推迟到 P2b/c，不伪装完成。

### 3. 硬性验收（防假绿）
- `find_similar_node` 对相似内容返回候选、对不相关内容返回 None（非空断言测试）
- digest 合并闭环：写入两相似 L3 → 触发 digest → 合并为 1 条 + SimilarTo edge
- 失调闭环：写入 Conflicts edge 对 → `get_unresolved_dissonance` 返回 → `resolve_dissonance` 产生 Corrects edge

## 权衡

| 优势 | 代价 |
|:---|:---|
| 复用 P1 FTS5 索引，零新依赖、零模型下载 | Levenshtein 中文语义近义有限（诚实标注，P4+ 可换） |
| 失调消解零新表（复用 Conflicts/Corrects 契约） | Text 节点断言提取推迟到 P2b，P2a 覆盖范围收窄 |
| 候选召回与阈值裁决职责分离，测试可单点验证 | 两步阈值参数（FTS5 limit 与 merge_similarity_threshold）需调优 |

## 回滚阈值

- 若 FTS5 候选质量不足（相似节点召回不到）→ 提高候选 limit 或改用 matchinfo 加权重排，不动契约。
- 若 Conflicts edge 数据来源不足 → 可在 P4+ 引入 dissonance 队列（届时代谢期已就绪，选项 (b) 成立），现不新增。

## 关联

- 前置：ADR-0013（FTS5 检索）、ADR-0011（相态）、ADR-0012（Append-Only）
- 后续：P2b ADR-0017（CognitiveService，LLM 翻译器角色接入 M-05 Text 路径）、P2c（crystallize/ner 接 CognitiveService）
- F3 关系：`access_count` 是代谢输入之一，`bump_access_counts`（P2a 前置修复）已就绪
