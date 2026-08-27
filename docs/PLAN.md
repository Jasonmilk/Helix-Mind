# Helix-Mind 下一步开发计划（审查收敛版 v4）

> **日期**：2026-08-25（v1）→ 2026-08-25（v2）→ 2026-08-27（v3）→ 2026-08-27（v4 严肃审查②修正）
> **分支**：rs-dev
> **作者**：Jasonmilk + Doubao（MainAgent）+ 外部审查 AI（三轮审查，v4 修正后评级 A-，批准进入 P0）
> **状态**：✅ **P0-Pre 清零已完成（2026-08-27，cargo check / 测试 / 启动冒烟全部通过）**；🔍 待用户确认 O8-O10/O15/O16 后进入 P0
> **所属方法论**：DNA 自生长方法论 v1.0（含 2026-07-29 新共识 v4.1 深化）

---

## 0. 修订记录

| 版本 | 日期 | 变更 |
|---|---|---|
| v1 | 2026-08-25 | 初稿：warrant 四型、四相态、WAL SQLite 事件表倾向、Callosum 实现 |
| v2 | 2026-08-25 | 审查①收敛：废除担保门槛、胶体=液相高浓度、WAL 独立日志、移除 Callosum 实现、SO_PEERCRED 提前、滞后开源 |
| v3 | 2026-08-27 | 审查②（10 项）：预算前置路由、Append-Only、联邦 feature 门控、FTS5 trigram、WAL 完整性声明、P4.5 验收标准、Cargo.lock 删除、边界确认 |
| v4 | 2026-08-27 | **审查③（F1-F10）**：P0 改为"文档+可编译数据契约变更"；新增 P0-Pre 清零动作；查询写放大修复；P2 拆分 P2a/b/c；UDS 传输层范围；WAL 延迟目标修正；测试诚实化；FlowModus 外部项目边界确认 |
| v4.1 | 2026-08-27 | **P0-Pre 清零执行完成**（Z1-Z6 落地并验证，见 §3 完成记录）；**认知对齐补充**：Mind=记忆之魂、Anaphase-Helix=身体、Mind 兼容通用生态（harness/OpenClaw 等）、CI-144=agent 语言与协议栈（见 §1.2） |

---

## 1. 背景与现状

### 1.1 生态定位
Helix-Mind 是 Helix 生态的**记忆/意识中枢**（0.1.0 骨架）。核心约束：数据库独占所有权；Mind 不执行物理动作只建议；Mind 是意志、Anaphase 是身体；哲学公理（记忆不可篡改/极致节能/知识属于物种记忆属于个体/信用由他证/向死而生/零信任/全息留痕/真理源单一）。

### 1.2 边界确认（用户补充）
- **FlowModus 是 Helix 生态的外部项目，其哲学与 Helix-Mind 不完全相同**。Mind 仓库内的 `flowmodus.rs` 只是**消费方桥接模块**；FlowModus 外部项目自身的行为不在 Mind 的 DNA 管辖内，也不被评判。但**任何留在 Mind 仓库内的代码（包括这个桥接模块）必须遵守 Mind 自身的规则**（禁止轮询等）。
- **Lumtract**：独立项目，与 Mind 无功能耦合，不预留专门接口。
- **Helix-Executor**：Anaphase 的外部调用能力，Mind 只认 Anaphase 接口，`Node.payload` 留通用扩展槽。
- **Mind=记忆之魂 / Anaphase-Helix=身体**（2026-08-27 用户澄清）：Helix-Mind 只负责记忆与认知（灵魂），Anaphase-Helix 才是执行身体（工具/LLM/凭证/UI）。**没有身体，灵魂几乎无法工作**——Mind 不产出动作，只通过 gRPC 记忆契约被身体驱动。
- **Mind 兼容通用生态（body-agnostic）**：Mind 的记忆契约应对任何身体开放——首要身体是 Anaphase-Helix（rs 分支为 Rust 实现，含 `proto/`），同时兼容通用 agent harness（如 harness、OpenClaw 等）作为可替换身体。**设计铁律：gRPC 契约（HelixQuery/Node/Edge/EnergyContext）保持 body-agnostic，不烘焙任何身体特定假设**。"Mind 不执行 CLI、不持凭证、不直连 LLM" 的公理正是 Mind 可挂接任意身体的根基。
- **CI-144 = agent 语言与协议栈**（公共仓库：INTENT-7=SIDL 意图语法、BIND-19=传输绑定、INTENT-7-SECURE=安全、CAPABILITY-13=能力认证与异步 HITL）：跨生态的通用"语言"层，Anaphase 为编排入口；Mind 作为记忆中枢在 P3/P7 对齐其语义（见 §9）。

