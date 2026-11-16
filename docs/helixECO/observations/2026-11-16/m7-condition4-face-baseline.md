# M7 条件 4「无新增面」—— 面基线（2026-11-16）

> 条件原文：`center/SESSION-DAG-IS-THE-CENTER.md:136`「无新增『面』（六条硬禁令之 2）」。
> 判它需要一份**基线清单**；本文件就是那份清单。**只登记，不判红。**

## 现有 19 个 HTTP 面（`anaphase/src/main.rs`，行号为基线锚点）

| # | 路径 | 行 |
|---|---|---|
| 1 | `/v1/agent/snapshot` | 91 |
| 2 | `/v1/agent/events` | 100 |
| 3 | `/v1/trace` | 122 |
| 4 | `/v1/sessions` | 158 |
| 5 | `/v1/sessions/rename` | 205 |
| 6 | `/v1/crystallize` | 243 |
| 7 | `/v1/events` | 271 |
| 8 | `/v1/pins/release` | 345 |
| 9 | `/v1/periods/vacancies` | 364 |
| 10 | `/v1/periods/purge-content` | 390 |
| 11 | `/v1/periods/collect` | 410 |
| 12 | `/v1/periods/tombstone` | 440 |
| 13 | `/v1/refs` | 454 |
| 14 | `/v1/refs/*name` | 471 |
| 15 | `/v1/health` | 496 |
| 16 | `/v1/bind/start` | 503 |
| 17 | `/v1/bind/confirm` | 513 |
| 18 | `/v1/bind/status` | 524 |
| 19 | `/v1/chat` | 528 |

## 怎么用这份基线（给下一轮的我）

```
判"无新增面" = 用 `grep -cE '\.route\("' src/main.rs` 得 N，与 19 比：
  N == 19 且路径集合一致 ⇒ 条件 4 满足
  N > 19 且新路径不在上表 ⇒ 新增面（须人裁定，六条硬禁令之 2）
  N < 19 ⇒ 面被删（同样要说明，静默删除也会让基线失效）
```

**本轮读数**：`grep -c` = 19 · 路径集合与本表逐条一致 ⇒ 条件 4 **在基线意义上满足**
（此前写 UNMEASURABLE 是因为没有基线，现在有了）。

## 边界

```
本文件不是门：不新增判据、不跑 CI、不改路由。
它只是把"19 个面长什么样"钉在纸上，否则"无新增"永远不可判（等于永远为真）。
```
