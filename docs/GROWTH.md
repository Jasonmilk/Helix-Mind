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
## [2026-09-06] 完成：P10d 预约制闹钟（ana_wakeup + 高峰拥挤保护，ADR-0032）

### 触发条件
用户确认 P10d 设计方向 + 命名（ana_wakeup）+ 高峰拥挤保护要求（前后 1 小时弹性窗口）。

### 变更性质
- **预约本体**：L2 节点（provenance `alarm#{job_id}`，name-based 确定性 id 幂等），状态机 pending→claimed→done/renewed（节点 notes），复用 storage 管道零新存储
- **双档到期**：punctual（Helix 预约，准点 due_at）/ jittered（Ana 唤醒，弹性窗口 [due_at−m, due_at+m]，m 来自调用方 config 默认 60，0=关闭）——高峰保护无随机无定时器
- **双 RPC**：ana_wakeup（列出到期 + 原子认领防重入）/ ana_wakeup_ack（done 关闭 / renewed 从原 due 续约防漂移，幂等）
- **无心跳铁律保持**：Mind 永不主动执行，Anaphase 持有时钟按节奏唤醒（config 默认 8 点 + 交互间隙）
- **测试**：+6 集成（准点/窗口/关闭/原子认领/确认续约/迟到不丢弃），workspace 31 套件全绿 0 warning，Mind 107→113

### 兼容性
零破坏：proto Append-Only 扩展（新 RPC + 新消息，不影响现有 RPC）；不新建 crate/存储/定时器。

### 验收
ADR-0032（Accepted）｜ PLAN v6.5（P10d ✅）｜ README（113 tests）｜ ECOSYSTEM v1.50（Mind 113，全生态 1410）

### 状态
🧬 已完成
