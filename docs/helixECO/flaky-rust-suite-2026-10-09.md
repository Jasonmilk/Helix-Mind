# 🔴 anaphase 测试套件**不确定**：同一命令两次给出不同失败集合

> 2026-10-09 ｜ 起点：`run_cycle_pipeline` 有 2 条失败，我怀疑是自己的改动 ｜ **未改代码**

## 一、我先前的猜测被否证（记录，因为它是我又一次"猜"）

我怀疑那 2 条失败**由我早先的 `gene_lock_path`（身份块不再为空）造成** ——
依据是失败信息里那句 *"byte-identical ledger, **identity normalised out**"*。
**⇒ 否证：那两条**单独跑都 `exit=0`**。若是身份块的问题，单独跑也会红。**

## 二、★ 实测：**同一命令，两次不同结论**

```
cargo test --test run_cycle_pipeline
  第 1 次： 6 passed · 9 failed     ← {full_chain_met, finalize_reasks_once, finalize_reasks_contradicts,
                                       full_chain_unmet, contradictory_reply_degrades, finalize_never_leaks,
                                       prose_plus_tool_shape, soft_reflex_threshold, deterministic_replay}
  第 2 次：12 passed · 3 failed     ← {full_chain_unmet, soft_reflex_threshold, deterministic_replay}
  每条单独跑：exit=0（全绿）
```

**⇒ 失败集合**在两次之间变化**（第一次 9 条，第二次 3 条，且 6 条"自愈"）。**

## 三、结论（按 Ω 说话）

**同一输入、同一命令、两次不同结论 ⇒ 那些判据**此刻不构成裁决力**。
**⇒ 这不是"代码有 bug"，是"判据本身失效"** —— 而它比一个红更危险：**它会随机地报红或报绿，
于是没人知道该信哪一次**。

**⇒ 与"存活 ≠ 在环"同形**：**"它跑完了"≠"它裁定了"。**

## 四、可能原因（**未验证，不猜死**）

高度疑似**测试间共享可变状态**，候选（均未验证）：
1. `MockTentacle` 等 mock 服务**绑固定端口** ⇒ 并行时互相抢/串
2. 进程级环境变量或配置**被某个测试改动**，影响后续测试
3. 真实 store / 临时目录被多个测试共用

**⇒ 下一步**：找出共享物（先查 mock 的端口绑定与 `ANAPHASE_*` 环境变量的写入点），
然后**要么串行化（`--test-threads=1`）要么隔离**。**在修好之前，这个套件的读数不作数。**

## 五、这也解释了先前一部分"诡异数字"

前几轮观测到 `proven 55 → 77 → 80`、`red 2 → 8 → 6`，我当时归因于 `cdp/panel` 由 down 变 up。
**现在看，至少有一部分来自本仓测试套件自身的不确定性。**
⇒ **P6（账本行必须带环境）因此更有价值**：它让"环境"成为可查字段；
**但要连带记"判据套件是否确定"** —— 或许该有一条 `env.suiteDeterminism: "stable|unstable"`。

## 六、新陷阱第 28 条（建议入册）

**第二十八条 · 判据必须确定。** 同一命令两次给出不同失败集合 ⇒
**这些判据在裁决之前就已经失效**；此时"绿"与"红"都不构成证据。
判据：**同一条命令连跑两次，失败集合必须一致**（否则先修判据，再谈代码）。

---

## 七、缩小范围：**不是并行冲突**（第二阶）

```
cargo test --test run_cycle_pipeline -- --test-threads=1    连跑三次
  第1次 9 passed / 6 failed · 第2次 10 passed / 5 failed · 第3次 9 passed / 6 failed
```

**⇒ 串行**同样不确定** ⇒ 排除"并行抢资源"。**
（Mock 用 `127.0.0.1:0` 随机端口 ⇒ **也排除端口冲突**。）

## 八、★ 分成两族（三次跑出来的结构）

| 族 | 成员 | 行为 |
|---|---|---|
| **一致红** | `run_cycle_full_chain_met` · `run_cycle_full_chain_unmet` · `run_config_soft_reflex_threshold_blocks` | 三次都红 |
| **时红时绿** | `finalize_*`（3 条）· `contradictory_reply_*`（1 条） | 三次不同 |

**而两族的共同点是**：**单独跑都 `exit=0`，进套件就红。**

**⇒ 结论：**同进程内的共享状态** —— 先跑的测试改了它，后面的测试因此改变行为。
（不是并发：串行同样发生；所以是**跨测试的顺序依赖**，而非"同时跑"。）

## 九、下一步的探针（**具体，不必再猜**）

