# K16 设计（**落在 Tuck 既有能力上**，不是新建机制）

> 人类 2026-10-09：*"Tuck 是**生态之外**的安全闸门！可以给 Helix 的安全做**兜底**，
> 它结合 CI-144 可以**阻断风险行为**，避免酿成灾难！既然用户绑定好 Tuck 给生态，那就是对安全有要求 ⇒
> Tuck 就需要**坚守**（白名单、黑名单、混淆、截断、警报等），而这些**都可以审查** ——
> **谁在误导和诱骗 Helix，也会有迹可循！**"*
> **⇒ 本文件的作用：把 K16 的需求映射到 Tuck **已经有的**能力上，避免重复造、避免重复查。**

## 一、需求 → 既有能力（**先查后写**）

| 人类列的要求 | Tuck **已有** | 位置 |
|---|---|---|
| **混淆 / 脱敏** | `redact` | `tuck-audit/src/lib.rs` · `tuck-core/src/credential.rs` · `tentacle_bridge.rs` |
| **截断** | `truncat` | `tuck-core/src/{audit_store,frame,hot_reload}.rs` |
| **警报** | `alert` + `notify` | `catastrophic.rs` · `tuck-gateway/src/{matrix,policy}.rs` · `gov.rs` |
| **白名单/黑名单（准入）** | `access`（ADR-0005 "access allowlist gate"：**白/黑名单同表 `effect`**） | `tuck-gateway/src/access.rs` |
| **（额外的，你没列但它有）** | **注入防御** `injection.rs` · **防篡改** `tamper.rs` · `hsm.rs` · `hitl.rs` · `outbound.rs` · `sap.rs` · `audit.rs` / `audit_store.rs`（**可审查**） | `tuck-core/src/` |
| **决策入口** | `policy.rs:78 DecisionConfig` · `catastrophic.rs` | `tuck-core/src/` |

**⇒ 结论：K16 **不需要新机制**。它是"把既有的门接到既有的位置上"** ——
**缺的只有三处接线**（见 `THINK-SAY-DO.md` §一：生产者端点 / 消费者装配 / 配置项）。

## 二、端点形状（**照既有契约，不新造接口**）

```
请求  GateCheck    { job_id, index, tool, args_json, identity_labels }     ← anaphase/src/security.rs:46
响应  GateResponse { decision: "pass"|"reject"|"hitl_required"|"hard_override", reason }  ← adapters/security_gate.rs:57
★ anaphase 的兜底（已实现、且是 fail-closed 且具名）：
  unreachable_verdict → HitlRequired("security gate unreachable: …")  ⇒ 【问不到的门没有批准任何东西】
```

**⇒ Tuck 侧要做的是把 `GateCheck` 交给既有 `policy`/`access`/`catastrophic` 判，再按上表返回。**

## 三、**可审查**（人类特别强调：谁在诱骗，要有迹可循）

**已知可用的审查面**：`audit.rs` / `audit_store.rs`（审计链）· `tamper.rs`（防篡改）· `redact.rs`（脱敏后再留痕）。

**⇒ 设计约束（写进 M1 的验收）**：
1. **每一次 gate 判定都要落审计**（含 `job_id` / `tool` / `args_json` 的**脱敏**版本 / 判定与理由）
   ⇒ 这样"谁在误导和诱骗 Helix"**有迹可循**。
2. **`reason` 必须具名**（不是"被拦了"，而是"因为〈可观测条件〉"）—— 与 phyt-DNA 的
   `why` 要求同形（`「因为〈可观测条件〉，所以不吸收」`）。
3. **观察态下也上报**（Tuck 的 `observe_only` 设计：*"观察模式不拦 / 观察模式仍上报"*）——
   **"只记不拦"不等于"不记"**。

## 四、分阶段（**每一步独立可验证**，见 `D8-PLAN.md` 的 M0–M4）

