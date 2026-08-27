- **决策日期**：2026-08-28
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：审查③ S7（Mind 不直连 LLM 越界）/ P2 拆分 a/b/c（CognitiveService 端口依赖）→ P2b 计划（2026-08-28 起草，待审查）
- **状态**：Active（2026-08-28 审查通过，P2 编码完成）

# ADR-0017: P2b CognitiveService 端口 + 三适配器（LLM 安顿）

> 编号说明：ADR-0015 已预留给 P4.5 WAL 设计，故本决策取 0017（0016 已用于 L2 共享语义）。

## 状态

Active（2026-08-28 审查通过，P2 编码完成）

## 问题

审查③ F5/Z2 确认：`crystallize::summarize_with_llm` 与 `ner::remote_extract` 用 `reqwest` **无条件直连** LLM/NER 网关，绕过了 `llm_mode = disabled`（生产锁死）门控——"Mind 不直连 LLM"铁律的物理保障缺失。P2b 需建立唯一 LLM 访问端口；P2c 将越界代码接入该端口。

## 决策

### 1. `CognitiveService` 端口（metabolism 内新模块 `cognitive.rs`）
- **位置**：metabolism crate 内（当前唯一消费者是 crystallize/ner；未来可拆独立 crate，不预设）。不新增 crate。
- **能力**（最小，覆盖当前越界点 + M-05 翻译角色）：
  | 方法 | 语义 | 消费者 |
  |:---|:---|:---|
  | `summarize(&[Node]) -> String` | L3 观察 → 经验原则摘要 | crystallize（P2c） |
  | `extract_entities(&str) -> Vec<String>` | 文本 → 实体列表 | ner（P2c） |
  | `translate_assertions(&Node) -> Vec<LogicAssertion>` | 节点 → 结构化逻辑断言 | digest M-05 Text 路径（P2c 后） |

### 2. 三适配器 + llm_mode 门控（能力未就绪 = 功能不存在）
| 适配器 | 行为 | 门控 |
|:---|:---|:---|
| `DeterministicAdapter`（**生产默认**） | 无 LLM：summarize = 关键词/模板启发式摘要；extract_entities = 既有 local 分词；translate_assertions = 仅从 `NodeContent::Structured` 提取 | `llm_mode = disabled` 时唯一可用 |
| `RemoteAdapter` | HTTP 网关（复用现有 reqwest 逻辑，移入端口） | 仅 `llm_mode = debug_direct` 可构造/调用，否则返回 `Metabolism` 错误（物理禁止出网） |
| `FakeAdapter` | 确定性固定输出（测试用） | 测试注入 |

- **`llm_mode` 三态复核**：`disabled`（生产，Deterministic 唯一）、`debug_direct`（测试/调试，Remote 可用）、非法值 → 视为 `disabled`（fail-closed）。
- **零越界承诺**：`cognitive.rs` 是 metabolism 内唯一持有 `reqwest`/HTTP 出网的地方；crystallize/ner 删除直接 reqwest 调用（P2c）。

### 3. 接口形态（Rust trait，Append-Only 友好）
- `CognitiveService: Send + Sync` trait；`MetabolismEngine` 持有 `Arc<dyn CognitiveService>`，由 `MetabolismConfig.llm_mode` 在构造时选择适配器（显式注入，可测）。

## 权衡

| 优势 | 代价 |
|:---|:---|
| LLM 出网收敛为单点，铁律有物理保障 | 新增一个 trait + 三适配器（但替换两处越界，净复杂度下降） |
| Deterministic 生产默认 = 零外部依赖，符合极致节能 | 启发式摘要质量低于 LLM（诚实标注，debug_direct 可切换） |
| 适配器可测（Fake 确定性） | 门控需在构造与调用两层防护 |

## 回滚阈值

- 若 Deterministic 摘要质量不可接受 → 默认降级为"跳过结晶 + 记录待 LLM"（不臆造摘要），仍是确定性与诚实。
- 若未来 Anaphase/其它组件也需 LLM → 从 metabolism 抽出为独立 crate（端口不动，仅搬适配器）。

## 关联

- 前置：Z2（llm_mode 门控裁决）、P2a（ADR-0014）
- 后续：P2c（crystallize/ner 接入 CognitiveService，删除直接 reqwest）、M-05 Text 断言翻译
