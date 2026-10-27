# 三层判读（想 / 说 / 做）：机械可分辨性、第二条执行路径、以及 4.8% 的泄漏读

> 2026-10-28 · 对应 ROAD-14 甲 A-1 / A-2 · 全部读数为命令所出

## 0. 定名与修正

| 层 | 是什么 | 可见性 |
|---|---|---|
| 想 | 模型内部推理 | **可见**（`assistant/think` 行就落盘）——"想不可见"是错的 |
| 说 | 模型的输出 | 可见（`assistant/reply`、`turn/end.reply`） |
| 做 | 「说」的一个子类：携带执行意图那部分 | 可见（`tool/call`、`tool/result`） |

三层都可见 ⇒ 三层都可判、可审计。"做"寄生在"说"里 ⇒ **拦截的前置是先把"做"从"说"里认出来**。

## 1. A-1 问题一：三层在事件流里是否可机械区分？——**区分得开**

```
$ grep -n 'pub enum EventType' -A 40 anaphase-helix/src/session_events/types_and_stream.rs
  19 TurnStart · 21 UserMessage · 23 ContextInject · 25 Attempt · 28 Think · 30 ToolCall
  32 ToolResult · 34 Check · 36 Verdict · 41 AssistantReply · 51 Usage · 53 TurnEnd
```

三态各自有独立的 `type`，不需要从文本里猜语义。**但真实落盘用的是斜杠写法**（`assistant/think`、`tool/call`），
而 Rust 枚举声明的是 `snake_case` —— 这一条用命令核定过：

```
$ cat .helix/events/*.events.jsonl | python3 -c "...Counter(type)..."
  ref/move 2568 · assistant/reply 668 · turn/end 668 · assistant/usage 652 · turn/start 610
  user/message 610 · context/inject 610 · assistant/attempt 610 · assistant/think 606
  check/status 90 · tool/call 45 · tool/result 45 · verdict/status 45
```

## 2. A-1 问题二：是否所有「做」都经 tentacle？——**不是，有第二条执行路径**

| 路径 | 位置 | 门 |
|---|---|---|
| 结构化（经 tentacle） | `src/pipeline/mod.rs:234-239` `self.tentacle.execute_tool_with_labels(...)` | 安全门（`security_gate: Option<Arc<dyn SecurityGate>>`，K-133 说 Tuck 侧 404） |
| **legacy（直连执行）** | `src/run_cycle/mod.rs:1067` `self.tool.execute(command, &[action_str.clone()])` | **只有** `safety_gate::admit(...)`（`:1057-1063`），不经 tentacle、不经 Tuck |

legacy 的可达条件（命令核定，不是推断）：

```
src/pipeline/mod.rs:436-449  resolve_pipeline：endpoint 为空 或 连接失败 ⇒ 返回 None
                             （注意那句 `Falling back to legacy execution` 是**真话**，legacy 确实会执行）
src/run_cycle/mod.rs:1036    if self.pipeline.is_some() && !self.context.calls.is_empty() ⇒ 结构化
                             否则落到 legacy 分支（`:1045` 起，`:1067` 执行）
```

⇒ **批 3 的拦截点因此有两个**：只把 Tuck 的 `/v1/security/gate` 挂上，只管住结构化路径；
legacy 路径根本不问 Tuck。这一点不能靠"设计如此"推断，是上面两条命令定出来的。

旁证：`src/` 里 18 处 `Command::new` 全部在 `src/bin/up.rs`（拉起生态的运维工具）与测试支撑里，
**没有**第三处模型驱动的执行路径。

## 3. A-2 判据（三态，不许二分）

工具：`thinking/phyt-DNA的任务编排思路/tools/layer-census.py`（`--selftest` 13/13）

| 态 | 判据 |
|---|---|
| ① say | `assistant/reply` · `turn/end`，且载荷里**没有**执行结构 |
| ② act | `tool/call` · `tool/result` 出现（= 已被路由去执行）；再分 结构化 / legacy |
| ③ LEAK ★红 | **说**的载荷里出现执行结构：`json_calls` · `json_tool` · `fenced_plan` · `text_calls` · `text_tool` |

为什么必须三态：二分会把 ③ 静默读成 ①，那正是 fail-open 的入口。

### 真实读数（668 个事件文件，不是夹具）

```
文件 668 · 退出码 {0: 636, 1: 32}
行 7827 · 想 606 · 说 1336 · 做 90（结构化 90 · legacy 0）· **LEAK 64 行 / 32 个文件（4.8%）**
形状分布：json_calls 6 · fenced_plan 56 · text_calls 2
```

⇒ K-127（"工具计划被当成答复交付"）**不是孤例**：668 次运行里有 32 次如此，占 4.8%。
⇒ 且它是**可判的**：这 64 行全部由 `type`+载荷结构机械判定，没有一条靠人读语义。

## 4. 建这个判据时自己踩的四个坑（都属"读到空"家族）

| # | 坑 | 表现 | 现在由什么守住 |
|---|---|---|---|
| 1 | `THINK` 常量漏定义 | `NameError` | selftest 直接炸 |
| 2 | 只认 `snake_case` | 7827 行**三态全 0**，却打印"0 LEAK" | selftest 增斜杠样本 |
| 3 | 报告字段写死 `0` | `leak=64` 与 `say=0` 同时出现（不可能） | selftest 断言"报告里的数字必须来自 buckets" |
| 4 | 只认 `think`，不认 `assistant/think` | 「想」= 0（实际 606 行） | selftest 增 `assistant/think` 样本 |

第 2、4 条是同一形状：**枚举名写对了、落盘名不对** —— 判据自己绿着，读数全空。
这正好是 §3.12「让它别红」的反面：**让它读到空**。

## 5. 边界（没做的事）

- **没有**把 LEAK 变成硬判据：那要改交付路径（refuse / 转路由），属授权②。
- **没有**读 legacy 路径的审计：`safety_gate` 的 `OnAuditError::ReportSuccess`（`run_cycle/mod.rs:1061-1062`）
  是另一个已知缺口（K-033），本轮不动。
- 本工具的调用者写在文件头（GLOSSARY §3.10）：批 3 之前的读数 + 每次改「说/做」分流时按需。
