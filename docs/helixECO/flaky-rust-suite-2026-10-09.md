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
