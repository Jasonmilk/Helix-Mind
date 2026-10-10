# Helix 生态契约对齐核对（ECOSYSTEM v1.6 输入）

> **版本**：v1.6 草案（2026-08-28）
> **性质**：Helix-Mind 侧对齐核对报告——以代码真相源核对 ECOSYSTEM v1.5 契约，产出 v1.6 更新建议。
> **归属**：ECOSYSTEM.md 最终落位 CommonIntents 门面；本文件为 Helix-Mind 侧贡献的对齐核对。
> **前置**：ADR-0019（P3b 传输安全）/ ADR-0020（P3c CI-144 语义对齐）

## 一、核对结论

**ECOSYSTEM v1.5 契约与 Helix-Mind 当前代码实现全部对齐，无断链。** 逐项核对如下。

## 二、契约逐项核对表

### 2.1 Mind ↔ Anaphase（ECOSYSTEM §4.1）

| 契约 | 实现 | 状态 |
|:---|:---|:---:|
| `HelixQuery`（记忆检索） | gRPC 服务已实现（layer3，含 EnergyContext） | ✅ |
| `HelixConsolidate`（代谢触发） | gRPC 服务已实现 | ✅ |
| `TriggerReincarnation`（轮回） | gRPC 服务已实现（P6-3 补全完整轮回序列） | ✅ |
| `ReloadGeneLock`（基因锁重载） | gRPC 服务已实现 | ✅ |
| `SyncHumanView`（人类视图同步） | gRPC 服务已实现 | ✅ |
| **数据库独占约束**（ECOSYSTEM §1 关键约束） | ADR-0003 物理锁死，Mind 唯一持库 | ✅ |

### 2.2 Mind ↔ Callosum（ECOSYSTEM §4.2）

| 契约 | 实现 | 状态 |
|:---|:---|:---:|
| `GetStaticPrefix` / `UpdateStaticPrefix` | `docs/spec/callosum-contract.md`（P4 产出，调用契约文档化） | ✅ 契约化 |
| **边界**：Callosum 是 Anaphase 器官，Mind 单向调用请求分配，分配策略归 Anaphase | ADR-0017 边界声明 | ✅ |

### 2.3 认知预算（ADR-0010，v1.6 新增）

- `EnergyContext.budget_tier` 前置路由（Callosum 决定，Mind 只执行）——**v1.6 需写入契约字段**。

### 2.4 trace 透传（ADR-0020，v1.6 新增）

- `HelixQueryRequest/Result` 追加 `traceparent`（W3C），透传不生成（Anaphase 入口生成根 trace_id）。**v1.6 需写入契约字段。**

### 2.5 联邦共享（ECOSYSTEM §3.3）

| 项 | 状态 |
|:---|:---|
| 出站打包 | 代码存在，**feature flag 关闭**（ADR-0018：能力未就绪=功能不存在） |
| review 双盲 | P3a 确定性审查引擎就绪 |
| L0 哈希信用锚定 | 已实现 |
| Rhizax UDS 接口 | 仅接口预留（三阶段演进第一阶段） |

### 2.6 Anaphase ↔ Cellrix（ECOSYSTEM §4.4，Cellrix 侧）

| 端点 | 状态 |
|:---|:---|
| `/v1/agent/snapshot` | ✅ 已实现（Cellrix 侧） |
| `/v1/agent/action` | ✅ 已实现 |
| `/v1/cap/manifest` | ⏳ 待实现（Cellrix 侧，不阻塞 Mind） |
| `/v1/cap/decisions` | ⏳ 待实现（Cellrix 侧） |

## 三、ECOSYSTEM v1.6 更新建议（Helix-Mind 侧贡献）

1. **Helix-Mind 项目状态更新**：0.1.0 骨架 → **P0-P6 全部完成**（检索/代谢/安全/契约/WAL/轮回/商业化底座）。
2. **知识本体**：v4.1（认知相态河流范式，非"记忆中枢"海马体描述）。
3. **新增契约字段**：`EnergyContext.budget_tier`（ADR-0010）、`traceparent`（ADR-0020）。
4. **联邦出站声明**：P3 起 feature-gated，能力就绪才存在。
5. **认知工艺**（ADR-0021）：Mind 内"如何思考"系统，Phase 1 已定稿（v1.6 备注生态哲学约束：编排 Mind 内、执行 CognitiveService）。

## 四、CI-144 动词映射核对（P7-4）

ADR-0020 映射表 vs INTENT-7 spec（`CommonIntents/INTENT-7/spec/INTENT-7.md`）逐项核对：

| INTENT-7 动词 | spec 定义 | Mind 映射 | 核对 |
|:---|:---|:---|:---:|
| `FETCH` | §3.1 语义检索（summary-first） | `HelixQuery` | ✅ 一致 |
| `WRITE_NODE` | §3.2 提交记忆/知识 | `HelixConsolidate` / L1 `Remember` | ✅ 一致 |
| `TENTACLE` | §3.3 沙箱外部工具调用（需 CAPABILITY-13 权限声明） | Anaphase 编排，Mind 不执行 | ✅ 一致（body-agnostic） |
| `FINISH` | §3.4 成功结束认知循环 | Anaphase 生命周期，Mind 记录 L3 收尾 | ✅ 一致 |
| `CANCEL` | §3.4 中止执行树（深度僵局/安全违规） | Anaphase 生命周期，Mind 记录中止原因 | ✅ 一致 |

**结论**：5 个动词全部对齐，映射表与 INTENT-7 spec 无歧义。HXR payload schema 与 L3 节点 `content` JSON 完全同构（INTENT-7 §4.1）——与 Mind L3 情景记忆契约天然一致。

## 五、v1.6 落位路径

- **本文件为 Helix-Mind 侧对齐核对产物**（本仓库，已版本化）。
- **ECOSYSTEM.md v1.6 正式文本**：待用户确认后写入 CommonIntents 门面（`.github/`，与 GOVERNANCE 同路径治理）。
