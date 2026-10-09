# 重复 / 疏漏登记册（2026-10-09）· **带任务标记**

> 人类提醒：*"项目多次合拢和重构 ⇒ 可能有重复功能或疏漏；严谨一点，需要就用 x-ray 协助。"*
> 人类补充（**本条改变了两项判断**）：*"我的所有项目几乎都可以**单独运行**（可接入通用生态）——
> 理论上所有项目都可以独立运行，只是**无法获得完整生态能力**而已。"*
>
> **状态标记**：`[ ]` 待办 · `[x]` 已完成 · `[?]` **等人类裁决** · `[~]` 已记录但暂不动
> **每项都必须有：证据 → 处置 → 判据（能红）**

---

## ★ 先更正我两条判错的

### 更正 1 · D1 **不是重复** —— 是"每项目可独立运行"（人类原则）

我先判 `anaphase-helix/src/bin/up.rs` 与 `Cellrix/web/src/bin/up.rs` 是"两个 up ⇒ 重复"。
**⇒ 按人类原则，这多半是**每个项目自己的独立启动器**（各自能单独跑，只是没有完整生态能力）。**
**⇒ 处置从"谁权威"改为："各自独立可跑"这条性质要有判据"**（见 D6）。`[x] 已更正判断`

### 更正 2 · D2 **框架改**：不是"删字面量"，是"字面量必须与 SSOT 一致"

**独立运行**需要默认值（不能依赖 `ports.json` 在场）⇒ **字面量本身**是**合理的**。
**⇒ 真正的问题是**它们会不会与 SSOT 漂移**（本会话已修过一处：`up.rs` 的 `8080 → 50050`）。**
**⇒ 处置改为：一条**一致性闸门**（代码里的端口字面量必须在 SSOT 里，且值相同），**不删字面量**。`[x] 已更正框架`

---

## D1 · 两个 `up` → **改为 D6（独立可运行性）** `[x] 判断已更正`

```
Cellrix/web/src/bin/up.rs        1.6MB（10-01）
anaphase-helix/src/bin/up.rs     5.0MB（10-09）
```
**⇒ 不是重复，是"各自能独立跑"。** 保留。判据见 D6。

## D2 · 端口默认值必须与 SSOT **一致** `[ ] 待做`

```
ports.json 自述： "Every component's port lives HERE and nowhere else."
实测（在 Jasonmilk/ 下扫 Cellrix/web/src）：13 处端口字面量
  bin/up.rs:569,582(50051) · :1011(60052) · config.rs:9(50061),17(60053)
  main.rs:34,111(50050) · :206,213,296(60052) · :299(60053) · routes.rs:304(60053)
```
**处置**：**不删字面量**（独立运行需要默认值）· **加闸门**：每个字面量必须在 `ports.json` 里且值相同。
**判据（能红）**：把某处字面量改成 `50099` ⇒ 闸门必须红。（可正反例夹具）

## D3 · 「verdict」一族 5 处定义 · **其中两个 `GateVerdict` 同名不同物** `[?] 待裁决`

| 位置 | 名字 | 是什么 |
|---|---|---|
| `ledger/mod.rs:14` | `VerdictStatus` | 账本裁定（Met/Unmet） |
| `run_cycle/verdict.rs:64` | `PeriodVerdict` | 周期结束（HTTP 体那个） |
| `run_cycle/verdict.rs:24` | `EndReason` | 结束原因 |
| `security.rs:58` | **`GateVerdict`** | **安全策略决定**（Pass/Reject/HitlRequired/HardOverride） |
| `run_cycle/safety_gate.rs:52` | **`GateVerdict`** | **执行前检查结果**（Cleared/Refused） |

**两种病**：**一物两名**（`LedgerRecord::Verdict` vs `PeriodVerdict`）· **一名两物**（两个 `GateVerdict`）。
**⇒ 处置建议**：把 `safety_gate` 那个改为 `ToolGateOutcome` 之类。**⇒ 属重构，等一句话。**

