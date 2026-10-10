# 目标 1（起服务再跑）：**账上的东西真的跑起来，并当场抓到第二条真红**

## 实测（`node web/tests/run_all.js`，anaphase :50061 + 面板 :18932 在跑）

| | 起服务前 | **起服务后** |
|---|---|---|
| 环境行 | `panel=down` | **`panel=up`** |
| proven | 66 | **71** |
| held | 12 | **6** |
| red | 2 | **3** |
| unregistered | 4 | 4（成员变了） |

⇒ **+5 套真的跑并证明**；同时**冒出一条新的真红**。

## ★ 这是 D 的实证答案

我上一轮**拒绝**凭标签断言"12 个 HELD 只是缺服务、没有隐藏红"。**结果：有。**

| 新暴露 | 内容 |
|---|---|
| **`refs_round_trip_test.js` FAIL** | 「the POINTER lives in the server: round-trip, dangling refusal, and a browser with EMPTY localStorage recovers it (§310)」—— **真红，此前被 `panel-http absent` 的沉默盖住** |
| **`newest_first_contract_test.js` → UNREGISTERED/BLOCKING** | 跑器原话：**「capability panel-http IS present (probed), yet this suite is unproven — that is not an absent capability」** |
| `events_param_contract_test` · `three_state_rows_test` · `asset_parity_test` · `layering_test` | **PASS**（4 套，此前全是 HELD） |
| `2 skipped (criteria that RAN NOTHING: nothing to exercise)` | 跑器把"跑了但什么都没跑"单列 —— 又一个"沉默"被具名 |

**⇒ 规律已经出现两次**：沉默（env-missing/HELD）里**每次都藏着东西**。
**唯一可靠的读法是让它们跑。**

## 剩余（下一轮）

| # | 事项 | 状态 |
|---|---|---|
| 目标 2 | 修 `prove_track_rows_test` 夹具（未加载 `cell_metering.js` ⇒ 崩溃，**不是断言红**） | 未做 |
| 目标 3 | `agent_loop_probe_test` 登记为 **Rust 侧**探测（**保持它会红的形态，不改跳过**） | 未做 |
| 目标 4 | 4 个 unregistered 给身份（注册 / 或声明不是测试并移出） | 未做 |
| 目标 5 | 接闸门（真红阻断 / 环境缺失具名不阻断 / 未注册具名提示） | 未做 —— **现在 3 red + 4 unregistered，接了就是永久红灯** |

## 人类新提的三条纪律（已采纳，写进记录）

1. **`4 unregistered` 必须有身份** —— 与 HELD 不同：**HELD 在账上但不跑；unregistered 是根本不在账上**。
   处置二选一：注册进 roster，或明确声明不是测试（并移出测试目录）。**不许悬着。**
2. **能力变成默认可得后，"模拟它缺席"的测试会静默失效。**
   实例：jsdom 本地安装后擦 `NODE_PATH` 再也模拟不了缺席（`0 suite held` = 空转）。
   规矩：**模拟缺失必须走显式缝**（如 `CX_NO_JSDOM`），**且缝本身要有哨兵**（变异侧 held 0 ⇒ 红）。
3. **每个修复都做"修复前后总数对比"**（便宜且能当场抓到"修复制造新缺陷"）。
   实例：A 的 `3 red/65 proven → 2 red/66 proven` = 净 +1 没弄坏别的。
