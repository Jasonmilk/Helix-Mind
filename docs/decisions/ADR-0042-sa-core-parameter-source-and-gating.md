# ADR-0042: SA-Core 的参数来源、迭代预算与抑制门控（附一份外部建议书的审查结论）

- **状态**: Proposed（待人类批准）
- **日期**: 2026-09-17
- **决策范围**: Helix-Mind（`storage/topology.rs`、`storage/engine.rs`、`retrieval/`、`core/config.rs`、README §3.1）/ Anaphase（白盒消费侧）
- **关联**: ADR-0002（零硬编码）、ADR-0012（Append-Only Schema 演进）、ADR-0016（确定性优先）、ADR-0033（P10 召回增强 / 种子保底）、`anaphase:ADR-0041`（周期身份）
- **引用约定**: 不带仓名的 `ADR-XXXX` 一律指 `helix-mind:ADR-XXXX`。

---

## 1. 背景

一份外部建议书（由另一模型撰写）审查了本仓 README §3.1 的 SA-Core 公式，提出 5 项缺陷（1 项 P0 稀释、1 项 P0 clamp 矛盾、3 项 P1/P2）。**该建议书明确声明未能取得源码**，其诊断完全基于 README。

人类要求严肃审查该建议书。**本 ADR 的审查以源码为准**（证据行号随附），并在建议书之外补出它看不到的断层。

---

## 2. 审查结论：建议书说对的部分（逐条附代码证据）

| 建议书论点 | 代码证据 | 判定 |
|---|---|---|
| 更新式 `a_{t+1} = α·W·a_t + (1−α)·a_0` | `topology.rs:364` `alpha * a_next[j] + (1.0 - alpha) * a_0[j]` | ✅ 完全一致 |
| `W` 按**绝对值**行和归一化（`∑\|W_ij\|=1`） | `:344 sum_abs += w.abs()` → `:352 w / sum_abs` | ✅ |
| `CORRECTS = −1.0` 为抑制边 | `get_raw_weight`：`Corrects => -1.0`、`Doubts => 0.3`、否则 `edge.weight`；`is_soft` 再乘 `decay_factor` | ✅ |
| **P0-1：稀释使抑制失效**（源点有 1 条 CORRECTS 与正权和 `S` ⇒ 归一化分母 `1+S` ⇒ `S=0→−1.0`、`S=1→−0.5`、`S=9→−0.1`） | 同上归一化 | ✅ **算术精确**；"越 hub 越压不住"的反直觉推论成立 |
| **P0-2：clamp 声明与 `energy: −0.45` 示例打架** | `:365-369` 阈值门控 | ✅ 方向对（范围需修正，见 §3.3） |
| P1-3：`a_0` 未归一化 ⇒ 阈值跨 query 不可比 | 种子置 `1.0`，`Σ = 起点数` | ✅ 成立 |
| P1-4：缺 hub 惩罚 | 机制**存在但被关闭**，见 §4C | ✅ 方向对 |
| P2-5：缺迭代预算 / 收敛判据 | `:328 for _ in 0..max_hops` | ✅ 成立，**且比建议书所述更严重**（§3.1） |

**结论：建议书最重的那条判断——「用软权重去做本该硬门控的事」——成立，且值得改。**

---

## 3. 审查结论：建议书需修正的部分

### 3.1 收敛性论证在运行时是 moot 的：代码跑不到不动点

建议书按"α=0.8 到 1e-5 约需 50 次迭代"推理收敛行为。**而代码的迭代次数就是 `max_hops`**：

```rust
// topology.rs:328
for _ in 0..max_hops      // 迭代次数 = 图跳数；两个不同的量被合并成一个
```

实际取值：`skilled/anchor = beam_width.max(3)`、`imagination = 5`（`:429` / `:459` / `:485`）。
⇒ **3–5 次迭代 = 截断 PPR，不是收敛 PPR。** `ρ(αW) ≤ α < 1` 的唯一不动点**从未被逼近**。

**修正**：建议书花力气排除的"发散"本就不是这条流水线的行为；真正的问题是**迭代预算与跳数混用**。
**但不影响 P0-1**：行归一化的稀释**每次迭代都发生**，与是否收敛无关。

### 3.2 α 的取值错位

建议书按 README 的 0.8／0.2 分析，**代码是硬编码的三值**：

| 模式 | `alpha` | `decay_factor` | `max_hops` |
|---|---|---|---|
| skilled（`:430`；decay 为 `:439` 的**裸 `0.0`**） | **0.5** | 0.0 | `beam_width.max(3)` |
| anchor（`:460` / `:461`） | **0.7** | 0.8 | `beam.max(3)` |
| imagination（`:486` / `:487`） | **0.9** | 0.95 | 5 |

⇒ "α≤0.8"不成立；`imagination` 的 0.9 更接近 1（"α↑收敛慢"的方向仍对，程度更甚）。

