# Helix-Mind 开发导航牌（PLAN）

> **版本**：v5.1（导航牌化，2026-08-28）
> **状态**：🚧 P3 安全与契约（计划待起草）
> **分支**：rs-dev
> **所属方法论**：DNA 自生长方法论 v2.0（PLAN 动态流转闭环）
> **规则**：本文件只含当前阶段 + 下一阶段预览 + 阶段总览地图。完成阶段 → GROWTH.md。总行数 ≤150，超出触发历史迁移。

---

## 1. 当前阶段：P3 安全与契约（计划待起草）

> **状态**：P2 已完成（见 GROWTH 2026-08-28）。P3 计划待起草，经审查后再开工（DNA 方法论：计划先于代码）。

### 1.1 P3 目标（既定范围，详情待起草）
| 任务 | 内容 | 依赖 |
|---|---|---|
| M-07 | 联邦确定性审查（feature flag `federation` 在此阶段打开，能力未就绪=功能不存在） | P2 |
| M-08 | API/Health 补全；UDS SO_PEERCRED 鉴权（本地部署）/ 远程 mTLS（预留） | P2 |
| CI-144 | 语义对齐（INTENT-7 / BIND-19 / CAPABILITY-13） | P3 内 |

### 1.2 P2 完成摘要（详细记录 → `docs/GROWTH.md` [2026-08-28]）
- **ADR-0014/0017 Active**：P2a 确定性代谢（`find_similar_node` + 失调消解闭环）+ P2b `CognitiveService` 端口（三适配器 + llm_mode 门控）+ P2c 越界清零（reqwest 收敛到 `cognitive.rs` 唯一出网点）
- **数据契约 Append-Only**：`RelationType::Conflicts`、edges.created_at、`dissonance_window_hours`（消魔法数 24）
- **验收**：全量测试全绿（metabolism 15 单测 + 3 集成）、0 warning、Z2 门控物理生效

### 1.3 下一阶段预览：P4 硬冻结兑现 + 生态接口
- M-10 `activation_vector`（Append-Only 扩展，reserved 13 落地）
- Mind→Callosum 调用契约文档（消费方，非实现）
- M-12 Rhizax 预留接口

---

## 2. 阶段总览（地图，不展开）

| 阶段 | 内容 | 状态 |
|---|---|---|
| P0-Pre | Z1-Z6 清零（联邦/LLM 门控、Health、layer3 透传、测试诚实化、FTS5 验证） | ✅ |
| P0 | 认知基线 + 可编译数据契约（ADR-0010/11/12） | ✅ |
| P0.5 | 检索测试基线（ADR-0016） | ✅ |
| P1 | 检索闭环（FTS5 trigram + 异步索引 + 注入防御 + 相态加权，ADR-0013） | ✅ 2026-08-28 |
| P2 | 代谢闭环（a/b/c 拆分，无 LLM 起步，ADR-0014/0017） | ✅ 2026-08-28 |
| P3 | 安全与契约（联邦审查、UDS SO_PEERCRED / 远程 mTLS、API/Health） | 🚧 当前 |
| P4 | 硬冻结兑现 + 生态接口（activation_vector、Mind→Callosum 契约、Rhizax 预留） | ⏳ |
| P4.5 | 架构审查点（ADR-0015 WAL 设计 + 原型，4 项产出） | ⏳ |
| P5 | 领域 WAL（独立日志 + 完整性校验 + 投影器 + replay） | ⏳ |
| P6 | 数据诚实 + 轮回 + 商业化（parquet 名实相符、多租户 WAL 分区） | ⏳ |
| P7 | 生态文档同步（ECOSYSTEM v1.6 + CI-144 对齐） | ⏳ |

---

## 3. 活跃决策与契约指针（不展开）

| 项 | 指针 |
|---|---|
| 决策 D1-D7 | LLM 安顿（CognitiveService）/ 治理轻量 / 知识哲学 / 领域 WAL / 滞后开源 / 顺序 / 能力未就绪=功能不存在 |
| 知识哲学 | VISION 原子原则 + `spec/philosophy.md` + `spec/phase-states.md` |
| 相态模型 | `spec/phase-states.md` + ADR-0011（气/液/胶/晶 + 主体依赖轴，物化策略） |
| 认知预算 | ADR-0010（`budget_tier` 前置路由，Mind 只执行；P2 落地范围过滤） |
| 硬冻结扩展 | ADR-0012（Append-Only Schema Evolution，reserved 保护） |
| L2 共享 | ADR-0016（自动共享 + 洗脱前置，宪法级） |
| P1 检索 | ADR-0013（FTS5 trigram + 异步索引 + 注入防御 + 相态加权） |
| 领域 WAL | ADR-0015（P4.5 产出）+ 独立日志文件设计 |
| 商业化 | D5（滞后开源；实现闭源 / 协议公开 / 哲学自由传播） |
| CI-144 对齐 | VISION 生态位置 + INTENT-7 / BIND-19 / CAPABILITY-13（P3/P7） |
| 参考资源 | Event Sourcing / The Log / CI-144 / Lumtract（见 VISION 组件仓库索引） |

---

## 4. 文档生态 SOP（DNA v2.0）

PLAN 是导航牌不是历史档案；阶段收尾时（收尾 SLA：24h）完成记录追加 GROWTH.md 并从 PLAN 移除；GROWTH ≤3 条超则归档；PLAN ≤150 行超则触发历史迁移。详见 `docs/DNA.md`「文档生态 SOP」。
