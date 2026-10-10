# A–E 结论（2026-10-09）

## A ✅ 修好 `p64_env_class_test` 的模拟（守卫重新有人看守）

**根因**：它用 `delete env.NODE_PATH` 模拟"jsdom 缺席"。而 jsdom 一旦可**本地安装**
（`web/tests/node_modules`），Node 从**请求文件所在目录**解析裸 require ⇒ 擦 `NODE_PATH`
**再也去不掉这个能力** ⇒ 模拟静默变成空转（`0 suite(s) held`）。

> **一个正确的修复，当场制造了新缺陷** —— 装 jsdom 是对的，但它弄坏了"模拟缺席"的能力。
> **缝错了，不是依赖错了。**

**修**：模拟缝改为**显式** `CX_NO_JSDOM`（`run_all.js` 的探针与环境行都遵守它），
并把这个理由写在代码里。变异**仍然运行在无旗标的一侧** ⇒ 检查非空转。

**实测**：`p64` → `ok ×4`（`27 suite(s) held for the absent capability`；变异 `good run held 0`），**exit 0**。

**回归检查（"回头看它破坏了什么"）**：`run_all.js` = **2 red / 66 proven / 12 held**
（修前 3 red / 65 proven）⇒ **净 +1 proven，没弄坏别的**。

## B 三个入口：**该实现而没实现**，且**文件自己就这么写着**

`agent_loop_probe_test.js` 的文件头原文：

> "**Each probe is red today because the entry does not exist — a real, attributable red — and
> turns green when the proposition becomes true.**"
> "SURFACE (owner's note, §204.6): these probes exercise the JS surface (projection + view).
> **The Loop itself is a RUST concern (Anaphase); its probes must target the Rust entry point and
> are DECLARED ABSENT below rather than faked here.**"

⇒ **Q1：是"该实现而没实现"** —— 但它是**自声明**的真红（不是"看着像有"），符合 ADR-0049 §6.2.2 的体例。
⇒ **Q2：你的推断成立，且文件自己已经写下**：`retryBranch` / `runLoop` / `replay` 是
Anaphase 侧刚做完那些东西（分支/驳回 ↔ retryBranch；循环 ↔ runLoop；重放 ↔ replay）的
**Cellrix 侧对应物**，而**循环本身是 Rust 的事**。
⇒ **登记并报告，本轮不实现**（与你的裁决一致）。

## C `prove_track_rows_test`：**不是断言红，是崩溃**

```
TypeError: Cannot read properties of undefined (reading 'project')
    at compactGroupsOf ... var cellProj = window.CxCellMetering.project(...)
```

⇒ **夹具缺陷**：它的 sandbox **没有加载 `cell_metering.js`**，所以 `window.CxCellMetering` 是 undefined。
**既存缺陷，此前不可见**（jsdom 缺席时它 exit 3，被记为 env-missing）。
**未修**（本轮预算用尽）——修法是让该套件的 sandbox 加载 `cell_metering.js`。

## D 剩下 12 个 HELD：**全是具名服务依赖**（但我不下"没有隐藏红"的结论）

| 缺什么 | 套件 |
|---|---|
| `panel-http absent` ×6 | `newest_first_contract_test` · `events_param_contract_test` · `three_state_rows_test` · `asset_parity_test` · `refs_round_trip_test` · `layering_test` |
| `local-llm absent` ×2 | `chain_e2e_test` · `s303_http_e2e_test` |
| `cdp-browser absent` ×3–5 | `s303_continuation_test` · `layout_test` · `measure_test` · `perf_measure` · `hit_targets_test` |
| `deferral` ×1 | `prove_track_nodes_test`（owner jason · expires 2026-11-30） |

**⚠️ 只给分类，不给结论。** 上一轮"env-missing"里就藏着 **3 条真红** ——
同样的形状不能再用同样的方式读。**这 12 个里有 8 个（panel-http + local-llm）依赖的服务
我们**起得来**** ⇒ **下一轮应当起服务再跑一遍**，让它们真的执行，而不是凭标签断言它们"只是缺服务"。

## E ✅ 端口被占：**修正前提 + 修好**

**先修正我自己的前提**：`cellrix-web --port 8080`（llama-swap 占用）**并不静默** ——
它 `exit=1` 且打印 `AddrInUse`。**是我把输出管进日志没看**。又一次"没看就下结论"。

**真正缺的两条**：① 不点名占用者；② 是裸 `Debug` 转储，且**横幅印在失败之前**（读起来像"起来了"）。

**修后实测**：

```
cellrix-web: FAILED TO START — port 8080 is held by another process.
             (this is not the panel; the panel is not running)
             occupant:
               llama-swa 90117 jason ... TCP 127.0.0.1:8080 (LISTEN)
             fix: choose a free port, e.g. --port 18932
exit=1
```

`lsof` 是 **best-effort**：取不到名字**不等于**没有失败。