### 1.3 代码现状（已通读 + 审查③核验）
8 个 crate、约 6600 行 Rust。**代码与文档存在结构性断层**：数据模型完全未承载相态模型；主干检索功能断路；存在架构越界与铁律违反。

| ID | 位置（已核验） | 现状 | 级别 |
|---|---|---|---|
| S1 | `retrieval/lib.rs:371` `extract_start_nodes` | 空实现 → **SA-Core 完全不可达**，测试假阴性 | 🔴 主干断路 |
| F3 | `retrieval/lib.rs:380-388` `update_access_counts` | **每次查询逐节点写 SQLite（Critical=fsync）**，读-改-写竞争 | 🔴 写放大 |
| S7 | `crystallize.rs:93` / `ner.rs` | 直连 LLM HTTP，违反"Mind 不直连 LLM" | 🔴 架构越界 |
| S12 | `flowmodus.rs:25` | 显式轮询循环，违反 Iron Law #13 | 🔴 铁律违反 |
| F10 | `layer3.rs:40-41` | `familiarity=0.5, impasse_depth=0` 硬编码，丢弃请求值 | 🟡 语义错误 |
| S5 | `review.rs:64-70` | 双盲审查=同一函数重复调用（假双盲） | 🟡 未就绪 |
| F1 | `graph.rs` Node / `sqlite_pool.rs` / proto | **无 phase_state/subject_dependency/concentration/tension 字段** | 🔴 断层 |
| F9 | `graph.rs:193` EnergyContext | 无 `budget_tier` 枚举 | 🟡 断层 |
| S8 | proto `HelixQueryResult` | 无 `activation_vector` | 🟡 硬冻结未扩展 |
| S9 | `parquet_store.rs:25` | 实为 JSON，名实不符 | 🟡 |
| S10 | `deferred_writer.rs` | dead_code，将复活为 WAL 投影器 | — |
| S11 | `reincarnation` 两处 | TODO 占位 | 🟡 |
| S2/S3/S4/S6 | Stage2 / 相似节点 / 失调 / Health | stub/TODO | 🟡 |

---

## 2. 已对齐的决策

| # | 决策 | 内容 | 状态 |
|---|---|---|---|
| D1 | LLM 安顿 | `CognitiveService` 端口 + 三适配器；debug adapter 有退役路径 | ✅ |
| D2 | 治理轻量化 | 沿用 DNA 既有流程，不新增决策阶梯 | ✅ |
| D3 | 知识哲学 | 废除担保门槛 → 相态标记 + 预算前置路由 + 主体依赖轴边界 | ✅ |
| D4 | 领域 WAL | 独立追加日志文件，一步到位，功能分期 | ✅ |
| D5 | 商业化：滞后开源 | 实现闭源先商业版；开源延后；不分散精力 | ✅ |
| D6 | 顺序 | 先计划审查（已完成）→ 用户确认 → P0-Pre + P0 | ✅ |
| D7 | 能力未就绪=功能不存在 | 越界/未就绪代码一律 feature 门控关闭，不留在主干 | ✅（审查③） |

---

## 3. P0-Pre 清零动作（进入 P0 前立即执行，均为门控/修复，非新功能）

