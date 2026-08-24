> **所属知识本体**：v4.0
> **最后更新**：2026-08-25
> **变更**：从白皮书 v3.4 拆分，无内容变更

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

## 共享知识树同步策略

- 本地检索无结果（困境 Stage 2）→ 按需向 Rhizax 查询语义相似的 L2 节点 CID
- 代谢期（Micro-Sleep / Deep Dream）→ 检查本地引用的共享节点有无新版本
- 用户显式请求探索某领域 → 拉取该领域的高共识节点
- 联邦信标到达（被动接收）→ 沙箱接收，Mind 决定是否拉取内容
- **绝无定时轮询、心跳探测、全量同步**