| 里程碑 | 内容 | 验收 | 状态 |
|---|---|---|---|
| **M0** | 只读基线：确认生产现在是 `gate=none` | 从事件流读到 | ✅ **已完成**（并发现 `gate=none` 只进内存环、不落盘） |
| **M1** | Tuck 实现 `POST /security/gate`（薄适配 `policy`/`access`/`catastrophic`；**默认观察态**） | **curl 直连 Tuck**（不经生态）：干净 ⇒ pass；脏 ⇒ **记录**（此时不拦） | `[ ]` 待做 |
| **M2** | anaphase 装配（`config.toml` + `main.rs`），**仍观察态 ⇒ 行为不变** | 仍能应答（与 M0 对比）· 事件流**出现**门的判定 | `[ ]` |
| **M3** | 翻转执行态（**唯一真行为变化**） | 正常问照答；危险动作**具名拒绝**；**反证：停 Tuck ⇒ 必须具名拒绝，不得静默放行** | `[ ]` |
| **M4** | 判据：`gate=none` 时"做"不得记为成功，应记为**具名未保护** | 删配置 ⇒ 判据必红 | `[ ]` |

**⚠️ 三个坑（`D8-PLAN.md` §〇）**：跨三处同时改（一坏不知谁坏）· 装门=改行为（门一拒整条对话崩）·
**门是 fail-closed ⇒ "Tuck 挂了 = Helix 不思考"**（`run_cycle/mod.rs:671` 自己写着）。
**⇒ 故 M2 与 M3 必须分开**（M2 证明接线通，M3 才改行为）。

## 五、与其它条目的关系（避免重叠）

| 条 | 关系 |
|---|---|
| **K22**（已修） | **pre 时刻的形状拦**（`rm -rf`/force push/权威卷）—— 那是**门内**的第一道；K16 是**门外**的兜底。**两者互补，不重复。** |
| **K17** | 并发（含 `file_store.rs:210` 的 MutexGuard 跨 await）—— 与 K16 无关，但**都在"让系统更稳"这一族** |
| **K19** | 改名（`verdict`/`GateVerdict`）—— 独立 |

---

## 六、人类对 RS 版的两个提问 —— 逐条答（**有产物为据**）

> 人类 2026-10-09：*"Tuck 之前的 python 版本（Tuck-beta）可以接管 API 并把 LLM 与用户对话历史保存下来！
> 现在 rs 版本也需要这个功能吗？还是可以不需要？！而且还有 personas 功能（让 Helix 随时呼叫救兵 agent）——
> 这个暂时可能不需要了，应该交给 Anaphase-helix 更合理？！"*

### ① 接管 API —— **RS 版已经有了，且做得对**

```
tuck-gateway/src/lib.rs:9-11  "POST /v1/chat/completions — accepts an OpenAI-style JSON body,
                               forwards to the configured upstream, and streams back JSON / SSE chunks
                               untouched. Headers are forwarded"
:20   "物理事实优先: forwarding is byte-transparent — no buffering"
:134  .route("/v1/chat/completions", post(chat_completions))
proxy.rs  PFP 头解析（X-PFP）+ 框架无关拦截（"极致解耦"）
⇒ **不需要新增。**
```

### ② 对话历史 —— **不该由 Tuck 保存**（与 anaphase 重叠 ⇒ 两份真相 A5）

- **「对话历史」= 经历** ⇒ 归 `anaphase` 的 `session_events`（`turn/start`·`user/message`·`assistant/reply`…）。
- **Tuck 再存一份 ⇒ 两份真相**（A5）。**Tuck 的定位是"门 + 可审查"，不是"第二本经历账"。**
- **★ 但 Tuck 该记的是另一个量：**"**什么穿过了它**"**（审计行）** —— 见 ③。

### ③ ★ 而 RS 版**真正缺的那一件**：chat 路径上**没有落审计**

```
chat_completions（gateway/src/lib.rs:144-168）：构造 upstream → post → 转发 headers → 流式回传
   ⇒ 这条路径上【没有 audit / record / decide 的调用】
而 gateway 有可选的 AuditChain（state.rs:77 `chain: Option<Arc<Mutex<AuditChain>>>`）
   ⇒ 【有链，但没接在这条路上】
```

**⇒ 后果**：**"谁在误导和诱骗 Helix，有迹可循"目前不成立** ——
**接管了 API 却不记谁走过**，那对"可审查"是**零**。

**⇒ 而它与本文件的 M1 验收 ① 完全重合**：
```
M1 验收①  每一次判定落审计（含 args_json 的【脱敏】版本 + 判定 + 理由）⇒ 诱骗有迹可循
```
**⇒ 所以人类的提问补上了 K16 最易被忽略的一半。**

### ④ personas —— **人类判断正确：归 anaphase**（且已被有意退役）