候选（均**未验证**）：
1. **进程级 `static` / `OnceLock` / `LazyLock`**（配置或身份块只初始化一次 ⇒ 第一个测试决定了后面全部）
2. **`std::env` 的读取被缓存**（`env_overrides.rs` 自认有 env race，说明本仓确有 env 依赖）
3. **CWD 或固定路径**（`knowledge_base/fixture-codex.json` 是共享文件；若有测试写它，后面全变）

**⇒ 探针**：`cargo test --test run_cycle_pipeline -- --test-threads=1 --nocapture` 且**二分**到"哪一条测试改变了后续行为"
（先只跑 `full_chain_met`：绿？再把前 N 条一起跑，直到第一条变红 ⇒ 找到污染源）。

## 十、这条线的价值（为什么值得停在这里）

**一个不确定的套件会让后面每一步都不可信** —— 包括 `route_trace` 的 ③④。
而第 28 条给了它一个**机械判据**：**同命令连跑两次，失败集合必须一致**。
**⇒ 在修好之前，本仓的读数只作"线索"，不作"证据"。**

---

## 十一、⚠️ 我的一次**无效测量**（必须记，因为同一类错误本会话犯过两次）

我用 `cargo test --test run_cycle_pipeline <a> <b>` 跑了 14 组"配对"，
得到 **14/14 全部 `exit=1`**，于是写下"配任何一条都红 ⇒ 每个测试都污染"。

**⇒ 那是假的。**

```
error: unexpected argument 'time_anchor_injected_with_full_date' found
Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]
```

**`cargo test` 只接受一个过滤词** ⇒ 那些 `exit=1` 是**cargo 的用法错误**，**不是测试失败**。
**⇒ 「配任何一条都红」整个结论作废。**

**⇒ 这与本会话更早那次同类**（`timeout` 在 macOS 上不存在 ⇒ `exit=127` 被我读成"测试不红"）：
**退出码没有被归因** —— **用法错误的 `1` 与测试失败的 `1` 长得一模一样。**

### 第 29 条（建议入册）· **退出码必须可归因**

`exit=1` 至少有三种来源：测试失败、cargo/工具**用法错误**、环境错误。
**⇒ 判据：任何"红/绿"的读数，必须同时给出"它来自哪个命令的哪种结局"**；
否则 `1` 会被当成裁决。**（与"没有环境的计数不可比"同族：没有归因的退出码也不可比。）**

## 十二、更正后的干净数据

```
# 正确跑法：过滤词是【子串】—— `cargo test --test X run_cycle_` 跑该前缀的全部
run_cycle_ 子集（4 条）· --test-threads=1 · 连跑三次
  第1次  2 passed / 2 failed  →  {full_chain_met, full_chain_unmet}
  第2次  1 passed / 3 failed  →  + {deterministic_replay}
  第3次  2 passed / 2 failed  →  {full_chain_met, full_chain_unmet}

run_cycle_full_chain_met 单独跑三次： exit=0 · 0 · 0     ← 单独【稳定绿】
```

**⇒ 两个不同的病：**

| 病 | 成员 | 性质 | 下一步 |
|---|---|---|---|
| **A** | `full_chain_met` · `full_chain_unmet` | **确定的顺序依赖**（单独稳绿、进子集三次全红） | **可复现 ⇒ 可二分**（用子串过滤跑子集） |
| **B** | `run_cycle_deterministic_replay` | **不确定**（第2次才红） | 先抓失败正文 |

## 十三、关于人类的一个猜测（SA-Core / helix-mind）

人类猜：*"难道跟 helix-mind 的 sa-core 函数有关？"*
**⇒ 部分可答**：这几条测试经 `base_agent` 构造，**用的是 `NoopMemoryAdapter`** ⇒ **不经过 mind**。
**⇒ 但我尚未抓到失败正文**（`--nocapture` 那次跑的是无效命令）⇒ **不确认也不排除**，
**下一步第一件事就是抓正文**（它会直接点名是哪个断言、哪一侧的值）。

---

## 十四、★ 失败正文（终于抓到）—— 形状很清楚

```
run_cycle_full_chain_met   :73   assert_eq!(records.len(), 1)   left: 0   right: 1
run_cycle_full_chain_unmet :98   assert_eq!(records.len(), 1)   left: 0   right: 1
run_cycle_deterministic_replay :143  left: "{…\"status\":\"MET\"…}"   right: ""   ← 第二次运行账本【空】
```

**而它前面的断言都通过了**（`:62-71`：`calls.len()==1` · `evidence.len()==1` · `evidence[0].ok`）。
**⇒ 计划解析了、信封组装了、工具执行了、证据记了 —— 但「stage 5-6 写 verdict」没发生。**
**⇒ 共同形状：同进程里**第二次** `run_cycle` **什么都不写**。**

