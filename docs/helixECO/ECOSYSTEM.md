# Helix 生态导航（ECOSYSTEM.md）

> **版本**：v1.1
> **创建日期**：2026-08-30
> **最后更新**：2026-08-30（进度对齐）
> **性质**：Helix 生态唯一真相源（Single Source of Truth, SSOT）
> **维护者**：Jasonmilk / CommonIntents
> **所属方法论**：phyt-DNA v1.0

---

## 0. 工作区目录结构

```
~/Doubao/chats/Jasonmilk/           ← 固定工作区根目录（不按日期分）
├── BIND-19/                         ← 协议传输层（CI-144 家族核心实现）
├── Cellrix/                          ← 展示层（空间语义终端 UI）
├── Tuck/                             ← 安全闸门（免疫系统）
├── anaphase-helix/                   ← 编排中枢（执行体）
├── helix-mind/                       ← 记忆中枢（潜意识核心）
│   └── docs/helixECO/                ← 本导航文档所在地
├── helix-tentacle/                   ← 工具执行（手）
├── phyt-DNA/                         ← 方法论体系（自生长方法论）
└── commonintents/                    ← CommonIntents 组织仓库集合
    ├── BIND-19/                      ← 协议规范（权威来源）
    ├── CAPABILITY-13/                ← 能力授权协议
    ├── INTENT-7/                     ← 意图语义协议
    ├── INTENT-7-SECURE/              ← 安全加密协议
    ├── PFP-xCF14/                    ← 物理特征协议规范（发布窗口）
    └── SAP-xCF14/                    ← 安全证明协议规范（发布窗口）
```

---

## 1. 项目状态总览

