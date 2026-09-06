
## [2026-08-29] 完成：CI-144 v2.0 前置必填项锁定（11 项设计定案）

### 触发条件
CI-144 v2.0 协议规范升级计划通过完整审查（原 8 项前置必填 + 补丁 A/B/C 共 11 项设计锁定），DNA 方法论闭环完成（PLAN/GROWTH/ADR），准备进入 v2.0-alpha 编码阶段。

### 变更性质
- **`docs/vision/ci-144-v2.0-upgrade.md` v2.0-alpha.0**：CI-144 v2.0 协议规范升级计划完整版——PAL 扩展至 24 字节（容纳 64-bit PAH-Signature）、BIND-19 新增 16-bit Seq-Counter（防重放）、PAH 双层安全架构（64-bit 快速校验 + 512-bit 完整验证）、Replay-Enable=0 强化约束（强制 MEDIUM + 强制 PAH 验证 + 审计标记）
- **补丁 A：Replay-Enable=0 强化约束**：三条硬约束缺一不可——风险降级（强制 MEDIUM，代码语义修正：min() → 直接 RiskLevel::MEDIUM）+ 强化验证（强制 PAH 签名）+ 审计强制标记（REPLAY_DISABLED + 硬件时钟）
- **补丁 B：PAH 双层安全架构**：第一层 64-bit 快速校验（Tuck 亚毫秒级拦截决策）+ 第二层 512-bit 完整验证（载荷解密后，Helix-Mind/云端审计）；实时拦截 + 事后追责闭环
- **补丁 C：Seq-Counter 回绕与密钥轮换**：≥65534 触发密钥轮换（预留 2 帧缓冲）；KEY_ROTATION 控制帧（主密钥加密保护）；未收到轮换帧而回绕视为异常重放攻击；AtomicU16 + SeqCst 原子递增
- **6 个 ADR 占位符（ADR-0022~0027）**：Alpha 实现期间需逐个锁定的 6 个实现细节（B-1/B-2/B-3/C-1/C-2/C-3）
- **方法论闭环**：PLAN.md 任务更新 + GROWTH.md 记录 + 6 个 ADR 占位符创建

### 兼容性
纯文档阶段，零代码变更；CI-144 v1.0 契约零变更（向后兼容：BIND-19 新增 PAL-Present 标志位，v1.0 接收端忽略）；v2.0-alpha 分支待创建后开始编码。

### 验收
11/11 项设计约束全部评审通过；ci-144-v2.0-upgrade.md 版本 v2.0-alpha.0 状态 READY_FOR_ALPHA；6 个 ADR 占位符创建；PLAN ≤150 行；GROWTH ≤3 条（P7 已归档）。

### 状态
🧬 待提交（用户确认后 push）
---
## [2026-09-06] 完成：P10a 认知工艺触发链路（helix_craft RPC + Anaphase 按需触发）

### 触发条件
P10 任务书定稿（ADR-0031 Accepted，2026-09-06 用户确认）后开工；P10-0 零硬编码收口完成（trace_id 确定性化、阈值进配置、去 uuid）。

### 变更性质
- **helix_craft RPC**（P10a-T1/T2，`8c15a4f`）：Mind 新增独立编排 RPC（检索/编排解耦，D1）——`HelixCraftRequest/Result` + `ProcessStep/StepOutput`；server 携带 `Arc<CognitiveCraft>`；handler 做 wire 字符串 → 认知 Process/Mode 转换（未知值 fail-closed）；确定性 trace_id `craft#{job_id}`（去 uuid）；traceparent 透传（P3c 规则，Mind 不是 trace 根）；value_grade 诚实留空（P10b 填充）；cli 以 DeterministicAdapter（0 token）装配，B1 保持（编排不直连 LLM）
- **Anaphase 触发点**（P10a-T3，`db07c5b`）：proto 客户端同步；`MemoryAdapter.craft()` 默认方法（Err=不可用，Noop 零改动静默降级）；`GrpcMindAdapter.craft()` 调 helix_craft——工序集/约束来自 MindConfig（协议默认，DNA 原则 11），循环只问"要不要想"、adapter 决定"怎么想"（按需驱动）；run_cycle MemoryRetrieval 阶段对非结构化输入触发（确定性 job_id），Reasoning 阶段以 `[think-first]` 折入 synthesis（0 token 思考先于 LLM tokens）
- **测试**：Mind +3 集成（确定性 trace+synth / 跨调用字节级一致 / fail-closed）；Anaphase +3（触发+注入 / 结构化跳过 / Noop 降级）

### 兼容性
helix_query 语义零改动（向后兼容）；现有 195+98 测试零改动全绿；Noop/默认适配器无需实现 craft。

### 验收
Mind workspace 101 全绿 0 warning（`8c15a4f`）；Anaphase 198 全绿 0 warning（`db07c5b`）；跨仓库闭环：Anaphase 触发 → Mind helix_craft → CognitiveCraft orchestrate → 0 token synthesis → 注入 reasoning prompt。

### 状态
✅ 已推送（Mind rs-dev / Anaphase rs）
---
## [2026-09-06] 完成：P10 认知工艺与生态深度集成（P10a 触发 + P10b L1 落盘 + P10c Deep Dream）

### 触发条件
P10 任务书（ADR-0031）三阶段全部完成：P10a 触发链路、P10b 策略持久化、P10c Deep Dream 挂载。

### 变更性质
- **P10a 触发链路**：新增 `helix_craft` RPC（Mind 侧 server + Anaphase 侧 adapter），Anaphase run_cycle 按需触发认知工艺（`[think-first]` 折入 synthesis）；P10-0 零硬编码收口（trace_id 确定性化去 uuid、阈值进配置）
- **P10b L1 策略持久化**：synthesis 落 DAG L1 策略层（provenance `craft#{job_id}`，name-based 确定性 id 幂等；ValueAssessor 分级写元数据 + 响应回显；L1 进共享 FTS 索引，helix_query 天然命中）
- **P10c Deep Dream**：consolidate:hibernate → 遗忘冷 L3 → 睡眠复盘（L1 新旧覆盖差 ≥ 阈值 → Stale/Viable）→ AdaptiveMutation 适应（EMA + Bounded ε-Greedy）→ mutation-state 幂等落盘 + 跨重启 restore；全链路确定性 0 Token
- **挂载点修正**：代谢 → 认知依赖成环（CognitiveService trait 在 metabolism），改为 api 编排层 `sleep_review` 模块组合两者（无循环依赖，ADR D3 已诚实记录）

### 兼容性
零破坏：helix_query/helix_craft 语义向后兼容；不新建 crate/存储；认知测试 + 新增集成测试全绿。

### 验收
PLAN.md v6.4（P10 全 ✅）｜ ADR-0031（D2/D3 落地标注 + 挂载点修正）｜ README（107 tests）｜ ECOSYSTEM v1.49（Mind 107，全生态 1404）

### 状态
🧬 已完成
