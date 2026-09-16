# ADR-0043: 建边 —— 让日常写入路径记录关系（ADR-0042 T5）

- **状态**: Proposed（待人类批准；关键分歧见 §4 与 §8）
- **日期**: 2026-09-17
- **决策范围**: helix-mind（`proto/helix_mind.proto`、`api/`、`storage/`）/ anaphase-helix（`MemoryAdapter` trait + 写入调用点）
- **关联**: `ADR-0042`（T5 分期：本 ADR 是其前置）、`ADR-0012`（Append-Only 字段号演进）、`ADR-0020`（INTENT-7 动词映射：`WRITE_NODE` ↔ `Remember`）、`ADR-0033`（P10 召回）、INTENT-7 `WRITE_NODE`（`parent_ids`）、`ECOSYSTEM §3.1`
- **引用约定**: 不带仓名的 `ADR-XXXX` 一律指 `helix-mind:ADR-XXXX`。

---

## 1. 背景：边缺失不是配置问题，是**写入路径缺字段**

实测（2026-09-17）：

| 事实 | 证据 |
|---|---|
| 库内 `edges` **0 行**（545 nodes） | `SELECT count(*) FROM edges` |
| 全仓**唯一**的建边者是 metabolism | `metabolism/src/digest.rs:121,159`、`crystallize.rs:49,71`；此外只有 storage 的公开 `add_edge` 与其测试 |
| metabolism **从未运行** | `life_records` 0 行（ADR-0042 §4.2） |
| Anaphase 只经 `Remember` 写节点 | `anaphase-helix/src/adapters/mind.rs:109-131` |
| **`RememberRequest` 只有 `content` + `node_type`** | `proto/helix_mind.proto:42-48` —— **没有任何父引用字段** |
| **`RememberResponse.node_id` 被丢弃** | `remember_node` 返回 `()`（`mind.rs:116-131`） |

**结论**：即使 Anaphase 写一万个节点，也**永远不会产生一条边**——不是配置问题，是**协议字段缺失**。而 ADR-0042 的 D0/D1/D2/D3 四条在 `edges = 0` 下**全部不可观测**（零边时 `sum_abs == 0`，根本不传播）。**T5 是它们生效的前置条件。**

写入调用点（Anaphase 侧，全部要传血缘）：`run_cycle.rs:495`（`remember(&note)`）、`:1523`（`remember(&structured)`）、`:1525`（`remember(&self.context.reflection_notes)`）。

---

## 2. 既有约定（必须复用，不得另立）

`crystallize.rs` 是**今天唯一真正建边的地方**，其约定为：

```
source_id = 派生物（L2）        ← 新
target_id = 来源（L3）          ← 旧
relation_type = Refines
weight = 0.8
is_soft = false
```

⇒ **边方向是「派生者 → 来源」**（新 → 旧）。这一点极其重要：它决定激活的流向，选反了会让扩散沿时间倒流。

---

## 3. 决策

### D1：proto 补 `parent_ids`（Append-Only 字段号）

```proto
message RememberRequest {
  string content = 1;
  int32 node_type = 2;
  // INTENT-7 WRITE_NODE: the nodes this one derives from. Field number
  // appended per ADR-0012 (never reused, never renumbered).
  repeated string parent_ids = 3;
}
```

**理由**：INTENT-7 的 `WRITE_NODE` 已规定 `parent_ids`（`commonintents/INTENT-7/spec/INTENT-7.md:75,121`），而 `docs/spec/api.md` 已把 `WRITE_NODE` 映射到 `Remember`。所以这不是新增协议，而是**把既有协议对齐到实现**——与 ADR-0042 D0/D1 追平 `sa-core.md` 是同一类动作。

### D2：不新增存储——复用 `Node.derived_from` 与 `edges`

