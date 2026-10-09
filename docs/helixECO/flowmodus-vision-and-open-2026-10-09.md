# VISION 对照 + 「FlowModus 真算了吗」的答案 + 三个老问题的定位

> 2026-10-09 ｜ 人类指出：① 之前**全部跑通过** ⇒ 应能找出**对齐点** ② 经历会话卡**一直没正常展示**
> ③ Cellrix 界面**像东拼西凑** ④ FlowModus **不知道是否真调用函数计算并选 llm/api**；顾问建议：
> **保持核心干净 · 微内核 · 度量衡与调度解耦** ｜ 并给了工程哲学 ｜ **未改代码**

## 一、顾问那句"度量衡与调度解耦"= VISION 里本来就有的分界

`FlowModus/docs/VISION.md`（124 行，v2.0-rs）原文：

```
L1 协议与度量规范层   ┐
L2 供应商原始模型层   │  度量衡（标准 Token 当量 STE / 声明偏移量 / 标准化计价）
L2.5 确定性基准测试层 │
L3 标准化计价与评估层 ┘
L4 用户偏好与硬边界   ┐  调度（Hard Filter / Soft Weighting / 熵增路由）
L5 Agent 意图与动态偏好┘
```

**铁律**：*「FlowModus 不判断。FlowModus 只呈现。判断权永远属于用户。」*
**⇒ 顾问的建议不是新架构，是**把 L3/L4 那条已有的分界**在代码里**守住**。**（即：度量衡不该依赖调度，反之亦然。）
**⇒ 这条对人类工程哲学（单一职责/解耦/SSOT）是同一个要求，无冲突。**

## 二、★ 对「是否真调用函数计算并选 llm/api」的**确定答案**

```
FlowModus/flowmodus-rs/proto/flowmodus.proto
  service FlowModus { rpc Reason(ReasonRequest) returns (ReasonResponse); }   ← **只有这一个**
  ReasonResponse { string model = 3; Usage usage = 4; }                       ← 只有"谁答的"+"用了多少"

FlowModus/flowmodus-rs/proto/routing.proto
  message CostEstimate { float estimated_cost_usd = 3; … }                    ← 度量衡产量在此
  message … { CostEstimate cost = 6; }  { float max_cost_per_request_usd = 1; }
  ★ 该文件**没有 service / rpc** ⇒ 这些是消息类型，**没有任何 gRPC 口暴露它们**
```

**⇒ 结论：L1–L5 的产量（STE / 标准化计价 / 声明偏移量 / 候选集 / 分数 / 选中理由）
一个字都没有到达 anaphase。** anaphase 侧只调 `Reason`（`flowmodus.rs:63` 只建一个 client）。

**⇒ 所以"它是否真的算了"从消费侧**无法观测**。这与「存活≠在环」是同一形态：**

> **「FlowModus 被调用了」≠「它的度量衡跑了」。我上一步只证明了前者。**

**⇒ 而 VISION 自己的铁律要求它呈现**（"只呈现数据"）——**它现在什么都没呈现。**

### 修法（方向，未做）

让 FlowModus **呈现它的决定**：`ReasonResponse` 增一个 `route_trace`（候选、分数、选中项、理由、
`estimated_cost_usd`），或**独立的观测行**。**没有它，"在环"就是我们能证明的全部。**
**⇒ 与本轮的身份修复同族**：两者都是"算出来了但没送出去"。

## 三、身份修复与 VISION 的关系（我提的修法与解耦不冲突）

`ReasonRequest` 加 `system = 5` 属于 **L1 协议层**（VISION 说 L1 是"不可变资产"⇒ 要当协议契约对待，
追加 tag 不改既有字段）。它**只是一个传输位，不含任何判断** ⇒ **不侵入 L4/L5 的调度**。
**⇒ 与顾问建议相容。**

## 四、人类的三个老问题：定位（**均未修**）

| 问题 | 本轮定位 | 归属 |
|---|---|---|
| **经历会话卡没正常展示** | **与 P3 同族**：面板只有一个窗口且它是**血缘路径**（`script.html:143` `loadWindow`，自注 `L0 TEMPORARY`）。P3 已让**证轨表**改用"这一段"；**会话卡本身未查** | 展示投影 |
| **Cellrix 界面像东拼西凑** | 未查。已知线索：面板 32 皮片 + `base.html` + 多处 L0 TEMPORARY 注释（`script.html:133`、`mergeChain`）⇒ **"临时件"从未退役** | 展示层结构 |
| **FlowModus 是否真算了** | ✅ **已答**（§二） | 观测缺口 |

## 五、"之前全部跑通过 ⇒ 找对齐点"（人类最重要的提示）

**⇒ 这是一条方法论指令**：既然曾经全绿，那么**现在的不对齐点就是回归**。
**最该先修的是"能让回归现形"的那一件**，而不是逐个功能去猜。

我已登记的一个**具体对齐点**：**`asset_parity_test` 用 mtime 比较** —— mtime 是**签出**的属性，
不是**内容**的属性 ⇒ **它在内容漂移时会绿**（假绿）⇒ **它正是"全部跑过"却仍出错的那种测试**。
⇒ **改为内容哈希（content hash）** 是本条指令的**第一个落实点**。

（本轮未做，登记。）

## 六、人类工程哲学（记录，作为后续抉择依据）

**唯一事实来源 · 极致复用 · 极致解耦（单一职责优先）· 按需加载/驱动/渲染 · 物理事实优先 ·
确定性优先（拒绝编造）· 0 硬编码 · 不闭门造车（借鉴巨人，不照抄专有名）**

**⇒ 与本生态已有的判据对照一致**：
- 「物理事实优先」= 我们的「物理事实优先」（mtime/端口不是能力的物理事实 ⇒ 第 24 条）
- 「确定性优先，拒绝编造」= proto 里 `tokens_consumed` 退役、`cached_tokens` 用 `optional` 的同一个理由
- 「按需加载」= VISION 的"无定时心跳，真实流量遥测寄生 + 用户显式按需探测"
- 「0 硬编码」= `up.rs` 那条"0 硬编码：只有缺 scheme 才补"的注释同源

## 七、本轮**未做**（明说）

- 跨仓身份通道（proto `system=5` → FlowModus 转发 → anaphase 传入）
- FlowModus 的 `route_trace` / 观测呈现
- 经历会话卡 · Cellrix 界面结构 · `asset_parity_test` 改哈希
- P3 的 ②③ 与"每行标明归属" · 缓存 A/B · py/rs 差异审计
