# #3 route_trace：**判定为"做菜"（不是装盘）+ 确切做法**

> 2026-10-09 ｜ 人类：*"证轨可以审计所有环节…包括调度选择"* ｜ **未改代码**（预算用尽，不半途动刀）

## 一、判定依据：FlowModus 的观测面是**齐的**，但**没有路由决定**

`flowmodus-rs/src/serve_cmd.rs:93-115` —— 60053 的**完整**路由表：

```
GET    /healthz
GET    /api/status
GET    /api/suppliers
POST   /api/suppliers/probe
POST   /api/suppliers
DELETE /api/suppliers
```

实测：`/api/suppliers` → **200**；`/api/routes` · `/api/decisions` · `/metrics` → **404**。

**⇒ 没有任何端点暴露「这一轮的路由决定」。** ⇒ **这道菜没下锅**（与"菜已备好"的部分不冲突：
其余 8 类菜确实已备好、装盘器也已就位；**唯独调度决定既没做也没端**）。

## 二、原料就在手边（所以是"小做"，不是"从零"）

`flowmodus-rs/src/grpc_cmd.rs` 的 `reason` handler **已经算出**决定：

```
:241  let tier = if free.iter().any(|s| s.supplier_id == decision.supplier_id) { Free } else { Paid };
:260  supplier_id: decision.supplier_id.clone(),
:261  endpoint:    decision.endpoint_url.clone(),
:262  model:       decision.model_id.clone(),
:308  /* The ROUTED model — `decision.model_id`, not `req.model`. ADR-0036 */
:312  model: decision.model_id.clone(),          ← ReasonResponse.model 已在回它 ✅
```

⇒ **"谁答的"已经上线（`ReasonResponse.model`）**；缺的是**"为什么是它"（候选/分数/理由）与成本**。

## 三、确切做法（下一轮，四条 + 判据）

**① 线（唯一事实来源，一处声明）**
`flowmodus.proto` 的 `ReasonResponse` **追加** tag 5：

```proto
message RouteTrace {
    string chosen_supplier = 1;    // decision.supplier_id
    string chosen_model    = 2;    // decision.model_id（与 ReasonResponse.model 同值，可交叉校验）
    string chosen_tier     = 3;    // Free | Paid
    string reason          = 4;    // 具名理由（declared-model | auto-prefer | failover-after…）
    double estimated_cost_usd = 5; // L3 产量（当前为 0 也要记 0，不伪装）
    repeated Candidate candidates = 6;  // 候选集 + 各自分数
}
message Candidate { string supplier_id = 1; string model_id = 2; double score = 3; string why = 4; }
```

**② FlowModus 只转发不判断**（VISION：*不判断，只呈现*）：handler 已持有 `decision` 与 `cands`，
直接填 `route_trace`。

**③ anaphase 落账**：`ReasonResponse.route_trace` ⇒ 一行新事件（如 `route/decision`），
经 `event_family.js` **声明一次** wire→canonical 映射（与 `durationMs` 同一机制）。

**④ 装盘**：在 `prove_track.render.js` 的 `SUMMARY` 加一条模板 —— **不新造机制**（装盘器已在位；
且**新 kind 若不画又不在 NOT_DRAWN，会拒绝加载**，所以这一步是强制的，不会静默漏掉）。

**判据（必须能变红）**
- **正**：发起一轮 ⇒ 事件流出现 `route/decision`，`chosen_model` 与 `assistant/reply.model` **一致**
- **反证 1（变异在服务者上，不在配置上 —— 第 15 条）**：让 mock supplier 声明另一个模型 ⇒
  `route/decision.chosen_model` 必须跟着变
- **反证 2**：把 `reasoning_model` 清空（交给 Auto）⇒ `reason` 字段必须写 `auto-prefer` 而不是编一个理由

## 四、顺序建议（与"装盘"结论一致）

1. **`route_trace`（本节）** —— 唯一需回写侧的一道，做完"调度选择"这一格才真正可审计
2. 真实点击后的 DOM 断言（P3 点击层）
3. `16 vs 15` 差一行的待查
4. 缓存 A/B · 经历会话卡

## 五、本轮**未做**（明说）

- **未动任何代码**：跨两仓 proto + FlowModus 填充 + anaphase 落账 + 装盘模板，**四步都要验证**；
  预算用尽时**留半成品比留设计更危险**（本会话已两次因此付出代价：一次把栈带走、一次污染对照）。