## 十五、人类猜测（Tuck）—— 部分成立，且发现一条真陷阱

人类猜：*"也许问题在 tuck?!"* ⇒ **`run_cycle` 里**确实**有 Tuck 闸门**（`run_cycle/mod.rs`）：

```
:141  pub gate: crate::gate::Gate,
:721  if let Err(reason) = self.gate.check() { … return Err("TUCK-GATE-REFUSED: {reason}") }
```

**⇒ 但我的"熔断器是共享全局"假设被**结构性排除**：**

```
gate.rs:55   impl Default for Gate { fn default() -> Self {
gate.rs:145/159   inner: Mutex::new(Inner { state: GateState::Closed, opened_at: None, last_reason: None })
```

**⇒ 每次 `Gate::default()` 都造**新实例**，不是进程级全局。**
**⇒ 且测试里 `run_cycle(...).await.unwrap()` **没有 panic** ⇒ 走的**不是** `TUCK-GATE-REFUSED` 那条路。**

**⚠️ 顺手发现的真陷阱（入册）**：

```
gate.rs:165   self.inner.lock().map(|g| g.state).unwrap_or(GateState::Open)
```

**互斥锁一旦中毒（别的线程 panic 时正持锁），闸门就报「开」** —— fail-closed 的方向是对的，
**但它报的是一个**错误的理由**（"下游不可用"与"我们的锁坏了"读起来一样）。**
⇒ 与第 24 条同族：**一个量被当成另一个量。**

## 十六、诚实的计数（我这轮猜了三次，全没中）

| 我的假设 | 结果 |
|---|---|
| 由 `gene_lock_path`（身份块非空）造成 | ❌ **否证**（单独跑也绿…不，是单独跑**绿** ⇒ 与它无关） |
| 是**并行**资源冲突 | ❌ **否证**（串行同样发生） |
| 是 Tuck **熔断器**共享全局 | ❌ **结构性排除**（`Gate::default()` 造新实例；且未走 REFUSED 路） |

**⇒ 剩下的 solid 事实：「证据写了、verdict 没写」、「单独跑绿、进子集红」。**
**⇒ 下一步应该是**读 verdict 的写入路径**（stage 5-6），而不是继续猜共享物。**
**⇒ 在查清之前，本仓读数只作线索（第 28/29 条）。**

---

## 十七、★ 结构性发现（solid，证据是"grep 为空"）

### A. `run_cycle` **不调** `pipeline.run()`，而是**自己重做编排**

```rust
// src/run_cycle/mod.rs:1149
let Some(pipeline) = self.pipeline.as_mut() else { … };
match pipeline.execute_calls(&job, &labels).await {       // ← 直接调 stage 3，不走 run()
    Ok(records) => { … pipeline.record_evidence(records.clone()); … Ok(Success) }
    Err(e) => { warn!("[Execution] Pipeline execution failed: {}", e);
                Ok(TransitionCondition::Failure) }        // ← Err 被吞成状态
}
```

**⇒ 两条必须保持一致的编排路径**（`pipeline::run()` 与 `run_cycle` 的手工编排）——
本会话反复遇到的那个形态：**同一件事有两个实现，而只有一个被人读。**

**⇒ 且 stage 3 的 `Err` 只进 `warn!`**：测试看到的是"什么都没写"，**不是"为什么"**（具名缺失）。

### B. 那个 `Err` 分支**没有**被走到（有证据）

失败那次的 `--nocapture` 日志里 **`[Execution]` 警告 0 条** ⇒ `execute_calls` **成功**了
（这也与测试 `:68-70` 的 `evidence.len()==1` 通过一致）。
**⇒ 所以"没写裁定"不是 stage 3 出错造成的。** 我这条假设也**排除**。

### C. ★ 一个词指两个东西：**「verdict」**

| 名字 | 在哪 | 含义 |
|---|---|---|
| `LedgerRecord::Verdict { status: Met\|Unmet }` | `pipeline/mod.rs` | **账本裁定**（测试读的是它） |
| `PeriodVerdict` / `EndReason`（Completed/Impasse/UndefinedTransition/CycleCapExhausted） | `src/run_cycle/verdict.rs` | **周期结束原因**（HTTP 体里那个） |

**⇒ 同一词汇两个语义** ⇒ 与"一物一名"（A5）冲突。查一条失败时，"verdict"指哪一个必须先问。

### D. `run_cycle` 里**没有**写账本裁定的调用

`grep -nE "build_verdict|ledger\.append\(verdict" src/run_cycle/mod.rs` → **空**。
**⇒ 测试读到的 `LedgerRecord::Verdict` 来自别处 ⇒ 那就是下一步要读的那一行。**

