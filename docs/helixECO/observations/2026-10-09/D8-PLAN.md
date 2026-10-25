# D8 计划 · 把「做」的门接上（分阶段 · 每步独立可验证 · 可逆）

> 人类：*"这个你可能会容易绕进去！必须做好规划，确保可以边验证边推进，避免同时出错很难找到问题。
> 建议做好计划，明确阶段性里程碑，然后步步为营。"*

## 〇、三个坑（先写在最前，它们决定计划形状）

| # | 坑 | 为什么致命 |
|---|---|---|
| **1** | **跨三处同时改**（Tuck 端点 + anaphase 装配 + 配置） | 一坏就**不知是谁坏的** ⇒ 难定位 |
| **2** | **装门 = 改行为** | 门一拒 ⇒ `run_cycle` 失败 ⇒ **整条对话路径崩** |
| **3** | **门是 fail-closed 的** | **"Tuck 挂了 = Helix 不思考"** —— 而这条**代码自己写着**（`anaphase/src/run_cycle/mod.rs:671`） |

## 一、★ 关键发现：**Tuck 自带「观察模式」⇒ 不用我设计分段**

```
Tuck/docs/decisions/ADR-0005:176  观察模式（记录但不拦），使收敛过程可度量
ADR-0005:187  T6 | 观察模式（… 独立的 observe_only 开关，只记不拦）| ⬜
Tuck/docs/PLAN.md:96  H-1..H-8 准入闸门全部交付（… 观察模式 …）| ✅
```
**⇒ 「先记录后阻断」这条路径**本仓已有设计**（`observe_only`）⇒ **极致复用，不新造。**
**⚠️ 前置检查（M1 前必须做）**：两份文档**互相矛盾**（ADR 说 T6 ⬜ / PLAN 说 ✅）
⇒ **先去代码里确认 `observe_only` 到底存不存在。**

## 二、已有的契约（照它接，不新造接口）

| 件 | 位置 | 形状 |
|---|---|---|
| 契约 | `anaphase/src/security.rs` | `SecurityGate::check(&GateCheck) -> GateVerdict`；`GateVerdict::{Pass, Reject(String), HitlRequired(String), HardOverride}` |
| 传输 | `anaphase/src/adapters/security_gate.rs` | `HttpSecurityGate` **POST `<url>`，body = `GateCheck` JSON，响应 `GateResponse`** |
| 纪律 | `pipeline/mod.rs:171` | **`I7`：没有装门 ≠ 门通过了** |
| 纪律 | `run_cycle/mod.rs:722` | **拒绝是具名的行**，不是缺失的节点 |

## 三、★ 里程碑（每步：**单仓 · 最多一处行为变化 · 独立可验证 · 可逆**）

### M0 · 只读基线（0 改动 · 5 分钟）
- **做什么**：从事件流确认生产现在是 `gate=none`（`gate_presence()` 的输出）
- **验收**：能指出具体事件行 / 或指出"没有该字段"
- **风险**：0（只读）

### M1 · Tuck 侧实现 `security/gate`（**一个仓 · 独立可测**）
- **前置**：先确认 `observe_only` 是否已存在（上面那处文档矛盾）
- **做什么**：Tuck 提供端点，**默认观察态（只记不拦）**
- **验收**：**curl 直连 Tuck** —— 干净 payload 放行；脏 payload 被**记录**（此时仍不拦）
- **★ 关键**：**这一步完全独立于生态** ⇒ 坏了不影响对话
- **可逆**：端点是新增路由；关掉即回到现状

### M2 · anaphase 装配（**仍是观察态 ⇒ 行为不变**）
- **做什么**：`config.toml` 加 `security_gate_endpoint` + `main.rs` 装配一个 gate
- **为什么行为不变**：Tuck 侧是 `observe_only` ⇒ **判定被记录，但不阻断**
- **验收**：① 发一句话**仍然得到回答**（与 M0 基线对比）② 事件流里**出现**门的存在与判定
- **可逆**：删掉配置即回到 `gate=none`（而那本身被 `I7` 记为事实）
- **★ 这一步是"边验证边推进"的关键**：它证明**接线通了**，而**没改变**任何可见行为

### M3 · 翻转到执行态（**唯一的真行为变化**）
- **做什么**：Tuck 侧关掉 `observe_only`
- **验收**：
  - 正常问 ⇒ **仍照答**（不得因为装了门就变哑）
  - 危险动作 ⇒ **具名拒绝**（"拒绝是具名的行"）
- **★ 反证（必做）**：**把 Tuck 停掉** ⇒ 行为必须是**具名拒绝**（fail-closed 的声明），
  **不是静默放行** —— 这正是 `I7` 那条纪律要的
- **可逆**：重新打开 `observe_only`

### M4 · 判据落地（D8-4）
- **做什么**：`gate=none` 时"做"**不得**记为成功，应记为**具名未保护**
- **验收**：把配置删掉 ⇒ 该判据必须红

## 四、步步为营的纪律（这次特别强调）

1. **一次只有一个仓在动**；跨仓的改动**不在同一步里**
2. **每一步都先记录"改前读数"**（M0 的意义就在这）—— 否则 M2 无法判断"行为变没变"
3. **M2 与 M3 必须分开**：M2 证明**接线通**，M3 才改**行为** ⇒ **坏了两步都能定位**
4. **每步都有可逆路径**，且**逆路径本身被记录**（`I7` 的"没有门是事实"）
5. **不合并 M1 与 M2** —— 那正是"同时出错很难找到问题"的那件事

## 五、需要人类批准的点（AITL 契约：改行为 ⇒ 上报）

