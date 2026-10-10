# P0-P7 里程碑回顾（Helix-Mind 第一阶段收官）

> **版本**：v1.0（2026-08-28）
> **性质**：Helix-Mind 按 DNA 自生长方法论完成 P0-P7 主路线的完整里程碑回顾与历史归档。
> **归档**：随仓库版本化，永不删除。
> **基准**：`rs-dev` 分支（HEAD `fc78318`）｜ 知识本体 v4.1 ｜ DNA 方法论 v2.0

---

## 一、总览

Helix-Mind 从"骨架"（0.1.0）经 P0-P7 八个阶段，完成了记忆中枢 → **认知相态河流与催化器**（v4.1 范式）的架构落地。全路线遵循"哲学审查 → ADR 冻结 → 代码落地 → 测试验收 → 归档"的 DNA 闭环，**每一步只做当前阶段需要的事，不做 MVP 妥协、不圆谎**。

**最终状态**：`cargo test --workspace` **70/70 全绿、0 失败、0 warning**；`rs-dev` 与远程同步；文档生态闭环。

| 阶段 | 内容 | 状态 |
|---|---|---|
| P0-Pre | Z1-Z6 清零（架构越界 + 铁律违反物理移除） | ✅ |
| P0 | 认知基线 + 可编译数据契约（相态模型） | ✅ |
| P0.5 | 测试基线 + VISION.md 根索引 + L2 共享语义锁定 | ✅ |
| P1 | 检索闭环（FTS5 trigram 中文召回 + 注入防御 + 相态加权） | ✅ |
| P2 | 代谢闭环（确定性代谢 + CognitiveService 三适配器） | ✅ |
| P3 | 安全与契约（联邦确定性审查 + UDS 鉴权 + CI-144 语义对齐） | ✅ |
| P4/P4.5 | 硬冻结兑现 + WAL 架构审查点 | ✅ |
| P5 | 领域 WAL（独立日志 + BLAKE3 哈希链 + replay） | ✅ |
| P6 | 数据诚实 + 轮回 + 商业化底座 | ✅ |
| P7 | 生态文档同步 + 认知工艺 Phase 1 | ✅ |

---

## 二、逐阶段回顾

### P0-Pre：清零（Z1-Z6）
**目标**：清除与 DNA 方法论冲突的既有代码，立住架构铁律。
- **Z1** 物理删除 `flowmodus.rs`（外部桥接 + 轮询铁律违反，Iron Law #13）
- **Z2** `llm_mode` 默认 `disabled`，生产零 LLM HTTP 出站（能力未就绪=功能不存在）
- **Z3** HealthServer 注册（gRPC 标准探针，类比"营业灯牌"）
- **Z4** layer3 透传请求参数（移除 `familiarity=0.5`/`impasse_depth=0` 硬编码）
- **Z5** 检索测试诚实化（`#[ignore]` + 显式空断言，拒假阳性）
- **Z6** FTS5 启动探针（缺失即 panic 阻断）
- **提交**：`7ef1a30`

### P0：认知基线 + 数据契约
- ADR-0010 认知预算前置路由 / ADR-0011 相态模型（气/液/胶/晶 + 主体依赖轴）/ ADR-0012 Append-Only Schema Evolution
- `Node` struct + SQLite + proto 追加 `phase_state`/`subject_dependency`/`concentration`/`tension`（reserved 号保护）
- `EnergyContext.budget_tier` 字段（跨 crate 契约）
- **提交**：`b710ce6`

### P0.5：测试基线 + 根索引
- FakeAdapter + 内存 SQLite 测试基建；非空断言检索测试（暴露 S1 主干断路）
- **VISION.md v1.1**：10 条原子原则 + 叙事/决策映射 + L2 共享宪法澄清（ADR-0016：自动共享 + 洗脱前置）
- **提交**：`1289507`

### P1：检索闭环（M-01/02/03）
- ADR-0013：FTS5 trigram 外部内容表 + 异步批量索引（`FTS_BATCH_SIZE=100`）+ 查询路由（≥3 字符 FTS5 / 1-2 字符 LIKE fallback）
- **注入防御**：白名单清洗 + FTS5 转义（双引号包裹 + 引号加倍）+ 审计记录（三层纵深防御：CI-144/Tuck/Mind 各守一层）
- **相态加权**：`-bm25`（FTS5 负分转正）× 相态权重（Crystal > Liquid > Gas）
- **F3 前置修复**：`bump_access_counts` 单事务原子自增（消除查询写放大）
- **提交**：`92717f9`

### P2：代谢闭环（a/b/c）
- **P2a 确定性代谢**（ADR-0014）：`find_similar_node`（FTS5 候选 + 跳过 recessive 防环）、`resolve_dissonance`（SymbolicSolver 仲裁 + Corrects edge + corrected_by trace）、Text-only 失调诚实保留
- **P2b CognitiveService 端口**（ADR-0017）：trait + 三适配器（Deterministic 生产默认 / Remote debug_direct 门控 / Fake 测试）+ fail-closed 工厂
- **P2c 越界清零**：crystallize/ner 删除直接 reqwest，统一走 cognitive 端口
- **提交**：`97ecb50`

