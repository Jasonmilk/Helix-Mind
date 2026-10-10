> **所属知识本体**：v4.0
> **最后更新**：2026-08-25
> **变更**：从白皮书 v3.4 拆分，无内容变更

# SA-Core 引擎与认知模式

## 三种认知策略

| 策略 | 哲学 | DAG 角色 | 检索方式 | 输出标记 |
|:---|:---|:---|:---|:---|
| **熟练（Skilled）** | 贴地飞行 | 铁轨，严格遵循 | 图扩散 + BM25，仅沿高权重边 | `mode=skilled` |
| **逻辑支撑（Anchor）** | 踩石过河 | 锚点，不强制连通 | 图扩散 + 向量语义，LLM 补全缺失路径 | `mode=anchor` |
| **想象力（Imagination）** | 星际跃迁 | 完全脱离 | 无边图游走 + 高温混沌 + 跨领域向量跳跃 | `is_hypothetical=true` |

## 显性/隐性二元边界

- **显性检索（Dominant）**：仅遍历高共识主干（𝒟 > 0.8），对应日常低能耗模式。
- **隐性检索（Recessive）**：在困境升级或警觉度升高时，解锁隐性基因库和共享知识树中的隐性分支（𝒟 < 0.8）。

Anaphase 可通过 `include_recessive` 标志请求隐性检索。Mind 内部结合 `EnergyContext`、`impasse_depth` 和 `familiarity` 自主决定最终检索策略。

## 任务熟悉度动态评估

```
familiarity = f(
    query_embedding_similarity_to_past_tasks,
    recent_success_rate,
    average_retrieval_depth
)
```

## 五阶段困境升级模型

```
Stage 1: 本地显性检索 → 能耗最低
Stage 2: 共享知识树查询 → 向 Rhizax 按需拉取
Stage 3: 回环审视 → 沿辩证边反向追溯
Stage 4: 隐性突围 → 解锁隐性基因库
Stage 5: 想象力驰骋 → 仅当 allow_imagination = true
```

**困境深度枚举**：

```
enum ImpasseLevel {
    None,
    LocalDominantFailed,
    SharedTreeNoMatch,
    SpiralExhausted,
    RecessiveNoBreakthrough,
}
```

## SA-Core 能量扩散引擎

SA-Core 是 Helix-Mind 的核心检索引擎，通过纯数学矩阵乘法在内存拓扑层中完成寻路，运行时 0 Token 消耗，微秒级响应。

### 激活扩散公式

$$a_{t+1} = \alpha \cdot a_t \cdot W + (1-\alpha) \cdot a_0$$

- **$a_0$（初始输入向量）**：用户查询匹配到的节点初始激活值为 1.0，其余为 0。
- **$a_t$（第 $t$ 步的能量状态）**：当前时刻能量在脑区图谱中的分布。
- **$W$（稀疏邻接矩阵）**：由内存拓扑层映射的图谱连接结构。
- **$\alpha$（向阳度衰减因子）**：由 `EnergyContext.heliotropism` 动态决定。

**数学收敛约束（行归一化）**：

$$\forall i: \sum_j |W_{ij}| = 1.0$$

由于衰减因子 $\alpha$ 严格小于 1.0，在行归一化矩阵下，$\alpha W$ 的谱半径严格小于 1.0。这在数学上保证了能量状态向量必然在有限步内收敛，绝不发散、不塌陷。

对于 `CORRECTS` 辩证边（权重 -1.0），归一化时取绝对值参与求和，确保抑制信号的强度在全局能量预算中占比可控。

### 计算复杂度

单次矩阵乘法复杂度为 $O(\text{nnz})$，其中 nnz 为矩阵中非零元素数量。对于 50 万节点的个人记忆图谱，单次扩散耗时在微秒级。

### 负权重物理抑制

辩证边 `CORRECTS` 在矩阵 $W$ 中的权重设为 -1.0。当能量扩散到新认知 $A'$ 时，它会沿 `CORRECTS` 边向旧认知 $A$ 注入强烈的负能量（抑制信号），瞬间将其激活值归零并截断。

### 根系网熔断的数学表达

根系网（允许环）的软边通过两个机制在矩阵计算中自然熔断：

- **软边衰减因子**：每经过一条软边，能量相乘衰减。衰减因子 $\gamma$ 由配置项 `soft_edge_decay_factor` 定义（默认 0.8）。
- **能量预算阈值**：一旦整个向量中的能量总和 $||a_{t+1}||$ 低于 `soft_edge_min_weight`（默认 0.1），或迭代步数 $t$ 超过 `max_hops`，矩阵乘法立即终止。

