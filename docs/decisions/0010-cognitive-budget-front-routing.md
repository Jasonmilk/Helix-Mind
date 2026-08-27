- **决策日期**：2026-08-27
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：审查② B1（预算时序矛盾）→ 收敛为前置路由
- **状态**：采纳

# ADR-0010: 认知预算前置路由（cognitive budget front routing）

## 状态

采纳（2026-08-27）

## 问题

v2 计划同时存在两个互斥陈述：§5.2 称"认知预算是后置评估（检索后标注）"，§4.7 却要求紧急查询"只查晶体 + 高相关度胶体"。若预算是后置的，检索引擎在发起查询时不知道当前预算是 Endogenous 还是 Augmentable，无法决定扫描范围。这是审查② 的阻塞级发现 B1。

## 决策

- **预算前置路由**：认知预算等级 `budget_tier` 由**身体**（Anaphase/Callosum，或任何通用 harness）在调用 Mind 前根据 token 配额决定，随 `HelixQueryRequest.energy_context.budget_tier` 传入。
- **Mind 只做预算执行**：Mind 检索层按 tier 选择相态扫描范围（P1 落地），不自行评估预算、不后置标注。
- **契约扩展（Append-Only，ADR-0012）**：proto + core `EnergyContext` 追加 `budget_tier` 枚举：
  | tier | 语义 | 相态扫描范围 |
  |:---|:---|:---|
  | `AUGMENTABLE`（默认 0） | 常规深度查询 | 液态 + 胶体（既有全量行为） |
  | `ENDOGENOUS` | 0-token 紧急 | 仅晶体 + 高相关胶体 |
  | `EXOGENOUS_REQUIRED` | 探索 | 可触及气态痕迹 |
  | `VOID` | 无认知 | 仅元数据 |
- **默认 = `AUGMENTABLE`**：未指定 tier 的身体（尤其是通用生态 harness）保持既有检索行为，不被意外限制——这是"Mind 兼容通用生态（body-agnostic）"铁律的物理保证。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 检索范围可预算化，0-token 紧急路径可行 | 身体必须正确设置 tier，否则语义错配 |
| 知识树膨胀由预算路由控制，不由准入控制 | 跨层契约扩展（core + proto + api 转换） |
| 默认值保持向后兼容，通用身体开箱即用 | — |

## 回滚阈值

若默认 `AUGMENTABLE` 在 P1 检索落地后引发异常召回（如 0-token 场景未被显式触发），可改为按 `token_budget` 推导 tier，或退回复合模式。此回滚不破坏契约字段（仅改路由逻辑）。

## 相关原则

- 极致节能（优先级高于召回完整度）
- Mind 兼容通用生态（body-agnostic，契约不烘焙身体假设）
- Append-Only Schema Evolution（ADR-0012）
