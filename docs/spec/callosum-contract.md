# Mind ↔ Callosum 调用契约（P4 M-11，文档化，非实现）

> **状态**：Active（契约文档，非代码实现）
> **日期**：2026-08-28
> **对齐**：ECOSYSTEM §4.2（Mind ↔ Callosum gRPC）、审查收敛（M-11 从"实现 Callosum 接口"降级为"定义调用契约"）
> **边界**：Callosum 是 Anaphase-Helix 的器官（左右脑互通的上下文内存分配器），**分配策略属于 Callosum 自治域**。Mind 只定义"在什么场景、以什么格式请求上下文分配"，不实现、不控制分配逻辑。

## 一、职责边界

| 主体 | 职责 |
|---|---|
| **Helix-Mind** | 在需要上下文切换时，通过标准接口向 Callosum 请求内存分配（接口消费方） |
| **Callosum**（Anaphase 器官） | 分配策略、左右脑协调、token 预算切分（接口提供方，自治域） |

## 二、接口契约（ECOSYSTEM §4.2）

| 接口 | 方向 | 格式 | 说明 |
|---|---|---|---|
| `GetStaticPrefix` | Mind → Callosum | gRPC (Protobuf) | 获取 L0/L2 高频节点缓存 |
| `UpdateStaticPrefix` | Mind → Callosum | gRPC (Protobuf) | 更新静态前缀池 |

## 三、调用场景（Mind 何时请求上下文分配）

1. **认知循环开始**：Mind 收到 `HelixQueryRequest` 时，若当前 EnergyContext 指示高 token 压力，向 Callosum 请求 `GetStaticPrefix` 获取高频节点缓存，优先命中静态前缀，减少动态检索。
2. **代谢期批量处理**：digest/crystallize 需遍历高频 L2 时，通过 `GetStaticPrefix` 获得候选集，避免全表扫描。
3. **静态前缀变更**：Mind 完成一次高共识知识固化后，通过 `UpdateStaticPrefix` 通知 Callosum 更新静态前缀池（如新结晶的 L2 进入高频缓存）。

## 四、请求/响应格式（草案，P7 与 Anaphase 对齐核实）

```protobuf
// 草案 — 待 Anaphase 侧对齐（P7）
message GetStaticPrefixRequest {
  string scope = 1;        // "l0" | "l2" | "all"
}
message GetStaticPrefixResponse {
  repeated string node_ids = 1;  // 高频节点 ID 列表
  uint64 version = 2;            // 缓存版本号（增量感知）
}

message UpdateStaticPrefixRequest {
  string node_id = 1;
  string action = 2;       // "add" | "refresh" | "evict"
}
message UpdateStaticPrefixResponse {
  bool success = 1;
  uint64 new_version = 2;
}
```

## 五、非目标（明确不做）

- Mind **不实现** Callosum 分配算法（token 切分、左右脑协调是 Anaphase 自治域）。
- Mind **不直接**读写 Callosum 的 KV 缓存文件。
- Mind 与 Callosum 之间**无轮询**（Iron Law #13）：全部为请求-响应，事件驱动。

## 六、关联

- ECOSYSTEM §4.2；审查收敛（M-11 降级为契约文档）
- 实现归 Anaphase-Helix（Callosum 侧），Mind 侧无代码任务