附带一处细节：`anchor` / `imagination` 都有具名 `let decay_factor`，而 **`skilled` 把 `0.0` 裸传给调用点**（`:439`）—— 同一个参数三处三种写法，且裸字面量使意图不可见。D1/D2 收口时应一并统一。

### 3.3 clamp 的范围要收窄

代码**不是无条件 clamp**，而是**阈值门控 + 种子豁免**：

```rust
// topology.rs:364-369
let val = alpha * a_next[j] + (1.0 - alpha) * a_0[j];
a_current[j] = if val < weight_threshold && a_0[j] == 0.0 { 0.0 } else { val };
```

⇒ **种子贡献保持线性**（`a_0[j] != 0` 时永不被归零），非线性只作用于非种子。
⇒ **`−0.45` 与 clamp 并不矛盾**：只有**种子**可能为负（非种子必被归零）。**错的是 README 的措辞**（"automatically clamping their final energy to `0.0`" 过宽），示例本身可达。
⇒ 注释自证他们踩过此坑：「an isolated node is zeroed at `(1 - alpha) * 1.0 = 0.5 < threshold` and **recall silently dies**」——**他们用"种子豁免"解了**。

### 3.4 建议书的修法 #3（`a_0` 归一化）**不采纳**

归一化后种子下限从 `(1−α)` 变为 `(1−α)/|seeds|`——**更小**，孤立种子**更容易**被阈值杀掉，恰好恶化代码注释所记的那个失效模式。**保留种子豁免**更直接。

### 3.5 "α 叫 Decay 是反的"——对象是文档，不是代码

代码中 `alpha`（damping）与 `decay_factor`（软边衰减，仅作用于 `is_soft` 边）是**两个分开的参数、各自命名贴切**。建议书批的是 README 措辞（§5 D6）。

---

## 4. 源码中的断层（建议书未见，且比 P0 更根本）

### 4.1 `heliotropism` 是死字段 —— README 承诺的"动态 α"从未发生过

README `:157`：`α (Decay / Heliotropism Factor)`：**动态计算**，基于 `EnergyContext.heliotropism`（Optimistic 0.8 / Defensive 0.2）。

全仓 `.heliotropism` **只有两处读取**：

```
core/src/graph.rs:313   if self.heliotropism < -1.0 || self.heliotropism > 1.0   ← 范围校验
api/src/layer3.rs:35    heliotropism: ec.heliotropism                            ← API 透传
```

**零消费者**。`retrieval/src/lib.rs:32-36` 的 `energy_degraded` 只读 `system_load / latency_limit_ms / token_budget`。

而**主检索阶段恒为 Skilled**：

```rust
async fn stage_local_dominant(...) -> ... {
    self.storage.skilled_retrieve(...)   // ← 永远 α=0.5
}
```

⇒ **无论 `effective_mode` 为何、`heliotropism` 为何，主路径永远是 α=0.5。README 的"Optimistic 0.8 / Defensive 0.2"在主路径上永不发生。**

### 4.2 缺陷当前全部休眠

`edges` **0 行**、`life_records` **0 行**（metabolism 是唯一的建边者，从未运行）⇒ **CORRECTS 边从未存在过**。
⇒ P0-1 是**未来风险**而非当前故障；**建边之前无法验证 P0-1 的修复**。这直接决定分期（§6）。

### 4.3 `min_k_core` 剪枝被关闭

`sa_core_diffusion` 有 k-core 剪枝（`:272`），但三个包装器**全传 `0`**（`:444` / `:471` / `:498`）⇒ 现成的 hub/噪声控制机制闲置。这正是建议书 P1-4 所需的一半——**复用胜过引入**。

### 4.4 `ModeConfig::for_mode` 是死代码，两套参数并存且互相矛盾

`retrieval/src/mode.rs:15-45` 定义了 mode 感知的参数层（Skilled `max_hops=2`、Imagination 衰减 `0.5`…），**零调用者**；而实际执行的是 `topology.rs` 的字面量。同一概念两处定义、数值不一致 —— 违反 DNA 原则 11（0 硬编码）。

---

## 5. 决策

### D1：α 有单一来源（对齐 README 的承诺）

`alpha` 不再由 `topology.rs` 的字面量决定：从 `EnergyContext.heliotropism` 按 README 声明的映射派生（乐观→广扩散、保守→聚焦），初值取 README 已公告的 0.8 / 0.2 作为端点。
**理由**：这是 spec 与代码重新对上的唯一方式；也是"白盒数字可解释"的前提。

### D2：迭代预算与跳数解耦

`max_iterations`（幂迭代次数）与 `max_hops`（图跳数）分离，各自有来源（config）；补**收敛判据**（残差阈值）并声明。
**理由**：当前 `for _ in 0..max_hops` 使"跑的是截断 PPR"这一事实不可见。

### D3：抑制改为确定性门控（采纳建议书 P0-1 的 A 方案）

