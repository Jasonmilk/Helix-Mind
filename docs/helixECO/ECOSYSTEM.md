# Helix 生态导航（ECOSYSTEM.md）

> **版本**：v1.14
> **创建日期**：2026-08-30
> **最后更新**：2026-09-06（候选 G 完成 + G-4..G-7 + **G2 Web 面板首拉**——一条命令选择题闭环 + 浏览器白盒窗口，全生态 1255）
> **性质**：Helix 生态唯一真相源（Single Source of Truth, SSOT）
> **维护者**：Jasonmilk / CommonIntents
> **所属方法论**：phyt-DNA v1.0

---

## 0. 工作区目录结构

```
~/Doubao/chats/Jasonmilk/           ← 固定工作区根目录（不按日期分）
├── BIND-19/                         ← 协议传输层（CI-144 家族核心实现；默认分支 v2.0-alpha，main=规范正文）
├── Cellrix/                          ← 展示层（空间语义终端 UI）
├── Tuck/                             ← 安全闸门（免疫系统）
├── anaphase-helix/                   ← 编排中枢（执行体）
├── helix-mind/                       ← 记忆中枢（潜意识核心）
│   └── docs/helixECO/                ← 本导航文档所在地
├── helix-tentacle/                   ← 工具执行（手）
├── HelixECO-Glove/                   ← 生态手套（原生系统适配层）
├── Helix-MCP-Learner/                ← MCP 消化器（MCP Server → Tentacle 插件）
├── lodestone-spec/                   ← 知识表示协议（v2.0-draft 磁石 DAG；v1.3 冻结）
├── lodestone-md/                     ← 协议参考实现（crate mddag，v2 线，零依赖）
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
| 1 | **Cellrix** | rs2 | 319 | P0-P6 + 驾驶舱（G-2..G-6）+ **Web 面板 G2 首拉**（cellrix-web，ADR-0014，浏览器白盒窗口）；下一步 Web 优化（React 组件接入/up 菜单第 5 项） | 2026-09-06 | ✅ 完成 + 🔄 Web 优化待启 | [Jasonmilk/Cellrix](https://github.com/Jasonmilk/Cellrix) |
| 2 | **Tuck** | rs | 316 | P0-P7 全部完成；P6-T5 Cellrix 状态流（StatusProvider，ADR-0003）已落地 | 2026-09-05 | ✅ 完成 | [Jasonmilk/Tuck](https://github.com/Jasonmilk/Tuck) |
| 3 | **Anaphase** | rs | 140 | 候选 E + D' + **候选 G 完成**（驾驶舱 + bootstrap + 易用引导 + 交互菜单 + **配置向导**，ADR-0010..0015）；**编排哲学显式化 ADR-0016**（确定性优先分诊 + 认知工艺四拍挂点 + 按需感知 + 依赖边界）；下一步候选 O 系列（O-1..O-4） | 2026-09-06 | ✅ 完成 + 🔄 O 系列待启 | [Jasonmilk/Anaphase-Helix](https://github.com/Jasonmilk/Anaphase-Helix) |
| 4 | **BIND-19** | v2.0-alpha（默认） | 142 | 核心实现完成（PFP+SAP 解析器）；默认分支已切 v2.0-alpha，main=规范正文（tag v1.0.0-RFC-4） | 2026-09-06 | ✅ 完成 | [CommonIntents/BIND-19](https://github.com/CommonIntents/BIND-19) |
| 5 | **Helix-Mind** | rs-dev | 98 | P0-P9 全部完成，P10 准备完成（认知工艺与生态深度集成） | 2026-08-31 | 🚧 P10 待启动 | [Jasonmilk/Helix-Mind](https://github.com/Jasonmilk/Helix-Mind) |
| 6 | **Helix-Tentacle** | rs | 153 | P6 生态联调进行中（M1.5 grpc transport + fixture 插件完成，d902151）；T4 部署文档 + CI-144 全组件联调待做 | 2026-09-06 | 🚧 进行中 | [Jasonmilk/Helix-Tentacle](https://github.com/Jasonmilk/Helix-Tentacle) |
| 7 | **HelixECO-Glove** | main | 45 | P4-T1 完成（L1 静态审查 9 条规则），P4-T2 预览 | 2026-09-06 | 🚧 进行中 | [Jasonmilk/HelixECO-Glove](https://github.com/Jasonmilk/HelixECO-Glove) |
| 8 | **Helix-MCP-Learner** | main | 42 | P2/P3/P4-T1 完成（生态联调全链路 + post_learn 审查管道）；1 失败测试未修（非阻塞） | 2026-09-06 | 🚧 进行中 | [Jasonmilk/Helix-MCP-Learner](https://github.com/Jasonmilk/Helix-MCP-Learner) |
| 9 | **phyt-DNA** | main | - | 方法论 v1.0 定稿生效 | 2026-08-29 | ✅ 完成 | [Jasonmilk/phyt-DNA](https://github.com/Jasonmilk/phyt-DNA) |

**全生态测试总数**：**1255**（Cellrix 319 + Tuck 316 + Anaphase 140 + BIND-19 142 + Helix-Mind 98 + Helix-Tentacle 153 + HelixECO-Glove 45 + Helix-MCP-Learner 43）（Cellrix 316 + Tuck 316 + Anaphase 132 + BIND-19 142 + Helix-Mind 98 + Helix-Tentacle 153 + HelixECO-Glove 45 + Helix-MCP-Learner 43）（Cellrix 316 + Tuck 316 + Anaphase 129 + BIND-19 142 + Helix-Mind 98 + Helix-Tentacle 153 + HelixECO-Glove 45 + Helix-MCP-Learner 43）（Cellrix 316 + Tuck 316 + Anaphase 126 + BIND-19 142 + Helix-Mind 98 + Helix-Tentacle 153 + HelixECO-Glove 45 + Helix-MCP-Learner 43）

> **注**：Helix-Mind P0-P9 全部完成，P10 准备工作已完成（现状探查 + 执行计划制定），待正式启动。Helix-Tentacle 与 Helix-MCP-Learner 生态联调成功，全链路畅通：MCP-Learner 学习 → L1 静态审查 → stable/ → Tentacle 加载 → 执行工具。HelixECO-Glove P4-T1 完成（L1 静态审查 9 条规则），P4-T2（L2 dry_run）预览中。Helix-MCP-Learner P2/P3/P4-T1 完成（生态联调全链路 + post_learn 审查管道），有 1 个测试失败（非阻塞，待修复）。

### 项目状态详情

#### ✅ 已完成项目（6个）

| 项目 | 完成内容 | 关键成果 |
|---|---|---|
| **Cellrix** | P0-P6 全部完成 + 候选 G 驾驶舱 | 316 测试，Helix 四大组件全部接入，生产就绪；候选 G：AnaphaseClient get_snapshot（一次拉全）+ CockpitWidget（模式栏/经历时间线/Ledger 审查视图）+ renderer strip + attach_cockpit 轮询 + cli --anaphase-endpoint（ADR-0009），双端协议（TUI 先行，Web=G2） |
| **Tuck** | P1-P7 全部完成 | 316 测试，PFP 第一个消费者，亚微秒级决策，fail-closed，全息审计，四层管控接口（Mind/Anaphase/Tentacle bridge + Cellrix StatusProvider，ADR-0003） |
| **Anaphase** | M1 + M1.5 + 候选 E + F + D' 4/4 + G + 编排哲学 ADR-0016 | 140 测试，六 stage 确定性流水线（MET/UNMET/replay 字节级一致），真实 Tentacle gRPC 连通（fixture 插件全链路），Reasoning 结构化输出协议，run_cycle ↔ pipeline 完整 merge，零硬编码收口（RunCycleConfig），会话即经历（ADR-0006），重放守卫指纹（ADR-0007），SecurityGate 接线点 + ledger blocked（ADR-0008），真实场景插件 D'-4（ADR-0009），AgentSnapshot 共享快照投影端点（ADR-0010） |
| **BIND-19** | v2.0-alpha 核心实现（默认分支） | 142 测试（实测），33 组测试向量，14 个基准测试，PFP+SAP 解析器；main=规范正文（v1.0.0-RFC-4，tag 锚定） |
| **Helix-Tentacle** | P5 完成 + P6 进行中（M1.5） | 153 测试，性能基准+资源限制+可观测性+STDIO/gRPC 传输层，fixture 插件（numbers/rate，SHA-256），MCP-Learner 全链路联调畅通 |
| **phyt-DNA** | 方法论 v1.0 立项 | DNA/RNA/PLAN/GROWTH/ADR 闭环，项目自生长方法论锚点 |

#### 🚧 进行中项目（3个）

| 项目 | 当前阶段 | 待办内容 | 阻塞项 |
|---|---|---|---|
| **Helix-Mind** | P10 准备完成，待启动 | P10a: Anaphase 触发链路 + P10b: L1 策略持久化 + P10c: Deep Dream 复盘挂载 | P0-P9 已完成，P10 现状探查+执行计划已制定，待正式开工 |
| **HelixECO-Glove** | P4-T1 完成，P4-T2 预览 | P4-T2: L2 dry_run 沙箱预执行 + 审查规则自进化 | L1 静态审查 9 条规则已完成（10 测试全绿），macOS Glove 核心已实现 |
| **Helix-MCP-Learner** | 核心完成 + 生态联调成功 | 升级 mcp_proxy.js 为真实 MCP 代理执行体 + 修复 1 个失败测试 | 全链路联调畅通（学习→审查→stable/→Tentacle加载→执行），真实 MCP 执行待升级 |

---

## 2. CI-144 协议家族状态
> **v1.7 诚实修正（物理事实核验，2026-09-05）**：v1.6 误标 INTENT-7 / CAPABILITY-13 / INTENT-7-SECURE 为"✅ 稳定"。实测 spec 原文（各仓库对齐 commit 2026-08-28）均为 `v1.0.0-RFC-4`——spec 自述 "currently in the early draft stage"，未冻结、未启动正式社区治理。**机制已完备、可支撑落地**：扩展保留区（`x-*` 前缀、`custom_scopes`、开放枚举）在 `.github/CONTRIBUTING.md` 明确。

| 协议 | 状态 | spec 版本 | 说明 |
|---|---|---|---|
| **PFP-xCF14** | ✅ 冻结 | v1.0 | 4 字节固定偏移物理特征头，魔数 0xCF14 |
| **SAP-xCF14** | ✅ v1 完成 | v1.0 | 28 字节安全证明层，防重放 + 双层签名 |
| **BIND-19** | ✅ alpha 完成（默认分支） | v2.0-alpha | 传输层集成 PFP+SAP，142 测试，33 组测试向量；规范正文在 main（tag v1.0.0-RFC-4） |
| **INTENT-7** | 🔄 early draft | v1.0.0-RFC-4 | 语义意图协议，7 核心字段 + 最小语法层，不定义行为；动词 FETCH/WRITE_NODE/TENTACLE/FINISH/CANCEL；autonomy_level=AGENT/OPEN/SURVIVAL；HXR↔L3 对齐 |
| **CAPABILITY-13** | 🔄 early draft | v1.0.0-RFC-4 | 能力授权协议；HITL 挑战-响应队列；dynamic capability_mapping.toml（standard_scopes + custom_scopes + Ed25519） |
| **INTENT-7-SECURE** | 🔄 early draft | v1.0.0-RFC-4 | 安全加密协议；UDS SO_PEERCRED（Linux）/ LOCAL_PEERCRED（macOS）物理身份验证；mTLS 1.3 可选 |

**规范权威来源**：`commonintents/{协议名}/spec/`
**规范发布窗口**：`commonintents/PFP-xCF14/`、`commonintents/SAP-xCF14/`

---

## 3. 当前优先级（2026-09-05）

> **前置条件已满足**：CI-144 协议家族机制完备（PFP/SAP 冻结；INTENT-7/CAPABILITY-13/INTENT-7-SECURE 为 v1.0.0-RFC-4 draft，扩展保留区已明确，不阻塞落地），Anaphase 候选 E + 候选 F 完成，生态联调成功（MCP-Learner → Tentacle 全链路畅通），Helix-Mind P10 准备工作完成。

### 第一优先级（立即启动，并行）
1. **Anaphase 候选 D'** — M1.5 深化（seen_entropy_bloom 重放守卫 ✅ / main.rs pipeline 接线 ✅ / **D'-2 Tuck 深度集成 ✅（SecurityGate 接线点，ADR-0008，真实 TuckSecurityGate 连通验证）** / D'-4 真实场景插件待 MCP-Learner）
2. **Anaphase 候选 G（✅ 完成 2026-09-06）** — Anaphase 驾驶舱（正名：监控意识层，Helix-Mind 灵魂本体不驾驶）：G-T2 AgentSnapshot 端点（ADR-0010）+ G-T3..T5 Cellrix 消费/渲染/live（ADR-0009）+ G-T6 文档；**候选 G2（待启动）**：Web 面板（消费同一 snapshot 协议，DSH 式可视化，低摩擦）
3. **Helix-Mind P10 开工** — 认知工艺与生态深度集成（P10a 触发链路 → P10b L1 策略持久化 → P10c Deep Dream 复盘挂载）

### 第二优先级（可并行启动）
4. **真实 MCP 执行升级** — 将 mcp_proxy.js 从占位脚本升级为真实 MCP 代理执行体
5. **HelixECO-Glove P4-T2** — 审查体系 L2（dry_run 沙箱预执行）
6. **Tuck 管控接线** — CAPABILITY-13 层模式权限映射（三模式令牌：驾驶无 memory scopes / 伙伴 x-fetch+x-write / 生存 x-enter-dream）

### 第三优先级（核心闭环后）
7. **Anaphase 下一阶段裁决** — 候选 A（Tentacle Rust 重构）
8. **HelixECO-Glove-macOS 完善** — 本地系统适配（macOS 生态手套，更多原生工具）
9. **Cellrix 物理沙盒 PoC** — 验证 CPPC v1.1.0 愿景可行性（类 Unity 物理引擎）

---

## 3.5 生态联调成果（2026-08-31 里程碑）

### 全链路验证通过

```
MCP-Learner 学习 mock MCP Server → 4 个工具
    ↓