`Node.derived_from: Vec<Uuid>` **已存在**、已入库（`sqlite_pool.rs:67`）、已由 codec 解码、且 `crystallize.rs:35` **已在写它**。`parent_ids` 因此是它的**协议投影**：请求字段 → `derived_from` + `edges`。
**理由**：新增字段即重复发明既有实体（违「极致复用」）。**本 ADR 不引入任何新的持久化字段。**

### D3：`RememberResponse.node_id` 必须上浮

`RememberResponse` 已经有 `node_id = 1`，Mind 侧已返回，**是 adapter 把它丢了**。必须让调用方拿到它，否则「我刚写的是谁」不可引用，血缘无法在后续轮次里串起来。
**实现含义**：`MemoryAdapter::remember` / `remember_node` 的返回类型从 `Result<(), String>` 变为 `Result<Uuid, String>`（trait 级改动，波及所有实现与测试替身）。

### D4：建边由**日常写入路径**负责，不只 metabolism

Mind 侧的 `Remember` 处理器在写入节点后，按 `parent_ids` 建边。
**理由**：metabolism 是**节律性**的（Micro-Sleep / Deep Dream）。若只有它建边，则两轮消化之间写入的节点全是孤岛，检索在消化前一律退化——这正是今天的实际状态。

### D5：幂等必须**修在 storage 层**，而不是靠各调用方自觉

**实测**：`add_edge` 在 SQL 侧是幂等的，在内存侧**不是**：

- `edges` 表有 `PRIMARY KEY (source_id, target_id, relation_type)`，且 `sqlite_pool.rs::upsert_edge` 用 `ON CONFLICT(...) DO UPDATE` ⇒ **SQL 会去重**；
- 而 `MemoryTopology::add_edge` 直接 `petgraph::graph::DiGraph::add_edge` ⇒ **会再加一条平行边**。

⇒ 同一三元组调用两次的结果是 **SQL 1 行 / 内存 2 条边**。而 `sa_core_diffusion` 读的**正是内存图**，所以：

1. **事实来源（SQL）与图（内存）静默分叉**；
2. 平行边让同一条关系在 `sum_abs` 与 `a_next` 里**各计两次**，从而**静默改变扩散权重**——**但这一点是有条件的，必须说准**：若源**只有这一个目标**，平行边使 `sum_abs` 与 `a_next` **同比**放大，归一化后**完全抵消**（份额仍是 1，扩散结果逐位不变）；只有源**还有兄弟出边**时，重复边才会**抢走兄弟的份额**。⇒ 因此回归测试的图**必须包含兄弟边**，否则它测不出任何东西（本条初版的测试正是这么空转的，由变异测试抓出）。

**决策**：幂等性由 `StorageEngine::add_edge` 自己保证（重复三元组即**更新**而非**追加**），不要求每个调用方先查存在性。
**理由**：修在源头一次，优于在每个调用方重复一次检查——而且调用方检查还有 TOCTOU 缝隙。
**登记**：这是**独立于 T5 的既有缺陷**（`crystallize.rs` 的循环建边在重跑时会踩到），本 ADR 一并修，因为它正是 T5 必须依靠的那条路径。

### D6：容忍降级（对齐生态信条）

旧客户端不传 `parent_ids` ⇒ `derived_from` 为空 ⇒ **不建边，行为与今天完全一致**。不许因缺字段而报错。
**理由**：`ECOSYSTEM` 的「固定骨架 + 扩展保留 + 容忍降级」。这也是 T5 能**分两步安全落地**的依据（见 §5）。

### D7：关系类型映射表（显式、可审、单一来源）

映射**不另立**：它必须服从 `docs/spec/data-contract.md` 的关系类型表（本仓 SSOT）。

| 场景 | relation_type | is_soft | 方向 | 依据 |
|---|---|---|---|---|
| L3 情节续接（第 N 轮承接第 N−1 轮） | `TEMPORAL` | `false`（硬边） | 派生者 → 来源 | spec 表允许 `TEMPORAL` 的源/目标为 **L2, L3**，硬边、严格无环。**`REFINES` 被 spec 限定为 L2 → L2**，故它从一开始就不是 L3 续接的可选项 |
| L2 抽象自 L3（crystallize 既有） | `REFINES` | `false` | 派生者 → 来源 | 代码现状，**但见 §3.10 的第 2 条**：它与 spec 的类型约束冲突，需裁决 |

