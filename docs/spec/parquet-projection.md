# Parquet 投影 Schema 约束（P4.5 架构审查点产出 ③）

> **状态**：约束文档（P4.5 产出，P6 落地"名实相符"）
> **日期**：2026-08-28
> **对齐**：审查③ F7（Parquet 名实不符）、ADR-0015（投影约束）

## 一、现状诚实声明

`storage/parquet_store.rs` 当前**名实不符**：模块名为 parquet，实际写 JSON（为规避 polars 兼容问题）。
- 影响：依赖方（深冷归档等）误以为获得列式存储；JSON 读取效率差数量级。
- P6 目标：parquet 名实相符（引入列式写入/读取，或明确改名并保留 JSON 语义）。

## 二、投影原则

- **投影 = 状态派生**（ADR-0015）：SQLite 是液态投影，Parquet 是晶体/深冷归档投影。
- **必须投影字段**：检索与归档路径依赖的核心字段，缺一不可。
- **可延迟字段**：低频/大体积字段，允许延迟投影（P6 或按需反序列化）。

## 三、Node 投影字段分级（v4.1 数据契约）

| 等级 | 字段 | 理由 |
|---|---|---|
| **必须投影** | `id`（UUID 字符串） | 主键 |
| | `node_type` | 类型路由 |
| | `content`（JSON 序列化） | 知识本体 |
| | `heat` / `dominance` / `utility` | 检索权重信号 |
| | `created_at` / `last_accessed_at`（epoch） | 时间索引 |
| | `access_count` | 访问统计 |
| | `sensitivity` | 权限路由 |
| | `phase_state` / `subject_dependency` | 相态路由（v4.1） |
| **可延迟** | `concentration` / `tension`（meta） | 相态元数据，按需 |
| | `notes` / `derived_from` / `corrected_by` | 备注与衍生关系，低频 |
| | `attribution_ledger` | 归属账本，大体积，按需 |
| | `abstract_provenance` | 抽象来源，P 阶段预留 |
| | `source`（Federated 明细） | 联邦来源，可延迟反序列化 |

## 四、Schema 约束（P6 落地时的硬性要求）

```text
列：id(string) node_type(string) content(string) heat(f64) dominance(f64)
    utility(f64) created_at(int64) last_accessed_at(int64) access_count(int64)
    sensitivity(string) phase_state(string) subject_dependency(string)
延迟列（按需）：meta(string JSON) notes(string) derived_from(string JSON)
    attribution_ledger(string JSON) abstract_provenance(string JSON) source(string JSON)
```

- **不可变**：列名/类型一经冻结，追加采用新列（Append-Only，ADR-0012 精神）。
- **分片**：按 `created_at`（月）分片，与深冷归档周期对齐。
- **压缩**：`content` 等大列启用压缩（snappy/zstd），其余列原始列式。

## 五、验收标准（P6）

- `parquet_store` 实际写出 Parquet 列式文件（不再是 JSON）。
- 必须投影字段全部在列中；可延迟字段不在列中（按需加载）。
- 与 Node struct / SQLite schema 三者对齐（proto ↔ core ↔ 存储，契约一致性检查）。
