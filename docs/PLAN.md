# Helix-Mind 开发导航牌（PLAN）

> **版本**：v6.3（P10a 完成，2026-09-06）
> **状态**：🚧 P10 认知工艺与生态深度集成（P10a 触发链路完成，P10b 待开工）
> **分支**：rs-dev
> **所属方法论**：phyt-DNA 方法论 v1.0（PLAN 动态流转闭环，方法论锚点项目 https://github.com/Jasonmilk/phyt-DNA）
> **规则**：本文件只含当前阶段 + 下一阶段预览 + 阶段总览地图。完成阶段 → GROWTH.md。总行数 ≤150，超出触发历史迁移。
> **⚠️ 状态修正记录**：v5.2 顶部状态停留在 P3（计划待起草），但阶段总览已显示 P0-P9 全部完成（2026-08-28）。v6.0 修正顶部状态与阶段总览一致。v6.2 定稿 P10 任务书（ADR-0031）。v6.3 P10a 完成（helix_craft RPC + Anaphase 触发）。

---

## 1. 当前阶段：P10 认知工艺与生态深度集成（任务书）

> **状态**：🚧 P10a 完成（P10-0 零硬编码 + helix_craft 触发链路全通），P10b 待开工。
> **前置依赖**：P9 认知工艺 Phase 3 已完成（价值评估 + 自适应突变 + 睡眠复盘 + bm25 门控增强）。

### 1.1 P10 任务书（ADR-0031 已立）

| # | 任务 | 内容 | 验收 |
|---|---|---|---|
| P10-0 | 零硬编码收口 | trace_id 确定性化（`craft#{job_id}`，去 uuid）+ mutation/review 阈值进 CraftConfig | ✅ 8f7110b：无 uuid、认知测试全绿 |
| P10a-T1 | proto + server | 新增 `helix_craft` RPC（请求/响应/转换） | ✅ 8c15a4f：gRPC 编译 + 3 集成测试 |
| P10a-T2 | 接线 CognitiveCraft | helix_craft → CognitiveCraft.orchestrate（CraftConfig 注入） | ✅ 并入 T1：DeterministicAdapter 默认 0-token |
| P10a-T3 | Anaphase 触发点 | mind adapter 新增 craft + run_cycle 按需触发 | ✅ db07c5b：MemoryRetrieval 触发 + [think-first] 注入 |
| P10b-T1 | 策略节点 schema | L1 策略节点 + provenance `craft#` | 类型落地 |
| P10b-T2 | synthesis 持久化 | CraftResult → L1（storage 复用，WAL 投影器） | 落盘 + 回读一致 |
| P10b-T3 | 价值分级 + 检索复用 | ValueAssessor 分级写元数据；helix_query 命中策略节点 | 策略复用生效 |
| P10c-T1 | Deep Dream 挂载 | trigger_hibernate → SleepReview.review_path | 事件驱动复盘执行 |
| P10c-T2 | 突变应用 | AdaptiveMutation 状态持久化 + Stale 降权 | 权重更新落盘 |
| P10-T | 测试 + 文档 | 集成测试全绿；ADR-0031 + PLAN v6.2 + GROWTH + README + ECOSYSTEM | 全绿 + 文档对齐 |

**边界（ADR-0031 D5）**：认知工艺不直连 LLM（B1 保持）；不做自动路由（M3 边界，触发由 Anaphase 判断）；不新建存储/crate（极致复用）；不破坏 helix_query 语义（向后兼容）。

### 1.2 P0-P9 已完成内容（历史记录）

P0-Pre → P9 全部于 2026-08-28 完成，详见阶段总览（第 2 节）。
关键里程碑：
- **P1 检索闭环**：FTS5 trigram + 异步索引 + 注入防御 + 相态加权
- **P2 代谢闭环**：a/b/c 拆分，无 LLM 起步
- **P3 安全与契约**：联邦审查 + UDS SO_PEERCRED + API/Health
- **P5 领域 WAL**：独立日志 + 完整性校验 + 投影器 + replay
- **P8 认知工艺 Phase 2**：编排器最小原型，DeterministicAdapter 闭环（0 Token）
- **P9 认知工艺 Phase 3**：价值评估 + 自适应突变 + 睡眠复盘 + bm25 门控增强