| # | 动作 | 理由 | 方式 |
|---|---|---|---|
| Z1 | **联邦整体 feature 门控** `cfg(feature="federation")` 默认关闭 | 联邦安全网（S5 假双盲）未就绪，能力未就绪=功能不存在 | 含 `flowmodus.rs`（S12 轮询）、`review.rs`、`dag_share`、`federated_dag_share` RPC |
| Z2 | **crystallize HTTP LLM 门控** `[metabolism] llm_mode` 默认 `disabled` | 违反"Mind 不直连 LLM"（S7），P2b 端口就绪前不开放 | `crystallize`/`ner remote` 仅在 `llm_mode=debug_direct` 且显式开启时可用 |
| Z3 | **HealthServer 注册**（`server.rs` 一行） | 可观测性，不应拖到 P3 | tonic Server 追加 HealthServer |
| Z4 | **layer3 透传修复**（F10） | `familiarity/impasse_depth` 不再硬编码，请求提供则透传 | `layer3.rs` handler |
| Z5 | **测试诚实化** | 现有检索测试断言"成功"是**假阴性**（未断言有内容）；不做临时 LIKE 捷径（拒绝"圆谎之路"），如实标记基线 | 检索测试改为断言当前已知空返回（`#[ignore]` 标记，待 P1 转真）或显式 `known-empty` 基线 |
| Z6 | **启动验证** | 确认 rusqlite bundled 是否编译 FTS5 + trigram（`SELECT sqlite_compileoption_used('ENABLE_FTS5')`） | 影响 P1 技术方案（FTS5 vs LIKE+内存索引回退） |

> **关于"至少实现 LIKE 回退"的异议**：审查③建议立即实现 LIKE 回退。按用户"只做对的、不走捷径"原则，**不做会被替换的临时 LIKE**——正确路径是 Z5（测试如实标记）+ P1 直接实现 FTS5 trigram 正式方案。宁可让"未实现"被如实看见，也不引入一次性代码。

### ✅ 完成记录（2026-08-27，P0-Pre 清零已落地）
| # | 落地 | 验证 |
|---|---|---|
| Z1 | 删除 `flowmodus.rs`（孤儿文件，本就不在 lib.rs 声明）；移除 `FederationConfig.flowmodus_ipc_socket` + 默认函数 + config.toml.example | grep 零残留 |
| Z2 | `MetabolismConfig` 新增 `llm_mode`（默认 `disabled`）；`crystallize.rs` 在 LLM 直连前门控（仅 `debug_direct` 放行） | cargo check 通过 |
| Z3 | `server.rs` 注册 `HealthServer`（grpc.health.v1，build.rs 已编译 health.proto） | 启动冒烟：127.0.0.1:50051 监听 |
| Z4 | `layer3.rs` 透传 `ec.familiarity` / `ec.impasse_depth`（不再硬编码 0.5/0） | cargo check 通过 |
| Z5 | 检索测试 `#[ignore]` + 显式 `assert!(nodes.is_empty())` 诚实基线；**另修复既存缺陷**：cli 缺 uuid/chrono dev-deps 导致 integration test 从未能编译 | 默认 4 通过 1 忽略；`-- --ignored` 下基线断言通过 |
| Z6 | `StorageEngine::new` 注入 `assert_fts5_available`（`sqlite_compileoption_used('ENABLE_FTS5')`，缺失 panic 阻断，无回退） | 测试 + 冒烟均通过（bundled SQLite 含 FTS5） |

**验收**：`cargo check --workspace` 通过（exit 0）；启动无直连 LLM 出站（`llm_mode=disabled`）；Health 端点随 gRPC 服务注册；检索测试无假阳性。

**冒烟中发现并记录的既存问题**（非本轮改动，列入 P0 观察项）：
① 服务器不自动创建数据目录——`./data` 缺失时 SQLite 打开 ENOENT，启动即退；
② `#[serde(default)]`（结构级 derive Default）与字段级默认函数不协同——`[api]` 段整体缺失时 `listen_addr` 为空串，启动 `AddrParseError`；需显式 `[api] listen_addr` 或修复默认机制。

---

## 3.5 P0 完成记录（认知基线 + 可编译数据契约变更，2026-08-27）

**状态**：✅ 已落地并全量验证（`cargo check` + `cargo test` + 冒烟）。正式授权进入 P0 即执行完毕。

