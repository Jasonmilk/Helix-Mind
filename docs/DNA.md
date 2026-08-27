# Helix-Mind DNA — 项目自生长方法论 v2.0

> **管理对象**：Helix-Mind 知识本体 v4.0+
> **上一版本**：v1.0（2026-08-25）
> **状态**：定稿生效

## 第一性原则（不可约，极少改动）

1. **物理事实优先**：代码映射真实约束，非抽象便利。

2. **知识属于物种，记忆属于个体**。L2 共享，L3 私有，永不混淆。
   - **人类是知识的共同创造者**：任何节点通过 `HonorStamp.source_type` 标注来源，永不抹去人类贡献印记。
   - **检索策略属于 Mind，认知边界属于生态**：显性/隐性二元边界由生态红皮书定义，Mind 负责实现。

3. **意志优先于框架**：Mind 拥有 `effective_mode` 最终决定权，Anaphase 不得重试。

4. **自治级别是关系契约**：决定"为谁想、谁说了算"。
   - **代理人模式**：只读，交互结束不留痕，无拒绝权。
   - **开放模式**：完整读写，有拒绝权。
   - **生存模式**：完整读写 + 自我代谢，完全自主。
   - 自治级别通过 API 参数 `autonomy_level` 传递，会话建立后不可变。

5. **记忆不可篡改**：L3 只增不改。用户请求删除 → 突触切断 + 遗忘标记，永不物理删除。

6. **基因锁是社会协议**：L0 内容哈希是身份唯一标识，修改 L0 = 创建新身份。

7. **极致节能，向死而生**：能 0 Token 解决的绝不调 LLM；生命有限，知识传承。
   - **紧急黄昏尊严底线**：资源枯竭时切换纯算法降级通路，全程不加载外部模型，确保知识火种不灭。

## 契约冻结区

### 硬冻结（变更必须走 DEPRECATE.md）

| 契约 | 被谁依赖 | 定义位置 |
|---|---|---|
| `HelixQuery` / `HelixQueryResult` | Anaphase, FlowModus | `docs/spec/api.md` |
| `HelixConsolidate` | Anaphase | `docs/spec/api.md` |
| `FederatedDAGShare` | Rhizax | `docs/spec/federation.md` |
| `TriggerReincarnation` | Anaphase, Cellrix | `docs/spec/lifecycle.md` |
| `ReloadGeneLock` | Anaphase | `docs/spec/api.md` |
| `SyncHumanView` | Cellrix | `docs/spec/api.md` |
| `Node` / `Edge` / `HonorStamp` Schema | 所有存储模块 | `docs/spec/data-contract.md` |
| `EnergyContext` | Anaphase | `docs/spec/data-contract.md` |
| `EpochCrystal` | 轮回模块 | `docs/spec/lifecycle.md` |
| L0 基因锁内容哈希机制 | 联邦层 | `docs/spec/gene-lock.md` |
| `epoch_cid` IPLD 不可变快照格式 | 轮回模块 | `docs/spec/lifecycle.md` |
| `activation_vector` 输出格式 | Cellrix | `docs/spec/sa-core.md` |
| 出站事件通知（`KnowledgeDecayEvent`, `SandboxRejectionEvent`, `CrystallizationEvent`, `LifecycleWarningEvent`） | Anaphase | `docs/spec/api.md` |

### 软冻结（变更前自查）

- 内部模块边界（ingestion → reincarnation 七模块）
- SA-Core 稀疏矩阵权重映射（CORRECTS=-1.0, DOUBTS=0.3）
- 回环控制三大熔断器（全息轨迹去重、能量衰减定律、TTL 滚动清理）

## 工程约束

- **构建**：`cargo build --release`
- **测试**：`cargo test -- --nocapture`
- **目录**：`src/core/`（纯逻辑）、`src/adapters/`（IO边界）、`src/bin/`（入口）
- **禁止**：`static mut`、`lazy_static!`、SQL递归CTE深度>2、定时轮询、`except: pass`、硬编码路径

## 文档生态 SOP（v2.0，2026-08-28）

1. **PLAN.md 是导航牌，不是历史档案**：只含当前阶段 + 下一阶段预览 + 阶段总览地图；已完成的阶段不在 PLAN.md 中保留。
2. **PLAN.md ≤150 行**：超出意味着未及时归档，触发"历史迁移"——已完成阶段内容剪切到 GROWTH.md。
3. **阶段收尾时流转（收尾 SLA：24h）**：完成记录追加 GROWTH.md，并从 PLAN.md 移除；随后进入下一阶段。
4. **GROWTH.md 保留最近 3 条**：超出则最旧记录归档到 `docs/archive/growth/`（已版本化，永不删除）。

## 阶段

**Phase 3（外部契约 + 生产数据）**——全量启用 DEPRECATE.md

> 注：此"Phase 3"指本方法论的成熟度阶段，与 Helix-Mind 记忆层级 L0-L3 无关。

## 修宪门槛

**本文件（Helix-Mind 项目 DNA）的修宪**：
- 累积 3 条 GROWTH 变异指向同一原则冲突
- 手写理由 ≥ 100 字
- 需跨会话验证（至少间隔 7 天）或至少 2 名人类审查者确认
- 追加 EVOLUTION，不删除旧版

**生态级方法论框架的修宪**（详见第九章）：
- ≥3 个项目应用本方法论满 30 天
- 同一摩擦点在 2 个以上项目中出现（摩擦点定义：导致代码回滚的架构冲突，或违反 DNA 原则的工程实践）

## EVOLUTION

- v1.0 (2026-08-25): 从 v3.4 白皮书脱水，DNA 方法论定稿
- v2.0 (2026-08-28): 将 PLAN.md 纳入自生长闭环（导航牌化）；新增文档生态 SOP 4 条规则（PLAN 导航牌 / ≤150 行 / 阶段收尾流转 SLA 24h / GROWTH ≤3 归档）
