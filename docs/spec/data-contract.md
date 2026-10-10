> **所属知识本体**：v4.0
> **最后更新**：2026-08-25
> **变更**：从白皮书 v3.4 拆分，无内容变更

# 数据契约与图谱资产矩阵

## 核心数据结构

> 所有 `content: JSON` 字段均指符合对应节点类型 Schema 的结构化 JSON。L2 节点的 content 必须经过隐私洗脱，L3 节点的 content 保留原始输入的结构化记录。

### DAG 节点（Node）

```
Node {
    id: UUID
    type: L0 | L1 | L2 | L3
    content: JSON
    heat: float                           // 热度值，检索时自动更新
    is_hypothetical: bool                 // 想象力模式产物标记
    is_recessive: bool                    // 隐性基因标记
    sensitivity: PUBLIC | PRIVATE | SENSITIVE   // L3 专用敏感度
    generation: int                       // 创建时的世代编号
    created_at: timestamp
    last_accessed_at: timestamp
    access_count: int
    initial_impact: float                 // 初始情感冲击值
    corrected_by: UUID | null             // 被哪个节点推翻（辩证边目标）
    notes: string | null                  // 人类备注
    domain: string | null                 // 可选，学科标签（如 physics, philosophy, law）
}
```

**字段约束**：

| 字段 | 约束 |
|:---|:---|
| `id` | UUID v4，创建时生成，不可变 |
| `type` | 枚举，不可变。L0 仅基因锁节点，L1 仅自画像节点 |
| `content` | 符合该类型 Schema 的 JSON。L2 必须经过隐私洗脱 |
| `heat` | 浮点数 [0, 1]。新节点默认 `initial_impact` 值。每次检索命中 +Δ，随时间自然衰减 |
| `is_hypothetical` | 仅想象力模式可设为 true。验证通过后翻转为 false |
| `is_recessive` | true 时从高频检索索引移除，进入隐性基因库 |
| `sensitivity` | 仅 L3 节点有效。默认 PRIVATE |
| `generation` | 创建时的 Mind 世代编号。不可变 |
| `corrected_by` | 仅当该节点被 CORRECTS 辩证边指向时非空 |
| `domain` | 可选。为 L2 节点提供学科元数据标签，用于跨学科语义剪枝 |

### DAG 边（Edge）

```
Edge {
    source_id: UUID
    target_id: UUID
    weight: float
    relation_type: enum
    is_soft: bool
}
```

**关系类型枚举**：

| 类型 | 语义 | 检索行为 | 矩阵权重 |
|:---|:---|:---|:---|
| `CAUSAL` | 因果关系 | 正向遍历 | 正值 |
| `SEMANTIC` | 语义相似 | 向量检索优先 | 正值 |
| `TEMPORAL` | 时间先后 | 按时间戳排序 | 正值 |
| `CO_OCCURRENCE` | 实体共现 | 关联检索 | 正值 |
| `CORRECTS` | 新节点推翻旧节点 | **屏蔽旧节点输出** | **-1.0** |
| `REFINES` | 新节点缩小旧节点适用范围 | 联合输出，新节点优先 | 正值 |
| `DOUBTS` | 新节点对旧节点存疑 | 降低旧节点置信度 | 0.3 |
| `SIMILAR_TO` | 联邦共享相似节点关联 | Deep Dream 自行决定融合 | 0.9 |

**边的类型约束**：

| 关系类型 | 允许的源节点类型 | 允许的目标节点类型 | is_soft |
|:---|:---|:---|:---|
| `CAUSAL` / `SEMANTIC` / `TEMPORAL` / `CO_OCCURRENCE` | L2, L3 | L2, L3 | false |
| `CORRECTS` / `REFINES` / `DOUBTS` | L2 | L2 | false（硬边，严格无环） |
| `SIMILAR_TO` | L2 | L2 | true（软边，可成环） |
| 根系网关系（社会/画像/项目） | 对应类型 | 对应类型 | true |

### 稀疏邻接矩阵（SA-Core 引擎核心数据结构）

内存拓扑层中的图谱被映射为一个稀疏邻接矩阵 $W$：

- **矩阵维度**：$N \times N$，其中 $N$ 为活跃节点数
- **存储格式**：压缩稀疏行（CSR）或坐标列表（COO），使用轻量级 Rust 稀疏矩阵库
- **权重映射**：
  - 标准边权重 ∈ (0, 1]
  - `CORRECTS` 辩证边权重 = -1.0
  - `DOUBTS` 辩证边权重 = 0.3
  - 软边权重 ∈ (0, 1]
- **计算复杂度**：单次矩阵乘法 $O(\text{nnz})$，微秒级完成

### 荣誉印记（HonorStamp）