## D4 · CI 第 3 步**复制**了引擎的闸门发现逻辑 `[ ] 待做`

```
tools/validate.sh 的 gates()                    ← 引擎（我已加 ADR-*.md 限制）
.github/workflows/phyt.yml step 3 的 for 循环   ← ★ 自己又写一遍 ⇒ 我修 gates() 它不受益
```
**处置**：CI 第 3 步改用 `--probe-all`（**只剩一份发现逻辑**）。
**⚠️ 前置**：先决定"**无夹具的闸门**"算不算失败 —— 按 P11，它是 `unproven`，**不该算通过**。

## D5 · 编排两份 `[~] 已记录，暂不动`

```
pipeline::run()                  六阶段
run_cycle + Reflection           活路径（自己调 execute_calls / record_evidence）
```
**⇒ 查"裁定为何没写"时读 `pipeline/mod.rs` 会读错文件**（本会话实证：连猜四次全错）。
**处置**：合一属大重构；**⇒ 现在**只需在 `pipeline/mod.rs` 的 `run()` 上**加一行注释**指向活路径（低成本）。

## ★ D6 · 新：**「每个项目可独立运行」这条性质，目前没有判据** `[ ] 待做`

**人类给的原则**：所有项目几乎都能单独运行（可接入通用生态），只是没有完整生态能力。
**⇒ 这是一条**可验证的性质**，而它现在**没有任何闸门**。
**判据（能红）**：对每个仓，**在兄弟仓缺席时**启动它 ⇒ **必须能起来**（或**具名降级**，不得崩溃）。
**⚠️ 已知反例（本会话）**：`up` 是**前台监督进程** ⇒ 调用被杀会把整个栈带走
（第 26 条）⇒ 这正是"独立运行"的一个真实脆弱点。

---

## 任务总表（按优先级）

| # | 事项 | 状态 | 判据 |
|---|---|---|---|
| **1** | **D2** 端口默认值与 SSOT **一致性闸门** | `[ ]` | 改一处字面量 ⇒ 必须红 |
| **2** | **D6** 独立可运行性闸门（兄弟缺席时可起） | `[ ]` | 移走兄弟仓 ⇒ 该项目仍能起（或具名降级） |
| 3 | **D4** CI 用 `--probe-all`（去第二份发现逻辑） | `[ ]` | 先定"无夹具"语义 |
| 4 | **D3** `GateVerdict` 改名 / `verdict` 一物两名 | `[?]` | 等裁决 |
| 5 | **D5** 编排合一（先加指向注释） | `[~]` | 低成本那半可先做 |

## 附：本册自身的诚实记录

**我在本轮犯过一次**：第二次端口扫描从 `anaphase-helix/` 里跑、路径写 `Cellrix/web/src` ⇒ 空结果。
**⇒ "空结果"不能当作"没有重复"。**（与第 31 条同族：观测缺失 ≠ 事件缺失。）

---

# 附：思路整理 —— **生态 / 通用 的边界**（人类指示：生态优先，通用后做；先标记 + 留接口）

## 工作区真实结构（我先前只看见 6 个仓）

```
生态（先做） anaphase-helix · helix-mind · Cellrix · FlowModus · Tuck
             helix-tentacle(rs) · HelixECO-Glove(main) · Helix-MCP-Learner(main)
               ↑ 它们之间的共同契约 = CI-144
通用（后做） commonintents/ ← CI-144 协议家族：
             INTENT-7 · BIND-19 · CAPABILITY-13 · INTENT-7-SECURE · PFP-xCF14 · SAP-xCF14
               ↑ 上游协议家族；**生态只是第一个消费者**（与 FlowModus 自述"Helix 只是第一个消费者"同形）
方法/其它    phyt-DNA(v2) · lodestone-md · lodestone-spec · lumtract
```

## 从 `ECOSYSTEM.md` 读到的事实（非猜测）

