# 「LLM 不知道自己是谁」的**根因**：活的通道（gRPC）没有 system 位

> 2026-10-09 ｜ 人类报告三件事：① helix-mind 好像没接通 ② tentacle 好像没连上 ③ LLM 不知道自己是谁
> ｜ **端到端实测已定案** ｜ 已改一行配置（必要但不充分）

## 一、端到端实测（决定性）

```
POST /api/chat  {"message":"你是谁？只回你的名字。"}
→ {"job_id":"run-322a530215d6b5fa","model":"coder-3b","reply":"Qwen","success":true}
```

**⇒ 它回答「Qwen」—— 不是「Dash」（gene_lock 里的 Lineage Name）。身份没生效。**

## 二、★ 根因：`ReasonRequest` **没有 system 字段**

```proto
message ReasonRequest {
    string prompt = 1;
    string cognitive_mode = 2;
    string model = 4;
    uint32 max_tokens = 3;
}   // ← 没有任何位置可以携带 system prompt / 身份
```

对照两条路径：

```rust
// main.rs:922 —— HTTP（**非活路径**）：身份传进去了
Arc::new(HttpReasoningAdapter::new(&config.anaphase, Some(identity_system)))

// main.rs:936 —— gRPC（**活的路径**）：构造函数只有两个参数，**没有身份**
GrpcFlowModusAdapter::new(endpoint, config.anaphase.reasoning_model.as_deref().unwrap_or(""))
```

而 `http_reasoning.rs` 自己写明了这件事的性质：

```rust
/// L0 identity + L1 tool awareness — sent as the **system** message so
/// it overrides the model vendor's default identity ("I am Agnes…").
/// A user-role identity claim is context, not authority;
/// a system-role identity claim is who the model IS for this session.
```

**⇒ 结论：活的 gRPC 通道上，身份在 `main.rs:902` 被算出来，到 `:936` 被丢弃。**
**⇒ `build_identity_block` 的全部工作（gene_lock + tentacle 工具清单）在活路径上是白做的。**
**⇒ 模型回答「Qwen」是因为*从来没有人告诉过它是谁* —— 不是它不听话。**

## 三、我改的那一行：**必要但不充分**（必须说清）

`anaphase-helix/config.toml` 加了：

```toml
gene_lock_path = "/Users/jason/Developer/Jasonmilk/.helix/mind/gene_lock.md"
```

理由：`config.rs:141` 注释写着 *"None = no identity injection (honest degraded state)"*，
默认值就是 `None`（`:402`），而 `build_identity_block` 靠它才进得去（`main.rs:1289`）。
**这个指针缺了，身份块的第一段就永远是空的。**

**⚠️ 但它不足以修好** —— 因为**即使算出来了，活的通道也送不出去**（§二）。
**⇒ 我加了指针（必要），但同一个 bug 还有第二层（充分条件未满足）。**

## 四、人类三件事的裁决

| 报告 | 判定 | 证据 |
|---|---|---|
| helix-mind 没接通 | ❌ **接上了** | `ANAPHASE_MIND_ENDPOINT=http://127.0.0.1:50052`；`up` 日志 `mind: ✅ 已就绪`；50052 在听 |
| tentacle 没连上 | ❌ **接上了** | `ANAPHASE_TENTACLE_ENDPOINT=http://127.0.0.1:50051`；50051 在听（我先前查 `60051`，端口查错） |
| **LLM 不知道自己是谁** | 🔴 **真问题，根因已定** | 见 §二：活的 gRPC 通道没有 system 位 |

**⇒ 两个"没接上"是误判（端口都在、环境变量都对），真问题只有第三件。**

## 五、修法（跨仓，未做）

1. **`anaphase-helix/proto/flowmodus.proto` + `FlowModus/flowmodus-rs/proto/flowmodus.proto`**：
   `ReasonRequest` 加 `string system = 5;`（**追加字段号，不改既有 tag**）
2. **FlowModus 侧**：把它作为 system message 转给上游（与它转 `prompt` 同级）
3. **anaphase 侧**：`GrpcFlowModusAdapter::new(endpoint, model, system)` + `reason()` 填 `system`
4. **判据（必须能变红）**：重启后问「你是谁」⇒ 必须答 **Dash**；把 `system` 传空 ⇒ 必须回到 **Qwen**
   （**变异在"服务者/通道"上，不是配置上** —— 第二十四条）

**⚠️ 未做原因**：跨两仓 + proto 改动 + 需重建两侧二进制；本轮预算不足，且**留半成品比留记录更危险**。

## 六、人类给的方向（记录，作为设计约束）

1. **helix-mind 必须闭环**（工具可调用、任务可完成）—— 见 §二：工具清单经 `build_identity_block`
   进身份块，**所以身份通道一通，工具意识同时恢复**。两件事是**同一个洞**。
2. **tentacle 需连 Glove 与 MCP-learner** —— 未查（本轮只确认 tentacle 进程在听）。
3. **会话经历混乱问题仍未完全解决** —— 同意。P3 只做了"展示只含这一段"，
   **①"每行标明归属"、②"字段缺失具名"、③"无选中具名空态"未做**。
4. **★ 目标交互形态：Deepseek harness = 借 llama.cpp 的对话壳** ——
   ⇒ 面板应朝 **llama.cpp server 的对话界面/逻辑**收敛（多会话、逐会话历史、流式、usage 展示）。
   **⇒ 这也是 `usage.cached_tokens` 的来源**（llama.cpp 的 `cache_n` 语义）—— 我们那条链的源头。

## 七、本轮**未做**

- §五 的跨仓修法（身份通道）
- tentacle ↔ Glove / MCP-learner（未查）
- P3 的 ②③ 与"每行标明归属"
- 缓存 A/B · `1,000` 口径（已由一手数据否证"块=1000"，见上一份记录）
- py/rs 差异审计