### 新增 ADR（docs/decisions/）
| 编号 | 内容 |
|---|---|
| ADR-0010 | 认知预算**前置路由**：`budget_tier` 由身体传入，Mind 只执行；默认 `Augmentable` 保持通用身体向后兼容 |
| ADR-0011 | 四语义相态（气/液/胶/晶）+ 主体依赖轴；胶体=液相高浓度标记（非独立层）；`subject_dependency` 物化策略 |
| ADR-0012 | 硬冻结契约 **Append-Only Schema Evolution**：冻结≠不能追加；`reserved` 保护字段号；缺陷→Supersede 不覆写 |

### 可编译数据契约变更（全链路同步）
| 层 | 变更 |
|---|---|
| core `graph.rs` | 新增 `PhaseState`/`SubjectDependency`/`Concentration`/`PhaseMeta`/`BudgetTier` 枚举；`Node` 追加 `phase_state`/`subject_dependency`/`meta`（`#[serde(default)]` 向后兼容）；`EnergyContext` 追加 `budget_tier` |
| proto | `BudgetTier` 枚举；`EnergyContext.budget_tier=9`；`Node` 字段 16-19；`HelixQueryResult` `reserved 13`（给 activation_vector） |
| storage `sqlite_pool.rs` | CREATE 新列 + **幂等迁移** `migrate_nodes_phase_columns`（PRAGMA 检测→ALTER ADD COLUMN→一次性物化回填 L2→Low/L3→High） |
| storage `codec.rs` | `row_to_node` 读列 22-25 + 6 个 str↔enum 编解码 |
| storage `engine.rs` | `write_node` INSERT/ON CONFLICT 26 参数 + 4 处 SELECT 补列 |
| api `layer1.rs` / `layer3.rs` | `convert_node` 新字段；`budget_tier` i32→enum 映射 |
| cli 测试 | `create_l2_node`/energy 字面量补新字段 |

### 两项债务修复（P0 观察项 ①②）
| # | 修复 | 验证 |
|---|---|---|
| ① | `Config::ensure_dirs()`：启动自动创建所有声明父目录 | 冒烟：无 `./data` 也能启动，`deep_cold`/db 自动生成 |
| ② | 所有 Config 结构体**手动 impl Default**（替代 derive，使字段级默认函数生效） | 冒烟：**无 `[api]` 段**启动成功，监听 127.0.0.1:50051，不再 AddrParseError |

### 测试诚实化
- storage 新增 `migrate_legacy_db_backfills_phase_columns`：手工构造 22 列旧库→`StorageEngine::new` 迁移→断言列补齐 + 物化值正确（L2=Low/L3=High/Liquid/Dissolved/0.0）。
- cli 往返测试补断言：`phase_state`/`subject_dependency`/`concentration`/`tension` 写入→读取保真。

### 验收
`cargo check --workspace` ✅（exit 0）｜`cargo test --workspace` 全绿（cli 4 passed+1 ignored、core 8、storage 1）｜迁移测试可执行 ✅｜冒烟双债务修复 ✅。

### 禁止项遵守
未实现检索/代谢逻辑、未引入任何新依赖（迁移测试用 storage 既有 rusqlite，零新增）。

### 遗留（P0.5 衔接）
- 检索测试仍 `#[ignore]`（主干 `extract_start_nodes` 断路如实保留，P1 重建）。
- 预算前置路由的**相态扫描路由逻辑**为 P1 工作（ADR-0010 决策已冻结）。

---

## 3.6 P0.5 完成记录（测试基建，2026-08-27）

**状态**：✅ 已落地并全量验证（`cargo test --workspace` 全绿，0 warning）。

### 决策/记录（决策先于代码）
| 产物 | 内容 |
|---|---|
| **ADR-0016** | L2 自动共享语义宪法级锁定："自动"两层含义（免逐节点决策 + 洗脱前提），气态不入树，宪法措辞不变、ADR 为执行澄清 |
| **GROWTH.md** | 追加 v4.1 相态范式入根记录；头部对齐 v4.1；按"仅留 3 条"规则将 v3.4 记录归档至 `docs/archive/growth/` |