### 1.3 技术前提
- ✅ `SymbolicSolver` 就绪（P2a，5 单测）；`is_high_risk_node` 可用
- ✅ HealthServer 已注册；tokio "full" 含 UDS
- ✅ CognitiveCraft / SleepReview / AdaptiveMutation / ValueAssessor / system0_gate 已实现（确定性 0 Token）
- ✅ 代谢 trigger_by_event / trigger_hibernate 已存在（P10c 挂载点）
- ⚠️ tonic `serve_with_incoming` + `UnixListener` peer_cred 注入需原型验证（连接级身份 → 鉴权）

### 1.4 入口 ADR
- **ADR-0031**（✅ Accepted，2026-09-06）：P10 认知工艺生态深度集成（helix_craft RPC ✅ / L1 策略持久化 ⏳ / Deep Dream 挂载 ⏳ / 零硬编码收口 ✅）
- ADR-0018/0019/0020（P3 时代，已完成）

### 1.5 下一阶段预览：P11（待 P10 实测后规划）
- P11 候选：认知工艺策略库的跨会话复用深化 / 编排质量指标采集（GROWTH 期实测锚定）

---

## 2. 阶段总览（地图，不展开）

| 阶段 | 内容 | 状态 |
|---|---|---|
| P0-Pre | Z1-Z6 清零（联邦/LLM 门控、Health、layer3 透传、测试诚实化、FTS5 验证） | ✅ |
| P0 | 认知基线 + 可编译数据契约（ADR-0010/11/12） | ✅ |
| P0.5 | 检索测试基线（ADR-0016） | ✅ |
| P1 | 检索闭环（FTS5 trigram + 异步索引 + 注入防御 + 相态加权，ADR-0013） | ✅ 2026-08-28 |
| P2 | 代谢闭环（a/b/c 拆分，无 LLM 起步，ADR-0014/0017） | ✅ 2026-08-28 |
| P3 | 安全与契约（联邦审查、UDS SO_PEERCRED / 远程 mTLS、API/Health） | ✅ 2026-08-28 |
| P4 | 硬冻结兑现 + 生态接口（activation_vector、Mind→lodestone 投影契约、Rhizax 预留） | ✅ 2026-08-28 |
| P4.5 | 架构审查点（ADR-0015 WAL 设计 + 原型，4 项产出） | ✅ 2026-08-28 |
| P5 | 领域 WAL（独立日志 + 完整性校验 + 投影器 + replay） | ✅ 2026-08-28 |
| P6 | 数据诚实 + 轮回 + 商业化（parquet 名实相符、多租户 WAL 分区） | ✅ 2026-08-28 |
| P7 | 生态文档同步（ECOSYSTEM v1.6 对齐 + CI-144 核对 + 认知工艺 Phase 1） | ✅ 2026-08-28 |
| P8 | 认知工艺 Phase 2（编排器最小原型，DeterministicAdapter 闭环，ADR-0021） | ✅ 2026-08-28 |
| P9 | 认知工艺 Phase 3（价值评估、自适应突变、睡眠复盘、bm25 门控增强，ADR-0021） | ✅ 2026-08-28 |
| P10 | 认知工艺与生态深度集成（helix_craft 触发链路 ✅、L1 策略持久化 ⏳、Deep Dream 复盘挂载 ⏳，ADR-0031） | 🔄 P10a 完成 |

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
| 认知工艺 | ADR-0021 + `spec/cognitive-craft.md`（Mind=编排 / CognitiveService=执行，Phase 1 定稿） |
| 参考资源 | Event Sourcing / The Log / CI-144 / Lumtract（见 VISION 组件仓库索引） |

---

## 4. 文档生态 SOP（DNA v2.0）

PLAN 是导航牌不是历史档案；阶段收尾时（收尾 SLA：24h）完成记录追加 GROWTH.md 并从 PLAN 移除；GROWTH ≤3 条超则归档；PLAN ≤150 行超则触发历史迁移。详见 `docs/DNA.md`「文档生态 SOP」。