映射表**必须落在一处**（Mind 侧写入路径），不得在 Anaphase 侧重复定义。
**权重复用 0.8**：这是 crystallize 的既有取值，本 ADR **不动权重**（见 §6：同批再动权重会导致无法归因）。

### D8：环禁令**只作用于硬边**；软边成环是**联想与创造力的机制**，不得关闭

**更正（本节前一版写错了，在此更正而非静默替换）**：前一版把 INTENT-7「有向无环图」的措辞**过度泛化**到全部边，并据此把「环」整体当成要防的缺陷。**那是错的，而且越过了本仓自己的 spec。** 人类随即指出：**Helix-Mind 是「有环的 DAG」，因为加了时间维——这正是它最聪明的地方之一，否则就没有创造力（联想）了。**

**spec 早就写明了这件事**（`docs/spec/data-contract.md` 的边类型约束表）：

| 关系类型 | is_soft | 环 |
|---|---|---|
| `CORRECTS` / `REFINES` / `DOUBTS` | `false` | **严格无环** |
| `SIMILAR_TO`（联邦共享相似节点关联） | `true` | **可成环** |
| 根系网关系（社会/画像/项目） | `true` | 可成环 |

⇒ **「有环的 DAG」不是矛盾修辞，是两层结构叠加**：

- **硬边（辩证族）无环** ⇒ 给出 provenance 的**偏序**：「谁派生自谁」「谁纠正了谁」必须有向无环，否则先后与血缘失去意义；
- **软边（联想族）可成环** ⇒ 给出**回路**：两个概念互相激活、能量往复，这正是联想与创造性组合的力学基础。

**代码现状即正确**：`StorageEngine::add_edge` 的环检查**带 `if !edge.is_soft` 守卫**（`engine.rs:194-203`）—— 软边跳过检查、可成环。本决策**不需要新机制**。

**本决策要规定的是「写入路径如何应对拒绝」**：

> **节点写入与边建立必须可分离**：某个父引用若**因硬边**成环，应**跳过那条边并记录**，而不是让整个 `Remember` 请求失败。节点本身是合法的、且已被写入；用一个拓扑约束去否决一整条记忆事实，是本末倒置。

**同时更正我在同一句里给出的错误数学**：前一版称环会「静默泵送能量、抬高两者激活」。**这是错的**——行归一化下 `‖W‖₁ = 1`，且叠加项 `(1−α)·a_0`，所以 `ρ(αW) ≤ α < 1` 意味着环上的往复**逐轮衰减**（总质量上界是种子数 k，不会膨胀）。**衰减的往复正是想要的行为**：激活在关联回路里回响一下然后消退，而不是发散。把「回响」说成「膨胀」，是我把特征当成了缺陷。

**连带影响（让 ADR-0042 D0 的分量变大）**：既然回响/联想是创造力的力学基础，那么 D0 移除的那个**绝对闸门 0.8** 就不只是「深度被锁死」——它在第 1 轮把**所有非种子节点清零**，等于**从根本上取消了任何回响的可能性**。换句话说：在 D0 之前，即使软边联想回路被建出来，能量也**无法在回路上循环**。**D0 修复的是联想的前提条件，而不只是检索深度。**

**人类的补充（2026-09-17）**：「有环是**有风险**的，必须有**衰减**，否则可能会**死循环**。我应该已经简单处理过了。」⇒ 已逐层验证，并写成回归测试 `soft_edge_cycle_reverberates_but_never_runs_away`：