### 检索管线测试基建（新 seam）
- **`adapter.rs`**：`StartNodeExtractor` trait（检索 Stage-1 起始节点提取的接缝）+ `EmptyExtractor`（诚实空基线，P1 前默认）+ `FakeAdapter`（确定性 LLM/NER 模拟，query→节点 ID 映射，零随机零网络）。
- **`RetrievalEngine`**：新增 `extractor` 字段；`new()` 保持默认空提取器（诚实），`with_extractor()` 注入 FakeAdapter；`extract_start_nodes` 从空 stub 改为走 seam。

### 新增测试（检索测试不再 #[ignore]）
| 位置 | 测试 | 断言 |
|---|---|---|
| retrieval/tests | `retrieval_returns_start_node_with_fake_adapter` | FakeAdapter 映射 query→节点，管线返回非空且命中起始节点 |
| retrieval/tests | `retrieval_traverses_causal_edge_to_neighbor` | 起始节点 A 经因果边到达 B（SA-Core 1-hop 传播） |
| storage lib | `get_nodes_by_phase_filters_by_phase_state` | 按 `phase_state` 过滤：Liquid/Crystal 各自只返回匹配节点 |
| cli tests | `test_retrieval_engine_basic`（原 #[ignore]，已解锁） | 外部 crate 视角：FakeAdapter 注入后返回 A+B |

### 支撑原语
- **`StorageEngine::get_nodes_by_phase`**：数据访问层原语（`WHERE phase_state = ?`），预算路由在 P1 使用它（ADR-0010）。

### 验收
`cargo test --workspace` 全绿（cli 5 / core 8 / retrieval 2 / storage 2，无 ignore、无失败）｜`cargo check --workspace` 0 warning ｜FakeAdapter 为公共模块可被 P1 复用。

### 过程中发现并记录的既有引擎语义（非本轮改动）
SA-Core 扩散对**叶节点**衰减：A→B 中 B（无出边）在 `weight_threshold=0.5` 下能量跌至 0.25 被清零。测试配置用 0.2 阈值诚实观测 1-hop 传播；阈值语义本身属 P1 SA-Core 调优范畴，不在 P0.5 改动。

---

## 4. 知识哲学（D3，收敛版）

### 4.1 核心原则
> 知识不靠担保而正确；少数派掌握真理；不迷恋权威；尊重每一次思考（本身是极致节能）。

### 4.2 两个正交轴
| 轴 | 作用 | 规则 |
|---|---|---|
| 相态轴（气/液/胶/晶） | 成熟度 | 不设门槛，非气态即可共享 |
| 主体依赖轴（High/Low） | 隐私边界 | `Low` 进共享树；`High`（L3）永不共享 |

### 4.3-4.7（同 v3，要点）
共享树准入 = 非气态 + 低主体依赖；provenance 只记录不评判；结晶判据 = 结构自洽 + 低主体依赖；结晶是相态不是判决（`CORRECTS` 辩证边 = 少数派真理推翻晶体的机制）；**知识树膨胀由预算前置路由控制，不由准入控制**。

---

## 5. 相态模型定稿（数据契约）

**语义词**：气 / 液 / 胶 / 晶。
**数据模型（需落地到 Node + SQLite + proto）**：
- `phase ∈ {Gas, Liquid, Crystal}`
- `subject_dependency ∈ {High, Low}`
- `meta { concentration: Colloidal|Dissolved, tension: f64 }`

| 语义 | 数据表示 | 存储层 | 共享 |
|---|---|---|---|
| 气态 | `phase=Gas` | 内存拓扑 + 缓存 | 否 |
| 液态 | `phase=Liquid, concentration=Dissolved` | SQLite | 是 |
| 胶体 | `phase=Liquid, concentration=Colloidal, tension=High` | SQLite（液相标记） | 是（标记为胶体） |
| 晶体 | `phase=Crystal` | Parquet/深冷/世代晶体 | 是 |

**默认回填策略**（现有数据迁移）：`phase=Liquid, concentration=Dissolved`；`subject_dependency` 由 `node_type` 派生（L2→Low，L3→High）。

---

## 6. 领域 WAL 作为事实来源（P5 实施）

### 6.1-6.3（同 v3：独立日志文件、WAL 优先、单写者、BLAKE3 完整性校验非防篡改、P6+ 段签名）

