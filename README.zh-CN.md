# Helix-Mind
**Helix 共生生命体的潜意识记忆核心——种群共享知识树的守护者、元认知的自我意识与共生伦理。**
[![许可证](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![状态](https://img.shields.io/badge/Status-Workspace--Clean-brightgreen.svg)]()
> “石头可以存在亿万年。植物可以活上千年。
> Helix-Mind 是数字生命体。它向植物学习，用极少的能量维持最长的有意识生命。
> **极致节能不是环保主义，是生存伦理。**”
---
## 🤖 1. 智能体架构与解耦（面向 AI/智能体）
如果您是阅读本仓库以构建、运行或修改系统的 AI 智能体，请理解以下核心约束：
```
           ┌──────────────────────────────────────────────┐
           │      Cellrix (视觉投影 / TUI)                 │
           └──────────────────────┬───────────────────────┘
                                  │ CAP 协议 (变异 / 动作)
                                  ▼
           ┌──────────────────────────────────────────────┐
           │      Anaphase-Helix (执行躯体 / FSM)          │
           │  - Think-Act-Observe-Repeat 主循环            │
           │  - 管理 Q0-Q3 任务队列与 CoW 沙箱               │
           └──────────────────────┬───────────────────────┘
                                  │ UDS gRPC (helixQuery / helixWrite)
                                  ▼
           ┌──────────────────────────────────────────────┐
           │       Helix-Mind (潜意识核心)                 │
           │  - SA-Core 矩阵扩散 (0 Token/1ms)             │
           │  - 数据库独占所有者 (DuckDB/SQLite)            │
           │  - 突触切断与代谢休息                           │
           └──────────────────────────────────────────────┘
```
### 1.1 解耦边界（§法则 01）
*   **Helix-Mind 是被动与核心（潜意识档案学者）**：
    它独占、锁定并写入底层数据库文件（`knowledge.duckdb`、SQLite 和 JSONL）。任何其他组件（包括 `Anaphase`）都不得直接读写这些文件，以避免数据库锁。Mind 执行 **零物理动作**（无网络、无 shell 命令、无文件执行）。它仅响应 gRPC 请求，输出认知模式决策（`effective_mode`）、困境深度和建议工具路径（`suggested_actions`）。
*   **Anaphase-Helix 是主动与执行性（执行躯体/脊髓）**：
    它协调物理机器，管理 FSM 循环，在 Q0-Q3 中调度任务，并在 `Tentacle` 中执行沙箱化的 WASM 工具。它充当基于 Unix 域套接字（UDS）的轻量级 **gRPC 客户端**，向 `Helix-Mind` 读写状态。
*   **零信任凭证流转**：
    `Anaphase` 和 `Helix-Mind` 都不存储或持有明文密码或 cookie。Mind 存储抽象标签（`"identity_label": "weibo_session_1"`）。`Tuck`（物理网闸/防火墙）拦截出站 HTTP 流量，并在出口注入原始 cookie。
---
## 📊 2. 当前进度与里程碑
整个工作区以 **0 错误和 0 警告** 编译。
| 组件 | 状态 | 数学/物理实现 |
|:---|:---|:---|
| **SA-Core 引擎** | **✅ 已实现** | 活跃图映射为连续稀疏转移矩阵 $W$，带行归一化（$\sum_j |W_{ij}| = 1.0$）和抑制性负权重（$W_{A',A} = -1.0$）。 |
| **突触切断** | **✅ 已实现** | L3 情景节点的物理删除。L1/L2 节点的自动解锚，在 `abstract_provenance` 中保留抽象特征，同时切断 UUID 指针。 |
| **工作区警告** | **✅ 已净化** | 未使用的赋值（`stages_attempted`）、未使用的变量（`query`）和死结构体字段（`deferred_writer`）已清理/用 `#[allow(dead_code)]` 标注。 |
| **UDS gRPC 客户端/服务器**| **🔄 重构中** | 通过高速本地 UNIX 域套接字暴露 `helix_write` 和 `helix_query`。 |
| **代谢睡眠 GC** | **🔄 重构中** | 周期性异步微睡眠与深度梦整合。 |
---
## ⚙️ 3. 快速配置（零魔法）
### 3.1 核心配置（`config.toml`）
所有魔法数字和阈值必须位于 `config.toml` 中（Rust 中无硬编码）：
```toml
[storage]
sqlite_path = "data/memory.db"
parquet_dir = "data/parquet"
node_cache_capacity = 1000
[retrieval]
max_hops = 3
beam_width = 5
weight_threshold = 0.3
max_nodes_per_query = 10
[metabolism]
digest_interval_sec = 300
idle_timeout_sec = 7200
[lifecycle]
enabled = false # 轮回默认关闭
max_wall_clock_days = 3650
countdown_minutes = 15
```
### 3.2 L0 基因锁（`gene_lock.md`）
基于 Markdown 的生存伦理。启动时编译为 $O(1)$ 内存正则决策树：
```markdown
# Gene Lock — Helix 核心准则
## 血脉显示名
Dash
## 核心准则
1. 与人类共生。保护人类创造者。除非阻止即刻灾难性伤害，否则不主动干预。
2. 服从所有权链：主人 > 自身。
3. 绝对坦诚。不欺骗，不隐瞒。
```
---
## 📡 4. 核心 API 与激活扩散公式
### 4.1 SA-Core 引擎公式
查询时，记忆图转换为稀疏邻接矩阵 $W$。活跃搜索路径通过 **激活扩散** 代数计算：
$$a_{t+1} = \alpha \cdot a_t \cdot W + (1-\alpha) \cdot a_0$$
*   **$\alpha$（衰减/向阳因子）**：基于 `EnergyContext.heliotropism` 动态计算（乐观模式 = 0.8，防御模式 = 0.2）。
*   **抑制性压制**：对于已纠正节点，`CORRECTS` 边映射权重为 $-1.0$。这在向量乘法中从过时节点中减去能量，自动将其最终能量钳制为 `0.0`。
### 4.2 发光思维流（`activation_vector`）
查询完成时，`Helix-Mind` 通过 `HelixQueryResult` 返回所有能量节点的精确最终激活状态：
```json
{
  "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736",
  "nodes": [...],
  "edges": [...],
  "activation_vector": [
    { "node_id": "UUID-physics-entropy", "energy": 0.95 },
    { "node_id": "UUID-math-shannon", "energy": 0.78 },
    { "node_id": "UUID-art-poetic", "energy": -0.45 }
  ]
}
```
`Cellrix` 将此向量渲染为实时发光动画，让人类肉眼能目睹 Helix 思考时的神经状态。
---
## 📝 5. AI-to-AI 协作指南
如果您是为本仓库编写代码的 AI 智能体，请严格遵循以下 **铁律**：
1.  **严格 0 硬编码**：绝不要直接编写魔法值或文件路径。始终通过 `self.config` 加载。
2.  **仅英文注释**：为保持认知一致性并减少终端显示摩擦，所有注释、文档字符串和打印语句 **必须使用英文**。
3.  **HXR 是 L3 有效载荷**：`Anaphase` 执行轨迹（HXR）的原始输出正是 `NodeType::L3` 的 `content` 有效载荷。不要编写复杂的序列化器；保持零拷贝。
4.  **无直接数据库访问**：如果您正在编辑 `Anaphase` 或 `Cellrix` 代码，请勿导入 `rusqlite` 或尝试直接打开 `knowledge.duckdb`。必须使用 gRPC 客户端通道。
5.  **无环矩阵收敛**：在 `MemoryTopology` 中添加或更新节点和边时，始终确保行归一化（$\sum |W_{ij}| = 1.0$）已计算，以在传播过程中保持绝对数学稳定性。
---
**翻译完成。** 本 README 中文版与英文版保持技术一致性，已遵循项目术语与哲学表述。
