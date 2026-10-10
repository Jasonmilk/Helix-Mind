# 三目标结论（2026-10-09）：现场是否干净 / 一直信的绿是不是真的 / 数据过没过代理

## 目标 1 —— `Date.now()` **不在渲染输出中**（结论给全）

实测：`web/assets/chat.js` 的三处 `Date.now()` 都在
`d.note({ …, at: Date.now() })` —— **S303 指针解析的记账**，不是行渲染。

去向（逐跳）：

```
d.note(state) → noteS303(state) → Cx.state.s303 = state     // 只进状态
s303dbg(...)  → 受 window.CX_S303_DEBUG 门控的 console.log  // 只进调试台
```

**消费者**：`grep -rn "s303" web/assets/*.js` 除 `chat.js` 自身**零命中**
⇒ **没有任何渲染路径读它**。而行渲染在 `panel_tree.js`，该文件**本就没有**
`Date.now()` / `Math.random()`。

⇒ **结论：渲染输出对时间/随机是确定的。** 且**不注入任何东西**的那条断言
（`panel_tree_test.js` 的 `PURE: 同一 state 渲染两次 ⇒ 输出逐字节相同`）**独立成立**
—— 它不依赖 `Math.random()` 变异那条。

## 目标 2 —— 真相是 **(a)**，而且比 (a) 更细一层

`node web/tests/run_all.js` 实跑：

```
FAILED — 1 red, 44 proven, 38 held, 0 unregistered
  [E: cdp=down panel=down siblings=3/3 jsdom=no], 2 env-missing (not red)
```

| 事实 | 证据 |
|---|---|
| **jsdom 本机缺失** | 环境行 `jsdom=no`；且每个 jsdom 套件被记为 `HELD [requires] jsdom absent **(probed)**` |
| **回归网并没有没跑** | **44 套真的跑并证明**；`jsdom` 缺失影响的是另一批，且**被具名记为 env-missing，不计入绿** |
| **跑器不骗人** | 它把未声明 REQUIRES 却用了 jsdom 的套件**照样按 jsdom 依赖计数**并打 `NOTE`（P64）—— 声明缺失不会变成绿 |
| **没有 CI 在门它** | Cellrix **无 `.github/workflows`**；`run_all.js` 是唯一闸门，且它**具名列出成员**（"a count is not attributable — name the members"） |
| **有一处真红** | `code_language_test.js (exit 1)` —— 既存，与 jsdom 无关 |

⇒ **我上一轮"30 个文件从不执行"的读法不完整，现更正**：它们**是** roster 成员，
今天被记为 **env-missing / HELD**，**不是**被当成绿。
**"8 套全绿 / 55 断言全过"与今天的状态并不矛盾** —— 那是 jsdom/面板在场时的读数。

**`exit 3` 会不会被静默吞掉**：不会被吞，但**也没有 CI 会因此失败** ——
`2 = environment missing` 是**正确**的归类（环境缺失不是代码红），
且它**在汇总里被计数并具名**。真正该担心的是**没有 CI**，不是 3 这个码。

## 目标 3 —— 字段**穿过代理**（HTTP 层，零依赖）

```
anaphase 直连 :50061/v1/sessions
  {"status":null,"gist":null,
   "rejection_log":["2026-10-08T23:25:32Z | human | reject | 因为该轮出现蓝色风车，所以不吸收"],
   "rejected":true}

经 Cellrix 代理 :18932/api/sessions
  {"status":null,"gist":null,
   "rejection_log":["2026-10-08T23:25:32Z | human | reject | 因为该轮出现蓝色风车，所以不吸收"],
   "rejected":true}
```

⇒ **逐字相同 ⇒ 穿过代理成立。**
（`status`/`gist` 为 `null` 是对的：`.state` 只由显式 `!settle` 写，`.gist` 由骨架**按需**产出。）

**顺带撞到一个既存坑**：`cellrix-web` 默认端口 **8080 与 llama-swap 相撞** ——
正是 `Cellrix:ADR-0046`（面板端口不得与 llama.cpp 相撞）记的那条，源码注释里也自认
"18932 in the shell launcher, 8080 in this binary"。我用 `--port 18932` 绕过。
**未修**（不在本轮范围）。