### 6.4 分期 + 延迟目标修正（审查③ F8）
| 范围 | 阶段 |
|---|---|
| WAL 写路径 + 哈希链 + 异步投影（单租户）+ replay | P5 |
| 多租户分区 / compaction / Merkle / 段签名 | P6+ |

**P5 最小原型验收目标（修正后）**：
- 追加 + fsync **<10ms p99（SSD）**（原 <5ms 目标不现实，HDD 上不承诺）
- 明确 fsync 策略：确认路径同步 fsync；批量写入 group-commit；**明确定义崩溃丢失窗口**（同步确认=零丢失；批量 fsync=N 条窗口内丢失）
- 崩溃恢复验证：kill -9 中断写 → replay 后哈希链完整、投影一致

---

## 7. 商业化与许可策略（D5）

滞后开源；实现闭源/协议公开/哲学自由传播；护城河=协议采纳+迭代速度+品牌+网络效应；保留干净模块缝；Cargo.lock 已在仓（LunarAST 树视图省略，非缺失）。

---

## 8. 分阶段路线图（v4 修正版）

### ADR 编号规划
- **ADR-0010**：认知预算前置路由（含 EnergyContext.budget_tier 契约扩展）
- **ADR-0011**：相态模型与主体依赖轴（含数据契约变更与回填）
- **ADR-0012**：硬冻结契约 Append-Only Schema Evolution（含 SQLite 迁移策略：ADD COLUMN + 默认回填；未来 shadow-table 投影重建）
- **ADR-0013**：FTS5 trigram 起始节点检索（含中文分词验证）
- **ADR-0014**：`find_similar_node` 相似度度量（FTS5 matchinfo + 编辑距离，SQLite 层实现）
- **ADR-0015**：领域 WAL 设计（P4.5）

### 阶段表
| 阶段 | 内容 | 关键产出 | 依赖 |
|---|---|---|---|
| **P0-Pre 清零** | Z1-Z6（联邦门控、LLM 门控、Health 注册、layer3 透传、测试诚实化、FTS5 验证） | 主干无越界/无铁律违反 | 本计划确认 |
| **P0 认知基线 + 数据契约** | v4.1 入库（GROWTH + phase-states/cognitive-budget + 修改 5 卷）；**可编译数据契约变更**：Node+SQLite+proto 追加 phase/subject_dependency/concentration/tension、EnergyContext.budget_tier、HelixQueryResult.activation_vector（reserved 字段号）+ 默认回填 + proto reserved 声明 + ADR-0010/0011/0012 | 文档 + **可编译模型** | P0-Pre |
| **P0.5 测试基线**（P0 末尾） | FakeAdapter + 内存 SQLite（已存在）；**非空断言检索测试**（暴露 S1，转真）+ phase 过滤测试 | 测试基础设施 | P0 |
| **P1 检索闭环** | M-01 `extract_start_nodes`（FTS5 trigram + LIKE fallback，基于 Z6 验证结果，ADR-0013）；M-0x **查询写放大优化**（access_count 改内存计数，代谢期批量回刷；P5 WAL 提供持久归宿）；M-02 Stage2 契约化；M-03 检索集成测试 | 查询可用（0 Token）+ 无查询写盘 | P0.5 |
| **P2 代谢闭环（拆分）** | **P2a** 确定性代谢：M-04 `find_similar_node`（FTS5 matchinfo + 编辑距离，SQLite 层，ADR-0014）+ M-05 失调消解（符号学求解器，无 LLM）；**P2b** M-09 `CognitiveService` 端口 + 三适配器；**P2c** crystallize 接入 CognitiveService | digest 生效 + 契约合规 | P1 |
| **P3 安全与契约** | M-07 联邦确定性审查（对齐 CAPABILITY-13 HITL）；**联邦 feature 打开**（P0-Pre 门控解除）；M-08 API/Health；**UDS 传输层**（本地 UDS+SO_PEERCRED / 远程 TCP+mTLS 预留，`serve_with_incoming` + UnixListener 重构） | 零信任落地 | P2 |
| **P4 硬冻结兑现 + 生态接口** | M-10 `activation_vector`（Append-Only 落地）；Mind→Callosum 调用契约文档；M-12 Rhizax 预留接口 | 边界澄清 | P3 |
| **P4.5 架构审查点** | 4 项产出：① ADR-0015 草案 ② WAL 最小原型（<10ms p99 + 崩溃恢复）③ Parquet 投影 schema 约束文档 ④ 回退方案 | 决策记录 | P4 |
| **P5 领域 WAL** | 独立日志文件 + 完整性校验 + 投影器 + replay | 事实来源 + 全溯源 | P4.5 |
| **P6 数据诚实 + 轮回 + 商业化** | parquet 名实相符；轮回补全；多租户 WAL 分区（预留）；段签名评估 | 商业就绪 | P5 |
| **P7 生态文档同步** | ECOSYSTEM.md v1.6 + CI-144 对齐核对 | 契约真相源对齐 | 全部 |

