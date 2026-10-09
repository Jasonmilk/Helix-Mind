# 证轨的"菜单"（菜已备好）+ 装盘差距 + 工具 2/524 的真相

> 2026-10-09 ｜ 人类：*"证轨可以审计所有环节…几乎什么都有了！菜好准备，只差装盘了。"*
> 本轮把**已备好的菜**列清楚，并对照**盘子上现在有什么** ｜ **未改代码**

## 一、已备好的菜（真实 store：13 类事件 / 6361 行）

**信封（每一行都带）**：`period_id · job_id · seq · time` ⇒ **可审计、可 join（period_id 是 ADR-0048 的跨仓键）**

| 环节 | 事件 | 已有字段 | 装盘 |
|---|---|---|---|
| **思考过程** | `assistant/think` (520) | `text` | ✅ |
| 阶段/失败 | `assistant/attempt` (524) | `empty, error, stage, text` | ✅ |
| 回复 | `assistant/reply` (524) | `chars, model, text` | ✅ |
| 轮次收束 | `turn/end` (524) | `done, impasse, model, reply, success, verdict` | ✅ |
| **tokens 消耗** | `assistant/usage` (523) | `prompt_tokens, completion_tokens, cached_tokens, reasoning_tokens, model` | ✅ **一手** |
| **注入/血缘** | `context/inject` (524) | `nodes, chars, injected_chars, choice, resume_from` | ✅ |
| **工具调用** | `tool/call` (**2**) | `tool, index, expect` | 🔴 见 §三 |
| **工具耗时** | `tool/result` (**2**) | `duration_ms, ok, outcome, outcome_sha, tool` | 🔴 见 §三 |
| 内存引用 | `ref/move` (2164) | `name, old, new` | 最大表，未审 |
| 判据 | `check/status` (4) · `verdict/status` (2) | `check_id, expect, actual, judge, passed, reason, gate, evidence_id` | 少 |

**⇒ 人类说得对：思考 · 阶段 · tokens · 缓存 · 注入 · 工具 · 耗时 · 判据，**都在事件流里**。

## 二、装盘差距（**唯一真正缺的一道菜**）

| 环节 | 状态 |
|---|---|
| 思考 · 阶段 · tokens · 缓存 · 注入 · 工具 · 耗时 · 判据 | ✅ **已备好** |
| **调度选择**（候选集 / 分数 / 选中理由 / 成本估算） | ❌ **没备好** —— FlowModus 的 L1–L5 产量（`routing.proto` 的 `CostEstimate` 等）**没有任何 rpc 暴露**，`ReasonResponse` 只回 `model + usage` |
| **延迟**（每轮端到端） | ⚠️ 可从 `time` 差算；`tool/result` 有 `duration_ms`；**没有一处直接记"这轮花了多久"** |

**⇒ 「只差装盘」在 90% 上成立；剩下那 10% 是 FlowModus 的决定从未上线。**

## 三、🔴 `tool/*` 只有 2 行 —— 而模型自己说出了原因

```
tool/call {'tool':'calc'}    → tool/result {duration_ms:53, ok:true, outcome:'{"ok":true,"result":"4"}'}
tool/call {'tool':'numbers'} → tool/result {duration_ms:55, ok:true, outcome:'{"series":[]}'}
```

**⇒ 工具链**是通的**（成功、带耗时、带 `outcome_sha`）；但 **524 轮里只用了 2 次**。

**而 523 条 `success=true` 里，回复字面写着：**

```
reply: 'no calls planned — answered directly'
```

**⇒ 模型说"没有计划调用" —— 因为它**从未被告知有哪些工具**。
而工具清单正是经**身份块**（`build_identity_block` 的 L1 部分）注入的，而**身份块在活的 gRPC 路径上被丢掉了**（今日 `4a4a148` 才修）。**

**⇒ 与身份是**同一个洞**。**⇒ 人类问的"tentacle 没连上 / 必须能调用工具"的确切答案：

| 疑问 | 答案 |
|---|---|
| tentacle 连上了吗 | ✅ 连着（50051 在听，`ANAPHASE_TENTACLE_ENDPOINT` 正确） |
| 工具能跑吗 | ✅ 能（calc/numbers，53/55ms，ok） |
| 那为什么"没闭环" | ❌ **模型不知道它们存在** —— 现在知道了吗？**今天起应该知道** |

### 可验证的预言（下一轮的判据）

**修好身份通道之后，需要工具的轮次应当出现 `tool/call`。**
⇒ 判据：问一个**必须用工具**的问题（如"算 1234×5678"）⇒ `tool/call` 必须出现；
**反证**：把 `system` 传空 ⇒ 回到 `no calls planned`。

## 四、`success:false` —— 查明（是我造的那条，不是损坏）

```
turn/end 的 success 分布：True: 523 · False: 1
唯一 false：{'done':True,'impasse':False,'model':'coder-3b','reply':'Dash',
             'success':False,'verdict':'Unmet'}
success=true 样本：{'reply':'你好','success':True,'verdict':None}
```

⇒ 它是**今天这次测试的那一条**（reply=`Dash`）：我问"只回名字"，判据判为**未满足**。
**⇒ 不是改动造成的损坏。**

**⚠️ 但暴露一处不对称**：`success=true` ⇒ `verdict=None`（**成功的理由不具名**）；
`success=false` ⇒ `verdict='Unmet'`（具名）。**两者都该具名**（第 13/17 条同族）。

## 五、结论与下一步

**"只差装盘"成立**：证轨的字段面已经覆盖人类列的全部环节，**唯一的空白是 FlowModus 的决定**。
**因此优先级更正**：**先装盘（读侧），后补菜（FlowModus 的 route_trace）**——
因为**装盘能把已有的 8 类菜一次端上桌**，而 route_trace 是**唯一必须回到写侧**的一道。
