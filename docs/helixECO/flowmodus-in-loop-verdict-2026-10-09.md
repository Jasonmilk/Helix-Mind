# FlowModus 在环 —— **静态定案**（并在路上找到一条 Priority 1 直连）

> 2026-10-09 ｜ 承接 `flowmodus-in-loop-2026-10-09.md`（那次变异因缺对照未定案）｜ **未改代码**

## 一、目标 0 ✅ **有确定答案**：proto 里**已经有** `served_model` + `usage`

`anaphase-helix/proto/flowmodus.proto`（与 `FlowModus/flowmodus-rs/proto/flowmodus.proto` 同文）：

```proto
message ReasonResponse {
  reserved "tokens_consumed";   // RETIRED（旧的总量字段；退役理由：不许伪造测量）
  // The model that ACTUALLY served the request — the routed fact, not the [request]
  string model = 3;
  // …no usage at all ⇒ Anaphase then writes no `assistant/usage` row
  Usage usage = 4;
}
message Usage {              // "DISJOINT — exactly the shape Anaphase stores"
  uint64 prompt_tokens = 1;  uint64 completion_tokens = 2;
  optional uint64 cached_tokens = 3;  optional uint64 reasoning_tokens = 4;
}
```

**⇒ 两样都有：`served_model`（`model` 字段）+ 完整 usage。**
**且 `cached_tokens` 是 `optional`** —— "未报告"与"0"**在类型上就分开**（正是我们反复要的那条）。

## 二、★ 而且**你要的 1–4 已经在 gRPC 适配器里实现了**

`anaphase-helix/src/adapters/flowmodus.rs`：

```rust
:46  /// … this is response metadata already on the wire, and it is the ONLY source for
     /// `assistant/usage` and for the routed model's name.
:97  m.model = if response.model.is_empty() { None } else { Some(response.model.clone()) };
:98  m.usage = response.usage.map(|u| UsageSnapshot {
:99      prompt_tokens: u.prompt_tokens, … cached_tokens: u.cached_tokens, …
```

逐条对照你给的 1–4：

| 你的要求 | 状态 |
|---|---|
| **1** 命名唯一写入者：FlowModus；anaphase 只转抄不推断 | ✅ **已是**（`response.model` 直抄；注释明说"ONLY source"） |
| **2** 取不到 ⇒ 具名 unknown，不回落 `reasoning_model` | ✅ **已是**（空 ⇒ `None`；且 proto 注释："写 no row 而不是假的一切零"） |
| **3** `reasoning_model` 降级为意图（请求） | ✅ **已是**（注释：`Empty = let FlowModus route`；它是 `ReasonRequest.model`） |
| **4** `reasoning_mode` 与"模型名"分离（一物一名/一槽一义） | ⚠️ **gRPC 路径已是**（`cognitive_mode: mode` 与 `model` 是**两个字段**）；**HTTP 适配器未**（`reasoning.rs:305` 用 `reasoning_mode.clone()` 当模型名） |

**⇒ 重要更正**：C1 那条 `chat_model_test` 的红，**不该用"改 `reasoning.rs:305`"来治** ——
**gRPC（在生产路径上）本来就是对的**。要查的是**面板的 `model` 显示读的是哪个适配器/哪一行**。

## 三、🟡 但在路上找到一条**真实的 Priority 1 直连**（你要的"bypass 在哪"）

`anaphase-helix/src/main.rs:914`：

```rust
// Priority 1: Use HTTP LLM reasoning adapter first        ← ★ 直连优先于 FlowModus
let reason = if let Some(endpoint) = &config.anaphase.reasoning_endpoint {
    if endpoint.is_empty() { Noop } else { Arc::new(HttpReasoningAdapter::new(…)) }
}
// Priority 2: Fallback to original FlowModus
else if let Some(endpoint) = &config.anaphase.flowmodus_endpoint { … GrpcFlowModusAdapter … }
else { NoopReasoningAdapter }
```

**⇒ 这就是那条"直连路径"，而且它是 Priority 1（HTTP 赢过 gRPC）。**
它不是隐藏 fallback，是**配置优先级**：`reasoning_endpoint` 一旦被设置（env 或 toml），**FlowModus 被静默绕过**。

