# 装盘判定：**机制已建好且严格** —— 差距不在这里，在窗口（已修）与那一味缺的菜

> 2026-10-09 ｜ 人类：*"证轨可以审计所有环节…菜好准备，只差装盘了"* ｜ **未改代码**

## 一、我先撤回自己刚才的一个怀疑

我怀疑 `prove_track.render.js:191` 的 `tpl: '{tool} · {ok} · {durationMs}'` 解析不出值
（模板 camelCase，事件是 `duration_ms`）。**核实后不成立：**

```js
// event_family.js:136 —— 声明式的 wire→canonical 映射（SSOT：线名只声明一次）
durationMs: ['duration_ms', 'snake'],
```

⇒ `fill()` 读的已是规范化后的 payload。**⇒ 怀疑撤回。** 这正是人类哲学里"0 硬编码 / SSOT"的落地样式。

## 二、装盘机制**已经建好，而且比预期严**

```js
// prove_track.render.js
 * THE RENDER TABLE IS THE ROW SET. A kind with no SUMMARY entry is not drawn.
 * `validate()` requires every contract kind to be either drawn or listed in NOT_DRAWN.
 *   A new kind landing in neither REFUSES TO LOAD, rather than disappearing from the
 *   trajectory with nobody having decided that.
var NOT_DRAWN = { metering: 'measured per call, not a step of the cycle' };
```

⇒ **三条纪律齐了**：kind → lane（`LANE_OF`，与契约 key-for-key 相等）· kind → 模板（`SUMMARY`）·
kind → 归属说明（`NOT_DRAWN`）· **新 kind 无处安放就拒绝加载**（不静默消失）。
⇒ `fill()` 是**替换，不是分派**（`Filling is substitution, not dispatch … No branch on kind`）。

**已上桌的菜**（表格里确有）：

| kind | 模板 |
|---|---|
| `reasoning`(think) | `think: {text}` |
| `plan`(attempt) | `planned calls: a/b` · `no calls planned — answered directly`（**已区分**） |
| `tool` | `{tool} · #{index} · expect={expect}` → `{tool} · {ok} · {durationMs}`（**耗时已上桌**） |
| `check` / `verdict` | 已画 |
| `reply` | 已画（含 `model`） |

⇒ **人类说"菜已备好"是对的；而且"装盘器"也已在位。** `think` / `tool` / `消耗` / `耗时` 都不是缺的。

## 三、那差距在哪 —— 两处，都不是"装盘器"

| 差距 | 性质 | 状态 |
|---|---|---|
| **面板只有"血缘路径"这一个窗口** | **装的是错的菜**（把 5 个 period 混一桌）—— 不是"没装盘"，是"装错了" | ✅ **今日 `e1d5902` 已修**（证轨表改用 `scope='period'`） |
| **FlowModus 的调度决定没有对外的口** | **那一味菜根本没下锅**（`routing.proto` 有 `CostEstimate` 等，但无 `service/rpc`） | ❌ **未做**（唯一必须回写侧的一道） |

**⇒ 结论：人类列的全部环节里，只有"调度选择"（候选/分数/选中理由/成本）是**真正缺的**；
其他的差距是**窗口装错了菜**，而那个已修。**

## 四、`nodes=— / chars=—` 的复判

我先前实测：真实 store 最近 60 条的 `context/inject` **全部有 `nodes`**（空 0 条）。
⇒ 面板上出现 `—`，最可能就是 §三 的"窗口装错菜"（把别的 period 的行混进来，
而那些行的载荷在**归一化后**没有该字段）⇒ **今日的 P3 修法应当一并解决。
⚠️ 但我**未在修后复验**（需要人类重启后用新二进制看）—— **这是待验，不是已验。**

## 五、因此修正后的优先序

| 序 | 事项 | 理由 |
|---|---|---|
| **1** | **验证预言**：问一个必须用工具的问题 ⇒ `tool/call` 必须出现 | 验证今日身份修复的真实效果（模型此前一直说 `no calls planned`） |
| **2** | **复验 P3**：重启后点开一段 ⇒ 只含这一段；`nodes`/`chars` 不再显示 `—` | 待验，不是已验 |
| **3** | **补那道缺的菜**：FlowModus 的 `route_trace`（候选/分数/理由/成本） | 唯一需回写侧的一道 |
| 4 | `verdict` 对称化（成功也具名） | 小；一槽一义 |
| 5 | 经历会话卡 · 缓存 A/B | — |

**⇒ "只差装盘"在机制层面几乎已成立；真正的空白只有两处，而其中一处今天已经修好、另一处是 FlowModus 的对外口。**
