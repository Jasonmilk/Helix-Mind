# 跨仓提案 · anaphase：工具可用性与事件时刻（**一份，两条**）

> 2026-09-16 由 Cellrix 侧提出。**证据全部是代码行与实际事件文件**，不是推断。
> 目的：让「模型为什么不调工具」与「时间戳代表什么」各自有一个**可判定**的答案。

## 一、背景：用户报「Tentacle / 搜索不可用」，此前的两种解释都不成立

| 说法 | 判定 |
|---|---|
| 「tentacle 挂了」 | ❌ 服务在跑，`start-panel.sh` 正常拉起，`list_tools()` 有实现（`adapters/tentacle.rs:34`） |
| 「工具声明没注入 ⇒ 回归」 | ❌ **注入了**：`main.rs:1042-1046` 把工具清单拼进 system prompt |
| 「只注入了散文、没有 API tool 声明」 | ✅ **属实**（`src/` 全仓无 `tools` 字段；`http_reasoning.rs:104` 的请求体只有 `messages`/`max_tokens`）——**但这不足以解释"不调用"，因为同一处还注入了调用契约** |

`main.rs:1043-1044` 实际注入的是两件事：

```
[tools available — use them before guessing; 0-token tools first, then few-token, then big LLM]
tentacle·fixture: calc(a,b): … | tentacle·mcp: …
When you need a tool, end your reply with ONLY: {"calls":[{"tool":"NAME","args":{...},"expect":"ok"}]} — no markdown, no prose.
```

⇒ 所以问题不是「没说」，而是下面两条**具体缺陷**。

## 二、P0-A：该块的**缺失是静默的**

`main.rs:1003-1010` 的嵌套条件：

```rust
if let Some(ep) = config.anaphase.tentacle_endpoint.as_deref() {
  if !ep.is_empty() {
    if let Ok(mut adapter) = GrpcTentacleAdapter::new(ep).await.map_err(|e| e.to_string()) {
      if let Ok(tools) = adapter.list_tools().await {      //  ← 失败即静默
        if !tools.is_empty() {                              //  ← 空即静默
```

**三种失败**（未配端点 / 连不上 / 列表为空）**产生同一个可观测结果：system prompt 里没有这一段**。
于是模型会如实回答「我的可用工具列表里没有」——**而这句话与"系统忘了给"完全同形**。
这和本仓已修过的形态同类：**缺失没有留下痕迹**。

**请求**：
1. `list_tools()` 的**每一种**结果都记一个可查的事实（成功/失败/空 + 端点 + 时刻），
   落到一个既有的诊断面（如 `health::checks` 或 `[identity]` 那行 eprintln 的同类）。
2. 把那句 `[Identity] system prompt EMPTY — no gene lock or tools`（`main.rs:651`）
   的**触发条件**与它**描述的缺失**对齐——目前它只覆盖「prompt 为空」这一种。

**判据**：跑一次真实一轮后，能从一个地方读到「本轮 tools 块：在 / 不在（原因）」。

## 三、P0-B：契约措辞与「reply = 交付物」冲突

指令说 `end your reply with ONLY: {"calls":…}`。但本生态里 **reply 同时指「交付给用户的答案」**
（`assistant/reply` 是每轮交付物）。实测三轮：模型把
「我需要计算 4 的 23 次方，但不口算。让我使用 Python 来计算。」
写进了 **REPLY 正文**，没有产出 `{"calls":…}`。

⇒ 「end your reply with ONLY」在**用同一个词指两个东西**。模型选择了它被反复训练的那个含义。

**请求**：把契约的措辞与「reply = 交付物」解耦，例如明确「若需要工具，**不要回答用户**，
只输出 `{"calls":…}`」——即给出**互斥的两种输出形态**并各自命名，而不是改一个已有名词的含义。

**判据（一条即可定性）**：抓一次真实 LLM 请求体（脱敏），确认 `tools` 字段确实不存在
且 system prompt 内含上面那段。**有 `tools` ⇒ 问题在模型/供应商侧；无 `tools` ⇒ 契约形态是根因。**

## 四、P1：事件时刻的语义（**本条已在本侧缓解，不阻塞**）

事实：
- `session_events.rs:282 fn ts()` 逐事件生成，格式 `%Y-%m-%dT%H:%M:%SZ` ⇒ **秒精度**。
- 同一轮的 `think/attempt/reply` 常共享同一秒（实测 `08:30:54` ×3），而**请求侧**
  `08:30:49` ⇒ 中间那 5 秒**就是**该次调用的时长。

⇒ **并非"时长没被记录"，而是它落在请求侧与响应侧之间**。本侧已修：
`prove_track.node.js` 的 gap 语义从「到**下一个**已绘制行的间隔」改为
「**结束于本行**的等待」，于是该链的 THINK 时长读出 **5000 / 3000 / 5000 ms**，
`LLM TIME` 从 `—` 变为 **13.00s**。

**仍值得跨仓做的（低优先）**：
- **不要**为「阶段内分解」加字段——那是**在物理上不存在的事实**（一次调用产出 think+reply）。
- 若要更细的归因，**唯一有意义**的是**毫秒精度**（能把同一批写入的事件按真实间隔分开），
  以及**为调用本身记录一对起止时刻**（请求发出 / 响应到达），与本侧现有的间隔读数互为校验。

## 五、本提案**不**要求

- 不改 `tools`/供应商协议（`http_reasoning.rs` 的形状不动）。
- 不引入 API tool-calling（那是另一个决策，不是本提案）。
- 不要求 Cellrix 侧任何改动（P0-A/P0-B 全在 anaphase；P1 本侧已落地）。

---
*登记 ≠ 认领。是否做、何时做，由 anaphase 侧裁决。本文件只固定**问题、证据与判据**。*