**⇒ 但它当前**没有**触发** —— 证据：`up` 启动 anaphase 时导出的环境变量里
**只有 `ANAPHASE_FLOWMODUS_ENDPOINT=grpc://127.0.0.1:60054`，没有 `ANAPHASE_REASONING_ENDPOINT`**；
且 `config.toml` 未设 `reasoning_endpoint` ⇒ **P1 被跳过 ⇒ 走 gRPC ⇒ FlowModus 在环 ✅**

## 四、因此"在环吗"**静态定案：在环**（不需再做变异）

```
P1 `reasoning_endpoint`  未设 ⇒ 跳过
P2 `flowmodus_endpoint = grpc://127.0.0.1:60054` ⇒ 非 http:// ⇒ GrpcFlowModusAdapter   ← 生产路径
   （若连接失败 ⇒ 具名 eprintln + Noop，不静默回落直连 ✅）
⇒ FlowModus 在环；模型名与 usage 都来自它的响应
```

**⇒ 我先前那次变异（停 60054 后 `Connection refused`）与静态结论一致**，
但仍以静态为准（那次缺对照，不重复引用）。

## 五、你"顺带"那一条的答案：**面板的 token 数不是二手数据**

面板 `CACHE HIT` 来自 `prove_track.view.js:90` 的 `fmtTok(u && u.cached)`，而 `u` 是
**`assistant/usage` 行**；该行在 gRPC 路径上**只能**来自 `ReasonResponse.usage.cached_tokens`
（`flowmodus.rs:46` 注释："the ONLY source"）。

**⇒ `20,967 / 1,000` 是一手数据（来自 FlowModus 转抄的上游 usage）。**
**⇒ 我们前几轮的缓存推理没有建立在二手数据上** —— 这一点可以放心。
（但 `1,000` 是否是 `fmtTok` 的取整产物，**仍未查**。）

## 六、你给的三条，我的审查与处置

| 你的点 | 我的判定 | 处置 |
|---|---|---|
| 0 读 proto 答 served_model+usage | ✅ **已完成，答案是"都有且已在消费"** | 结论：**不需要新增字段** |
| 1–4 命名唯一写入者/具名 unknown/降级为意图/一物一名 | ✅ 4 条全对，**但 gRPC 路径已实现**；缺的是 HTTP 适配器（**不在生产路径**） | **降级为"待查面板 model 显示读哪行"**，不是新工程 |
| 判据用变异（让 FlowModus 返回不同模型名 ⇒ 事件流须跟着变） | ✅ **判据形式正确**（变异落在服务者而非配置 —— 第 15 条用法正确）；**但当适配器已是"直抄响应"时，此变异验证的是"没人在中间覆盖"** | 可做，**便宜**（改 mock supplier 的 `model_id`） |
| 顺序排在 P3 ②③ 与缓存 A/B 之前 | ✅ **同意** —— 而且它已被证明**不是新工程**，是"验证 + 一条已存在的正确" | **保留在序首** |
| 顺带：token 应以 FlowModus usage 为准 | ✅ **已确认是**（见 §五） | 无需改 |

## 七、优先序（更新）

| 序 | 事项 | 变化 |
|---|---|---|
| **1** | **验证 FlowModus 在环的变异**（改 mock supplier 的 `model_id` ⇒ 事件流须变） | **新、便宜、且判据正确** |
| **2** | **Priority 1 直连的具名化**（`reasoning_endpoint` 若被设，须**具名声明"FlowModus 已被绕过"**，不得静默） | **新增**（这是真正的正确性缺陷：一条静默旁路） |
| 3 | 面板 `model` 显示读哪行（C1 真红的正解） | 更正方向 |
| 4 | P3 ②③ 判据收尾 | — |
| 5 | 缓存 A/B | — |

## 八、本轮**未做**

- 变异（改 mock supplier 的 `model_id`）
- `reasoning_endpoint` 的具名化改动
- 面板 `model` 显示读哪行
- P3 ②③ · 缓存 A/B · `fmtTok`/`1000` 口径 · py/rs 差异审计
