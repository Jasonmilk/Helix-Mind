# 生长记录归档：P0 认知基线 + 可编译数据契约变更

> 原记录位于 `docs/GROWTH.md`，按 DNA 方法论 v2.0「GROWTH ≤3 条」规则归档。归档文件版本化，永不删除。

## [2026-08-27] 完成：P0 认知基线 + 可编译数据契约变更

### 触发条件
审查③ F1/F9 断层修正：P0 从"纯文档"升级为"文档 + 可编译数据契约变更"，否则 P1 无 schema 可用。

### 变更性质
- **ADR 冻结**：ADR-0010（认知预算前置路由）、ADR-0011（相态模型与主体依赖轴）、ADR-0012（Append-Only Schema Evolution）
- **数据契约（全链路）**：Node+SQLite+proto 追加 `phase_state`/`subject_dependency`/`concentration`/`tension`；`EnergyContext.budget_tier`；`HelixQueryResult` reserved 13（activation_vector）
- **债务修复**：`Config::ensure_dirs()`（启动自动建目录）；所有 Config 手动 `impl Default`（修 serde 默认失效）
- **测试**：迁移测试（22 列旧库→补列+物化回填）；cli 往返测试补新字段断言

### 兼容性
- 硬冻结契约无变更（Append-Only 追加字段 16-19，reserved 保护）
- SQLite 幂等迁移 + 一次性物化回填（L2→Low / L3→High / Liquid / Dissolved / 0.0）

### 验收
`cargo check` ✅ ｜ `cargo test` 全绿（cli 4+1 ignored、core 8、storage 1）｜ 冒烟双债务修复 ✅

### 状态
🧬 已合并（提交 b710ce6）
