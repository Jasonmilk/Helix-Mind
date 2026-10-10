# Goal 0：让"没摆对"不再伪装成"东西坏了"（2026-10-09）

## 判据（人类给的，一条）

> 任何套件的前置不满足（store 不在预期位置 / 依赖模块没加载 / 服务没起）
> ⇒ 表现为 **HELD 并具名原因**，**不得表现为 FAIL**。

## 本次落地的那一格：`refs_round_trip_test` 的 store 前置

**修前**：`mt(legacy) <= mt(stream)` 硬编码**工作区默认 store**。anaphase 指向别处
（我的 `/tmp/*/events`）⇒ `stream` 不存在 ⇒ `mtime = -1` ⇒ `2026-10-01 <= -1` ⇒ **FAIL**。
**读起来是"功能坏了"，事实是"前置未满足"。**

**修后**（实测）：

```
exit=3
NEEDS-INPUT: workspace-store absent — /Users/.../run-8bba...-p006ac835af000000.events.jsonl does not
exist, so "the second book stays untouched" cannot be judged (anaphase is pointed somewhere else).
This is a PRECONDITION, not a red.
```

⇒ **同一情境，从 FAIL 变成具名 HELD。** 其余 14 条仍照常跑并通过。

## 顺带查的一处（人类点名）：**mtime 不是一个判据**

`mt(legacy) <= mt(stream)` 用**文件 mtime** 做判据。而 **mtime 是签出（checkout）的属性，不是代码的属性**
—— fresh clone 上所有文件的 mtime 都是签出时刻 ⇒ 该比较**无意义**，
于是这条判据**本地绿、在 CI 上随缘**。

**结论**：能活下来的断言是「**legacy 文件的字节未变**」（内容哈希），不是"它的 mtime 更旧"。
**已登记，本轮不改写**（改机制是另一件事，需单独裁决）。

## Goal 1 剩下的一半（**没读准，不猜**）

我读到了解析链的一段：`requires`（capability 探针）→ `deferrals` → 否则
`{ kind: 'unknown', why: 'not registered in deferrals.json' }`（`run_all.js:683`）。

**但 roster（`SELF_CONTAINED`）在"registered"里扮演什么角色，我没读准** ——
证据是它自相矛盾：`chain_e2e_test`（我加进 roster）**跑通并通过**，
而同样加进 roster 的 `fork_entry_test` **仍报 not registered**。
差别是前者声明了 `REQUIRES='local-llm'`（命中 `requires` 表），后者什么都没声明。

⇒ **下一轮第一件事**：读准"没有 REQUIRES 的套件靠什么注册"。**在读准之前不动手**
（本会话已经犯过一次"改错文件"）。

## 其余目标

| # | 事项 | 状态 |
|---|---|---|
| 2 | `deferrals.json` 给 `fork_entry_test` / `session_addressable_test` 身份 | 未做（依赖上面读准） |
| 3 | `newest_first_contract_test`：已 PASS 却仍报未注册 | 未做 |
| 4 | `prove_track_rows_test` 夹具（崩溃不是断言红） | 未做 |
| 5 | `agent_loop_probe_test` 登记为 Rust 侧（保持会红） | 未做 |
| 6 | 复跑 + **同时报进项与出项** | 未做 |
| 7 | 接闸门 | 未做 —— **前置未清** |
