- **决策日期**：2026-08-28
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：ECOSYSTEM.md v1.5 §3.1（trace_id 由 Anaphase 生成）/ §4.1（Mind↔Anaphase 契约）+ INTENT-7 spec §2-3（traceparent + 动词）+ P3 计划（2026-08-28 起草，待审查）
- **状态**：Active（2026-08-28 审查通过，P3 编码开工）

# ADR-0020: P3c CI-144 INTENT-7 语义对齐（动词映射 + traceparent 透传）

> 编号说明：ADR-0015 预留给 P4.5 WAL，0016-0019 已用，故本决策取 0020。

## 状态

Active（2026-08-28 审查通过，P3 编码开工）

## 问题

ECOSYSTEM.md §3.1 要求：外部请求 → Anaphase 解析意图生成根 `trace_id`，Mind 的 `HelixQueryRequest` 携带该 trace_id，结果原样回传。但代码现状：
- proto `HelixQueryRequest` **无 traceparent 透传字段**（ECOSYSTEM 要求全链路追踪的载体缺失）；
- Mind 的 gRPC 契约与 CI-144 INTENT-7 动词（FETCH / WRITE_NODE / TENTACLE / FINISH / CANCEL）**无显式映射表**，语义对齐无单一真相源。

## 决策

### 1. INTENT-7 动词 → Mind gRPC 契约映射表（文档，单一真相源）
| INTENT-7 动词 | Mind gRPC 契约 | 说明 |
|:---|:---|:---|
| `FETCH` | `HelixQuery` | 语义检索（summary-first） |
| `WRITE_NODE` | `HelixConsolidate` / L1 `Remember` | 记忆写入 |
| `TENTACLE` | （Anaphase 编排，Mind 不执行） | Mind 只建议，不执行工具 |
| `FINISH` | 认知循环结束（Anaphase 生命周期） | Mind 记录 L3 收尾 |
| `CANCEL` | 中止执行树（Anaphase 生命周期） | Mind 记录中止原因 |

- 映射表落位：`docs/spec/api.md`（与硬冻结契约同文件，追加映射节）。
- **边界**：Mind 只认 Anaphase gRPC 接口，不直接对接 TENTACLE/Executor（body-agnostic 铁律）。

### 2. `traceparent` 透传（Append-Only Schema Evolution）
- proto `HelixQueryRequest` / `HelixQueryResult` 追加 `traceparent` 字段（reserved 号保护，Append-Only，不修改已有字段）。
- Mind 行为：透传不生成——trace_id 由 Anaphase 在入口生成（ECOSYSTEM §3.1），Mind 原样回传，支持全链路审计。
- 对齐 INTENT-7 §2.1（`metadata.traceparent`，W3C 标准）。

### 3. 对齐边界（P3 范围）
- 只做**语义层（INTENT-7）映射** + traceparent 透传。
- CAPABILITY-13 / BIND-19 / INTENT-7-SECURE 的传输层对接归 Anaphase / Tuck，不在 Mind 范围。
- CI-144 v2.0（PAL 物理锚定层）是协议家族侧工作，**不阻塞**本 ADR，另行落地。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 全链路 trace 追溯落地（ECOSYSTEM 承诺兑现） | proto 契约追加字段（Append-Only，向后兼容） |
| 语义对齐有单一真相源（映射表入 spec/api.md） | 映射表需与 Anaphase 侧对齐核对（P7） |
| 边界清晰（Mind 不越权执行） | 纯文档 + 字段追加，无检索/代谢逻辑改动 |

## 回滚阈值

- 若 Anaphase 侧尚未实现 INTENT-7 动词 → 映射表保留为 Draft 节，traceparent 字段先落地（透传无损）。
- 若 W3C traceparent 格式与现有 trace_id 语义冲突 → 采用双字段（保留 trace_id + 新增 traceparent），不删已有。

## 关联

- 前置：P0 ADR-0012（Append-Only Schema Evolution）
- 对齐：ECOSYSTEM §3.1 / §4.1、INTENT-7 spec §2-3
- 后续：P7 生态文档同步（ECOSYSTEM v1.6）、CI-144 v2.0（PAL，另行）
- 受审决策：D6（对齐产物形式）