| 层 | 机制 | 强度 |
|---|---|---|
| 1. **结构性** | 迭代是 `for _ in 0..max_iterations` 的**有界**循环，并在收敛时提前 `break` | **最强**：不存在无界循环，死循环在结构上不可能 |
| 2. **数值** | 行归一化给 `‖W‖₁ = 1`，叠加 `(1−α)·a_0` 使 `ρ(αW) ≤ α ≤ 0.95 < 1`；总质量上界恒为**种子数 k** | 环上往复**震荡后收敛**，不放大 |
| 3. **实证** | α=0.7 的二环收敛到 `(0.588, 0.412)`，和恒为 1.0；预算 1→64 总质量始终 ≤ 1 | 已成回归测试；抽掉 α 阻尼 ⇒ 该测试变红（变异已验证） |

**但「简单处理过」的那一处需要更正**：看起来像是防环衰减的 `decay_factor`（`topology.rs:359-360`，`is_soft ⇒ base_weight × decay_factor`）**并不衰减幅值**。原因是它作用在**行归一化之前**，而归一化是 `w / Σ|w|`——分子分母**同比缩放**，所以任何一个源的出边份额恒和为 `1`：

```
decay=1.0: B=0.4737 C=0.5263  合计=1.0000
decay=0.8: B=0.4186 C=0.5814  合计=1.0000
decay=0.5: B=0.3103 C=0.6897  合计=1.0000
decay=0.0: B=0.0000 C=1.0000  合计=1.0000   ← 软边被归零（= Skilled 屏蔽软边）
```

⇒ `decay_factor` 只在**同一节点的多条出边之间重新分配份额**，**不改变总能量**。因此：

- 「靠软边衰减防止环失控」这个直觉**在本实现下不成立**——真正兜住失控的是上表第 1、2 层；
- 若希望「每跳都按几何级数衰减幅值」，需要的是**在归一化之后**再乘一个逐跳 `<1` 因子（或直接依赖 `α<1`）；那是**未实现的语义**，此处登记而不擅改。

已由 `soft_edge_decay_only_redistributes_it_never_attenuates` 钉死：只有一条软出边时（无兄弟可分配），`decay_factor` 从 1.0 改到 0.5 **结果逐位相同**。

**另一条相关事实**：`Skilled` 的 `decay_factor = 0` ⇒ **软边在主力检索模式下完全惰性**（Stage 1 恒为 Skilled），而联想回路只存在于软边上（`skilled_mode_disables_soft_edges_entirely` 钉住）。⇒ 创造力被**设计性地**放在 Anchor/Imagination，这也解释了「为什么创造力不会自然出现」。

### D9：验收（首次可观测效果）

1. `edges > 0`，且**至少一条链**可用 `sa_core_diffusion` 走出 ≥2 跳（合成图测试之外的第一条真实证据）；
2. 白盒 `activation_vector` **非退化**：不再是「只有种子、能量全 0.5」，`Node.heat` 与当轮 `activation` 不再相等；
3. ADR-0042 的 D0/D1/D2/D3 四条**首次可观测**：`max_hops` 有影响、α 随 heliotropism 变、过时节点被硬门控拦住；
4. 召回**重新验收**（ADR-0042 §7 已声明这笔账）。

**数据说明**：人类已裁定「现在是测试，原始数据不重要」⇒ 验收可在**新库**上进行，不必保全现有 545 个节点，也允许重建。

### D10：T5 只建得起「血缘」那一半；「联想」那一半**当前无人建**

由 D8 的两层结构直接推出的一条**范围限定**，必须写明以免高估 T5 的效果：

| 半边 | 关系类型 | is_soft | 谁建 | 状态 |
|---|---|---|---|---|
| **血缘**（provenance 偏序） | `TEMPORAL`（本期）/ `REFINES` | 硬 | **本 ADR 的写入路径** | 本期交付 |
| **联想**（回路 / 回响 / 创造力） | `SIMILAR_TO`（0.9）/ 根系网关系 | 软 | spec 说 `SIMILAR_TO` 由 **Deep Dream 自行决定融合** | **无实现，无人建** |

