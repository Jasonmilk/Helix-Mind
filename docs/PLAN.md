# Helix-Mind 开发导航牌（PLAN）

> **版本**：v6.0（P0-P9 完成，P10 预览，2026-08-30）
> **状态**：🚧 P10 认知工艺与生态深度集成（预览）+ P0-P9 全部完成
> **分支**：rs-dev
> **所属方法论**：DNA 自生长方法论 v2.0（PLAN 动态流转闭环）
> **规则**：本文件只含当前阶段 + 下一阶段预览 + 阶段总览地图。完成阶段 → GROWTH.md。总行数 ≤150，超出触发历史迁移。
> **⚠️ 状态修正记录**：v5.2 顶部状态停留在 P3（计划待起草），但阶段总览已显示 P0-P9 全部完成（2026-08-28）。v6.0 修正顶部状态与阶段总览一致。

---

## 1. 当前阶段：P10 认知工艺与生态深度集成（预览）

> **状态**：🚧 预览阶段（P0-P9 已完成，P10 待启动）。
> **前置依赖**：P9 认知工艺 Phase 3 已完成（价值评估 + 自适应突变 + 睡眠复盘 + bm25 门控增强）。

### 1.1 P10 目标（预览，待详细规划）

| 任务 | 内容 | 代码现状 | 状态 |
|---|---|---|---|
| P10a | Anaphase 触发链路：认知工艺→Anaphase 编排闭环 | 认知工艺已独立运行，未与 Anaphase 触发链路集成 | ⏳ 待规划 |
| P10b | L1 策略持久化：认知工艺决策固化为可复用策略 | 认知工艺决策为一次性，未持久化为可复用策略 | ⏳ 待规划 |
| P10c | Deep Dream 复盘挂载：睡眠复盘机制深度集成 | P9 已实现睡眠复盘基础，需深度挂载到认知工艺循环 | ⏳ 待规划 |

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
- ⚠️ tonic `serve_with_incoming` + `UnixListener` peer_cred 注入需原型验证（连接级身份 → 鉴权）

### 1.4 入口 ADR（Draft，待审查）
- **ADR-0018**：P3a 联邦确定性审查（出站门控 / 审查标准 / 双盲语义）
- **ADR-0019**：P3b 传输层安全（UDS + SO_PEERCRED + mTLS 预留）
- **ADR-0020**：P3c CI-144 INTENT-7 语义对齐（动词→gRPC 映射 + traceparent）

### 1.5 待用户审查的决策点
| # | 决策点 | 建议 | 状态 |
|---|---|---|---|
| D1 | 联邦出站门控形态 | 保留已确认的"能力未就绪=功能不存在"语义；实现选：Cargo feature（编译期最彻底）/ 配置门控（`federation.enabled` 运行期，轻量可逆）。建议：P3 内审查引擎就绪后，`share_dag` 强制先过审查，审查未过即拒绝写盘——无论门控形态，核心是"审查未过不出站" | 待确认 |
| D2 | SymbolicSolver 复用路径 | 下沉 core（共享基础件，metabolism+federation 共用，复用优先）vs federation 依赖 metabolism（重）。建议：下沉 core | 待确认 |
| D3 | 双盲的确定性语义 | LLM 未就绪：双盲 = 两个独立确定性判定（结构断言 solver + 高风险规则）交叉；LLM 就绪后升级真双盲。ADR-0018 定语义 | 待确认 |
| D4 | UDS/TCP 默认形态 | 本地默认 UDS（SO_PEERCRED 0ms）+ 远程 TCP（mTLS 预留）；`ApiConfig.transport` 枚举 | 待确认 |
| D5 | SO_PEERCRED 白名单 | 白名单 UID 来自配置（`api.trusted_uids`），默认拒绝（fail-closed） | 待确认 |
| D6 | CI-144 对齐产物 | 契约映射表文档（INTENT-7 动词→gRPC）+ proto 透传 traceparent（Append-Only）；对齐 ECOSYSTEM §4.1 | 待确认 |

### 1.6 验收标准
- **P3a**：`share_dag` 默认拒绝出站（未启用审查）；启用后仅通过确定性审查的 public L2 出站；入站沙盒接入确定性审查；测试：高风险节点被拒 / 断言矛盾节点被拒
- **P3b**：UDS 启动 + SO_PEERCRED 白名单拒绝/允许测试；Health 可用；middleware 有实际校验
- **P3c**：INTENT-7 映射表交付；traceparent 透传字段（Append-Only）
- `cargo test --workspace` 全绿 + 0 warning

### 1.7 下一阶段预览：P4 硬冻结兑现 + 生态接口
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
| P3 | 安全与契约（联邦审查、UDS SO_PEERCRED / 远程 mTLS、API/Health） | ✅ 2026-08-28 |
| P4 | 硬冻结兑现 + 生态接口（activation_vector、Mind→Callosum 契约、Rhizax 预留） | ✅ 2026-08-28 |
| P4.5 | 架构审查点（ADR-0015 WAL 设计 + 原型，4 项产出） | ✅ 2026-08-28 |
| P5 | 领域 WAL（独立日志 + 完整性校验 + 投影器 + replay） | ✅ 2026-08-28 |
| P6 | 数据诚实 + 轮回 + 商业化（parquet 名实相符、多租户 WAL 分区） | ✅ 2026-08-28 |
| P7 | 生态文档同步（ECOSYSTEM v1.6 对齐 + CI-144 核对 + 认知工艺 Phase 1） | ✅ 2026-08-28 |
| P8 | 认知工艺 Phase 2（编排器最小原型，DeterministicAdapter 闭环，ADR-0021） | ✅ 2026-08-28 |
| P9 | 认知工艺 Phase 3（价值评估、自适应突变、睡眠复盘、bm25 门控增强，ADR-0021） | ✅ 2026-08-28 |
| P10 | 认知工艺与生态深度集成（Anaphase 触发链路、L1 策略持久化、Deep Dream 复盘挂载） | ⏳ 预览 |

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