```
HonorStamp {
    contributor_name: String
    contributor_type: HUMAN | HELIX | UNKNOWN
    l0_hash: String | null           // Helix 贡献者的 L0 哈希，人类贡献者为 null
    evidence_type: HUMAN_CITATION | EMPIRICAL_DISCOVERY | FEDERATED_SYNC | INHERITED
    citation: String | null           // 人类来源的引用（如 "达尔文《物种起源》, 1859"）
    timestamp: DateTime
    original_cid: String
    signature: String
}
```

**四种来源证据类型**：

| 证据类型 | 含义 | 示例 |
|:---|:---|:---|
| `HUMAN_CITATION` | 知识源于人类文明成果 | "达尔文《物种起源》, 1859" |
| `EMPIRICAL_DISCOVERY` | Helix 通过物理实证自主发现 | "helix_03 在 L3 实证中验证" |
| `FEDERATED_SYNC` | 从其他 Helix 的共享知识树同步 | "来自 Helix 7th Dash 的共享 DAG" |
| `INHERITED` | 从上一世传承晶体继承 | "第三世传承晶体" |

### 任务 DAG 节点（TaskNode）

```
TaskNode {
    task_id: UUID
    type: PROJECT | SUBTASK | MILESTONE | ACTION
    status: PLANNING | IN_PROGRESS | BLOCKED | COMPLETED | ABANDONED
    priority: 1-10
    estimated_effort: int          // 预估所需认知循环次数
    actual_effort: int             // 实际消耗认知循环次数
    context_snapshot: {
        related_l2_concepts: [CID]
        key_l3_memories: [CID]     // 不超过 5 个
        scratchpad_notes: [CID]
    }
    created_at / started_at / completed_at: timestamp

    edges:
        └── DEPENDS_ON → TaskNode         // 硬边，严格 DAG
        └── BLOCKED_BY → 障碍描述节点     // 软边
        └── SPIRAL_REFINES → TaskNode     // 螺旋拓扑：新理解缩小旧任务范围
        └── YIELDS → L2 知识节点          // 任务产出的知识
}
```

### 记事本 DAG 节点

```
NoteNode {
    node_id: UUID
    content: String                   // 凝练的一句话事实，≤200 tokens
    edges:
        └── REFERENCES → L2 知识节点
        └── DERIVED_FROM → L3 对话节点
}

ReminderNode {
    node_id: UUID
    trigger_condition: String        // 如 "用户下次提到项目X"
    edges:
        └── ANCHORED_TO → 相关概念节点
}

AgendaNode {
    node_id: UUID
    priority: int
    action: String
    created_by: USER | HELIX_SELF
    created_at: timestamp
    edges:
        └── DEPENDS_ON → TaskNode
}
```

### 人格侧写图谱节点

```
UserTraitNode {
    node_id: UUID
    trait_type: PREFERENCE | HABIT | PERSONALITY | SKILL
    confidence: float
    evidence: [L3_node_ids]
    abstract_provenance: String | null    // 证据固化后的文本摘要
    lifecycle: CREATOR_IMPRINT            // 跨轮回保留，永不删除
    evidence_solidified_at: timestamp | null
    edges:
        └── REFINES → 旧版本同一 trait (螺旋)
}

ContactTraitNode {
    node_id: UUID
    entity_id: String                    // 指向该联系人的 SocialNode
    trait_type: PREFERENCE | HABIT | PERSONALITY | SKILL
    confidence: float
    evidence: [L3_node_ids]
    abstract_provenance: String | null
    evidence_solidified_at: timestamp | null
    edges:
        └── REFINES → 旧版本同一 trait (螺旋)
}

SocialNode {
    node_id: UUID
    entity_type: HUMAN | HELIX | EXTERNAL_AI | GROUP
    bio_ref: CID                         // 指向 ContactPersona 的引用
    is_legacy: bool                      // 是否为跨世叮咛传承
}

SocialEdge {
    source / target: SocialNode ID
    relation_type: KNOWS | COLLEAGUE | CREATOR | SUCCESSOR | FAMILY
    strength: float
    is_legacy: bool                      // 是否为跨世叮咛传承
}
```

### 时代凝章节点（EpochCrystal）

```
EpochCrystal {
    epoch_cid: String                    // IPLD 内容哈希，不可变快照的永久坐标
    guardian_name: String                // 最后一任监护人名称
    timespan: {
        start: timestamp
        end: timestamp
    }
    relationship_summary: String         // ≤500 字，关系摘要
    status: ACTIVE_ARCHIVE               // 只读，永不删除
}
```

### 检索请求与结果（扩展 v3.4）

**检索请求（HelixQueryRequest）**