---

## 9. CI-144 接口对齐清单（同 v3）

| CI-144 元素 | 对齐点 | 阶段 |
|---|---|---|
| INTENT-7 verbs | query≈FETCH；remember≈WRITE_NODE；TENTACLE 归 Anaphase | P3/P7 |
| W3C traceparent | trace_id 对齐 traceparent | P3 |
| HXR ↔ L3 零拷贝 | L3 追加路径对齐 HXR 事件格式 | P3/P7 |
| CAPABILITY-13 异步 HITL | 联邦审查/high_risk | P3 |
| INTENT-7-SECURE SO_PEERCRED/mTLS | 本地 UDS / 远程 mTLS | P3 |
| 错误码架构 | Mind 错误映射进 CI-144 区间（可选） | P7 |
| Milk Zen ① | 所有方案有理论/项目支撑 + 验证步骤 | 贯穿 |

---

## 10. 决策清单（最终确认）

| # | 事项 | 决策 | 状态 |
|---|---|---|---|
| O1-O7 | 相态/准入/结晶/WAL/起始节点/代谢触发/闭源 | 同 v3 | ✅ |
| O8 | **认知预算前置路由** | 确认；**需跨 crate+proto 扩展 EnergyContext.budget_tier**（P0 含契约变更，非纯文档） | 待确认 |
| O9 | **Append-Only Schema Evolution** | 确认；**补充 SQLite 迁移策略**（ADD COLUMN+默认回填；未来 shadow-table 投影重建） | 待确认 |
| O10 | **`find_similar_node` 度量** | 确认；**实现为 FTS5 matchinfo + 编辑距离（SQLite 层）**，非 Rust 全表扫描 | 待确认 |
| O11-O14 | 联邦门控/中文 FTS5/WAL 完整性/P4.5 产出 | 同 v3 | ✅ |
| O15 | **P0 定义** | 从"纯文档"改为"**文档 + 可编译数据契约变更**"（F1/F9 修正） | 待确认 |
| O16 | **P0-Pre 清零** | Z1-Z6 立即执行（含 Z5 不做临时 LIKE 的异议） | 待确认 |

---

## 11. 审查③综合评级记录

| 维度 | 评级 | v4 对应修正 |
|---|---|---|
| 架构方向 | A | 不变 |
| 文档质量 | A | 不变 |
| 代码现状 | C+ → 经 P0-Pre 清零后不再有越界/铁律违反 | Z1/Z2/Z3/Z4/Z5 |
| 阶段可行性 | B- → P0 含数据契约变更后消除断层 | O15 |
| 测试覆盖 | D → P0.5 非空断言 + phase 过滤测试 | Z5 / P0.5 |

---

## 12. 参考资源（同 v3）

- Event Sourcing：https://martinfowler.com/eaaDev/EventSourcing.html
- Event-Driven：https://martinfowler.com/articles/201701-event-driven.html
- Event Sourcing pattern（Microsoft）：https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- The Log（译）：https://github.com/oldratlee/translations/blob/master/log-what-every-software-engineer-should-know-about-real-time-datas-unifying/README.md
- Turning the Database Inside Out：https://martin.kleppmann.com/2014/09/18/turning-database-inside-out-at-strange-loop.html
- Transactional Outbox：https://agentica.wiki/articles/outbox-pattern
- CI-144：https://github.com/CommonIntents
- Lumtract：https://github.com/Lumtract/Lumtract
