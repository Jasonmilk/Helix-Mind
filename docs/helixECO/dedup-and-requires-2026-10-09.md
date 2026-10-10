# 去重 + 补 REQUIRES 声明（2026-10-09）—— 三条转述**全部核实成立**

## 一、先复核（不凭转述）

| 你的结论 | 我的复核 |
|---|---|
| 4 个 roster 条目**各重复一次** | ✅ **且比转述更狠**：它们**本来就在** `SELF_CONTAINED`（`fork_entry:116` / `session_addressable:117` / `newest_first:72` / `chain_e2e:75`）—— **我上一轮的"注册"纯粹是重复插入** |
| 「第三因素」不存在，是**声明机制** | ✅ `run_all.js:789` 是 `UNREGISTERED/BLOCKING` 唯一来源；不声明 ⇒ `extra=[]` ⇒ 拿不到面板地址 ⇒ NEEDS-INPUT；跑器 `~:433` 的注释**已记录同一坑**（先例 `events_param_contract_test.js`） |
| `proven` 因此**虚高** | ✅ 去重后 **74 → 72（−2）** |

**我上一轮猜的"路径归一化不一致"撤回。** 修法就是**撤回重复项 + 在文件里声明 `REQUIRES`**，
不是改 roster、不是改 `deferrals.json`。

## 二、做了什么

1. **撤掉重复的 4 条**（roster 恢复原状），并留一条注释记下"重复 = 双重计数"。
2. **给 `fork_entry_test.js` / `session_addressable_test.js` 补 `const REQUIRES = 'panel-http';`**
   （附注释指向 `:789` / `:433` 的机制与先例）。

## 三、实测（面板 up，**同时报进项与出项**）

| | 去重前 | **去重后** |
|---|---|---|
| `proven` | 74 | **72**（**−2**） |
| `red` | 3 | **2**（−1） |
| `held` | 6 | 6 |
| `unregistered` | 6 | **5**（−1） |
| 那 4 套出现次数 | 各 **2** | 各 **1** ✅ |

**两条矛盾的消失**：

- `newest_first_contract_test`「已 PASS 却仍报未注册」⇒ **消失**（同一成员被数了两次）。
- `refs_round_trip_test` 的假红 ⇒ **消失**（Goal 0 的前置分类器把它变成具名 HELD/前置）。

## 四、仍未清（下一轮）

| # | 事项 | 备注 |
|---|---|---|
| 1 | **3 个真·未注册**：`chat_model_test` / `port_table_test` / `render_test` | **优先判 `render_test.js` 是否为 `panel_tree_test.js` 的旧副本**（若是 ⇒ 按 A5 禁止第二份清单合并/具名） |
| 2 | `prove_track_rows_test` 夹具（**崩溃不是断言红**） | 未做 |
| 3 | `agent_loop_probe_test` 登记为 Rust 侧（**保持会红**） | 未做 |
| 4 | **接闸门**（真红阻断 / 前置缺失具名不阻断 / 未注册具名提示） | **必须在 fresh clone 上验一次**（mtime 那类"本地绿、CI 随缘"只有这样才会现形） |
| 5 | **人类用一次真面板**（起面板、开几轮、从中间续接、看沉淀区） | 所有判据至今只在合成的 21 轮链上跑过，**真实使用一次都没做过** |

## 五、战略

infra 这一支已跑七轮。**剩下的是两件小事**（3 个未注册定身份 + 两个夹具/登记）**加一件正事**（接 CI + 真面板一次）。
**收尾 infra，然后停。**
