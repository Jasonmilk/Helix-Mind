# [2026-08-28] 完成：P7 生态文档同步 + 认知工艺 Phase 1

### 触发条件
P0-P6 全部完成（PLAN 阶段表修正滞后）；用户批准认知工艺 v1.1（吸收 Gemini 审查补丁 7 项 + 豆包审查 B1/B2/B3 三项修正）。

### 变更性质
- **ADR-0021（Active）**：认知工艺决策——①执行边界：Mind=编排（0 Token 纯逻辑），执行经 ADR-0017 CognitiveService 注入（B1）；②System 0 门控：规则 + FTS5 bm25 相关度阈值，非 embedding（B2，零向量依赖）；③与 ADR-0010 划界：预算路由=外部前置定扫描范围，System 0=Mind 内定思考深度（B3）
- **`docs/spec/cognitive-craft.md` v1.1**：认知工艺完整规范（工序×模式、MSC 最小充分上下文、黑格尔辩证确定性合题、Bounded ε-Greedy+EMA 抗僵化、Token 熔断+30s 超时）
- **`docs/ecosystem-alignment.md`**：ECOSYSTEM v1.6 对齐核对——Mind↔Anaphase/Callosum 契约逐项核对无断链；CI-144 动词映射 5/5 一致（FETCH/WRITE_NODE/TENTACLE/FINISH/CANCEL vs INTENT-7 spec §3）；新增 budget_tier/traceparent 契约字段建议
- **文档生态同步**：PLAN 阶段表 P3-P7 全部 ✅ + P8 预览（认知工艺 Phase 2）；GROWTH 超 3 条归档最旧记录（v4.1 变异 → docs/archive/growth/，已版本化）

### 兼容性
纯文档阶段，零代码变更；硬冻结契约零变更；cargo test 全绿（回归确认 70 通过）

### 验收
ADR-0021 Active 且含 B1/B2/B3；cognitive-craft.md 落位；对齐核对 5/5 动词一致、零断链；PLAN ≤150 行

### 状态
✅ 已归档（2026-08-29，GROWTH 超 3 条自动归档）