L1 静态审查（9 条规则）→ 0 warning, 0 error
    ↓
stable/ 目录（4 个 .manifest.json + mcp_proxy.js）
    ↓
Tentacle 扫描 + SHA-256 完整性校验 → 4 个工具注册
    ↓
ProcessTool 实例化 → 4 个工具可用
    ↓
tentacle-cli 执行 mock-filesystem.list_files → ✅ 成功返回结果
```

### 联调中修复的 6 个问题

| # | 问题 | 修复 | 仓库 |
|---|---|---|---|
| 1 | MCP 工具名不符合点分命名空间规范 | 新增 `extract_tools_with_namespace`，格式 `<server>.<name>` | MCP-Learner |
| 2 | Manifest 文件后缀不匹配（.json vs .manifest.json） | post_learn 输出改为 `.manifest.json` 后缀 | MCP-Learner |
| 3 | MCP 代理执行体 mcp_proxy.js 不存在 | post_learn 自动在所有状态目录创建占位执行体 | MCP-Learner |
| 4 | 完整性哈希不匹配（全零占位 vs 真实 SHA-256） | 计算 mcp_proxy.js 真实哈希并更新所有 manifest | MCP-Learner |
| 5 | tentacle-benchmarks 编译错误（缺少 platform_support） | 添加 `platform_support: Default::default()` | Tentacle |
| 6 | 插件懒加载未实现（只注册 Manifest，没实例化 Tool） | 添加 ProcessTool 临时实现，扫描后自动实例化 | Tentacle |

### 提交记录

- **MCP-Learner**: `d21b897`（代码修复）+ `500a461`（文档记录）
- **Tentacle**: `79270c5`（插件加载与执行链路修复）

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

┌─────────────────────────────────────────────────────────────────┐
│                     生态适配层（外部世界接入）                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────┐        ┌──────────────────────────┐   │
│  │  HelixECO-Glove       │        │  Helix-MCP-Learner       │   │
│  │  原生生态手套          │        │  MCP 消化器               │   │
│  │  (macOS/Linux/鸿蒙)   │        │  (MCP Server → CI-144)   │   │
│  │  手写原生适配          │        │  自动学习+提炼+重封装      │   │
│  └──────────┬───────────┘        └───────────┬──────────────┘   │
│             │ 静态链接（极致节能）              │ 动态加载（热插拔）  │
│             └──────────────┬───────────────────┘                  │
│                            ▼                                        │
│                   Tentacle 插件体系（统一执行层）                    │
└─────────────────────────────────────────────────────────────────┘
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

### HelixECO-Glove
- [README.md](../../HelixECO-Glove/README.md) — 项目说明
- [PLAN.md](../../HelixECO-Glove/docs/PLAN.md) — 开发导航牌
- [GROWTH.md](../../HelixECO-Glove/docs/GROWTH.md) — 生长记录
- [ADR 目录](../../HelixECO-Glove/docs/decisions/) — 架构决策记录

### Helix-MCP-Learner
- [README.md](../../Helix-MCP-Learner/README.md) — 项目说明
- [PLAN.md](../../Helix-MCP-Learner/docs/PLAN.md) — 开发导航牌
- [GROWTH.md](../../Helix-MCP-Learner/docs/GROWTH.md) — 生长记录
- [ADR 目录](../../Helix-MCP-Learner/docs/decisions/) — 架构决策记录

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
| **v1.27** | **2026-09-06** | **Anaphase 编排哲学显式化（ADR-0016）+ lodestone lode 术语定稿 + lodestone-md CI 修复**。①Anaphase ADR-0016（Proposed）：确定性优先分诊（六 stage 仅"理解自由文本/生成表达"两处必须 LLM，其余 0 tokens 通道）+ 认知工艺触发点（四拍/五工序含批判性全归 Mind，Anaphase 只触发 helixQuery，Anaphase 零工序实现）+ 按需感知 = 设置 budget_tier（ADR-0010，任务前/升级 LLM 前各一次，看口袋过日子）+ 依赖边界（并行池/窗口感知→FlowModus，前缀稳定→Callosum）+ 轨迹三层（ledger+evidence+会话 DAG+stage 事件）；VISION 补编排哲学指针（不冻结），PLAN 增候选 O 系列（O-1..O-4 + 两条等待项）；②lodestone-md/spec 内部术语 ball→node→**lode**（lodestone 词根/矿脉意象，零歧义，mddag 38 tests 全绿）；③lodestone-md CI 修复（clippy -D warnings 12 项全清，CI 全绿）；④全生态测试总数 1255 不变（纯文档轮）|
| **v1.26** | **2026-09-06** | **Lodestone v2 术语改名（ball→node）** — crate 即 mddag（markdown DAG），内部实现名回归 DAG 标准语：磁石 = node、CLI `nodes`/`node`、操作符 `add-node`；"磁石/磁力线"保留为中文概念名（ADR-0002 术语修订）；38 测试全绿（mddag 3a05088 / spec 75f58f1） |
| **v1.25** | **2026-09-06** | **Lodestone v2 跨文档库层（ADR-0003）** — 同一协议两层作用域：单文档 = 库大小 1 特例，`#slug` = `path#slug` 的 path 空特例；文档层形状检查（W-CROSS-DOC）+ 库层目标解析（E-CROSS-MISSING / E-CROSS-SLUG / E-CYCLE-CROSS）；`.lodestone` 确定性快照（mddag 语料自举、无时间戳、git 可提交）+ `--check` 过期检测；入边派生不落盘（双射，极致节能）；源码关系标注分层投影（cargo metadata / rustdoc JSON / Aider repo-map 同构）；mddag 38 测试（f609f27），spec ADR-0003 + §3.7/§5.2c + fixture 08（098cfed） |
| **v1.24** | **2026-09-06** | **Lodestone v2 窗口机制（显性/隐性分支）** — 第七追加操作 strip（剔边）+ library 投影（按 created 排序，最近 keep 完整 L0、更旧折叠）；会话元数据 §3.6（session/created 由消费方写入）；keep 注入参数示例 12 零硬编码；断链防护 keep 列表；mddag 33 测试（d6490c5），spec §3.6/§5.1/§5.2b + fixture 07（c7f8bff）；前沿锚定：ACT-R 幂律遗忘 / MemGPT 窗口分层 / 检索重巩固（reconsolidation）——Helix 显性有迹可循、隐性按需回忆 |
| **v1.23** | **2026-09-06** | **Lodestone v2 decay（遗忘半环）** — 第六追加操作 decay + DecayPolicy（root_ttl/near_ttl/other_ttl 注入式配置，21/14/7 天为示例值零硬编码）；mddag 30 测试（af89719→fa33506），spec §5.1/§5.2a + fixture 06 前后对照（035e0ab→b6fde3a）；收敛=compress 已有，遗忘=decay 补齐——"忘了对话，记得教训"落盘 |
| **v1.22** | **2026-09-06** | **Lodestone v2.0-draft 实现完成** — lodestone-md 换血 v2（markdown 原生 DAG：磁石/磁力线/状态列表/沉淀区，零依赖 25 测试，CLI balls/ball/body/sediment/check）；lodestone-spec 术语裁定 磁石（lodestone）+ fixtures/v2 纯 markdown 语料 + ADR-0002 Accepted；v1.3.0 冻结于 git tag 不回写；lodestone 为 M2 消费点（会话即经历的协议载体） |
| **v1.21** | **2026-09-06** | **G-7 配置向导（LLM 引导输入）** — Anaphase up 菜单选项 4（ADR-0015）：base_url/model/api_key 一问一答（Enter 保持现值）；api_key 不回显（stty -echo，pty 实测无泄漏）；写盘前备份 config.toml.bak，行级替换其余字节保留；Anaphase 135→140，全生态 1251→1255 |
| **v1.20** | **2026-09-06** | **G2 Web 面板首拉（浏览器白盒窗口）** — Cellrix 新 crate `cellrix-web`（ADR-0014）：零依赖 std-only HTTP + 单文件内嵌 HTML + 原生 JS 轮询（2s）；同源代理 /api/snapshot → Anaphase /v1/agent/snapshot（规避 CORS，共享 ADR-0010 契约）；路由白名单 + 真实状态码；实测全链路 mock reasoning → 真实 Tentacle numbers → 真实 MET ledger（run-8bba24c5ee368a4a#0）经代理可见；Cellrix 316→319，全生态 1248→1251；下一步 Web 优化（React 组件接入 / up 菜单第 5 项 / SaaS 种子） |
| **v1.19** | **2026-09-06** | **G-6 交互菜单（一条命令之后只有选择题）** — Anaphase `up`（ADR-0013）启动后端后进入交互菜单（tty）：1 打开驾驶舱（Enter 默认）/ 2 查看状态（物理探测 + 真实 snapshot 摘要，手写 HTTP GET 无新依赖）/ 3 配置说明 / 4 停止退出（q）；非 tty 自动降级挂起（is_terminal 物理判断）；parse_choice 纯函数 3 单测（未知输入重提示）；Anaphase 132→135，全生态 1245→1248；下一步 G2 Web 面板（SaaS 种子） |
| **v1.18** | **2026-09-06** | **G-5 易用引导 UX（首跑零困惑）** — Anaphase `up` 升级为引导四段式（ADR-0012）：欢迎 banner / 前置检查（缺失项带 `cargo build` 提示，Anaphase 缺失=致命，Tentacle/Cellrix=fail-open）/ 启动（中文模式标签）/ 下一步（驾驶舱命令 + 一键重来）；Noop 引导（reasoning 未配置 → 明确提示 + 配置方式）；输出中文（用户母语）代码注释英文；Anaphase 129→132，全生态 1242→1245；下一步 G2 Web 面板（SaaS 种子）或候选裁决 |
| **v1.17** | **2026-09-06** | **G-4 bootstrap（一条命令起全栈）** — Anaphase 新增 `up` bin（ADR-0011）：tentacle（grpc + fixtures）→ anaphase（`ANAPHASE_TENTACLE_ENDPOINT` env 注入，config.toml 零改动）→ 物理探测（TCP 就绪）→ 可选 `--cockpit` 拉驾驶舱 TUI；config.rs `apply_env_overrides`（12-factor env 优先 + fail-open）+ 3 单测；实测双就绪 + 退出端口全清；Anaphase 126→129，全生态 1239→1242；下一步 G-5 易用引导 UX（首跑向导，降低"一大堆 CLI"门槛） |
| **v1.16** | **2026-09-06** | **G-3 transport 契约修复（驾驶舱可坐进去）** — ①物理验证发现：Cellrix stdio/UDS transport 从未与 mock-agent 真实联调（stdio 读 Manifest 超时 = mock-agent BE vs transport stdio LE；UDS decode 失败 = 首帧 AgentEvent 包装 vs 裸 CapabilityManifest）；②修复（ADR-0010）：mock-agent 参数化 Endian（stdio=LE / uds=BE）+ map-form rmp（decode 对称）+ UDS 裸 Manifest；③**驾驶舱 TUI 双通道实测渲染**：`[PARTNER] state=Perception` + `MET run-8bba24c5ee368a4a (trace=...)` 真实 ledger 白盒投影；④Cellrix 316 全绿无回归；⑤下一步候选：**引导（bootstrap launcher，一键起全栈）** 解决"一大堆 CLI"易用性问题 + G2 Web 面板 |
| **v1.15** | **2026-09-06** | **候选 G Anaphase 驾驶舱完成** — ①Cellrix 307→316：AnaphaseClient get_snapshot（一次拉全，极致节能）+ HttpAnaphaseClient（consumes /v1/agent/snapshot）+ CockpitWidget（模式栏/经历时间线/Ledger 审查视图，白盒投影）+ AppState.cockpit + renderer strip + attach_cockpit 轮询 + cli --anaphase-endpoint，ADR-0009；双端策略：snapshot 协议 TUI/Web 共享，TUI 先行，Web 面板=G2；②Anaphase 124→126：AgentSnapshot 共享快照投影端点（AgentLoop::capture + Arc<Mutex> 共享槽，HTTP 层不触碰 agent 内部，消除 token_consumed:1234 硬编码），ADR-0010；③live 联调：真实 Anaphase cap_http 50061 ↔ HttpAnaphaseClient roundtrip 解析成功（anaphase_live.rs #[ignore]），serde 契约修正（mode snake_case）；④全生态测试总数 1228→**1239**；⑤修复 §1 Anaphase 行滞后（D' 实际 4/4） |
| **v1.14** | **2026-09-06** | **Anaphase 候选 D' 4/4 完成 + MCP-Learner 全绿** — ①MCP-Learner 失败测试修复（过时断言，产物后缀应为 `.manifest.json` 生态契约），42+1f → 43；②D'-4 真实场景插件：`Expect::Ok` 结构判据（零阈值，字段来源=执行体契约）+ tests/m1_5_d4_live.rs 3 例实测全绿（真实 Tentacle + MCP-Learner 学习产物，插件 MET / 未知工具 transport Err / run_cycle 全链路 MET），ADR-0009；③Anaphase 121→124，候选 D' 四项全部落地；④全生态测试总数 1224→**1228** |
| **v1.13** | **2026-09-06** | **BIND-19 默认分支切换** — 默认分支 main → v2.0-alpha（Rust 参考实现，142 tests）；main 保持协议规范正文身份并打 tag `v1.0.0-RFC-4` 锚定（spec-only，不覆盖不合并）；v2.0-alpha README 标注仓库双身份（4321c09） |
| **v1.12** | **2026-09-06** | **全生态测试数物理核验 + Tuck test-utils 复用** — ①实测校准 HelixECO-Glove 测试数 23→45（README 44 亦滞后，已修，8901a4c），全生态测试总数 1201→1224；②Tuck 暴露 `test-utils` feature（InMemoryCredentialStore 从 `#[cfg(test)]` 改 `#[cfg(any(test, feature="test-utils"))]`，0001dde），Anaphase tuck_gate 测试改复用 Tuck store（删本地自持实现，极致复用） |
| **v1.11** | **2026-09-06** | **Anaphase 候选 D'-2 完成** — Anaphase 测试数 110→121（SecurityGate 接线点：`src/security.rs` 本地契约零 Tuck 依赖 + pipeline `with_security_gate` + ledger `Blocked` 独立记录类型 + 真实 TuckSecurityGate 连通测试 3 例，ADR-0008），管控闭环咽喉（三闸门之三）落地，D'-2 阻塞解除（Tuck 侧接口早已就绪）；全生态测试总数 1190→1201；D'-4 仍待 MCP-Learner 升级 |
| **v1.10** | **2026-09-06** | **生态文档对齐（三仓库）** — ①Helix-Tentacle P6 状态修正：M1.5 grpc transport + fixture 插件完成（PLAN v4.3 + GROWTH 记录4 + README，dce6c70）；②Helix-MCP-Learner P2/P3/P4-T1 完成：生态联调全链路 + post_learn 审查管道（PLAN v3.2 + GROWTH + README，2606dbb），1 失败测试如实标注；③HelixECO-Glove P4-T1 完成（PLAN v1.4 + GROWTH，0133e14） |
| **v1.9** | **2026-09-05** | **Tuck P0-P7 全部完成** — Tuck 测试数 310→316（P6-T5 Cellrix 状态流：StatusProvider 拉模式查询接口 + DecisionSummary/DecisionEvent 投影，ADR-0003；status 集成测试 6 例），Tuck 四层管控接口面全齐（SAP/Mind/Anaphase/Tentacle bridge + Cellrix 状态流），全生态测试总数 1184→1190；Anaphase 候选 D'-2（Tuck 深度集成）的 Tuck 侧接口已就绪 |
| **v1.8** | **2026-09-05** | **Anaphase 候选 D' 部分完成** — Anaphase 测试数 105→110（D'-1 seen_entropy_bloom 真实确定性指纹 `bl-`+FNV-1a 替换空串占位 + D'-3 `pipeline::resolve_pipeline` fail-open 启动接线，ADR-0007；replay_guard 集成测试 4 例），全生态测试总数 1179→1184，D'-2（Tuck 侧接口）/ D'-4（MCP-Learner 升级）仍阻塞 |
| **v1.7** | **2026-09-05** | **Anaphase 候选 F 完成 + 协议状态诚实修正** — ①Anaphase 测试数 94→105（会话即经历：episode 边界 + 三模式参与度，ADR-0006；episode_lifecycle 集成测试 10 例），全生态测试总数 1168→1179；②§2 诚实修正：INTENT-7/CAPABILITY-13/INTENT-7-SECURE 从"✅ 稳定"改为"🔄 early draft（v1.0.0-RFC-4，spec 自述 early draft 未冻结）"，BIND-19 测试数统一为 142；③§3 优先级重组：候选 D' + 候选 G（Cellrix 经历时间线门面）+ Mind P10 三线并行；④Anaphase 下一阶段候选新增候选 G |
| v1.6 | 2026-09-03 | Anaphase 候选 E 完成 — Anaphase 测试数 78→94（Reasoning 结构化输出协议替换 contains 匹配 + run_cycle ↔ pipeline 六 stage 完整 merge + 零硬编码收口 RunCycleConfig；run_cycle_pipeline 集成测试 8 例 + live 3 条），全生态测试总数 1153→1168，更新 Anaphase 下一阶段候选（候选 D' / 候选 A） |
| v1.4 | 2026-08-31 | 生态联调成功 + P10 准备完成 — 新增 HelixECO-Glove（23测试）和 Helix-MCP-Learner（42测试）两个项目，全生态测试总数 1060→1125；生态联调全链路畅通（MCP-Learner→L1审查→stable/→Tentacle加载→执行），修复6个联调问题；Helix-Mind P10准备工作完成（现状探查+执行计划）；更新生态架构图（新增生态适配层）；更新当前优先级（P10为第一优先级） |
| v1.3 | 2026-08-30 | 全项目进度对齐 — 逐个检查7个项目commit历史和实际测试数，修正Helix-Tentacle测试数(76+→127)、BIND-19测试数(140→142)、全生态测试总数(910+→963)，修正Anaphase阶段描述(P11b→P10a-P11b)，Helix-Mind PLAN.md v6.0状态修正完成 |
| v1.2 | 2026-08-30 | Helix-Mind 进度对齐 — 修正 helix-mind 实际进度（P0-P9 已完成，当前 P10），测试数修正为 27 通过/59 定义，当前优先级更新，发现 PLAN.md 顶部状态与阶段总览不一致问题 |
| v1.1 | 2026-08-30 | 进度对齐 — 项目状态总览添加最后提交日期列 + 已完成/进行中项目详情拆分 + 当前优先级更新（CI-144已冻结，Tentacle P5可并行启动）+ Helix-Mind P3状态细化（计划已起草，待审查） |
| v1.0 | 2026-08-30 | 初始版本 — 工作区迁移完成，生态导航文档创建 |

---

**文档结束**

> 本文件是 Helix 生态的唯一真相源（SSOT）。任何项目状态变更必须同步更新本文件。
> 维护原则：按需更新，保持准确，不冗余。
