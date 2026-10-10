# 注册那 4 个（2026-10-09）：做了一半，**落点判错了**

## 实测（面板 + local-llm 在跑）

| | 注册前 | **注册后** |
|---|---|---|
| proven | 71 | **74** |
| red | 3 | **3**（**同一批成员，没多也没少**） |
| held | 6 | 6 |
| **unregistered** | 4 | **6**（我加了 4 个，其中 2 个又被标） |

**4 个的结局**：

| 套件 | 结局 |
|---|---|
| `chain_e2e_test.js` | **PASS** ✅「THE CHAIN, END TO END — automatically (§261)」 |
| `newest_first_contract_test.js` | **PASS** ✅「the newest-first dependency, declared on the Cellrix side (§272)」 **但仍被标** `UNREGISTERED/BLOCKING — capability panel-http IS present (probed), yet this suite is unproven` |
| `fork_entry_test.js` | ⚠️ 仍 `UNREGISTERED/BLOCKING — **not registered in deferrals.json**` |
| `session_addressable_test.js` | ⚠️ 同上 |

## ★ 落点判错了（本轮最该记的一条）

我把它们加进了 `run_all.js` 的 **roster（`SELF_CONTAINED`）**。结果显示：

- **roster 只决定"跑不跑"**（两套因此跑起来并通过 ✅）
- **"registered" 的判定在 `deferrals.json`**（`run_all.js:683` 原话：
  `return { kind: 'unknown', why: 'not registered in deferrals.json' }`）

⇒ **一个套件要被"看见"，需要两处**：roster（跑）+ `deferrals.json`（身份）。
**我只做了一处，所以 unregistered 从 4 涨到 6** —— 注册动作本身可以制造新的 unregistered。

> 与"一个正确的修复可能当场制造新缺陷"同形：**一个正确的注册，可以当场制造新的未注册。**

## 下一轮（精确到行）

1. 在 `deferrals.json` 给 `fork_entry_test.js` / `session_addressable_test.js` 身份
   （无 `REQUIRES` ⇒ 它们不需要能力，缺的是**登记本身**）。
2. 查 `newest_first_contract_test.js`：它**已经 PASS**，却仍被标
   「capability panel-http IS present, yet this suite is unproven」 ⇒
   这是**判定器与实际结局不一致**，属第 10 条（检查会静默吞行）的同族，**要查清**。
3. `refs_round_trip_test.js`（绿→红）—— 人类指出它可能指向最初的痛感（**会话寻址**），**优先查**。
   本轮未查（预算用尽）。**两种可能都要写清**：回归 vs 从未绿过。

## 本轮新增的两条纪律（已在上一份记录，此处重申）

- **沉默不是"没红"，是"未知"**。实测两次命中（38 HELD 里 3 条、8 个可跑里 1 条）⇒
  **命中率 8%~12%** ⇒ 剩下 6 个 HELD 仍**不能判"清了"**，只能说"又清掉一批"。
- **禁止从 HELD / env-missing 标签推断"没有隐藏红"；唯一可靠的读法是让它跑。**
