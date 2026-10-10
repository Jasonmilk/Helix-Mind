# 想 / 说 / 做 · 契约与**当前的洞**（2026-10-09）

> 人类指示：*"helix 必须可以清晰知道 **想、说、做**！Tuck 也必须知道！
> LLM 可以**想** `rm -rf` 但是不能**想做就做**！CI-144 + Tuck 就可以阻断这个风险。
> 让硅基与碳基更好的协作共生。"*
> 人类同时点出：*"Tuck 会协助你我**阻断危险的行为，避免被诱骗**。"*

## 一、看代码的结论：**三样都有设计，但没有一样接上**

| # | 应有 | 现状 | 证据 |
|---|---|---|---|
| **①** | **想/说/做 三条通道** | ✅ **存在** | 事件流：`assistant/think`（想）· `assistant/reply`（说）· `tool/call`+`tool/result`（做）· `assistant/attempt`（计划） |
| **②** | **"做"要过门** | ✅ **契约存在** | `pipeline/mod.rs:96` `security_gate: Option<Arc<dyn SecurityGate>>`；`pipeline/mod.rs:171` **`I7 (§13.3): "no gate installed" must not look like a gate that passed`** |
| **③** | **生产装门** | ❌ **没装** | `anaphase/src/main.rs` 里 **grep `with_security_gate` / `SecurityGate` → 空** |
| **④** | **配置能配它** | ❌ **没有** | `config.toml` **grep `security` / `tuck` → 空** |
| **⑤** | **Tuck 提供该端点** | ❌ **没有** | `Tuck/src` **grep `security/gate` → 空** |

**⇒ ⇒ 所以：此刻「想」与「做」之间**没有门**。**
**⇒ 而"没有门"被 `I7` **如实记录**（`gate_presence()` ⇒ `gate=none`）—— **这是好设计**；
但**"如实记录没有门"不等于"有门"**。保护本身不存在。**

## 二、★★★ 而 `I7` 与我在 phyt-DNA 层立的判据是**同一条**

```
I7 (§13.3)   "no gate installed" must not look like a gate that passed      ← 生态层
P11 (测电仪)  pass 不可独报 —— 必须是 pass + alive                            ← 方法论层
```
**⇒ 同一个判据，两个层次。** 而**本会话在两层都撞到了它的反面**：
- **方法论层**：一组 gate 全绿而无夹具 ⇒ 那不是绿，是**未测量**
- **生态层**：`gate=none` ⇒ 那不是"通过了"，是**没有门**（而"没有门"若被读成"没事"，就是同一种伪证）

## 三、这条洞的具体风险（人类说的"被诱骗"）

**提示注入 / 诱骗** 的目标是让 LLM **"想"**出危险动作（`rm -rf`、外发数据、越权）。
**⇒ 想本身无害**（`assistant/think` 只是一行文本）。
**⇒ 危险在「想做就做」——而**当前"做"没有门**。**

**★ 所以这条洞的性质是**正确性缺陷**，不是"缺个功能"**：**
- 契约**已定义**（`SecurityGate` trait）、**已有实现骨架**（`HttpSecurityGate`）、
  **已有纪律**（`I7` + "a refusal is a declared row"）
- **缺的是三处接线**（生产者=Tuck 的端点 · 消费者=`main.rs` 的装配 · 配置项）

## 四、★ 建议的处置（按 AITL 契约：接线涉及**行为**，需人类批准）

| # | 事项 | 归属 | 需批准？ |
|---|---|---|---|
| **D8-1** | **Tuck 实现 `security/gate` 端点**（承接既有 `SecurityGate` 契约，不新造接口） | Tuck | **是**（改行为） |
| **D8-2** | **`main.rs` 装配**：`with_security_gate(Some(HttpSecurityGate::new(tuck_endpoint)))` | anaphase | **是**（改行为） |
| **D8-3** | **`config.toml` 加 `security_gate_endpoint`**（缺失 ⇒ `I7` 记 `none`，**不静默放行**） | anaphase | **是** |
| **D8-4** | **一条判据**：`gate=none` 时**不得**把"做"记为成功 —— 而应记为**具名拒绝/未保护** | anaphase | **是** |

**⇒ 我**不擅动**这四项（它们全是"改行为"，且 D8-2/3 决定"LLM 能不能不装门就动手"）。**

## 五、一句话

> **想 / 说 / 做 的通道都在，门的契约也在，`I7` 的诚实也在 —— 缺的只是**把门接上**。
> 而"没接上"这件事本身被 `I7` 诚实记录了（`gate=none`）⇒ **下一步不是加新机制，是把既有的门装到既有的位置上。**