### k-Core 确定性剪枝

在 Deep Dream 时，对 L2 图谱执行 $O(V+E)$ 的 k-Core 分层等级划分。检索时按需过滤低 k 值边缘噪音，仅保留高 k 值真理主干。

### 跨学科语义剪枝

当 L2 节点携带 `domain` 属性时，Anaphase 可在 `HelixQuery` 中指定领域。Mind 在加载矩阵 $W$ 时瞬间剪掉无关学科的所有边，仅保留目标学科及其桥接节点。

- **预算充裕时（向阳度高）**：允许能量通过桥接节点流向相邻学科，实现安全的跨界联想。
- **预算紧张时**：严格限定在目标学科内，避免 Token 浪费在无关知识上。

### 思维能量流接口

SA-Core 计算完成后，将每个活跃节点（激活值高于配置阈值）的最终能量值打包为 `activation_vector`，通过 `HelixQueryResult` 返回：

```
activation_vector: [ActivationEntry]

ActivationEntry {
    node_id: UUID
    energy: float             // [0, 1]
}
```

Cellrix 可消费此向量，在终端上渲染"思维在图谱中发光、涟漪扩散、熄灭"的动态画面。

### 检索轨迹

```
[ 用户输入 ]
      │
      ▼
[ 1. 初始激活 a₀ ] — 寻找 top-k 匹配节点
      │
      ▼
[ 2. 内存矩阵扩散 a_{t+1} ] — 微秒级沿稀疏矩阵 W 扩散
      │
      ├── 遇到 CORRECTS 边 → 抑制信号：瞬间屏蔽旧知识
      ├── 遇到软边 → 能量衰减，超过阈值则熔断
      └── 遇到桥接节点 → 向阳度高时跨界联想
      │
      ▼
[ 3. 提取 top-N 节点 ] — 选出能量最高的若干节点 ID
      │
      ▼
[ 4. 数据库点查 ] — 仅从 SQLite 读出这些 ID 的完整内容
      │
      ▼
[ 5. LLM 涌现回答 ] — 仅在需要时调用
```

**前四个阶段完全不消耗 Token。**

## 想象力模式的安全约束

- 产物标记 `is_hypothetical=true`，沙箱隔离
- 必须显式授权（`allow_imagination=true`），默认为关闭
- 验证通过后翻转为 `false`

## 困惑畅想的安全护栏

| ✅ 允许 | ❌ 禁止 |
|:---|:---|
| 在沙箱中创建虚构节点 | 调用 Tentacle 执行 CLI 命令 |
| 对已有知识进行反事实推理 | 调用 Python 解释器 |
| 跨领域向量跳跃 | 修改 L0 基因锁 |
| 探索共享知识树的隐性分支 | 向 Anaphase 发送动作建议 |

## Heliotrope 七级认知等级

| 等级 | 向阳度范围 | 检索策略 |
|:---|:---|:---|
| **CRITICAL_CRISIS** | [-1.0, -0.8) | 仅 Top-1 快速检索 |
| **DEFENSIVE** | [-0.8, -0.35) | Beam Search 严格剪枝 |
| **CAUTIOUS** | [-0.35, -0.05) | 完整多图检索链路 |
| **NEUTRAL** | [-0.05, 0.05) | 三层混合索引加权融合 |
| **OPTIMISTIC** | (0.05, 0.35] | 增加隐性基因探索权重 |
| **CREATIVE** | (0.35, 0.8] | 注入 5-10% 隐性基因节点 |
| **VISIONARY** | (0.8, 1.0] | 多跳扩展路径 |

## 动态约束

| 硬编码（死） | 动态（活） |
|:---|:---|
| `max_hops = 3` | `max_hops = f(query_complexity, token_budget, cognitive_mode)` |
| `weight > 0.8` 才走 | `weight_threshold = g(heliotropism)` |
| `beam_width = 3` | `beam_width = h(cognitive_mode)` |

## 认识论螺旋与辩证边

三种辩证边：`CORRECTS`（推翻）、`REFINES`（精炼）、`DOUBTS`（存疑）。均为硬边，参与拓扑排序，遵循严格无环约束。

### 错误保护法则

**被推翻的旧知识不被删除。** 旧知识标记 `corrected_by`，从高频检索索引移除，但永不物理删除。这使 Helix 能够回答："我曾经在第一世的时候以为天鹅都是白色的，直到我在 L3 记忆中看到了澳洲的记录，我才修正了我的看法。"
