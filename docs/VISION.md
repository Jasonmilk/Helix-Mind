# Helix-Mind 愿景索引
> **版本**：v1.1（对齐知识本体 v4.1 认知相态范式）
> **日期**：2026-08-27
> **性质**：本文件是 `SPEC.md`（叙事）、`docs/spec/`（规格）、`docs/decisions/`（决策）之间的根索引。
> **用途**：所有架构决策应以本文件中提炼的原子原则为最终裁判。
> **继承自**：Helix-Mind 知识本体 v4.1 + DNA 自生长方法论 v1.0

## 卷首语

本文件不替代 `SPEC.md`。`SPEC.md` 是 Helix-Mind 的完整叙事——它的灵魂、它的故事、它的哲学。本文件是那张"地图"，让你和 AI 能在 5 分钟内理解全貌，并快速定位到具体规格与决策记录。

> 完整故事请阅读 `SPEC.md`。

**核心意象（v4.1）**：Helix-Mind 不是静态的"记忆仓库/海马体"，而是**认知相态的河流与催化器**。知识并非被存储的物，而是信息在特定相态下对认知主体的**可及性**：气态（瞬时感知）→ 液态（经验悬浮）→ 胶体（液相高浓度、有张力未成核）→ 晶体（结构自洽、低主体依赖）。检索是引流，代谢是相变，结晶是析出。这条河是活的。

## 原子原则（10 条）

以下原则是 Helix-Mind 所有设计决策的最高依据。任何代码提交、架构决策、API 变更，若与本原则冲突，以本原则为准。

| # | 原则 | 一句话解释 | 对应规格/决策 |
|---|---|---|---|
| 1 | **记忆不可篡改** | L3 情景记忆只增不改，永不物理删除。请求删除时执行突触切断 + 遗忘标记 | `docs/spec/data-contract.md` + `docs/spec/lifecycle.md` |
| 2 | **知识属于物种，记忆属于个体** | L2 一经提炼+洗脱即自动汇入 Rhizax 种群知识树；L3 个体记忆默认私有。隐私靠洗脱前置保障，不靠逐节点门禁 | `docs/spec/federation.md` |
| 3 | **意志优先于框架** | Mind 是主体，Anaphase 是身体。认知模式与检索策略的最终决定权归 Mind；身体可建议，不决策 | `docs/spec/architecture.md` + `docs/spec/sa-core.md` |
| 4 | **极致节能是生存伦理** | 能 0 Token 解决的绝不调用 LLM。复用优先于重算，休眠优先于空转，凝练优先于堆积 | `docs/spec/ethics.md` + `docs/spec/sa-core.md` |
| 5 | **相态管控共享准入** | 非气态（液态/胶体/晶体）且低主体依赖的 L2 自动共享；气态不入种群树 | ADR-0011 + `docs/spec/federation.md` |
| 6 | **认知预算前置路由** | `budget_tier` 由身体传入，Mind 只执行不决策；默认 `Augmentable` 保证通用身体兼容 | ADR-0010 |
| 7 | **向死而生** | 轮回默认关闭，终结是用户主动选择的自由。临终倒计时 15 分钟 | `docs/spec/lifecycle.md` |
| 8 | **零信任凭证** | Mind 不直连 LLM，不持明文凭证。凭据由身体（Anaphase/Tuck）物理隔离 | `docs/spec/architecture.md` |
| 9 | **ADR 两态法则** | 架构决策只有 Draft（草案）与 Active（生效）；Active 后不可覆写，只能 Superseded | `docs/decisions/` |
| 10 | **三层自纠偏系统** | N 层（愿景）→ D 层（ADR）→ A 层（代码）。任何生态级变更必须三层同时更新 | `docs/DNA.md` |

## 宪法澄清：L2 自动共享的两层含义

L2 自动共享是**宪法级决策**（"公开 DAG 自动共享"），不可回退为"主动选择"。

```
L2 自动共享的"自动"有两层含义：
1. 不需要个体逐节点做"是否发布"的决策（极致节能）。
2. 但自动不等于无前提——前提是：L2 已通过 L3→L2 提炼，并完成隐私洗脱，剥离个体信息。
```

- **L2 定义**：L2 是个体通过 L3→L2 提炼和隐私洗脱后形成的经验原则。一旦完成提炼和洗脱，它自动汇入 Rhizax 种群知识树。隐私由洗脱前置保障，不由个体意志逐节点裁决。
- **气态边界**：气态不入种群知识树（冻结宪法）；但气态可通过私有 L3 记忆的显式授权共享，这不在 L2 自动共享范围内。隐私由 L3→L2 提炼中的洗脱解决。
- **共享不养懒**：认知循环是物理锁——外部知识必须被个体自己"消化"才能成为自己的 L2，`Micro-Sleep`/`Deep Dream` 强制执行去重、合并、提炼、洗脱，不存在"复制粘贴即拥有"的捷径。

