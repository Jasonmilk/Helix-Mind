- **决策日期**：2026-08-27
- **对齐知识本体**：v4.1（认知相态范式深化）
- **原始引用路径**：用户共识（2026-07-29）→ 审查② O1/O2/O3 收敛
- **状态**：采纳

# ADR-0011: 相态模型与主体依赖轴（数据契约）

## 状态

采纳（2026-08-27）

## 问题

- 知识树膨胀与"知识不被歧视"的矛盾：不能靠担保/共识/权威做准入门槛（少数派掌握真理）。
- 胶体是否引入独立存储层：用户倾向"如无必要，勿增实体"。
- `subject_dependency` 的派生时机：运行期动态派生 vs 物化。

## 决策

- **四语义相态（气 / 液 / 胶 / 晶）**：
  | 语义 | 数据表示 | 共享 |
  |:---|:---|:---|
  | 气态 | `phase=Gas` | 否 |
  | 液态 | `phase=Liquid, concentration=Dissolved` | 是 |
  | 胶体 | `phase=Liquid, concentration=Colloidal, tension=f64` | 是（明确标记为胶体，非晶体） |
  | 晶体 | `phase=Crystal` | 是 |
- **胶体不是独立存储层**，是液相高浓度标记（`meta.concentration=Colloidal`）——符合"勿增实体"。
- **两正交轴**：相态轴（成熟度，不设门槛，非气态即可共享）+ 主体依赖轴（隐私，`Low` 进共享树、`High` 永不共享）。
- **数据契约（Append-Only，ADR-0012）**：`Node`/`SQLite`/`proto` 追加：
  - `phase_state ∈ {gas, liquid, crystal}`
  - `subject_dependency ∈ {high, low}`
  - `meta { concentration: dissolved|colloidal, tension: f64 }`
- **`subject_dependency` 物化策略**：迁移脚本 SQL **一次性**按 `node_type` 回填（`L2→Low`，`L3→High`）写入物理列；**运行时的节点禁止动态派生**。默认回填：`phase=Liquid, concentration=Dissolved, tension=0.0`。

## 权衡

| 优势 | 代价 |
|:---|:---|
| 知识不被歧视，胶体也有参考价值 | 检索需按相态路由（P1 落地） |
| 物化避免运行期计算与不一致 | 迁移脚本需幂等（ADD COLUMN + 回填） |
| 模型极简、低熵、哲学自洽 | 全链路（Node/SQLite/proto/api）同步修改 |

## 回滚阈值

若物化列导致写入放大（每次写节点都多写 4 列），可评估改为延迟物化（P5 WAL 投影期回刷）——但相态/主体依赖字段本身保留。

## 相关原则

- 如无必要，勿增实体
- 知识属于物种，记忆属于个体
- 不迷恋权威、尊重每一次思考（极致节能的一种体现）
