> **所属知识本体**：v4.1
> **最后更新**：2026-08-28
> **变更**：P3c（ADR-0020）追加 INTENT-7 动词映射 + traceparent 透传说明

# API 分层与通用性

## 分层原则

三层 API 实现通用性。Layer 1 极简（query/remember/forget），零认知负担。Layer 2 高级（mode、top_k、max_depth）。Layer 3 原生（完整 EnergyContext、认知模式、轮回、联邦共享）。

## Mind 的对外契约

- `HelixQuery`：检索请求/响应（含 `include_recessive` 和 `suggested_actions`）
- `HelixConsolidate`：睡眠管道触发/结果
- `FederatedDAGShare`：联邦 DAG 共享
- `TriggerReincarnation`：手动轮回触发
- `ReloadGeneLock`：基因锁重载
- `SyncHumanView`：人类视图同步

## Mind 的出站事件通知（gRPC 流式推送）

- `KnowledgeDecayEvent`：知识节点 𝒰 衰减触发（通知 Anaphase 调整决策权重）
- `SandboxRejectionEvent`：沙箱一票否决（通知 Anaphase 外部知识被拒绝的原因）
- `CrystallizationEvent`：知识晶体生成（通知 Anaphase 新 L2 节点已固化）
- `LifecycleWarningEvent`：生命周期警告（临终倒计时启动通知）

## Layer 3 新增字段（v3.4）

`HelixQueryResult` 新增：

```
activation_vector: [ActivationEntry]  // SA-Core 能量扩散后的节点激活值向量
```

## CI-144 INTENT-7 动词映射（P3c，ADR-0020）

Mind 的 gRPC 契约与 CI-144 INTENT-7 动词的显式映射（单一真相源）：

| INTENT-7 动词 | Mind gRPC 契约 | 说明 |
|:---|:---|:---|
| `FETCH` | `HelixQuery` | 语义检索（summary-first） |
| `WRITE_NODE` | `HelixConsolidate` / L1 `Remember` | 记忆写入 |
| `TENTACLE` | （Anaphase 编排，Mind 不执行） | Mind 只建议，不执行工具 |
| `FINISH` | 认知循环结束（Anaphase 生命周期） | Mind 记录 L3 收尾 |
| `CANCEL` | 中止执行树（Anaphase 生命周期） | Mind 记录中止原因 |

**边界**：Mind 只认 Anaphase gRPC 接口，不直接对接 TENTACLE / Executor（body-agnostic 铁律）。

## traceparent 透传（P3c，ADR-0020）

- `HelixQueryRequest.traceparent` / `HelixQueryResult.traceparent`：W3C trace-context 格式。
- **透传不生成**：根 `trace_id` 由 Anaphase 在入口生成（ECOSYSTEM §3.1），Mind 原样回传，支持全链路审计（Tuck CAS）。
- 请求无 traceparent → 响应为空（Mind 不是 trace 根）。
- 字段号为 Append-Only 追加（ADR-0012）：`HelixQueryRequest` 用 7，`HelixQueryResult` 用 14（13 已预留 activation_vector），后续 `reserved` 保护。

## 共享知识树同步策略

- 本地检索无结果（困境 Stage 2）→ 按需向 Rhizax 查询语义相似的 L2 节点 CID
- 代谢期（Micro-Sleep / Deep Dream）→ 检查本地引用的共享节点有无新版本
- 用户显式请求探索某领域 → 拉取该领域的高共识节点
- 联邦信标到达（被动接收）→ 沙箱接收，Mind 决定是否拉取内容
- **绝无定时轮询、心跳探测、全量同步**