| # | 项目 | 分支 | 测试数 | 当前阶段 | 最后提交 | 状态 | 仓库 |
|---|---|---|---|---|---|---|---|
| 1 | **Cellrix** | rs2 | 307 | P0-P6 全部完成 | 2026-08-30 | ✅ 完成 | [Jasonmilk/Cellrix](https://github.com/Jasonmilk/Cellrix) |
| 2 | **Tuck** | rs | 310 | P0-P7 全部完成 | 2026-08-30 | ✅ 完成 | [Jasonmilk/Tuck](https://github.com/Jasonmilk/Tuck) |
| 3 | **Anaphase** | rs | 50 | P10a-P11b 完成，待裁决下一阶段 | 2026-08-29 | ✅ 待裁决 | [Jasonmilk/Anaphase-Helix](https://github.com/Jasonmilk/Anaphase-Helix) |
| 4 | **BIND-19** | v2.0-alpha | 142 | 核心实现完成（PFP+SAP 解析器） | 2026-08-29 | ✅ 完成 | [CommonIntents/BIND-19](https://github.com/CommonIntents/BIND-19) |
| 5 | **Helix-Mind** | rs-dev | 27（通过）/ 59（定义） | P10 认知工艺与生态深度集成（预览） | 2026-08-30 | 🚧 进行中 | [Jasonmilk/Helix-Mind](https://github.com/Jasonmilk/Helix-Mind) |
| 6 | **Helix-Tentacle** | rs | 127 | P5 性能优化（T1 待启动） | 2026-08-29 | 🚧 进行中 | [Jasonmilk/Helix-Tentacle](https://github.com/Jasonmilk/Helix-Tentacle) |
| 7 | **phyt-DNA** | main | - | 方法论 v1.0 定稿生效 | 2026-08-29 | ✅ 完成 | [Jasonmilk/phyt-DNA](https://github.com/Jasonmilk/phyt-DNA) |

**全生态测试总数**：963（Cellrix 307 + Tuck 310 + Anaphase 50 + BIND-19 142 + Helix-Mind 27 + Helix-Tentacle 127）

> **注**：Helix-Mind 定义了 59 个测试，实际通过 27 个（部分测试被 feature gate 或条件编译控制，需在对应 feature 下运行）

### 项目状态详情

#### ✅ 已完成项目（5个）

| 项目 | 完成内容 | 关键成果 |
|---|---|---|
| **Cellrix** | P0-P6 全部完成 | 307 测试，Helix 四大组件全部接入，生产就绪（配置/日志/监控/健康检查） |
| **Tuck** | P1-P7 全部完成 | 310 测试，PFP 第一个消费者，亚微秒级决策，fail-closed，全息审计 |
| **Anaphase** | P11b 验证闭环完成 | 50 测试，认知工艺双向复用轨道裁决完成，编排链路已通 |
| **BIND-19** | v2.0-alpha 核心实现 | 140 测试，33 组测试向量，14 个基准测试，PFP+SAP 解析器 |
| **phyt-DNA** | 方法论 v1.0 立项 | DNA/RNA/PLAN/GROWTH/ADR 闭环，项目自生长方法论锚点 |

#### 🚧 进行中项目（2个）

| 项目 | 当前阶段 | 待办内容 | 阻塞项 |
|---|---|---|---|
| **Helix-Mind** | P10 认知工艺与生态深度集成（预览） | P10a: Anaphase 触发链路 + P10b: L1 策略持久化 + P10c: Deep Dream 复盘挂载 | P0-P9 已完成，PLAN.md v6.0 已修正状态，P10 待详细规划后开工 |
| **Helix-Tentacle** | P5 性能优化（T1 待启动） | T1 性能基准测试 + T2 资源限制 + T3 可观测性 + T4 部署文档 + T5 STDIO 传输层 | CI-144 v2.0 已冻结，Tuck 重构已完成，可启动 T1 |

---

## 2. CI-144 协议家族状态

| 协议 | 状态 | 说明 |
|---|---|---|
| **PFP-xCF14** | ✅ 冻结 | 4 字节固定偏移物理特征头，魔数 0xCF14 |
| **SAP-xCF14** | ✅ v1 完成 | 28 字节安全证明层，防重放 + 双层签名 |
| **BIND-19 v2.0** | ✅ alpha 完成 | 传输层集成 PFP+SAP，140 测试，33 组测试向量 |
| **INTENT-7** | ✅ 稳定 | 语义意图协议，7 核心字段 |
| **CAPABILITY-13** | ✅ 稳定 | 能力授权协议 |
| **INTENT-7-SECURE** | ✅ 稳定 | 安全加密协议 |

**规范权威来源**：`commonintents/BIND-19/docs/spec/`
**规范发布窗口**：`commonintents/PFP-xCF14/`、`commonintents/SAP-xCF14/`

---

## 3. 当前优先级（2026-08-30）

> **前置条件已满足**：CI-144 v2.0 协议家族已冻结（PFP-xCF14 + SAP-xCF14 + BIND-19 v2.0-alpha），Tuck 重构已完成，所有进行中项目的前置依赖均已清除。

### 第一优先级（立即启动）
1. **Helix-Mind P10 开工** — 认知工艺与生态深度集成（P0-P9 已完成）
   - P10a: Anaphase 触发链路（认知工艺→Anaphase 编排）
   - P10b: L1 策略持久化（认知工艺决策固化为可复用策略）
   - P10c: Deep Dream 复盘挂载（睡眠复盘机制深度集成）
   - **⚠️ 注意**：PLAN.md 顶部状态未更新（停留在 P3），需先修正 PLAN.md 状态不一致问题

2. **MCP-Learner Glove PoC** — 证明 Helix 可以高效接入现有互联网
   - 选 mcp-server-filesystem 做验证
   - 学习 MCP 工具清单，提炼为 CI-144 语义事件
   - 效率对比：MCP 直接调用 vs CI-144 重封装

### 第二优先级（可并行启动）
3. **Tentacle P5 性能优化** — 工具执行性能基准 + 资源限制
   - T1: 性能基准测试（ARM 端侧 100 并发，HTTP/gRPC/MCP 三传输层对比）
   - T2: 资源限制（内存/CPU/文件描述符配额）
   - T3: 可观测性（Prometheus metrics + OpenTelemetry tracing）
   - **前置依赖已满足**：CI-144 v2.0 已冻结，Tuck 重构已完成

4. **Anaphase 下一阶段裁决** — 候选 A（Tentacle 深度集成）或候选 B（生态手套协议渐进）
   - 候选 A: Tentacle Rust 重构深度集成（凭证标签流转 + 布隆过滤器 + 异步协程沙箱）
   - 候选 B: 生态手套协议渐进（Cellrix 原生手套协议接入）

### 第三优先级（核心闭环后）
5. **HelixECO-Glove-macOS** — 本地系统适配（macOS 生态手套）
6. **Cellrix 物理沙盒 PoC** — 验证 CPPC v1.1.0 愿景可行性（类 Unity 物理引擎）

---

## 4. Helix 生态架构图

```
                    ┌─────────────────────────────────────────┐
                    │           Cellrix (展示层)               │
                    │    空间语义终端 UI / 物理沙盒            │
                    └──────────────────┬──────────────────────┘
                                       │ CAP 协议 (Mutations/Actions)
                    ┌──────────────────▼──────────────────────┐
                    │        Anaphase (编排中枢 / 执行体)       │
                    │  Think-Act-Observe / Q0-Q3 任务队列     │
                    └──────┬───────────────────┬───────────────┘
                           │ gRPC (UDS)         │ 工具调用
              ┌────────────▼─────────┐  ┌──────▼──────────────┐
              │   Helix-Mind (记忆)   │  │  Tentacle (工具执行) │
              │  潜意识核心 / 认知工艺 │  │  插件系统 / 调用链    │
              └────────────┬─────────┘  └──────┬──────────────┘
                           │                      │
                    ┌──────▼──────────────────────▼───────┐
                    │         Tuck (安全闸门 / 免疫系统)     │
                    │   PFP 4字节决策 / fail-closed / 审计  │
                    └──────────────────┬─────────────────────┘
                                       │
                    ┌──────────────────▼─────────────────────┐
                    │      BIND-19 / CI-144 协议家族         │
                    │  PFP-xCF14 + SAP-xCF14 + INTENT-7     │
                    └────────────────────────────────────────┘
```

---

## 5. 快速入口（各项目文档索引）

### Cellrix
- [PLAN.md](../../Cellrix/docs/PLAN.md) — 开发导航牌
- [GROWTH.md](../../Cellrix/docs/GROWTH.md) — 生长记录
- [ADR 目录](../../Cellrix/docs/adr/) — 架构决策记录

### Tuck
- [PLAN.md](../../Tuck/docs/PLAN.md) — 开发导航牌
- [GROWTH.md](../../Tuck/docs/GROWTH.md) — 生长记录
- [README.md](../../Tuck/README.md) — 项目说明

### Anaphase
- [PLAN.md](../../anaphase-helix/docs/PLAN.md) — 开发导航牌
- [GROWTH.md](../../anaphase-helix/docs/GROWTH.md) — 生长记录

### Helix-Mind
- [PLAN.md](../PLAN.md) — 开发导航牌
- [GROWTH.md](../GROWTH.md) — 生长记录
- [DNA.md](../DNA.md) — 宪法（7 公理）
- [RNA.md](../RNA.md) — 加载协议

### Helix-Tentacle
- [PLAN.md](../../helix-tentacle/docs/PLAN.md) — 开发导航牌
- [GROWTH.md](../../helix-tentacle/docs/GROWTH.md) — 生长记录

### BIND-19
- [README.md](../../BIND-19/README.md) — 项目说明
- [规范目录](../../BIND-19/docs/spec/) — PFP/SAP 规范权威来源

### phyt-DNA
- [README.md](../../phyt-DNA/README.md) — 方法论说明
- [DNA.md](../../phyt-DNA/docs/DNA.md) — 方法论宪法

---

## 6. 新对话启动 SOP（标准操作流程）

每次新对话开始时，按以下流程恢复上下文：

1. **读取本文件** — 了解生态全局状态和当前优先级
2. **定位目标项目** — 进入对应项目目录
3. **读取项目 PLAN.md** — 了解当前阶段和待办
4. **读取项目 GROWTH.md** — 了解最近 3 次健康快照
5. **检查 git 状态** — 确认分支和未提交变更
6. **开始工作** — 在固定目录下操作，不新建日期目录

---

## 7. 方法论闭环检查清单

每次阶段完成后，必须完成以下闭环动作：

- [ ] **PLAN.md 更新** — 当前阶段标记完成，切换到下一阶段
- [ ] **GROWTH.md 追加** — 记录本次健康快照（超过 3 条时归档最旧的）
- [ ] **ADR 创建** — 重要架构决策记录到 `docs/adr/ADR-XXXX-*.md`
- [ ] **README 更新** — 项目状态、测试数、完成阶段同步更新
- [ ] **本文件更新** — 生态全局状态同步更新
- [ ] **提交信息关联 ADR** — Commit Message 标注关联的 ADR 编号

---

## 8. 变更日志

| 版本 | 日期 | 变更内容 |
|---|---|---|
| v1.3 | 2026-08-30 | 全项目进度对齐 — 逐个检查7个项目commit历史和实际测试数，修正Helix-Tentacle测试数(76+→127)、BIND-19测试数(140→142)、全生态测试总数(910+→963)，修正Anaphase阶段描述(P11b→P10a-P11b)，Helix-Mind PLAN.md v6.0状态修正完成 |
| v1.2 | 2026-08-30 | Helix-Mind 进度对齐 — 修正 helix-mind 实际进度（P0-P9 已完成，当前 P10），测试数修正为 27 通过/59 定义，当前优先级更新，发现 PLAN.md 顶部状态与阶段总览不一致问题 |
| v1.1 | 2026-08-30 | 进度对齐 — 项目状态总览添加最后提交日期列 + 已完成/进行中项目详情拆分 + 当前优先级更新（CI-144已冻结，Tentacle P5可并行启动）+ Helix-Mind P3状态细化（计划已起草，待审查） |
| v1.0 | 2026-08-30 | 初始版本 — 工作区迁移完成，生态导航文档创建 |

---

**文档结束**

> 本文件是 Helix 生态的唯一真相源（SSOT）。任何项目状态变更必须同步更新本文件。
> 维护原则：按需更新，保持准确，不冗余。
