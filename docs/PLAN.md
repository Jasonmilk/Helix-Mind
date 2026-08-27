# Helix-Mind 开发导航牌（PLAN）

> **版本**：v5.1（导航牌化，2026-08-28）
> **状态**：🚧 P2 代谢闭环 进行中
> **分支**：rs-dev
> **所属方法论**：DNA 自生长方法论 v2.0（PLAN 动态流转闭环）
> **规则**：本文件只含当前阶段 + 下一阶段预览 + 阶段总览地图。完成阶段 → GROWTH.md。总行数 ≤150，超出触发历史迁移。

---

## 1. 当前阶段：P2 代谢闭环（a/b/c）

### 1.1 目标
| 任务 | 内容 | 状态 |
|---|---|---|
| P2a-M-04 | `find_similar_node` 真实实现（FTS5 matchinfo + 编辑距离，SQLite 层，ADR-0014；度量：结构重叠，非 LLM embedding） | ⏳ |
| P2a-M-05 | 失调消解（符号学求解器，无 LLM，确定性） | ⏳ |
| P2b-M-09 | `CognitiveService` 端口 + 三适配器（Fake / Deterministic / Remote） | ⏳ |
| P2c | crystallize / NER 接入 CognitiveService（替换遗留 reqwest 直连，LLM 门控默认 disabled） | ⏳ |

### 1.2 技术前提
- ✅ P1 检索闭环落地（ADR-0013）：FTS5 trigram 索引 + `FtsExtractor` 生产默认
- ✅ `get_nodes_by_phase` 原语就位（预算路由，ADR-0010）
- ✅ `digest.rs` Levenshtein / `symbolic_solver` 现有（审查③ O10 指出现状）

### 1.3 入口 ADR
- **ADR-0014**（P2a）：`find_similar_node` 相似度度量 + SQLite 层实现（待创建）

### 1.4 F3 前置修复（P2a 前置条件，已完成 2026-08-28）
- **裁决**：选项 (a) 单事务批量更新，立即执行；选项 (b) 内存计数 + 代谢期回刷归 **P4+**（等 Micro-Sleep 就绪后自然接入）。
- **落地**：`StorageEngine::bump_access_counts`（单事务、SQL 级原子 `access_count = access_count + 1`，去读-改-写竞争、去逐节点 fsync）；`update_access_counts` 改为批量调用；测试 `bump_access_counts_is_atomic_batch`（批量自增 / 原子累加 / 空批 no-op）。
- **验收**：`cargo test` 全绿（27）｜ `access_count` 正确累加 ✅ ｜ 查询热路径从 N×Critical 写盘降为 1×deferred 事务 ✅

### 1.5 下一阶段预览：P3 安全与契约
- M-07 联邦确定性审查（feature flag `federation` 在此阶段打开，能力未就绪=功能不存在）
- M-08 API/Health；UDS SO_PEERCRED 鉴权（本地部署）/ 远程 mTLS（预留）
- CI-144 语义对齐（INTENT-7 / BIND-19 / CAPABILITY-13）

---

## 2. 阶段总览（地图，不展开）

| 阶段 | 内容 | 状态 |
|---|---|---|
| P0-Pre | Z1-Z6 清零（联邦/LLM 门控、Health、layer3 透传、测试诚实化、FTS5 验证） | ✅ |
| P0 | 认知基线 + 可编译数据契约（ADR-0010/11/12） | ✅ |
| P0.5 | 检索测试基线（ADR-0016） | ✅ |
| P1 | 检索闭环（FTS5 trigram + 异步索引 + 注入防御 + 相态加权，ADR-0013） | ✅ 2026-08-28 |
| P2 | 代谢闭环（a/b/c 拆分，无 LLM 起步） | 🚧 当前 |
| P3 | 安全与契约（联邦审查、UDS SO_PEERCRED / 远程 mTLS、API/Health） | ⏳ |
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