⇒ **T5 落地后，扩散会沿血缘走（可多跳、可观测），但联想回路仍然为空**——所以「创造力」不会随 T5 出现。让它出现需要**第二个建边者**（软边：相似度阈值写入，或 Deep Dream 的融合判定），那是**独立决策、另立 ADR**。

**本 ADR 同时拒绝**「顺手把 `SIMILAR_TO` 也建了」：那需要相似度度量、阈值标定与「谁有权判定相似」三件未决之事，塞进本批就会把「一次改一个变量」的纪律破掉。

### D11：本次取证发现的**三处 spec ↔ 代码不一致**（登记，不擅改）

1. **`CORRECTS` 的「矩阵权重 = -1.0」列已过期。** `data-contract.md` 的**同一行**里，「检索行为」列写「**屏蔽旧节点输出**」——那是**门控**语义；「矩阵权重」列写 `-1.0`——那是**旧实现**。**spec 的正文与它自己的数字本来就不一致**，ADR-0042 D3 选了**正文**。⇒ 该列需同步为「—（由确定性门控替代，ADR-0042 D3）」。
2. **`REFINES` 的类型约束被 crystallize 违反。** spec 约束表写 `CORRECTS / REFINES / DOUBTS` 均为 **L2 → L2**；而 `crystallize.rs:49,71` 建的是 **L2 → L3**（L2 原则精炼 L3 情节）。两者必有一错：放宽 spec，还是改 crystallize 的关系类型。
3. **`DOUBTS` 的「降低置信度」与 `+0.3` 正权重方向相反。** spec 写「新节点对旧节点存疑 → **降低旧节点置信度**」，而矩阵权重是 **+0.3 正值**；在 `source=新, target=旧` 的既定方向下，能量从怀疑者流向被怀疑者，是**抬高**而非降低。这是 **spec 自身的正文与数字不一致**，不是我能单方面改的。

**处置**：三条**只登记、不擅改**。它们的共同点是「spec 正文与 spec 数字不一致」，因此**任何单方面改动都会把不一致搬到另一处**——必须由人类裁决以哪一侧为准。

---

## 4. 待人类决策的关键分歧

1. ~~**D7 的 `Temporal` vs 沿用 `Refines`**~~ → **已由 spec 判死，取 `TEMPORAL`，无悬念**。`data-contract.md` 的关系类型表允许 `TEMPORAL` 的源/目标为 **L2, L3**（硬边、无环），而把 `REFINES` **限定为 L2 → L2**。所以对 L3 情节续接，`REFINES` **从一开始就不是合法选项**——这不是口味问题。`TEMPORAL` 与 crystallize 既有的 `REFINES` 权重同为 0.8，本期不动权重。
2. ~~`parent_ids` 是否就是 `derived_from` 的同一物~~ → **已由原文证据解决**：INTENT-7 §3.2 `WRITE_NODE` 的 `params` 是 `{node_id, type, parent_ids, content}`，且该节说明是「追加写入**有向无环图**」⇒ `parent_ids` = DAG 父节点 = `Node.derived_from`。
3. **T6 是否并入**：Anaphase 改读 `activation_vector`（而非持久化 `Node.heat`）是让效果**可见**的最后一步。并入则一次交付完整体验；不并入则 T5 的效果只在白盒协议里可见。
4. **【新】D11 的三处 spec 不一致以哪一侧为准**（`CORRECTS` 的矩阵权重列 / `REFINES` 的 L2→L2 约束 vs crystallize 的 L2→L3 / `DOUBTS` 的正文 vs `+0.3`）。三条**都不会阻塞 T5**，但都是「spec 正文与 spec 数字打架」，必须由人类定谁为准。
5. **【新】软边（联想 / 创造力）何时建**。见 D10：T5 只交付血缘半边，`SIMILAR_TO` 那半边**当前无人建**，所以「创造力」不会随 T5 出现。需要第二个建边者（相似度阈值写入或 Deep Dream 融合判定），**另立 ADR**。