## 十八、诚实计数：我这轮**四个假设，全未中**

| # | 假设 | 结果 |
|---|---|---|
| 1 | `gene_lock_path`（身份块非空） | ❌ 否证 |
| 2 | 并行资源冲突 | ❌ 否证（串行同样发生） |
| 3 | Tuck 熔断器共享全局 | ❌ 结构性排除（`Gate::default()` 造新实例） |
| 4 | stage 3 `Err` 被吞 | ❌ 排除（0 条 `[Execution]` 警告） |

**⇒ 共同教训**：我四次都是**从"形状"推断机制**（"第二次就不写" ⇒ 猜"全局"）。
**⇒ 而每次真正推进的，都是"去读那条路径"**（读 `run()`、读 `execute_calls` 的调用点、读 `verdict.rs`）。
**⇒ 这就是本会话反复出现的那条**：**不要从形状推机制，去读代码**；形状只用来**选择读哪里**。

## 十九、下一步（**具体到行**）

1. 找 `pipeline.ledger.append(LedgerRecord::Verdict…)` 的**全部**调用点，确认测试那条路径上**谁**写它
2. 若只有 `pipeline::run()` 写 ⇒ 则 `run_cycle` 的测试**本不该有裁定** ⇒
   **"单独跑绿"这一观察本身需要复核**（它可能绿在别处，例如 `PipelineOutcome` 被忽略）
3. 顺带：C 项（「verdict」一词两义）应作为命名债务登记

---

## 二十、★ 对人类「居然没有遵守极致复用?!」的回答：**复用是有的，编排有两个**

**先说公道话**：`reflection.rs:48` **调用了** `pipeline.build_verdict(...)` —— **没有重新实现判据/裁定的逻辑**。
**⇒ 所以"没复用"这个判断不准确。**

**但"编排"确实有两份：**

| 路径 | stage 3-4 | stage 5-6 |
|---|---|---|
| `pipeline::run()` | `execute_calls` + `record_evidence` | `check_results` + `build_verdict` + `ledger.append` |
| **活路径** `run_cycle` + `Reflection` | `run_cycle/mod.rs:1149` **自己调** `execute_calls` + `record_evidence` | `reflection.rs:24` → `build_verdict` + `append` |

**⇒ 「六个阶段由谁串」有两份实现，而只有一份被人读** ⇒ 这正是人类感觉到的那个东西，
**也是本会话第三次遇到同一形态**（面板的窗口、`loadScreen`、以及这里的编排）。

**⇒ 而且这解释了为什么失败查起来那么难**：读 `pipeline/mod.rs` 会以为裁定的写入条件是
"stage 5-6 全部通过"，而**活的那条路根本不走它** —— **读错了文件，就会一直猜。**

## 二十一、写入点已定位（含两道守卫）

```rust
src/run_cycle/reflection.rs
:24  pub(super) async fn arm_reflection(...)            // HelixState::Reflection 时被调（mod.rs:1056）
:28      if !self.context.evidence.is_empty() {         // 守卫一：证据非空
:29          if let Some(pipeline) = self.pipeline.as_mut() {   // 守卫二：pipeline 已接线
:48              let verdict = pipeline.build_verdict(&job_id, evidence_ids, &reports, None);
:55              pipeline.ledger.append(verdict);        // ← 写入
```

**而测试的 `evidence.len()==1` 断言**通过**了 ⇒ **守卫一满足**。
**⇒ 所以"没写"要么是**状态机没走到 Reflection**，要么是**写入之后被覆盖**——但**这两条我都没验证**。

**⚠️ 顺手一个可疑形状**：`arm_reflection` 的函数体**缩进得像被包在一个不存在的块里**
（首句 12 空格、函数头 8 空格）⇒ 像是某次编辑拿掉了一层 `if` 留下的。**未改动、仅登记。**

## 二十二、本轮的净收益（虽然没找到根因）

| 得到 | 价值 |
|---|---|
| 失败正文（证据写了、裁定没写） | 形状，不再是"玄学抖动" |
| **四个假设全部否证**（含两个结构性排除） | 省下后来人四次白走 |
| **编排有两份**、活的是 `run_cycle`+`Reflection` | **查错必须先确认走的是哪条路**（否则读错文件就一直猜） |
| 写入点与两道守卫具名 | 下一步只需验两条：是否走到 Reflection / 是否被覆盖 |
| **「verdict」一词两义**、`gate.rs:165` 中毒报 Open | 两条命名/归因债务，可各自立 ADR |

**⇒ 在修好之前，本仓读数只作线索（第 28/29 条）。**