| 步 | 是否需要批准 |
|---|---|
| M0 | 不需要（只读） |
| **M1** | **需要**（Tuck 新增路由 = 改行为面） |
| **M2** | **需要**（anaphase 装配 = 改行为面；虽在观察态） |
| **M3** | **需要**（唯一的真行为变化） |
| M4 | **需要**（改判据语义） |

**⇒ 我的建议：先做 M0（只读、0 风险），把基线读数拿到手；然后你批准 M1。**

---

## 六、M0 已完成（只读）· 结果与一个真发现

**命令与读数（物理事实优先：读真实产物，不读配置）**
```
grep -rl "gate=none\|gate=" .helix/events/*.jsonl        → gate_presence 的输出【无命中】
grep -rh '"gate"' .helix/events/*.jsonl                  → 只有 check/status 的 "gate":"hard"（判据的 gate）
src/pipeline/mod.rs:173  emit_event(&job.job_id, 3, "gate", &presence)
src/pipeline/mod.rs:135  self.events.lock().unwrap().emit(...)      ← 内存事件环
```

### 基线结论
**① 生产现在是 `gate=none`**（`security_gate: None` ⇒ `main.rs` 未装配 ⇒ 与代码事实一致）✅
**② ★ 但"没有门"这个事实只进了【内存事件环】，没有落盘** ❌

**⇒ 而对照鲜明**：`check/status`（**判据**的裁定）**是落盘的**；**安全门的在场与否不是**。

### 这是 `I7` 对着自己的反面
> `I7`: *"no gate installed must not look like a gate that passed"* —— **记录确实发生了**（好）
> **⇒ 但它只活在内存环里 ⇒ **事后读的人无法核实"当时有没有门"**。**
> **⇒ "被记录了" ≠ "可被读到"** —— 正是本会话第 33 条（**产出 ≠ 被读取**）在生态层的同形。

### 因此 D8-4 的形态更正
~~`gate=none` 时"做"不得记为成功~~
**⇒ 改为：`gate=none` 这个事实必须**① 持久（落盘）② 具名 ③ 可被消费**（能被判据/面板读到）。**
（同源：P8「被委派的产物必须存在于仓库」· P11「信号必须有具名消费者」。）

**⇒ 而它顺带解释了 `tests/stage_events.rs` 为什么要读 `pipeline.events.lock()`** —— 那个"活的 Mutex 环"
（本会话早先登记为同类危险）**就是阶段事件的唯一去处**；M4 要做的正是把它的一部分**落盘**。

### M0 风险
**0**（全只读）· **未改任何文件**（本册仅追加记录）

---

## 七、M1 recon 完成 · **解决了那处文档矛盾**（并产出一个真发现）

**契约已读准（极致复用的前提，**不新造接口**）**
```
请求  GateCheck   { job_id, index, tool, args_json, identity_labels }
响应  GateResponse{ decision: "pass"|"reject"|"hitl_required"|"hard_override", reason }
★ anaphase 的兜底很干净：unreachable_verdict → HitlRequired("security gate unreachable: …")
   ⇒ "问不到的门"【没有批准任何东西】（fail-closed 且具名）
```

**Tuck 真实布局（我之前猜 `Tuck/src/` 是错的）**
```
Tuck/crates/{tuck-core, tuck-audit, tuck-gateway, tuck}
  tuck-core: policy.rs · catastrophic.rs · hitl.rs · audit.rs · injection.rs · metrics.rs …
  tuck/main.rs:157 TcpListener + :181 axum::serve(listener, router)
  tuck-gateway/tests/proxy.rs（axum，POST /v1/chat/completions）
```
**⇒ `security/gate` 应当是既有 `policy`/`hitl` 之上的一层**薄适配**（极致复用），不是新机制。**

### ★ 发现：`observe_only` **不存在** ⇒ 文档矛盾判定为「**PLAN.md 错**」

```
ADR-0005:187   T6 | 观察模式（… observe_only 开关，只记不拦）| ⬜   ← 标未做
PLAN.md:96     H-1..H-8 准入闸门全部交付（… 观察模式 …）| ✅        ← 标已做
实测 crates/   grep observe_only → 【无】
               grep observe      → 只有 metrics.rs 的 observe_decision()（原子计数，不是开关）
```
**⇒ 结论：`observe_only` 开关**没有实现**；`PLAN.md` 的 ✅ **是错的**。**
**⚠️ 这属于 A5（第二份真相）：同一个事实在两份文档里相反，而**代码是唯一事实**。**（与 D2/D7 同形。）**

### 因此 M1 的设计（用你的哲学定：确定性 · 0 硬编码 · 按需）

**观察态**只能是**配置项**，不是硬编码、也不是我现造的隐式行为：
```toml
[gate]
observe_only = true      # 默认 true（无害那一侧：第 17 条 —— 不可逆的不对称错误默认从无害侧开始）
```
**⇒ 端点行为**：
- **`observe_only = true`** ⇒ **记录**将要判定的结果（落到**可读**的地方），**返回 `pass`** ⇒ 行为不变
- **`observe_only = false`** ⇒ 返回真实判定（`pass|reject|hitl_required`）

**⇒ 而"记录到哪里"必须回答 M0 那个发现**：**不能只进内存环**（`gate=none` 就是那样丢的）
⇒ **落 `ledger/`（既有 append-only 账）**，与 `hits-*.jsonl` 同源（极致复用）。

### 本轮状态（诚实）
- **M1 的 recon 完成**；**端点本身未写**（预算用尽，且要先定 `observe_only` 的落点：配置项 + 写在哪个账本）
- **未改 Tuck 任何文件** · 六仓 dirty=0
