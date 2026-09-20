- **决策日期**：2026-09-20
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：K-059（模式权威）/ VISION 原则 3（意志优先于框架）/ P0①
- **状态**：采纳

# ADR-0044: 决策先于仪器 —— 报告的模式必须是执行的那个

## 状态

采纳（2026-09-20，P0①）

## 问题

`RetrievalEngine::query` 先检索、后协商：`stage_local_dominant` 在 `negotiate_mode`
之前运行，因此这一层拿不到模式，只能硬编码参数。原注释自己写明了原因：

> `Skilled` is the *semantically correct* parameter set here: this stage is the
> focused local pass, and `negotiate_mode` has not run yet (it is called after
> this stage).

由此产生三个后果，一层比一层重：

1. **三个模式参数里两个从执行路径不可达。** `SaCoreParams::for_mode` 有三个分支
   （Skilled α 0.5 / 软边关；Anchor α 0.7 / 软边 ×0.8；Imagination α 0.9 / 软边 ×0.95），
   而查询路径只到达第一支。`alpha_anchor`、`alpha_imagination`、`decay_imagination`
   成了**没有消费者的配置项**——即本仓反复出现的「声明了但没有入口」。
2. **报告的模式是假的。** `effective_mode: Imagination` 返回给身体，而 stage 1 实际用
   α=0.5、软边关闭扩散。**报告与执行不一致，而报告的那个是谎。** 证轨若建在这之上，
   记录的将是**一个标签**，而不是**一次执行**。
3. **退化结果会伪造一个模式。** 起始节点为空时**硬编码返回 `Anchor`**——请求 Skilled
   却被告知 Anchor。

**⇒ 身体侧的镜像缺陷**：`allow_imagination` 由身体自己的建议推出
（`suggested_mode == Imagination`），即**请求者向自己签发许可**，这个门永远拒绝不了
（违反 VISION 原则 3：身体可建议，不决策）。而且它**反转了**：`suggested_mode` 按查询
**长度**回退，于是短查询说「探索」反而拿不到想象，而长得像技术问题的查询却自动获得。

## 决策

### 1. 决策先于仪器

`negotiate_mode` 提到检索之前，其结果一次算定，供全部返回路径使用。

**这是纯重构**：`negotiate_mode` 读的是函数顶端取的 `current_impasse`（上一循环的遗留值），
所以上提**逐字不改任何既有输出**。可验证，不是"应该没问题"。

### 2. 模式选择仪器，不是一个标签

stage 1 按协商出的模式**分发到各自的 traverse**
（`skilled_retrieve` / `anchor_retrieve` / `imagination_retrieve`）。

**依据（实测，非推断）**：`skilled_traverse` 与 `anchor_traverse` **除注释外逐字相同**，
都调 `sa_core_diffusion(.., None)`；`imagination_traverse` 是唯一有实质差异的——它按温度
放松相对门 τ。**因此模式的全部语义就是三个数**：

| 模式 | α | 软边衰减 | 相对门 |
|:---|:---|:---|:---|
| Skilled | 0.5 | `SOFT_EDGES_DISABLED`（0.0） | 基准 |
| Anchor（参照） | 0.7 | `soft_edge_decay_factor`（0.8） | 基准 |
| Imagination（想象） | 0.9 | `decay_imagination`（0.95） | 基准 × (1−temperature) |

而这三个数**早已在 `for_mode` 里编码完毕**，缺的从来不是参数，是**入口**。

### 3. 退化结果如实报告

起始节点为空时，返回**协商出的**模式与其理由，并注明"未找到起点"。
**一个退化的结果必须描述真实的决策，不得发明一个。**

### 4. 授权来自人类，身体只转达

`allow_imagination` 由**人类自己的措辞**决定（`exploratory_intent`：命中
`explore_keywords` 即视为要求探索）。该词表是**单一来源**——`derive_budget_tier`
早已信任同一份。

**⇒ 由此把「门」修回它该有的方向**：身体仍然**建议**（`suggested_mode` 按复杂度与长度
推导），Mind 仍然**决策**（可推翻，并给出理由）。不同的是**许可不再由建议自行签发**，
于是这个门第一次真的能拒绝。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 报告的模式与执行一致，证轨记录的是真实执行 | Anchor / Imagination 查询的首轮扩散变宽，节点数与延迟上升 |
| 三个模式参数从死参数变为活参数 | 原先只有 stage 4/5 才用的宽半径，现在 stage 1 就可能使用 |
| 承诺（suggested）与许可（allow_imagination）分离 | 长的技术性问题不再自动获得想象——需要人类在措辞中表达 |
| 请求构造抽为纯函数，接线可测 | 四个策略函数进入公开 API |

**⇒ 关于最后一项**：`build_query_request` 从 `query()` 里抽出，是因为 adapter 需要活的
gRPC 通道，**内联字面量只能靠读、不能靠跑**——这正是那句自我签发能活下来的原因。
断言因此移入 `tests/`（`src/` 受行数棘轮约束，`tests/` 不受），代价是四个函数公开。

## 回滚阈值

- 若首轮扩散变宽导致检索质量下降（无关节点进入结果）：先降 `max_nodes_per_query`，
  再考虑收窄 Anchor / Imagination 的软边衰减，**不改契约字段**。
- 若短探索查询的误判率高（`explore_keywords` 假阳性）：扩词表或改为需要显式标记，
  **不回退为"由身体自行签发"**——那个方向的失败模式是静默的，且无法被证轨发现。

## 变异证据

把 `for_mode(mode, ..)` 改回 `for_mode(CognitiveMode::Skilled, ..)`，
`the_reported_mode_is_the_mode_that_diffuses` **必须红**。实测红，报
`Anchor ... must reach through; got [<仅种子节点>]`——正是旧行为。

**杠杆是软边**：`decay_factor` 乘在软边权重上，Skilled 的 `SOFT_EDGES_DISABLED = 0.0`
把它归零，因此**仅靠软边可达的节点对 Skilled 不可达**。

**⇒ 独立佐证**：存储层既有测试 `skilled_mode_disables_soft_edges_entirely` 单独立住了
"Skilled 关软边"这一半。**两半合起来才闭环**——一半在参数，一半在接线。

身体侧的变异证据：把 `allow_imagination` 改回 `suggested_mode == Imagination`，
`imagination_grant_comes_from_the_human_not_from_the_bodys_suggestion` **两个方向都红**。

## 未决

**梯子没有按模式封顶。** `negotiate_mode` 说 `Survival mode: only Skilled available`，
而 stage 4/5 照样用 Anchor / Imagination 参数——**Mind 的决定被自己的梯子违反**。
修法是把梯子按模式封顶（Skilled 不升级），但那是关于"生存是否该省"的语义决定，
须单独裁决，未决。

## 关联

- 前置：ADR-0042（SA-Core 参数来源与门控——`for_mode` 是**唯一**的模式参数来源，
  本 ADR 补上它在查询路径上的入口）
- 同族：K-059（模式权威）、K-045(b)（身体自我授权）、VISION 原则 3（意志优先于框架）
- 后续：证轨记录 `suggested` / `effective` / `negotiation` 三元（已接，见 `MindProvenance`）