### P3：安全与契约
- **P3a 联邦确定性审查**（ADR-0018）：dual-judge + 出站 feature-gated（能力未就绪=功能不存在，非临时禁用）
- **P3b 传输安全**（ADR-0019）：UDS + SO_PEERCRED 白名单鉴权（本地）/ mTLS 设计（远程）+ 真实 ValidationLayer
- **P3c CI-144 语义对齐**（ADR-0020）：INTENT-7 动词映射表（FETCH/WRITE_NODE/TENTACLE/FINISH/CANCEL）+ `traceparent` 透传（W3C，透传不生成）
- **提交**：`aa7be51`/`8f2160c`/`245e22f`/`5712b5f`/`29b8a8f`

### P4/P4.5：硬冻结兑现 + 架构审查点
- **M-10 `activation_vector`**（Append-Only 扩展，reserved 号）；移除 M-11 Callosum 实现 → 改为**调用契约文档**（Mind→Callosum 单向请求，分配策略归 Anaphase）；Rhizax 预留接口
- **P4.5 四份产出**：ADR-0015 WAL 设计 / WAL 最小原型（append+fsync+单段读取，延迟验证）/ Parquet 投影约束 / 回退方案
- **提交**：`cabd926`/`c7a3177`

### P5：领域 WAL
- 独立追加日志文件（非 SQLite 事件表，一步到位）+ BLAKE3 哈希链（`prev_hash` 跨段延续，**防位腐坏/意外损坏，不声明防篡改**）+ 段轮转 + 异步投影 + replay 恢复 + truncate 截断尾
- **提交**：`6e9f1bf` / `47302bd`（P6-2 主写路径接入）

### P6：数据诚实 + 轮回 + 商业化
- **P6-1 Parquet 名实相符**（F7 关闭）：真列式（Apache Arrow 生态），PAR1 magic 验证；12 必须投影列，可延迟列回落默认（诚实声明）
- **P6-3 轮回补全**：`trigger_reincarnation` 完整轮回（sunset 归档 → rebirth 新生）；emergency_dusk 真实世代号（原硬编码 1）；epoch/inheritance 晶体写 `deep_cold_dir`（原污染工作目录）
- **P6-4 多租户 WAL 分区预留**：`WalConfig.tenant` + `root()`，租户段隔离互不串链，单租户行为不变
- **提交**：`d6dbe82`/`4ed1869`/`d7b3443`

### P7：生态文档同步 + 认知工艺 Phase 1
- **ADR-0021 认知工艺**（Active）：Mind=编排（0 Token）/ CognitiveService=执行（B1）；System 0 门控用规则 + FTS5 bm25 非 embedding（B2）；与 ADR-0010 预算边界划界（B3）
- **`docs/spec/cognitive-craft.md` v1.1**：完整规范（工序×模式、MSC、黑格尔确定性合题、Bounded ε-Greedy+EMA、熔断）
- **ECOSYSTEM v1.6 对齐核对**：契约逐项零断链；CI-144 动词映射 5/5 一致；budget_tier/traceparent 字段建议
- **提交**：`fc78318`

---

## 三、架构基座（未圆谎的根）

| 基座 | 决策 | 锚 |
|---|---|---|
| 真理源单一 | SQLite/Parquet 仅 Mind 独占持有，其他组件只走 gRPC | ADR-0003 |
| 领域 WAL | 独立日志文件（非 SQLite 事件表），BLAKE3 完整性校验 | ADR-0015 |
| 相态模型 | 气/液/胶/晶 + 主体依赖轴，物化策略（非运行时派生） | ADR-0011 |
| 认知预算 | `budget_tier` 前置路由，Mind 只执行（0 Token 优先） | ADR-0010 |
| LLM 安顿 | Mind 零直连，一切经 CognitiveService 端口（三适配器） | ADR-0017 + Z2 |
| 检索 | FTS5 trigram 中文召回 + 注入防御 + 相态加权 | ADR-0013 |
| 契约演进 | Append-Only Schema Evolution（追加不修改，reserved 保护） | ADR-0012 |
| 联邦 | 能力未就绪 = 功能不存在（feature-gated） | ADR-0018 |
| 商业化 | 多租户 WAL 分区预留；滞后开源（实现闭源/协议公开/哲学自由传播） | P6-4 + D5 |

---

## 四、生态对齐状态

| 生态组件 | 与 Mind 关系 | 状态 |
|---|---|---|
| Anaphase-Helix | 身体，gRPC 驱动，Mind 只建议不执行/不持凭证/不直连 LLM | 契约对齐 |
| Helix-Callosum | 上下文分配器官，Mind 单向调用契约（分配策略归 Anaphase） | 契约化 |
| Cellrix | 语义投影终端，消费语义快照 | CAPABILITY-13 端点 Cellrix 侧推进中 |
| Rhizax | 种群知识树（去中心化），三阶段演进第一阶段（仅接口） | 预留 |
| Tuck | 不可变审计与安全网关，CI-144 传输层 | 外部项目 |
| CI-144 协议家族 | INTENT-7/BIND-19/SECURE/CAPABILITY-13 | 动词映射 5/5 一致 |

---

## 五、下一步（P8 预览）

**认知工艺 Phase 2——编排器最小原型**（ADR-0021）：
- System 0 门控（规则 + FTS5 bm25）
- 工序编排（2-3 道）+ 独立会话隔离（MSC）
- 黑格尔辩证确定性收敛
- Token 熔断 + 30s 超时
- **先用 DeterministicAdapter 闭环验证编排逻辑（0 Token）**，Remote 仅 debug_direct 可插拔

---

*《P0-P7 里程碑回顾》v1.0 完。后续阶段完成时，以追加/新卷方式续写本回顾。*
