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