---

## 5. 分期（每步可独立回滚）

| 步 | 内容 | 不改 Anaphase？ | 可观测变化 |
|---|---|---|---|
| **T5a** | proto 补 `parent_ids`；Mind 侧 `Remember` 建 `derived_from` + 边（幂等）；`node_id` 继续返回 | ✅ 是（`parent_ids` 为空 ⇒ D6 降级 ⇒ 行为不变） | 无（但为 T5b 铺好路） |
| **T5b** | `MemoryAdapter` 签名：`remember`/`remember_node` 收 `parent_ids` 且返回 `Uuid`；三个调用点传血缘 | ❌ | **`edges` 由 0 变正** |
| **T5c** | T6：Anaphase 白盒改读 `activation_vector` | ❌ | 面板/白盒首次显示当轮激活 |
| **T5d** | 召回重新验收 + D4（`min_k_core`，此时 `k_core` 才非零、可标定） | — | — |

**T5a 先行的理由**：它是**行为中性**的一半，可以独立验证（合成图 + 集成测试：传 `parent_ids` 建边、不传不建边、重复传不重复建边），把风险最大的协议改动与「血缘从哪来」的业务改动**分开**。

---

## 6. 后果与风险

**正面**：扩散首次真正可用；ADR-0042 的四条决策首次可观测；白盒 `activation_vector` 从「字段已通、数据恒空」变为真实数据；`derived_from` 从「写了但没人读」变为可导航的血缘。

**代价与风险**：
- 边一出现，**召回必然改变**（ADR-0042 §7 已声明）。且 `Refines` 权重 0.8 与 `Temporal` 权重 0.8 是**未标定**的值——它们是既有约定，不是标定结果。**登记为已知风险**：不要在 T5 同批再动权重，否则无法归因。
- `MemoryAdapter` 返回类型变化是 trait 级改动，波及 `adapters/mod.rs` 的两个实现（`:72` 空实现、`:114` 委托实现）与所有测试替身。
- 重复边会让归一化**静默**改变权重（D5 的理由），必须有测试钉住。
- 边方向（新→旧）若写反，扩散会**沿时间倒流**且不会报错。测试必须断言方向。

---

## 7. 替代方案

| 方案 | 否决理由 |
|---|---|
| 只跑 metabolism 让它建边 | 建的是 `Refines`（L2←L3）与 `Corrects`，**没有情节续接边**；且它是节律性的，两轮之间写入的节点仍是孤岛 |
| 在 Anaphase 侧直接调 `add_edge` | Anaphase 无直接数据库访问（DNA 铁律 4），且会绕过 Mind 的写入语义与幂等 |
| 用 `derived_from` 的 JSON 当边用（不建 `edges`） | `sa_core_diffusion` 只读 petgraph 的内存图，不读 `derived_from`；等于让扩散继续看不见任何关系 |
| 新增 `parent_ids` 持久化列 | 重复发明 `derived_from`（违「极致复用」） |
| 本期就加 `relation` 请求字段（D7 第三行） | 在没有第二个真实关系语义之前，那是**假精度**（与 ADR-0042 D2 同类） |

---

## 8. 待人类批准

- 状态：批准后转 **Accepted**，方可改 `proto` 与 `anaphase-helix` 的 trait 签名。
- 需要答复：§4 的三个分歧（`Temporal` vs `Refines`、`parent_ids` 语义边界、T6 是否并入）。
- 允许的降级：若希望更小步，可只批 **T5a**（行为中性的一半），把 T5b 留到下一批。

---

*本 ADR 的全部背景事实均以源码与实测为准（行号随附）。它不引入任何新的持久化字段，也不引入新的协议动词——它做的是让既有的 `WRITE_NODE`/`parent_ids`/`derived_from` 三者第一次真正连起来。*