- **`ADR-0001`（rust rebuild realignment）**：*"Python beta 的所有代码：`Tuck/`、**`personas/`**、`pyproject.toml`
  全部移入 archive，**不复用**"* —— `docs/DEPRECATE.md:16` 同步登记。⇒ **RS 版有意不做。**
- **定位上也更该归 anaphase**：**personas = "该派哪个救兵" = 判断/编排**，
  而 **Tuck 的立场是"不判断，只呈现"**（VISION） ⇒ **冲突**。
- **⇒ 交 anaphase-helix 合理。**

### ⑤ 由此对 K16 的**顺序**有一处**增强**

**M1 除了"端点"，还必须有"落审计"** —— 否则 M2/M3 装上去的是一个**看不见的门**。
**⇒ M1 的验收因此写成两条**（缺一不可）：
```
① curl 直连：干净 ⇒ pass；脏 ⇒ 记录（观察态不拦）
② 每次判定在审计里留下一行（含【脱敏】的 args、判定、理由）—— 否则"可审查"不成立
```

---

## 七、M1 的**行级设计**（已读到确切锚点；下一轮可直接落，不必再考古）

### 要动的四处（行号为 2026-10-09 实测）

| # | 位置 | 现状 | 要做什么 |
|---|---|---|---|
| 1 | `crates/tuck-gateway/src/lib.rs:133-137` | `Router::new().route("/v1/chat/completions", post(chat_completions)).with_state(state)` | 加 `.route("/v1/security/gate", post(security_gate))` |
| 2 | `crates/tuck-gateway/src/state.rs:77/83` | `chain: Option<Arc<Mutex<AuditChain>>>`（`feature="audit"`）· `access: Option<AccessTable>`（`feature="access"`） | 两者**都可能为 None** ⇒ handler 必须**按 cfg 与 None 分支具名处理**（"没装门"≠"门通过了"，与本仓 I7 同源） |
| 3 | `crates/tuck-core/src/audit.rs:149` | `append(decision, risk_level, modality, override_flag, source, identity_label) -> &AuditEntry` | **每次判定落一行**（M1 验收②）；`args_json` **先经 `redact` 再入库** |
| 4 | `crates/tuck-core/src/policy.rs:78` | `DecisionConfig { Pass, Reject, NeedHumanConfirm, HardOverridePass }` → `Decision` | **直接复用**；不新造枚举 |

### 请求/响应（照既有契约，逐字对齐 anaphase `src/security.rs:46` / `adapters/security_gate.rs:57`）

```rust
#[derive(Deserialize)] struct GateCheckIn { job_id: String, index: u32, tool: String,
                                            args_json: String, identity_labels: BTreeMap<String,String> }
#[derive(Serialize)]   struct GateResponseOut { decision: String, reason: String }
```

### 观察态（默认，无害侧 —— 第 17 条）

```
observe_only = true（配置项，**不硬编码**）
  ⇒ 记录【将要判定的结果】到审计，但**返回 `pass`** ⇒ 行为不变（M2 才接线，M2 也因此不改行为）
observe_only = false
  ⇒ 返回真实判定（pass | reject | hitl_required | hard_override）
```

### M1 的两条验收（缺一不可）

```
① curl 直连 Tuck（不经生态）：干净 ⇒ pass；脏 ⇒ **记录**（此时仍不拦）
② **每次判定在审计里留下一行**（脱敏 args + 判定 + 理由）—— 否则"可审查"不成立
```

## 八、★ 新登记 K23：**两个仓用不同的名字表达同一件事**（跨仓的"一物两名"）

| anaphase 的 `GateVerdict` | Tuck 的 `DecisionConfig` | 语义 |
|---|---|---|
| `Pass` | `Pass` | ✅ 同名同义 |
| `Reject(String)` | `Reject` | ✅ |
| **`HitlRequired(String)`** | **`NeedHumanConfirm`** | ★ **一物两名** |
| `HardOverride` | `HardOverridePass` | ✅ |

**⇒ 为什么它危险**：**接线时要写一张映射表；而**最容易漏的那一档，恰好是"需要人工确认"** ——
**最该守的那一档。**（本仓已有同族判例：`verdict` 一族的一物两名/一名两物，K19。）

**⇒ 处置（按"通用后做"）**：**先登记 + 在 M1 的映射处写明这一档**；
**要不要把名字统一（改哪个仓）属**改行为/跨仓契约**，等人类裁决。**