| 事实 | 原文要点 |
|---|---|
| **Cellrix 是 CI-144 的「法定参考实现」** | `INTENT-7 §15 法定参考实现` + `CAPABILITY-13 PC-2 可视化共识层`；"生态内的 `Cellrix:ADR-0021` 仍以 CI-144 协议为设计源" |
| **anaphase 有 CI-144 传输层** | `ADR-0017`：`--stdio` 换成 **CIB/1.0 MessagePack 握手 + LE u32 长度前缀帧 + Manifest 首帧 + 1s Snapshot 推流 + ActionRequest/Response** |
| **★ 类型是 `vendored`** | `anaphase/src/ci144/`：**"vendored 类型 … serde 逐字段对齐 Cellrix"** |
| CI-144 动词映射 | `FETCH / WRITE_NODE / TENTACLE / FINISH / CANCEL`（5/5 一致） |
| traceparent | W3C **透传不生成** |

## 因此这条指示如何改变我的优先级

| 我先前的项 | 按"生态优先"重新定位 |
|---|---|
| **D2** 端口默认值与 SSOT 一致 | ✅ **生态内** ⇒ 照做 |
| **D4** CI 去第二份发现逻辑 | ✅ **生态内**（phyt-DNA 侧）⇒ 照做 |
| **D3** `GateVerdict` 改名 | ⚠️ 生态内代码整洁 ⇒ 低优先，等裁决 |
| **D5** 编排合一 | ⚠️ 生态内 ⇒ 已记录，暂不动 |
| **D6** 「每项目可独立运行」 | 🔶 **通用向**（"可接入通用生态"）⇒ **按人类指示：先标记，不现在做** |

## ★ 新增 D7（CI-144 边界上的会漂点）`[?] 待裁决`

```
anaphase-helix/src/ci144/    ← "vendored 类型 … serde 逐字段对齐 Cellrix"
```

**⇒ 这是**手抄对齐**：上游（CI-144/Cellrix）一改，此处必须有人记得同步。**
**⇒ 与 D2（端口字面量）**同一形状**：不是"有第二份"，而是"**第二份会不会漂**"。**
**⇒ 处置方向（按"通用后做"）**：**现在只标记** —— 加一条**一致性判据**（vendored 字段集 CIB 协议定义一致），
**不重构**。等通用层真的开始被别的项目接入时再动。

## 一句话（我此刻的理解，供人类校正）

> **生态先闭环（我把 identity/tools/会话卡/rail 做完），通用层先**标记边界 + 留接口**（CI-144 vendored 类型、
> 每项目独立可运行），等生态稳了再谈通用化 —— 否则会像人类担心的那样"混淆和复杂化"。**

---

## D9 · ★ Tuck 的自证入口**静默跳过整个 crate** `[x] 已修（3 份文档）`

**坑：**
```
crates/tuck/Cargo.toml:13  default = []                                ← gateway 不在默认 feature
:15  gateway = [dep:tuck-gateway, dep:tuck-audit, dep:axum, …]
:14  # Content-governance gateway service (按需加载: only built when needed)   ← ★ 人类的哲学，已实现
tuck-gateway = { path = "…", optional = true }
⇒ `cargo test --workspace` ⇒ tuck-gateway 整个 crate【不参与编译】⇒ 它的全部测试【静默跳过】
```

**而 3 份文档在教人跑它：**
```
README.md:176, :214        cargo test --workspace
README.zh-CN.md:91, :129   同上
docs/RNA.md:89             测试通过（`cargo test --workspace` 全绿 + 0 warning）   ← ★ 这是【验收判据】
```

**⇒ 已修（只改文档 · 可逆 · 不改行为）**：三处改为 `--all-features`，**并在 README 补一句【为什么】**
—— 否则下一个人会改回去（第 34 条：不许只靠提醒）。

**⇒ 同形于主线**：**「不带 feature 的全绿」≠「全绿」**（与 `--probe-all` 只报一个世界 /
`I7` 的"没装门不得看起来像门通过了"**同一条**）。

**★ 而它同时确认了一件好事**：`# 按需加载: only built when needed` —— **人类的「按需加载」哲学在 Tuck 已经实现** ✅

