# 目标 1 定性：`refs_round_trip_test.js` 的红 —— **不是回归，也不是文档说谎；是我的测量无效**

## 一、它到底哪一条红（14 过 / 1 红）

**全过的关键条目**（会话寻址**没坏**）：

- 「a browser with EMPTY localStorage recovers the pointer **FROM THE SERVER**」
- 「the ref act was written **INTO the target period's stream**（one book, not two）」
- 「a NAMED ref per conversation is writable and reported」
- 「nesting is allowed and **TRAVERSAL is still refused**」
- 「an unreachable server leaves the client on the CACHE, **and it DECLARES that**」

**唯一红**：

```
FAIL M2-③: the SECOND book is no longer written
  [legacy 2026-10-01T13:34:34.838Z <= stream 1969-12-31T23:59:59.999Z]
```

## 二、根因（读代码，不是猜）

```js
const legacy = <repo>/.helix/events/.refs/.log.legacy-v315;         // 仓库内 fixture
const stream = <repo>/.helix/events/<period>.events.jsonl;          // ← 硬编码「工作区默认 store」
const mt = (f) => (fs.existsSync(f) ? fs.statSync(f).mtimeMs : -1);
ok('…the SECOND book is no longer written…', mt(legacy) <= mt(stream));
```

实测里 `stream 1969-12-31T23:59:59.999Z` **就是 `mt = -1`** ⇒ **那个文件不存在**。
因为**我把 anaphase 跑在 `/tmp/rr/events`**（临时 store），而这条断言**硬编码读工作区默认 store**
⇒ 拿不到文件 ⇒ `-1` ⇒ `2026-10-01 <= -1` 为假 ⇒ **必红**。

## 三、结论（两个答案都给）

| 问 | 答 |
|---|---|
| **是回归吗？** | **不是。** 寻址功能 14/15 通过，指针往返、单本账、命名 ref、越界拒绝、离线声明全部成立。 |
| **是从来没绿过、文档抄来的？** | **不是。** 它在**默认 store** 下可以绿 —— **VISION v3.0 §二 那句不作废**。 |
| **那红是什么？** | **我的测量无效**：我改了它的前置条件（store 位置），却拿它的结论当"绿→红"。 |

> **按你们自己的 §四「声明是线索，判据是真相」**：这里的判据**没有**推翻声明，
> 它暴露的是**一个我造成的环境偏离**。**差一步就把"我的错误"记成"产品的回归"。**

## 四、这条红的真实价值：一个**硬编码耦合**被暴露（既存）

该套件假设 `session_events_path` 是**工作区默认值**。这与本仓自己的纪律相冲 ——

- `run_all.js` 的 `local-llm` 探针**刻意从 FlowModus registry 读地址**，注释写明
  「**the probe cannot disagree with the thing it probes（0 hardcoded ports）**」；
- 而这条断言**硬编码 store 路径**，于是**它会因为"store 搬到别处"而红**，
  与"第二本账是否还在写"**无关**。

⇒ **真实缺陷是耦合，不是行为**。处置（下一轮，不属本轮范围）：
让它像 `local-llm` 探针一样**从链的声明里取 store 位置**；或**显式声明它需要默认 store**
（`REQUIRES='workspace-store'` 并配探针），让"前置不满足"表现为 **HELD（具名）**，而不是 **FAIL**。

> 这正是本轮反复出现的同一形状：**一个前置不满足，被报成了一个功能失败。**

## 五、本轮**未做**（明说）

- **目标 0（让"半程注册"自己变红）**：**未做，而且我拒绝盲改**。
  原因：`chain_e2e_test.js` 我加进 `SELF_CONTAINED` 后**跑通并通过**，
  而同样加进去的 `fork_entry_test.js` **仍报 not registered** ——
  **两者行为不一致**，说明 `registered()` 查询的**不是我改的那个数组**。
  **在没读准 `registered()` 之前动手，就是本会话已经犯过一次的"改错文件"**。
  下一轮第一步：读 `run_all.js:619-683` 的解析顺序，再落 Goal 0。
- 目标 2/3/4/5：未做。
