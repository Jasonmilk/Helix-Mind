- **决策日期**：2026-08-27
- **对齐知识本体**：v4.1
- **原始引用路径**：审查② B2（硬冻结契约"补齐"的语义悖论）→ 收敛为 Append-Only
- **状态**：采纳

# ADR-0012: 硬冻结契约 Append-Only Schema Evolution

## 状态

采纳（2026-08-27）

## 问题

v2 声称"硬冻结契约一律不动，只补齐"，但又将 `activation_vector` 缺失标记为"硬冻结契约未兑现"。若契约真的"不动"，未兑现字段永远无法加入；若允许"补齐"，则实际允许了向后兼容的追加——二者存在语义悖论（审查② B2）。

## 决策

明确区分两个概念：

- **冻结（Frozen）**：已有字段的语义、类型、序号**不修改、不删除、不重排**。
- **扩展（Extension）**：允许**追加**新字段，protobuf 的 `reserved` 语法保护未来字段号。
- **"补齐" = 扩展，不是修改**。变更策略统一为 **"Append-Only Schema Evolution"**。

### 本次应用（P0）
- `HelixQueryResult`：追加 `reserved 13;`（为 `activation_vector` 预留字段号，P4 M-10 落地）。
- `Node`：追加字段号 16-19（`phase_state`/`subject_dependency`/`concentration`/`tension`，ADR-0011）。
- `EnergyContext`：追加字段号 9（`budget_tier`，ADR-0010）。

### SQLite 侧迁移策略
- `ALTER TABLE ADD COLUMN` 仅追加、不删除、不改类型。
- 迁移幂等：以 `PRAGMA table_info(nodes)` 检测列存在性，缺失才 ADD。
- 回填为一次性 UPDATE（`subject_dependency` 物化，ADR-0011）。
- 未来如需大规模投影重建，使用 shadow-table 迁移（新建表 → 复制 → 原子换名），不动原表语义。

### 缺陷处置（强制）
若 ADR-0012 迁移逻辑在执行中被发现缺陷：**新建 ADR-0012b 并标记本版 `Superseded by 0012b`，绝不覆写**。这是"记忆不可篡改"在工程层的投射。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 硬冻结契约既有语义绝对稳定 | 字段号需要 reserved 纪律 |
| 旧客户端/旧数据向后兼容 | SQLite 需幂等迁移框架 |
| 缺陷可追溯（Supersede 而非覆写） | 未来字段永远只能追加 |

## 回滚阈值

若某个"追加"字段事后证明方向错误，不删除字段——`reserved` 其字段号并停用，另开新字段号，保持 append-only 不变量。

## 相关原则

- 记忆不可篡改（工程层投射）
- 真理源单一性
- 零信任管道（数据自校验）