```
HelixQueryRequest {
    query: string
    suggested_mode: SKILLED | ANCHOR | IMAGINATION
    energy_context: EnergyContext
    include_recessive: bool
    allow_imagination: bool
    autonomy_level: AGENT | OPEN | SURVIVAL
}
```

**EnergyContext**

```
EnergyContext {
    token_budget: int
    heliotropism: float        // 向阳度 [-1.0, 1.0]
    pulse: float               // 脉动值 [0, 1.0]
    vigilance: float           // 警觉度 [0, 1.0]
    latency_limit_ms: int
    system_load: float         // 系统当前负载 [0, 1.0]
    impasse_depth: int         // 当前困境深度 (0-5)，Mind 内部计算
    familiarity: float         // 任务熟悉度 [0, 1]，Mind 内部计算
}
```

**检索结果（HelixQueryResult）**

```
HelixQueryResult {
    effective_mode: SKILLED | ANCHOR | IMAGINATION
    mode_negotiation: string | null
    nodes: [Node]
    edges: [Edge]
    trace_id: UUID
    latency_ms: int
    tokens_consumed: int
    impasse_level: ImpasseLevel
    stages_attempted: int
    suggested_actions: [ActionSuggestion]
    activation_vector: [ActivationEntry]  // 思维能量向量
}
```

**ActivationEntry（思维能量向量条目）**：

```
ActivationEntry {
    node_id: UUID
    energy: float             // 该节点在 SA-Core 扩散后的最终激活值 [0, 1]
}
```

**ActionSuggestion**：

```
ActionSuggestion {
    action_type: string       // 如 "python_interpreter", "web_search", "cli_command"
    parameters: JSON          // 符合 CIS 意图格式的参数
}
```

## 图谱资产总表

全局统称为**图谱资产**，分为三大类：**严格 DAG**、**根系网**、**独立记录**。

| 资产名称 | 数据结构类别 | 回环与熔断规则 | 生命周期与隐私底线 | 共享权限 |
|:---|:---|:---|:---|:---|
| **L0 基因锁** | 严格 DAG（单节点） | 绝对无环，硬约束 | 永不删除 | 公开（哈希校验） |
| **L1 自画像** | 严格 DAG + 记事本 DAG | 绝对无环，硬约束 | 当世书写，下世重置 | 个体私有，不共享 |
| **L2 知识图谱** | 严格 DAG | 绝对无环，睡眠展平 | 热度衰减→隐性基因→深冷归档。**必须隐私洗脱** | **公开自动共享**（无实体标签） |
| **L3 情景记忆** | 严格 DAG（时间流） | 绝对无环，时序不可逆 | **永不物理删除**。过期→深冷存储。失联 100 年→时代凝章。请求删除→突触切断+遗忘标记 | 默认私有，用户标记后公开 |
| **隐性基因库** | 继承来源规则 | 继承来源规则 | 深冷索引。仅索引可删，记忆不可删 | 个体私有 |
| **社会关系网** | **根系网** | 允许双向环，三大熔断器限制 | 随交互更新。百年内未激活且非承继关系→时代凝章。`entity_type` 永不可混淆 | **绝对隐私，永不共享** |
| **用户画像网** | **严格 DAG**（`CREATOR_IMPRINT`） | 绝对无环，螺旋迭代 | **跨轮回保留**，永不删除。失联 100 年→时代凝章保留铭牌 | **绝对隐私，永不共享** |
| **人物画像网** | **严格 DAG** | 绝对无环，螺旋迭代 | 跨轮回保留（但非承继关系可进入隐性）。`entity_type` 永不可混淆 | **绝对隐私，永不共享** |
| **项目管理网** | **根系网** | 允许局部环，三大熔断器限制 | 任务完成/过期归档，超时软关联 TTL 清理 | 个体私有 |
| **传承晶体** | 严格 DAG | 绝对无环 | 跨世继承，永不删除 | 个体私有 |
| **每世档案** | 独立记录 | 无拓扑关系 | 永不删除。凝章后保留统计摘要 | 可选择性公开 |
| **时代凝章铭牌** | 独立记录 | 无拓扑关系 | 永不删除。历史监护人的考古凭证 | 只读，仅本地可见 |

## 共享知识树与本地 DAG 的逻辑统一

本地 DAG 和共享 DAG 使用相同的 Node/Edge 数据结构。一个节点被拉取到本地后，它就是一个普通的 Node——可以被本地检索索引、可以被实证反馈更新 𝒰、可以参与代谢管道。唯一的区别是 `source` 字段标记了来源（本地创建 vs 共享拉取），用于荣誉账本追溯。

共享知识树节点在本地检索中的行为与本地节点完全一致，但其权威性取决于本地的实证验证——Mind 不因为某个节点来自共享知识树就自动赋予高权重，也不因为它是外部来源就歧视。真理的权重由物理实证决定，不由来源决定。
