# ADR-0033: P10 召回增强——分词检索 + 种子保底 + 注入语义化

- **状态**: Accepted（2026-09-07 端到端验收通过）
- **日期**: 2026-09-07
- **决策范围**: Helix-Mind（检索层 / SA-Core 扩散 / 传输层）/ Anaphase（注入折叠）
- **关联**: ADR-0031（P10 认知工艺集成）、ADR-0021（认知工艺）、ADR-0016（确定性优先）、ADR-0002（零硬编码）、ADR-0023（O-5 注入预算）

## 1. 背景与问题

写入侧已验证可用（L3 写入 + Reflection remember），但读取侧真实失效：
"我叫什么名字？"零召回——Helix 答"不知道"。

物理事实核验（2026-09-07 实测）：
- `FtsExtractor` 生产默认：整句 sanitize → `escape_fts`（包双引号短语）+ FTS5 trigram MATCH——中文问句必不命中；库中 0 条边 → start_ids 空 → 直接返回空 + "No start nodes found"
- SA-Core 扩散衰减 `val = alpha*a_next + (1-alpha)*a_0`：孤立（无邻边）start 节点每轮能量 = `(1-0.5)*1.0 = 0.5`，而默认 `weight_threshold = 0.8` → **查询命中节点每轮被清零**
- gRPC `content_json` 用 `serde_json::to_string(&node.content)` 包裹 JSON + 既有节点全是 L2 提炼形态（"结论（正题）…"）→ 注入内容淹没原文
- L3 记忆写的是 `"Cycle completed. p_death: …, impasse: …"` 流程摘要而非对话内容 → 库中没有任何"人类说过什么"
- Anaphase `memory_inject_chars` 字段 `#[serde(default)]` 对 usize 反序列化为 **0**（协议默认 800 只在 `impl Default`，Deserialize 路径完全没用上）→ 检索结果从未进 prompt

## 2. 决策

### D1: 分词级召回作为 v1（非语义级）
`tokenize_query`：ascii 整词保 + 小写 + 过滤停用词；Han 连续段按**最早位置、同位置最长**的 stopword 子串切分；`bigram_candidates`（≥4 字 Han token 生成双字窗口）兜底；单字 token 丢弃（LIKE 噪声）。token 逐个 `fts_search`(≥3字)/`like_search`(<3字) 累积（HashSet 去重，max_results 上限）→ bigram 兜底 → 整句短语兜底（行为不退化）。
语义 embedding 未加载（models/ 空）——**分词级是诚实可用的 v1，不伪装语义**。

### D2: 种子即硬证据——扩散只延展种子，绝不熄灭种子
`a_current[j] = if val < weight_threshold && a_0[j] == 0.0 { 0.0 } else { val }`——查询命中（确定的证据）不被扩散衰减吞掉。回归测试 `isolated_seed_survives_default_threshold`。

### D3: LIKE 排序——短内容优先（具体经历优先于冗长抽象）
`ORDER BY length(content) ASC, phase_state ASC`——简洁的具体记忆（"User said: …"）先注入，冗长抽象（"结论（正题）…"）后置。极致节能：少 tokens 多信息。

### D4: 传输原文透传
`NodeContent::Text(t)` 直接返回 `t.clone()`；非 Text 形状保持 JSON。物理事实优先：原文不被包裹/淹没。

### D5: L3 经历化
Reflection note 改为 `"User said: {}\nCycle completed. …"`——L3 记"这一轮经历了什么"，不是账本行。

### D6: 注入折叠语义化
- fold 时剥离 `\nCycle` 账本尾行（provenance 不是经历，不进 prompt）
- 注入标签 `[memory: Helix's past experiences — true history, answer from them]`
- `memory_inject_chars` serde 默认改 `default_memory_inject_chars`（单一常量 `DEFAULT_MEMORY_INJECT_CHARS = 800`，Default impl 与 Deserialize 同源）

## 3. 备选与拒绝

| 备选 | 拒绝理由 |
|---|---|
| 语义 embedding 召回 | models/ 空、零额外依赖约束（P10 tokenizer 刻意无新 crate）；v1 诚实分词级，语义级留待有 embedding 时 |
| 提高 weight_threshold 或衰减 alpha | 治标不治本：孤立命中仍会被能量耗尽，阈值调参是猜 |
| 注入全量节点不折叠 | 上下文预算失控（25 轮有界测试约束）；极致节能要求截断+标记 |
| 保留 L3 流程摘要 | 库中无"经历"可检索，召回永远空 |

## 4. 后果

**正面**: 中文自然语言问句真实召回（"我叫什么名字？"→"User said: 我叫Jason…"→"你叫Jason。"）；检索证据不被阈值吞；注入即白盒可审计（memory_nodes 预览日志）；日志每轮一行（审计价值，非膨胀）。

**代价**: 分词级召回对同义改写（"我的称呼是？"）不命中——接受为 v1 边界，语义级挂 v2（有 embedding 时）；L3 写"经历"后旧节点（流程摘要形态）仍在库中——靠 D3 排序保证新经历优先注入，不清理（保护历史数据）。

## 5. 验证
- Mind workspace 118 passed 0 failed；Anaphase 228 passed 0 failed
- 端到端：`"我叫什么名字？"` → `[MemoryRetrieval] 14 memory node(s)`（User said 排前）→ LLM 回复"你叫Jason。"（重复实测稳定）
- 回归：`p10_natural_language_question_recalls_name` / `isolated_seed_survives_default_threshold` / `fold_strips_bookkeeping_tail_keeps_experience`
