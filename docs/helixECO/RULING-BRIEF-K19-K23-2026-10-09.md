# 裁决简报 · K19 与 K23（2026-10-09）—— 让"要用户裁决"变成"用户能裁决"

> 每页四问：**两个名字各是什么 / 各出现在哪（内部 · 序列化 · 跨仓 API）/ 波及面 / 建议目标名与建议裁决。**
> **分野规则（本简报的判据）**：名字若**进序列化格式** ⇒ 改名动**行为面/指纹** ⇒ 需单独授权；
> **纯内部** ⇒ 一次提交即可。**下列计数与文件名全部取自 grep/git 事实，不凭记忆。**

---

## K19 · 同一仓内"一物多名 / 一名多物"（`verdict` 一族）

| 名字 | 定义处 | 出现在哪一类面 | 波及面 | 序列化影响 |
|---|---|---|---|---|
| **`GateVerdict`（公开的那个）** | `anaphase-helix/src/security.rs:79` | **公开 API + 线上契约**（适配器按字符串解析对面） | anaphase **7 个文件** | **无**：线上词汇由 `adapters/security_gate.rs:89-95` 的**字面量匹配**定死（`"pass"` / `"reject"` / `"hitl_required"` / `"hard_override"`）⇒ **改 Rust 标识符不改线上字符串** |
| **`GateVerdict`（内部的那个）** | `anaphase-helix/src/run_cycle/safety_gate.rs:52` | **纯内部**（`pub(super)`） | 同 crate 少数文件 | 无 |
| **`LedgerRecord` / `VerdictStatus`** | `anaphase-helix/src/ledger/mod.rs`（3 文件） | **写进 JSONL 账本**（`#[serde(tag="record_type", rename_all="snake_case")]`） | anaphase 3 文件 | **有**：变体名会变成 `record_type` 的取值 ⇒ 改它**动指纹**（除非显式 `#[serde(rename=…)]` 钉住） |

**问题**：同 crate 内**两个 `GateVerdict` 语义完全不同**（一个是安全策略决定，一个是执行前检查结果）⇒ 读者会误读。

**建议裁决（三档，互不冲突）**：
1. **内部那个改名** —— 建议 `ToolGateOutcome`（纯内部 ⇒ **一次提交即可**，零线上影响）✅ **低风险，建议做**。
2. **公开那个保留 `GateVerdict`** —— 它是生态契约的名字，且线上词汇已由字面量定死 ⇒ **改它没有收益，只有churn** ⇒ 建议**不改**。
3. **`LedgerRecord::Verdict`** —— 它**进 JSONL**（`record_type: "verdict"`）⇒ 属于"进序列化格式" ⇒ **改名需你单独授权**（因为会让历史账本与新账本出现两种 `record_type`，除非显式 `rename` 钉住旧值）。**建议：本轮不动。**

---

## K23 · 跨仓"一物两名"（`HitlRequired` ⇄ `NeedHumanConfirm`）

| 侧 | 名字 | 出现在哪一类面 | 波及面 |
|---|---|---|---|
| anaphase | `GateVerdict::HitlRequired(String)` | **线上契约的解析侧** | **4 个文件** |
| Tuck | `DecisionConfig::NeedHumanConfirm`（+ `Decision::NeedHumanConfirm`） | **纯内部策略枚举**（`tuck-core/src/policy.rs`） | **15 个文件** |

**★ 关键量测（缩小了这条的严重度）**：**线上词汇其实已经一致**——
anaphase 的适配器按 `"pass"|"reject"|"hitl_required"|"hard_override"` 解析；
而 Tuck 的 gate 端点目前**只发出 `"pass"`**（M1a 观察态）⇒ **映射点今天还不存在**。

**⇒ 风险的真实形态**：不是"现在不一致"，而是"**将来 Tuck 要序列化一个 need-human-confirm 决定时，必须把它写成 `"hitl_required"`** ——
而这一步**没有测试守着**，且它落在**最该守的那一档**（要人工确认）。

**建议裁决**：
- **不建议跨仓改名**（Tuck 侧 15 个文件、anaphase 侧 4 个文件，**换来零线上收益**）⇒ 那是**按名字对齐**，不是按**事实**对齐。
- **建议**：在 Tuck 实现"序列化决定"的**那一个点**加映射 + 一条**能红的判据**（例如：`NeedHumanConfirm` ⇒ `"hitl_required"`，且 anaphase 的解析器认得）。**一处 + 一条测试，优于 19 个文件的重命名。**
- **⇒ 这条可以降级为"待 Tuck 实施序列化时同步做"**（不是本轮债务）。

---

## 一句话汇总（供裁决）

| 条目 | 建议 | 授权需求 |
|---|---|---|
| **K19-1** 内部 `GateVerdict` → `ToolGateOutcome` | **做**（纯内部，零线上影响） | 无需（属重命名重构，我可做） |
| **K19-2** 公开 `GateVerdict` | **不改**（生态契约名，改无收益） | — |
| **K19-3** `LedgerRecord::Verdict` 的名字 | **不动**（进 JSONL ⇒ 动指纹） | 若要动 ⇒ **单独授权** |
| **K23** 跨仓对齐 | **不改名**；改为"在 Tuck 序列化决定的那一点加映射 + 1 条能红的判据"，**降到 Tuck 实施序列化时同步做** | 无需（届时随 Tuck 那次改动一起） |

**⇒ 需要你点头的只有一件：K19-1 是否现在做**（我评估：低风险、纯内部、可一并跑全套判据）。
