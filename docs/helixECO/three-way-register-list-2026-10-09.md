# 三处不一致清单（roster / REQUIRES / deferrals.json）—— **先要表，不修**

生成方式：解析 `run_all.js` 的 `SELF_CONTAINED` + 每个套件的 `REQUIRES` + `deferrals.json`
的 `requires`/`deferrals`/`exempt_files`。套件总数 **88**，roster **80**，声明 `REQUIRES` **29**。

## 一、清单**当即否证了我自己的两条假设**（这就是先要表的价值）

### ❌ 假设 1：「roster 有、deferrals 无 ⇒ 不一致」

**错。** 那是 **~60 个套件的正常状态**（`acceptance_test` / `assembly_test` / `chain_merge_test` …
全都是）。⇒ **roster 成员资格本身就是普通套件的注册**。
**因此我上一轮提的 Goal 0 原始设计（roster 有 / deferrals 无 ⇒ 红）是错的**，
接了会一次点亮 60 条假红。**已撤回。**

### ❌ 假设 2：「`REQUIRES` 不在能力表 ⇒ 越界」

**是我列表的假阳性。** `jsdom` 不在能力表（表里是 **`jsdom-dom`**），
因为跑器有一行**显式别名**：`const alias = (cap === 'jsdom') ? 'jsdom-dom' : cap;`
—— **我没实现那条别名**，于是 14 条正常套件被我的脚本标成越界。

⇒ **一份会点亮 78 条的红清单，等于没有清单**（第 8 条 Ω 判别力 + 第 10 条同族）。
**这两条假阳性是这张表自己暴露的，不是我又去猜的。**

## 二、**真正**三处都没有的（去掉上面两类假阳性与"按能力条件入册"的设计）

| 套件 | 状态 | 判定 |
|---|---|---|
| `chat_model_test.js` | 不在 roster、无 `REQUIRES`、不在 deferrals | **真·未注册** |
| `port_table_test.js` | 同上 | **真·未注册** |
| `render_test.js` | 同上（`REQUIRES=jsdom`） | **真·未注册** |
| `all_views_test.js` | 不在 roster，`REQUIRES=panel-http` | **设计如此**：`run_all.js:218` 当面板可达时**推入** roster |
| `layout_test.js` · `measure_test.js` · `perf_measure.js` · `hit_targets_test.js` | 不在 roster，`REQUIRES=cdp-browser` | **设计如此**：`:221-226` 当 CDP 可达时推入 |

⇒ **真正要处置的是 3 个：`chat_model_test.js` · `port_table_test.js` · `render_test.js`**
（另外 `chat_model_test.js` 的 `run_all.js:81` 注释还写着「**NOT here: it exits 3 without a live panel**」
—— 它**有**一个手写理由，但**不在任何账上**。）

## 三、Goal 1 的答案：**我的注册模型错了，而且第三处我没找到**

跑器实测（同一批、同一个 roster 插入）：

| 套件 | in roster | REQUIRES | 结局 |
|---|---|---|---|
| `chain_e2e_test` | ✅ | `local-llm` | **PASS** |
| `newest_first_contract_test` | ✅ | `panel-http` | **PASS**（但仍被标） |
| `fork_entry_test` | ✅ | 无 | **not registered** |
| `session_addressable_test` | ✅ | 无 | **not registered** |

而 **~60 个"无 REQUIRES + 在 roster"的普通套件是正常跑的**。

⇒ **差别不在 REQUIRES 本身**（否则那 60 个也不跑）。
⇒ **有一个我还不会的第三因素**（可能是一份单独的允许名单，或 `classify()` 之外的一层判定）。

**⇒ 我不猜。** 下一步是把 `classify()` 及其**调用方**一并读完（`run_all.js:600-660` 与调用点），
而不是再改一次文件。

## 四、本轮结论（三句话）

1. **Goal 0 的原始设计（roster/deferrals 不一致 ⇒ 红）作废** —— 会把 60 条正常状态报成红。
2. **清单的第一版有两条假阳性**（别名、roster 即注册）——**它自己暴露的**，已修正。
3. **真·未注册是 3 个**（`chat_model_test` / `port_table_test` / `render_test`），
   加上 Runner 报的那 2 个（`fork_entry` / `session_addressable`）**另有第三因素未查明**。

**没有新代码**：这一轮按人类要求**只取表**，不动手修。