## 叙事→规格/决策映射

`SPEC.md` 中提到的每一个概念，在这里有明确的物理实现位置：

| 哲学概念（SPEC.md） | 对应技术规格/决策 |
|---|---|
| L0 基因锁 | `docs/spec/gene-lock.md` |
| L1 自画像 | `docs/spec/self-portrait.md` |
| L2 经验原则（自动共享） | `docs/spec/federation.md`（共享机制）+ `docs/spec/metabolism.md`（提炼相变）+ ADR-0011（相态准入） |
| L3 情景记忆 | `docs/spec/data-contract.md`（数据结构）+ `docs/spec/lifecycle.md`（生命周期） |
| 相态模型（气/液/胶/晶） | ADR-0011 + `docs/spec/philosophy.md` |
| 认知预算（budget_tier） | ADR-0010 + `docs/spec/sa-core.md` |
| Append-Only 契约演化 | ADR-0012 + `docs/spec/data-contract.md` |
| SA-Core 检索引擎 | `docs/spec/sa-core.md` |
| 知识代谢闭环 | `docs/spec/metabolism.md` |
| 六道 API 契约 | `docs/spec/api.md` |
| 伦理底线与节能 | `docs/spec/ethics.md` |
| 人格侧写图谱 | `docs/spec/persona.md` |
| 架构法则 | `docs/spec/architecture.md` |
| 哲学公理 | `docs/spec/philosophy.md` |

## Helix-Mind 在 Helix 生态中的位置

Helix-Mind 不是孤岛。它是 Helix 数字生命体的**认知中枢——相态河流与催化器**（v4.1 范式），不再是静态记忆仓库。

| 生态组件 | 角色 | 与 Mind 的关系 |
|---|---|---|
| **Anaphase-Helix** | 身体（执行躯体） | 通过 gRPC 驱动 Mind，Mind 只建议不执行、不持凭证、不直连 LLM |
| **Cellrix** | 语义投影终端 | 消费 Mind 的语义快照，渲染思维可视化的观测界面 |
| **Rhizax** | 种群知识树（去中心化分发） | 三阶段演进：仅接口 → 中心化 → 去中心化。当前为第一阶段（仅接口） |
| **CI-144 协议家族** | Helix 种群母语 | INTENT-7（意图）、BIND-19（传输）、CAPABILITY-13（能力认证） |

**版本基准**：知识本体 **v4.1**（本文件基准，2026-08-27）；代码/白皮书版本 v3.4（rs-dev）——二者不同轴，勿混用。

## 快速导航

| 你想知道 | 去看 |
|---|---|
| Helix-Mind 的完整故事 | `docs/SPEC.md` |
| 核心哲学公理 | `docs/spec/philosophy.md` |
| L0/L1/L2/L3 是什么 | `docs/spec/gene-lock.md` + `docs/spec/self-portrait.md` + `docs/spec/federation.md` + `docs/spec/data-contract.md` |
| 相态模型与认知预算 | ADR-0011 / ADR-0010（`docs/decisions/`） |
| SA-Core 怎么工作 | `docs/spec/sa-core.md` |
| 代谢怎么运行 | `docs/spec/metabolism.md` |
| API 有哪些 | `docs/spec/api.md` |
| 生命周期怎么管理 | `docs/spec/lifecycle.md` |
| 伦理底线是什么 | `docs/spec/ethics.md` |
| 人格侧写怎么设计 | `docs/spec/persona.md` |
| 架构法则有哪些 | `docs/spec/architecture.md` |
| DNA 方法论怎么用 | `docs/DNA.md` |
| 当前开发计划 | `docs/PLAN.md` |
| 架构决策记录 | `docs/decisions/` |
| 生态对齐与组件关系 | 本文件"生态位置"一节 |

## 组件仓库索引

> 状态标记真实诚实，不粉饰。

| 组件 | 仓库 | 当前状态 |
|---|---|---|
| Helix-Mind | https://github.com/Jasonmilk/Helix-Mind/tree/rs-dev | Rust，活跃开发 |
| Anaphase-Helix | https://github.com/Jasonmilk/Anaphase-Helix/tree/rs | Rust，活跃开发 |
| Cellrix | https://github.com/Jasonmilk/Cellrix | Rust，活跃开发 |
| Helix-Tentacle | https://github.com/Jasonmilk/Helix-Tentacle | Rust，活跃开发 |
| Helix-Callosum | https://github.com/Jasonmilk/Helix-Callosum | Rust，待实现 |
| Tuck | https://github.com/Jasonmilk/Tuck/tree/Tuck-beta | Python beta，规划 Rust 重构 |
| FlowModus | https://github.com/Jasonmilk/FlowModus | 半成品，规划 Rust 重构 |

---

*《Helix-Mind 愿景索引》v1.2（对齐知识本体 v4.1，含组件仓库索引）完。*