`CORRECTS` **不进传播矩阵**；改为传播后的确定性门控：节点带 `superseded_by` / `valid_until`，检索时硬过滤（或能量乘 0），历史照留供审计。
**理由**：① 软抑制被 fan-out 稀释（§2 已验证）；② **与既有语汇同向** —— `is_recessive` / `generation` / `metabolism/decay.rs` 已是"硬门控 + 时态"机制，本决策是其**自然延伸**，不引入新范式。
**不采纳**建议书的 B 方案（signed normalization）：那仍是"靠数值压"。

### D4：打开 `min_k_core`（复用既有机制）

三个包装器不再恒传 0；阈值有 config 来源。
**理由**：hub/噪声控制机制已存在，先复用再谈引入 rescaling。

### D5：拒绝 `a_0` 归一化，保留种子豁免

**理由**：见 §3.4 —— 归一化会恶化代码注释所记的失效模式。

### D6：修 README §3.1 的措辞

- "automatically clamping their final energy to `0.0`" **不实**（对非种子为阈值门控、对种子豁免）；
- `α (Decay / Heliotropism Factor)` 的 "Decay" 与 PPR 的 `damping` 方向相反，改名以免后来者（含 AI）把参数调反；
- 补 `max_iterations` 与收敛判据的声明。
**理由**：README 是本仓的 spec，措辞不实会把下游推理带偏 —— 本 ADR 的建议书审查正是活例。

---

## 6. 分期

| 期 | 内容 | 依赖 |
|---|---|---|
| **T1** | D1 + D2（α 与迭代预算的单一来源、spec 对齐） | 无 —— **先做**，它让后续一切可解释 |
| **T2** | D6（文档更正） | 可与 T1 同批 |
| **T3** | D4（打开 k-core） | 与 T1 同批亦可（小） |
| **T4** | D3（抑制门控：`superseded_by`/`valid_until`） | **须在建边之前落地**，否则门控无输入 |
| **T5** | 建边（日常写入路径记关系；proto 补 `parent_ids` —— INTENT-7 的 `WRITE_NODE` 已规定该字段） | 独立决策，另立 ADR |
| **T6** | Anaphase 白盒改读 `activation_vector`（当轮激活）而非持久化 `Node.heat` | T5 之后才有意义 |

**顺序的理由**：T1 使算法可解释 → T4 在边出现**之前**把门控装好 → T5 建边（P0-1 的修复此时可验证）→ T6 让效果可见。**在 T5 之前修 P0-1 无法验证**（没有 CORRECTS 边可测）。

---

## 7. 后果与风险

**正面**：spec 与代码重新对上；白盒的数字可解释；抑制不再被 fan-out 稀释；两套矛盾的参数收敛为一处；README 的措辞不再误导下游。

**代价与风险**：
- D1 改变 α 的取值 ⇒ **检索结果会变**（当前 α=0.5 固定，改为按 heliotropism 派生）⇒ 必须带回归网（现有 `retrieval_test.rs` 的 P10 种子保底/因果边用例）并**重新验收召回**。
- D3 引入新字段（`superseded_by`/`valid_until`）⇒ 触及存储 schema 演进；按 ADR-0012（Append-Only）追加，不覆写。
- D2 若把 `max_iterations` 上调以逼近不动点，**能耗与延迟上升**（Helix 的"极致节能"需一并权衡）；本 ADR 要求把该权衡显式记在 config 注释里。

---

## 8. 替代方案

| 方案 | 否决理由 |
|---|---|
| 只按建议书修 P0-1，不动 α 来源 | 白盒数字仍不可解释：α 无来源、主路径恒 Skilled，修完之后**无法证明修好了什么** |
| 采纳建议书 B 方案（signed normalization） | 仍是"靠数值压"；且负权破坏 PPR 概率语义（建议书自己的 P1-3 已指出），路线不自洽 |
| 采纳建议书 #3（`a_0` 归一化） | 恶化孤立种子的存活（§3.4） |
| 保留两套参数（`ModeConfig` 与字面量并存） | 违反 DNA 原则 11；且数值矛盾（§4.4），后患更大 |
| 现在就建边、跳过 T1/T4 | P0-1 会**先进入生产**：边一出现，稀释立即生效，而过时知识回归最难事后发现 |

---

## 9. 待人类决策

1. **T1 是否即为下一刀**（α/迭代来源的单一化）？本 ADR 的立场是"是"。
2. **D1 的映射**：`heliotropism ∈ [−1,1] → α` 用 README 公告的端点（0.8 / 0.2）线性插值，可否？
3. **D3 的字段**：`superseded_by` / `valid_until` 命名是否照此（与 `invalid_at` 等既有生态语汇是否需对齐）？
4. **T5（建边 + proto 补 `parent_ids`）** 是否另立 ADR —— 它牵涉协议层与三仓。
5. **状态**：批准后本 ADR 转 Accepted，方可改 `topology.rs` 与 `config.rs`。

---

*本 ADR 的审查部分以源码为准；建议书未能取得源码，其结论在源码面前有三处需修正、两处需限定、一处不采纳，但其最重的判断成立。*