## D10 · **Tuck 没有 CI** `[?] 待裁决`

```
.github/workflows/  → 不存在
```
**⇒ D9 那个坑**本来有机会被 CI 抓到**（跑一次带 gateway 的测试就知道），但它**没有 CI** ⇒ **没人跑。**
**⇒ 与 phyt-DNA 的教训同源**（本会话在 phyt-DNA 上刚补了 `tools/ci-local.sh`）。
**⇒ 建议（不擅动）**：Tuck 加一个最小 CI，或至少让它有"一条命令全量自证"的入口（x-ray），
**且该入口必须带 `--all-features`**（否则又是 D9）。

---

## D11 · 并发 / 异步 / 队列 —— 地基形状与三处要盯的坑 `[ ] 待做（不急）`

**人类指示**：*"项目我希望异步、可高并发、可队列，提升未来扩展性和鲁棒性！这是我项目之初没考虑的，
你尽量帮我留意与避坑！或者先跑通再检查并完善也可以？"*

### ★ 先答"先跑通再完善可以吗"：**可以，而且现状的**形状是对的**

```
main.rs:512   // Each request assembles a fresh Helix (build_agent …)   ← 每请求一份【全新】agent
main.rs:548   let mut built = build_agent(&cfg).await;                 ← 在 handler 里新建
main.rs:692   built.agent.run_cycle(&msg).await                        ← 无共享可变 AgentLoop
```

**⇒ 「确定性优先 × 高并发」的张力**恰好两全**：每个请求一份**独立的、确定的**循环 ⇒ **无状态串扰**。**
**★ 而这一条反过来救了另一件事**：`lock().unwrap()` 共 **47 处**（中毒即 panic），
**因为每请求独立，panic 的爆炸半径被限制在**单请求**（不会整服务死）** —— 这比"共享 agent + 一把大锁"安全得多。

### 三处要盯的坑（按风险）

| # | 坑 | 证据 | 为什么是坑 |
|---|---|---|---|
| **1** | **每请求都跑一遍 `build_agent`** | `main.rs:548`；而 `build_agent` 内含 `build_identity_block`（**读文件 + 连 Tentacle 取工具清单**，`main.rs:902`） | **高并发下的延迟与资源成本**：一句话的请求要连一次 Tentacle ⇒ 并发一上来就是放大器 |
| **2** | **无界通道** | `tokio::sync::mpsc::unbounded_channel` ×2（`:593`, `:608`） | **没有背压** ⇒ 负载高时内存涨。与人类要的"队列/鲁棒"正相反（要的是**有界 + 背压 + 具名拒绝**） |
| **3** | **无并发上限** | `semaphore` / `rate_limit` / `concurrency_limit` **0 命中** | 被打满时**没有减速阀**；且"打满"这件事**没人具名**（第 13 条：缺失必须具名） |

### 已有的一件好东西（复用，不要重造）
```
ledger/mod.rs:192-199  UNMET 重入队：unqueueable_unmet() 具名"队列永远发不出去的记录"
                       注释："A queue that silently drops what it cannot schedule is the same [fault]"
```
**⇒ 队列语义**已有**且**具名**（"nothing leaves the queue without being said out loud"）**
⇒ **D11 要做的是给它**加背压**，不是另造一套队列。**

### 处置顺序（不急，按"先跑通"）
1. **不阻塞当前工作**（M1 等继续）
2. **等生态稳了**再做：**① `build_agent` 的可复用部分**（身份块按需缓存，符合"按需加载"）**② 无界 → 有界 + 背压**（满了要**具名拒绝**，不得静默丢）**③ 并发上限 + 具名**（打满时谁被拒、为什么）
3. **判据（可红）**：并发 N 个请求 ⇒ 必须有"上限生效"的**具名**证据；通道满 ⇒ 必须**具名拒绝**而非静默丢

**⇒ 与人类哲学一致**：**按需加载**（身份块不必每请求重取）· **物理事实优先**（背压要看真实队列长度）·
**确定性优先**（每请求独立循环已满足）。
