# 预言证实：**身份通道修好后，工具开始被调用** —— 并撤回我一条错误指控

> 2026-10-09 ｜ 判据：问一个**必须用工具**的问题 ⇒ `tool/call` 必须出现 ｜ **未改代码**

## 一、判据执行（同一次会话，修复后）

**问**：*"请用 calc 工具计算 1234 × 5678，然后告诉我结果。"*

```json
{"done":true,"model":"coder-3b","reply":"1234 × 5678 的结果是 7006652。","success":true}
```

**事件流**：

```
attempt:    'planned calls: calc'
tool/call   {tool:calc, index:0, expect:'ok'}
tool/result {tool:calc, duration_ms:101, ok:true, outcome:'{"ok":true,"result":"7006652"}', outcome_sha:'30018ac7'}
reply       model=coder-3b  '1234 × 5678 的结果是 7006652。'
turn/end    success=true  verdict='Met'
```

**⇒ 判据满足**：工具被规划 · 被调用 · 成功 · **结果正确（7006652）** · 耗时 101ms · `outcome_sha` 落账。

## 二、★ 行为变化是可测的（修复前 vs 修复后）

| | 修复前（历史全库） | 修复后（两轮） |
|---|---|---|
| `tool/call` | **2 / 524 轮（0.4%）** | **2 / 2 轮** |
| 回复的自我说明 | 字面 `no calls planned — answered directly` | `planned calls: calc` / `planned calls: numbers` |

**⇒ 前一轮（"你是谁"那次）也调了工具**（`numbers`，55ms，ok）—— 它在答"Dash"之前
**先规划并执行了一次工具调用**。

**⇒ 这印证了诊断**：模型不用工具，**不是工具坏了、也不是 tentacle 没连**，
而是**它不知道有哪些工具**（工具清单经身份块注入，而身份块在活的 gRPC 路径上被丢掉）。
**⇒ 同一条通道修复，同时恢复了**身份**与**工具意识**。**

## 三、撤回我上一条的一处指控（我错了，必须明说）

我先前据一条样本（`{'reply':'你好','success':True,'verdict':None}`）指控
**"成功不具名 ⇒ 不对称（第 13/17 条同族）"**。

**⇒ 撤回**：新写入的行是 `verdict:'Met'` / `verdict:'Unmet'` —— **对称**。
我读到的那条是**历史行**，产生时该字段还不存在（旧事件不会追溯改写）。
**⇒ 那是"旧数据缺字段"，不是"设计不对称"。我把历史数据的空洞当成了当下的设计缺陷。**

## 四、因此现在的实际状态（更新）

| 项目 | 状态 |
|---|---|
| 身份（问"你是谁"） | ✅ `Dash`（改前 `Qwen`） |
| 工具意识与调用 | ✅ 修复后每轮都规划工具；calc 结果正确 |
| tentacle 链路 | ✅ 通（calc 经它，101ms，`outcome_sha` 落账） |
| 证轨装盘 | ✅ 机制已在位（think/plan/tool/usage/check/verdict 都有模板，新 kind 无处安放会拒绝加载） |
| 面板窗口（P3） | ✅ 已修（`scope='period'`）；**⚠️ 界面上仍待人类重启后复验** |
| **FlowModus 的调度决定** | ❌ **唯一真正缺的一道菜**（`routing.proto` 无 `service/rpc`） |
| 缓存 A/B | 未做 |
| 经历会话卡 | 未查 |

## 五、下一步

**#2 复验 P3（界面）** —— 需要人类用新二进制看：点开一段 ⇒ 只含这一段；`nodes`/`chars` 不再显示 `—`。
**#3 FlowModus 的 `route_trace`** —— 唯一需回写侧的一道。
