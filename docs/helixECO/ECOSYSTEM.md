# Helix 生态导航（ECOSYSTEM.md）

> **版本**：v1.49
> **创建日期**：2026-08-30
> **最后更新**：2026-09-06（P10 认知工艺生态深度集成完成——P10a 触发 + P10b L1 落盘 + P10c Deep Dream，全生态 1404）
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
├── Helix-Callosum/                  ← ⚰️ 已归档（2026-09-06，DEPRECATE.md 见仓库；前缀稳定归 lodestone 投影 + FlowModus canonicalizer）
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
| 1 | **Cellrix** | rs2 | 319 | **CI-144 stdio 闭环完成**（ADR-0017：StdioTransport::send_action + 单 reader 分发，真实 Anaphase 二进制 manifest/snapshot/action 三通道实测通过）| P0-P6 + 驾驶舱（G-2..G-6）+ **Web 面板 G2 首拉**（cellrix-web，ADR-0014，浏览器白盒窗口）；下一步 Web 优化（React 组件接入/up 菜单第 5 项） | 2026-09-06 | ✅ 完成 + 🔄 Web 优化待启 | [Jasonmilk/Cellrix](https://github.com/Jasonmilk/Cellrix) |
| 2 | **Tuck** | rs | 316 | P0-P7 全部完成；P6-T5 Cellrix 状态流（StatusProvider，ADR-0003）已落地 | 2026-09-05 | ✅ 完成 | [Jasonmilk/Tuck](https://github.com/Jasonmilk/Tuck) |
| 3 | **Anaphase** | rs | 198 | P10a 认知工艺触发（ADR-0031：helix_craft 客户端 + MemoryRetrieval 按需触发 + [think-first] 折入）；O-6 判断点后端可配化完成（ADR-0024：JP-1 复杂度评估 Rules 默认 / SmallLlm 3B 可选 + 失败回退 + 零硬编码收口，judge-points contract 入 FlowModus）；O-5（ADR-0023）（ADR-0023：记忆折叠注入 Reasoning——修复检索断裂 + 25 轮近零增长 + 演示输入来源化）；O-4（ADR-0022）+ O-2/O-3 + Rails + 候选 E + O-1 + CI-144 传输层 |
| 4 | **BIND-19** | v2.0-alpha（默认） | 142 | 核心实现完成（PFP+SAP 解析器）；默认分支已切 v2.0-alpha，main=规范正文（tag v1.0.0-RFC-4） | 2026-09-06 | ✅ 完成 | [CommonIntents/BIND-19](https://github.com/CommonIntents/BIND-19) |
| 5 | **Helix-Mind** | rs-dev | 107 | P0-P10 全部完成：P10a helix_craft 触发链路 + P10b synthesis→L1 策略落盘（幂等+价值分级）+ P10c Deep Dream 睡眠复盘（突变适应） | 2026-09-06 | ✅ P10 完成 | [Jasonmilk/Helix-Mind](https://github.com/Jasonmilk/Helix-Mind) |
| 6 | **Helix-Tentacle** | rs | 153 | P6 生态联调进行中（M1.5 grpc transport + fixture 插件完成，d902151）；T4 部署文档 + CI-144 全组件联调待做 | 2026-09-06 | 🚧 进行中 | [Jasonmilk/Helix-Tentacle](https://github.com/Jasonmilk/Helix-Tentacle) |
| 7 | **HelixECO-Glove** | main | 45 | P4-T1 完成（L1 静态审查 9 条规则），P4-T2 预览 | 2026-09-06 | 🚧 进行中 | [Jasonmilk/HelixECO-Glove](https://github.com/Jasonmilk/HelixECO-Glove) |
| 8 | **Helix-MCP-Learner** | main | 42 | P2/P3/P4-T1 完成（生态联调全链路 + post_learn 审查管道）；1 失败测试未修（非阻塞） | 2026-09-06 | 🚧 进行中 | [Jasonmilk/Helix-MCP-Learner](https://github.com/Jasonmilk/Helix-MCP-Learner) |
| 9 | **phyt-DNA** | main | - | 方法论 v1.0 + **保护章节 v1.2**（docs/PROTECTION.md：大厂实践提炼 + 许可策略决策 + 文档语言规范 + 五条保护原则 + 零成本清单 + Prior Art as Code 规范） | 2026-09-06 | ✅ 完成 | [Jasonmilk/phyt-DNA](https://github.com/Jasonmilk/phyt-DNA) |
| 10 | **FlowModus** | rs | **83** | **rs 重构全部完成**（R-1..R-6：五层确定性管线 + 三调用模式 + 控制面 + judge-points 契约 v1.1 Rules 后端，clippy 零警告）；Python v1.7 保留 main 分支 | 2026-09-06 | ✅ rs 收口 | [Jasonmilk/FlowModus](https://github.com/Jasonmilk/FlowModus) |

**全生态测试总数**：**1404**（Cellrix 319 + Tuck 316 + Anaphase 198 + BIND-19 142 + Helix-Mind **107** + Helix-Tentacle 153 + HelixECO-Glove 45 + Helix-MCP-Learner 43 + **FlowModus 83**）（2026-09-06 物理核对重算：历史合计含累计误差 +11，v1.31/32 的 1287 实为 1276；Anaphase 160→169→176 后为 1292；v1.41 Anaphase 195 → 全生态 1311）

> **注**：Helix-Mind P0-P9 全部完成，P10 准备工作已完成（现状探查 + 执行计划制定），待正式启动。Helix-Tentacle 与 Helix-MCP-Learner 生态联调成功，全链路畅通：MCP-Learner 学习 → L1 静态审查 → stable/ → Tentacle 加载 → 执行工具。HelixECO-Glove P4-T1 完成（L1 静态审查 9 条规则），P4-T2（L2 dry_run）预览中。Helix-MCP-Learner P2/P3/P4-T1 完成（生态联调全链路 + post_learn 审查管道），有 1 个测试失败（非阻塞，待修复）。

### 项目状态详情

#### ✅ 已完成项目（6个）

| 项目 | 完成内容 | 关键成果 |
|---|---|---|
| **Cellrix** | P0-P6 全部完成 + 候选 G 驾驶舱 | 316 测试，Helix 四大组件全部接入，生产就绪；候选 G：AnaphaseClient get_snapshot（一次拉全）+ CockpitWidget（模式栏/经历时间线/Ledger 审查视图）+ renderer strip + attach_cockpit 轮询 + cli --anaphase-endpoint（ADR-0009），双端协议（TUI 先行，Web=G2） |
| **Tuck** | P1-P7 全部完成 | 316 测试，PFP 第一个消费者，亚微秒级决策，fail-closed，全息审计，四层管控接口（Mind/Anaphase/Tentacle bridge + Cellrix StatusProvider，ADR-0003） |
| **Anaphase** | M1 + M1.5 + 候选 E + F + D' 4/4 + G + 编排哲学 ADR-0016 + O-1 + **CI-144 传输层 ADR-0017** | 160 测试，六 stage 确定性流水线（MET/UNMET/replay 字节级一致），真实 Tentacle gRPC 连通（fixture 插件全链路），Reasoning 结构化输出协议，run_cycle ↔ pipeline 完整 merge，零硬编码收口（RunCycleConfig），会话即经历（ADR-0006），重放守卫指纹（ADR-0007），SecurityGate 接线点 + ledger blocked（ADR-0008），真实场景插件 D'-4（ADR-0009），AgentSnapshot 共享快照投影端点（ADR-0010）；**O-1（ADR-0016）**：结构化命令 `!tool` 分诊零 LLM + probe_ecosystem 生态点亮 + 感知点 + **run_cycle 单周期原语化**（7 状态 DAG 一圈返回 CycleOutcome，循环归调用方，模块 agent_loop→run_cycle 改名归位）；**CI-144 传输层（ADR-0017）**：--stdio 从 JSON-lines 切换为 CIB/1.0 MessagePack 握手 + LE u32 长度前缀帧 + Manifest 首帧 + 1s 节律 Snapshot 推流 + ActionRequest/Response（status/send_message 经真实 run_cycle，协议层业务无关经注入回调），vendored 类型 src/ci144/（serde 逐字段对齐 Cellrix），select 单任务事件循环（biased 确定性），live 实测真实二进制全链路 |
| **BIND-19** | v2.0-alpha 核心实现（默认分支） | 142 测试（实测），33 组测试向量，14 个基准测试，PFP+SAP 解析器；main=规范正文（v1.0.0-RFC-4，tag 锚定） |
| **Helix-Tentacle** | P5 完成 + P6 进行中（M1.5） | 153 测试，性能基准+资源限制+可观测性+STDIO/gRPC 传输层，fixture 插件（numbers/rate，SHA-256），MCP-Learner 全链路联调畅通 |
| **phyt-DNA** | 方法论 v1.0 立项 | DNA/RNA/PLAN/GROWTH/ADR 闭环，项目自生长方法论锚点 |

#### 🚧 进行中项目（3个）

| 项目 | 当前阶段 | 待办内容 | 阻塞项 |
|---|---|---|---|
| **Helix-Mind** | ✅ P10 完成（2026-09-06） | 无（P10a 触发链路 + P10b L1 策略落盘 + P10c Deep Dream 全通）；P10d 预约制闹钟为后续规划项 | 无 |
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
1.5. **Anaphase CI-144 传输层（✅ 完成 2026-09-06，ADR-0017）** — 驾驶舱闭环咽喉：--stdio 切换 CIB/1.0 MessagePack（握手 + LE u32 帧 + Manifest 首帧 + 1s 快照推流 + Action 响应），vendored 类型 src/ci144/，160 tests 全绿（+6）+ live 实测真实二进制
2. **Anaphase 候选 G（✅ 完成 2026-09-06）** — Anaphase 驾驶舱（正名：监控意识层，Helix-Mind 灵魂本体不驾驶）：G-T2 AgentSnapshot 端点（ADR-0010）+ G-T3..T5 Cellrix 消费/渲染/live（ADR-0009）+ G-T6 文档；**候选 G2（待启动）**：Web 面板（消费同一 snapshot 协议，DSH 式可视化，低摩擦）
3. **Helix-Mind P10 完成** — 认知工艺生态深度集成全通（P10a 触发链路 + P10b L1 策略持久化 + P10c Deep Dream 复盘）；下一候选：P10d 预约制闹钟（设计方向已获认可）

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

### FlowModus
- [README.md](../../FlowModus/README.md) — 项目说明（rs 重构段含 CLI 用法）
- [VISION.md](../../FlowModus/docs/VISION.md) — 度量衡宣言（v2.0-rs）
- [PLAN.md](../../FlowModus/docs/PLAN.md) — 开发导航牌（R-0..R-6 全 ✅）
- [ADR-0100](../../FlowModus/docs/decisions/ADR-0100-rs-refactor.md) — rs 重构决策记录
- [judge-points contract](../../FlowModus/docs/engineering-manual/judge-points-contract.md) — 判断点契约（v1.1，Anaphase 消费方已就绪 O-6）

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
| **v1.49** | **2026-09-06** | **P10 认知工艺生态深度集成完成** — ①P10b（5e7dd32）：synthesis 落 DAG L1 策略层（provenance `craft#{job_id}` + name-based 确定性 id 幂等；ValueAssessor 分级写元数据 + 响应回显；L1 进共享 FTS 索引，helix_query 天然命中策略复用）；②P10c（476b485）：Deep Dream 睡眠复盘（consolidate:hibernate → 遗忘冷 L3 → L1 新旧覆盖差 ≥ 阈值 → Stale/Viable → AdaptiveMutation EMA 适应 → mutation-state 幂等落盘 + 跨重启 restore，全链路确定性 0 Token；挂载点实现期修正为 api 编排层避免代谢↔认知循环依赖）；③Helix-Mind 测试 101→**107**（+2 P10b 落盘/幂等 + 4 P10c 复盘套件）；④全生态测试总数 1400→**1404**；⑤ADR-0031 D2/D3 落地标注 + 挂载点修正、PLAN v6.4（P10 全 ✅）、GROWTH v1.6（P10 记录 + P0-P9 归档）、README（107/P10 Complete） |
| **v1.48** | **2026-09-06** | **P10a 认知工艺触发链路完成** — ①Helix-Mind：helix_craft RPC（独立编排 RPC，检索/编排解耦）+ 零硬编码收口（trace_id 确定性化 `craft#{job_id}` 去 uuid、阈值进配置）+ 确定性 Adapter 0 token 默认，测试 98→**101**（+3 craft 集成：确定性 trace+synth / 跨调用字节级一致 / fail-closed）；②Anaphase：proto 客户端同步 + MemoryAdapter.craft() 默认降级 + GrpcMindAdapter 调 helix_craft（工序集/约束 MindConfig 协议默认）+ run_cycle MemoryRetrieval 按需触发 + Reasoning [think-first] 折入 synthesis，测试 195→**198**（+3：触发注入 / 结构化跳过 / Noop 降级）；③闭环：Anaphase 触发 → Mind helix_craft → CognitiveCraft orchestrate → 0 token synthesis → 注入 LLM prompt（先思考后花钱）；④全生态测试总数 1394→**1400**；⑤ADR-0031 Accepted、PLAN v6.3、GROWTH 双仓归档清理 |
| **v1.47** | **2026-09-06** | **Callosum 归档 + 生态引用清理（前缀稳定重定源）** — ①**Helix-Callosum 退役**（DEPRECATE.md + README 标记，Python v0.2.0 冻结为算法参考）：物理事实核验——方法论变化稀释 KV cache 价值前提（确定性优先 + 0 tokens 通道 + 按需组装）、70% 功能与 FlowModus 重叠（Composite Router/Economic Profiler/适配器）、生态零运行时依赖（Anaphase endpoint 默认 None / Tentacle bloom 从未接通 / Mind gene-lock 仅为 spec）、3.5 个月零活动；②**前缀稳定重定源**：lodestone L0/L1/L2 确定性投影（稳定前缀从源头生成，非事后重排）+ FlowModus canonicalizer（字节级确定性）——两者之间无 Callosum 位置；KV 缓存折扣为附带红利，成本节省主来自"prompt 本来就短且稳定"；③生态清理：Anaphase 移除 callosum_endpoint（config.rs/gloves.rs，测试全绿）、Tentacle bloom 注释改本地实现（零 Callosum 引用）、Helix-Mind gene-lock 静态前缀池 → lodestone 投影池；④Anaphase 存量 6 处 unused-import warning 披露（非本次引入，独立技术债）；⑤测试总数 1394 不变（本轮含 Anaphase 代码清理，测试数未变） |
| **v1.46** | **2026-09-06** | **Tuck 测试数核验完成（310→316）** — `cargo test --workspace` 实测 **316 passed, 0 failed**（bin 9 + lib 307 + doc-tests 8 ignored）；§1 Tuck 行从 310 修正为 316（原为历史滞后值）；全生态总数注释本就用 316（319+316+195+142+98+153+45+43+83=1394 自洽），**全生态测试总数 1394 不变** |
| **v1.45** | **2026-09-06** | **README 双语化 + Tuck 许可对齐 + 文档语言规范** — ①全生态 README.md 默认英文落地：phyt-DNA/FlowModus/Tuck/helix-tentacle 四仓库 README 转英文主文档，中文版转 README.zh-CN.md（双向互链），其余仓库本就英文；②**Tuck 许可修正：MIT → Apache 2.0**（LICENSE 文件替换，全生态统一许可策略落地，phyt-DNA PROTECTION v1.1）；③phyt-DNA PROTECTION v1.2 新增"文档语言规范"（公开文档默认英文，内部过程文档不受限）；④Tuck README 测试数 316 vs ECOSYSTEM 310 **口径不一致，待核验**；⑤测试总数 1394 不变（纯文档/许可轮） |
| **v1.44** | **2026-09-06** | **许可策略决策 + CI-144 协议级防御性公开落地** — ①phyt-DNA PROTECTION v1.1 新增"许可策略"章节：全生态统一 Apache 2.0 不分层（Apache 与 MIT 采用友好度等价，Apache 免费附带专利授权+报复条款；"消费者项目"承载协议之外独立价值；真正分层的是未来商业服务层而非代码层；宽松=开放采用而非降级许可）；②CommonIntents 协议级防御性公开落地：`.github/docs/prior-art-ci144.md`（四层协议栈 12 项创新点，架构级/传输级/能力级/安全级/语义级/扩展协议，各带证据路径，2026-09-06 公开，profile README 可发现性链接）；③FlowModus prior-art §三 状态更新（CI-144 家族从"待拍板"转"已落地"）；④测试总数 1394 不变（纯保护轮） |
| **v1.43** | **2026-09-06** | **生态保护方法论落地（phyt-DNA PROTECTION v1.0 + FlowModus 零成本保护）** — ①调研提炼 Google/Apple 公开保护实践（Google：分层开放+专有、Apache 2.0 条件专利授权、OPN Pledge、防御性公开计划、标准贡献；Apple：混合模式、Swift Apache 2.0+Runtime Exception、WebKit BSD/LGPL、商标护城河、贡献者审查）；②phyt-DNA 新增 docs/PROTECTION.md：五条保护原则（按需驱动/物理事实优先/极致节能/确定性优先/极致解耦）+ 零成本动作清单 + Prior Art as Code 规范 + 明确不做的四项（IP.com/正式专利/OIN/商标注册，挂议程）；③FlowModus 零成本保护落地（NOTICE + prior-art.md 10 代码级+2 协议级创新点 + README 许可段 + VISION 版权行 + main 分支 README 失效链接修复）；④全生态测试总数 1394 不变（纯文档/保护轮） |
| **v1.42** | **2026-09-06** | **FlowModus rs 重构收口 + 生态入库** — ①FlowModus rs 分支 R-1..R-6 全部完成（83 测试全绿，clippy 零警告）：五层确定性管线（STE/注册表/偏移量/成本/过滤/评分熵路由）、三调用模式（Manual 直连 / Group 优先级+确定性权重采样 / Auto 全五层，Python Group stub 真实落地）、控制面（canonicalizer 补 Unicode NFC 协议缺口 / Ed25519 单键+M-of-N 验签 / anti-corruption 白名单 fail-closed）、寄生遥测（零探测铁律 0，补齐 Python 无生产者的失败冷却 DEGRADED→TERMINAL）、judge-points 契约 v1.1 Rules 后端（JP-1/JP-2 确定性 0 tokens，Anaphase O-6 消费侧已就绪）；②CLI 真实可用（`flowmodus judge/measure/verify`，真机 smoke 全通）；③全生态测试总数 1311→**1394**（+FlowModus 83）；④协议 Apache 2.0 双分支（main/rs） |
| **v1.41** | **2026-09-06** | **O-6 判断点后端可配化（ADR-0024，Anaphase）** — ①用户务实修正编排哲学：0 tokens 是默认通道不是教条，3B 级小 LLM 判断 ROI 足够高时可用（固化 HANDOFF §1.3）；②新建 `src/judge.rs`：Judge trait（全后端必返回 1/2/3）+ RulesJudge（阈值来自 MindConfig，删除 assess_complexity 残留 10/40 字面量——0 硬编码收口）+ SmallLlmJudge（OpenAI 兼容 3B 端点，任何失败回退 Rules）；③config `judge_backend`/`judge_endpoint`/`judge_model`；④judge-points contract v1.0-draft 入 FlowModus docs（JP-1/JP-2 规格，显式选后端，不做 Auto Router）；⑤Anaphase 189→**195**，全生态 1305→**1311** |
| **v1.40** | **2026-09-06** | **O-5 按需认知注入（ADR-0023，Anaphase）** — ①探查确认记忆检索断裂：memory_nodes 存 context 但从未注入 LLM（检索白做）→ 修复：折叠注入 Reasoning prompt（join + 预算截断 + 显式折叠标记）；②`memory_inject_chars`（`[anaphase]`，协议默认 800）单一来源，0=纯无状态（legacy 兼容）；③25 轮上下文近零增长验收（注入量预算封顶与轮数无关，+6 测试）；④演示输入来源化：`--input` > `smoke_input` config > 协议默认 const（0 硬编码）；⑤窗口 L0 诚实标注：无对话入口（数据源缺失），切片待 UI 会话层接入，不伪造缓冲；⑥Anaphase 183→**189**，全生态 1299→**1305** |
| **v1.39** | **2026-09-06** | **O-4 认知工艺触发接线验证 + MindConfig 零硬编码收口（ADR-0022，Anaphase）** — ①探查确认触发链代码早已存在（ADR-0001）但从未跑通真实 gRPC：复用既有 mock Mind（mind_integration.rs）+ p11b 闭环（不重复建设，common 临时 MockMind 已回滚）；②T3 回归守卫：驾驶模式（Noop 装配）下 Mind 零接触——模式门=装配，零运行时分支；③mind.rs 12 处无来源字面量（EnergyContext 数值/推导阈值/探针回退/探索关键词）→ MindConfig（`[anaphase.mind]` 可覆盖）单一来源收口，grep 仅剩 heliotropism=0.0（注释声明派生）；④`GrpcMindAdapter::new(endpoint, config)` 签名变更（8 处调用点）；⑤Anaphase 182→**183**，全生态 1298→**1299** |
| **v1.38** | **2026-09-06** | **ADR-0021 时钟复用修正（Anaphase）** — ①审查发现 cycle 级事件初版用墙钟（`Utc::now`），违背极致复用（ledger 已有 `Clock` trait）与确定性优先（黑匣子不可回放）；②修正：`AgentLoop.clock`（默认 SystemClock）+ `with_clock`，cycle ts = `unix_secs_to_rfc3339(clock.now())`——与 stage/ledger 同一时间源；③FakeClock 下黑匣子字节级可回放（+1 测试 `black_box_replays_byte_identical_under_fake_clock`）；④Anaphase 181→**182**，全生态 1297→**1298** |
| **v1.37** | **2026-09-06** | **ADR-0021 模式无关事件环（Anaphase）** — ①事件环从 pipeline 提升 AgentLoop 级：驾驶模式（无 tentacle/Noop 装配）每次 run_cycle 也记录 cycle 黑匣子（stage=0：begin/state/tool/end，trace=derive_job_id）；②pipeline 装配复用同一环（stage 1..=6 同流同游标），一个 `?after=` 拉全部；③flush/恢复挂载点改 agent.events（驾驶模式同样持久化跨重启可回放）；④ts 分权：cycle=墙钟审计真值，stage/ledger=FakeClock 回放契约；⑤物理验证：无 tentacle 二进制 → events.jsonl 7 条 stage=0 事件；⑥Anaphase 180→**181**，全生态 1296→**1298**；⑦白盒四层模式无关（驾驶=黑匣子，伙伴=黑匣子+六 stage） |
| **v1.36** | **2026-09-06** | **O-3 事件轨迹持久化（ADR-0020，Anaphase）** — ①`EventRing::from_jsonl`：round-trip 字节一致 / seq 跨重启接续 / 坏行失败关闭 / cap 强制（4 新单元测试）；②`events_log_path` 默认 `events.jsonl`（session_notes 先例，实时逐轮追加，崩溃最多丢在飞轮）；③main 装配恢复历史（fail-open）+ 主循环增量 flush；④白盒四层（能力/状态/过程/事实）全部可跨重启追溯；⑤Anaphase 176→**180**，全生态 1292→**1298**；⑥PLAN O-3/O-5 编号修正（事件持久化 = O-3，按需加载落点移 O-5） |
| **v1.35** | **2026-09-06** | **Anaphase O-2 stage 事件总线（ADR-0019）**。过程白盒第四层：append-only 事件环（事件=过程，ledger=事实，evidence=支撑）+ 六 stage 边界插桩（stage1/2 Reasoning、stage3 execute_calls begin+per-call end、stage4 evidence、stage5/6 Reflection criteria+verdict）+ trace_id=派生 job_id（一次 cycle 一条 trace）+ `GET /v1/agent/events?after=N` 增量拉取（记录非控制流）+ events_cap 来自 codex contract。Anaphase 测试 169→176（+3 events 单元 +4 stage_events 集成），全生态 1285→**1298**。零新依赖（拒绝 OTel SDK/集中式后端）。⑧VISION.md v1.2 补"生态落地对照"（rails 0-token 引用 + 白盒四层 + 熟练 vs 硬铁轨同源）；Helix-Mind README 补生态同步行 |
| **v1.34** | **2026-09-06** | **Rails 输出契约层 + MCP doctest 修复**。①Anaphase rails 输出契约层（ADR-0018）：rail 命中时 Reasoning 短路 LLM——回答 = assemble_rail_answer 确定性拼装（0 tokens、无编造空间、含节点 id + 原文逐字引用），e2e 断言 LLM 调用数=0；代码注释 ADR-XXXX 全部落定 ADR-0018；测试数 169 不变。②Helix-MCP-Learner doctest 修复：glove 模块文档示例引用 HelixECO-Glove 的 crate（非本仓依赖）→ 标 rust,ignore，套件全绿（50 单元 + doctests）。全生态 **1285** 不变 |
| **v1.33** | **2026-09-06** | **Anaphase Rails 心智外铁轨（ADR-0018）**。人类知识 DAG（宪法/律法/SOP）只读引用铁轨：knowledge_base/rails/<kb>/ markdown + 确定性索引（SHA-256 版本冻结、断链即错）+ 铁轨导航（词项 + CJK bigram，无嵌入）+ 引用契约（原文引用 + visited check 验证器 + 引不到答 NO_RAIL_CONTENT）；RailScope 类型级只读；接线 MemoryRetrieval。Anaphase 测试 160→169；全生态合计物理核对重算：历史记录 1287 含累计误差（实为 1276，各仓库已逐仓核对），本轮后 **1285**（+9）。与熟练模式同源（心智内软铁轨 vs 心智外硬铁轨，硬度=错误的代价）|
| **v1.32** | **2026-09-06** | **Cellrix 侧 CI-144 stdio 闭环（ADR-0017 跨仓库完成）**。①StdioTransport::send_action 落地（此前 NotImplemented）：untagged Incoming enum 单 reader 分发——AgentEvent 走事件流、ActionResponse 走专用响应通道，无帧竞争（确定性）；②cli --exec 生态启动约定（追加 `--mode stdio`）与 Anaphase 兼容（Anaphase main.rs 接受 `--stdio`/`--mode stdio` 双标志）；③真实 Anaphase 二进制三通道实测：manifest（CapabilityManifest{anaphase-helix}）/ snapshot（partner + 3 节点布局引擎消费）/ action（status + send_message 真实 run_cycle）；④新 live 测试 transport/tests/ci144_anaphase_live.rs（#[ignore]，ANAPHASE_BIN env）；⑤测试数不变：Cellrix 319 / Anaphase 160，全生态 1287 |
| **v1.31** | **2026-09-06** | **Anaphase CI-144 传输层落地（ADR-0017，驾驶舱闭环咽喉）**。①--stdio 从 JSON-lines 临时协议切换为 CIB/1.0 MessagePack：握手首行 → LE u32 长度前缀帧 → Manifest 首帧 → 1s 节律 Snapshot 推流（SNAPSHOT_PUSH_INTERVAL，config 可调不硬编码）→ ActionRequest/Response；②协议类型 vendored 到 src/ci144/（serde 逐字段对齐 Cellrix，tag/content/snake_case/开放枚举降级），不跨仓库依赖（极致解耦）；③协议层业务无关：run_loop(reader, writer, snapshot, handle_action, interval) 注入回调，launcher 挂 status/send_message（真实 run_cycle，cap 尊重 cycle_cap）；④select 单任务事件循环（biased 确定性，无 spawn/Send 体操）；⑤测试 154→160（+6：握手 x2/帧往返/投影形状/vendored serde 形状/duplex 全协议会话）+ live 实测（tests/ci144_live.rs #[ignore]：真实二进制全链路握手→Manifest→Snapshot→status→send_message→unknown→EOF 退出）；⑥全生态 1281→1287 |
| **v1.30** | **2026-09-06** | **Cellrix 驾驶舱 P0 落地 + 0 warnings 收敛**。①cellrix-web（ADR-0014 G2）实测通过：Anaphase :50061 snapshot → :8080 代理，mode/state/episode/ledger/ecosystem 渲染 + 2s 轮询；②Cellrix 全 workspace 0 warnings（RiskLevel 三域歧义显式路径化、transport 死 import 清理、ui 组件库预留标注）；③测试 307→319（workspace 实测，含 G2 面板与 up 入口测试）；④全生态 1269→1281 |
| **v1.29** | **2026-09-06** | **Anaphase O-1 深化：run_cycle 单周期原语化**。①run_cycle() 从内置循环改为单周期原子原语（7 状态 DAG 走一圈返回 CycleOutcome{done/success/impasse}，循环策略归调用方，cap 作为防死循环保险丝）；②模块/文件 agent_loop → run_cycle 改名（git mv 11 文件，类型 AgentLoop 保留）；③cycle_cap 来源落地（config 注释：本地 LLM 上下文预算保守默认）；④测试 152→154（+2 单周期语义），全绿 |
| **v1.28** | **2026-09-06** | **Anaphase O-1 落地（ADR-0016 D1/D3 首个物理落点）**。①结构化输入分诊：`!tool {"json"}` 在 Perception 解析、Reasoning 跳过 LLM 直接组装 tt_job（计数 reasoning adapter 断言零调用）；②probe_ecosystem：任务开始前一次物理探测（TCP connect / UDS 文件存在性，fail-open），Cellrix=Native 手套，AgentContext/AgentSnapshot 携带生态点亮；③感知点：Reasoning 前看一眼口袋 + Execution 对 tentacle 未点亮记录降级事实；④测试数 140→152（+12），O-1 ✅，下一步 O-2 stage 事件总线 |
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
