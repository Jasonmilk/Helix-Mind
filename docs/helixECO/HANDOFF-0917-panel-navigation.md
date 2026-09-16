# 交接：面板导航 N2（选择态收敛 + hash 接线）

> 本文件写给**没有上一轮上下文的接手者**。读它 + 文末列出的 4 份产物，即可开工。
> 创建：2026-09-17。上一轮的所有改动均已提交、工作区干净。

---

## 0. 一句话目标

把面板的导航从「散落的工程决定」变成「受约束的一层」：**先让选择态只有一处真相，并让 URL 可寻址**（N2），再换骨架（N3）。

## 1. 现状快照（实测）

| 项 | 值 |
|---|---|
| 服务 | mind `bash-15`（:50052）/ anaphase `bash-19`（:50061）/ 面板 `bash-18`（:8080） |
| mind 存储 | `3 nodes / 0 edges`（仅 3 条 L2 物理知识；这是**干净基线**） |
| 经历 | 138 period（`.helix/events/run-*.events.jsonl` + `.name` 侧车） |
| 测试 | Helix-Mind **158** / Anaphase **277** / 全生态 **1630**；面板 JS 回归网 **10 suites green / 1 need input** |
| 工作区 | 干净。仅剩人类在飞文件：`helix-mind/docs/helixECO/HANDOFF-0916.md`、`anaphase-helix/session_notes.json`、Cellrix 的几何工作在飞文件 |

## 2. 已经做完（不要重做）

- **Helix-Mind**：ADR-0042 T1（相对峰值闸门 / α 由 `heliotropism` 派生 / 收敛判据 / 删矛盾 `mode.rs`）、D3 抑制改确定性门控（零 schema 变更，复用 `corrected_by`）、D4 `min_k_core` config 化 + **种子豁免**、ADR-0043（建边：`parent_ids` → `derived_from` + 边）、**线上 `edges` 0 → 135、最长路径 3 跳**
- **Anaphase**：T5b 传血缘、P0 修「人类摘要占父指针槽」、悬空父指针归一化、T6 白盒读 `activation_vector`（字段 `heat` → `activation`）、**卡片标题创建时冻结**（+141 个旧 period 已补内容标题）
- **Cellrix**：`ADR-0022`（导航约束 N 系列）、`all_views_test.js` 从"永不执行"接通、`parseHash`/`buildHash` + 6 条测试、`eng-assertion check`（N1，变异注入证明非空转）

## 3. 下一步：N2（唯一会动面板活动渲染的一步）

### 3.1 要做什么

把**三处选择态合并为一处**，并让 hash 成为它的序列化：

| 现在的三处 | 位置 |
|---|---|
| `st.chatJobId` | `web/assets/session.html` |
| `window.__proveTrackMeta` | `web/assets/session.html` |
| `Cx.selectPeriod()` 的内部态 | `web/assets/script.html` 附近 |

目标形态：**一份状态**（如 `Cx.state.nav = {view, period}`），UI 只读它；hash 由 `buildHash(state)` 写出、由 `parseHash(location.hash)` 读入；`hashchange` 时应用。

### 3.2 已经准备好的垫脚石

- `parseHash(hash)` / `buildHash(state)` **已实现、已导出、已测**（`web/assets/period_normalize.js`，测试 `web/tests/hash_state_test.js`）
- 格式已定：**`#view=<id>&period=<job_id>`**（键值对，非路径式）。理由见 ADR-0022 §2.3 N-009
- **全域**要求已实现：未知键忽略、畸形百分号编码跳过该对、陌生 hash 返回空态**而不抛错**（N-010：畸形 hash 不得导致面板不渲染）

### 3.3 验收（ADR-0022 的条款，不许自定标准）

- **N-003** 任意时刻只有**唯一**的「当前视图 + 当前 period」
- **N-009** 视图 + period 可由 hash 完整表达，**往返一致**
- **N-010** 手改/畸形 hash 仍能渲染
- **N-015** 同一事物**只有一个入口**（这是 N3 的，但 N2 改动时别加重它）

### 3.4 ⚠️ 验证路径（与其它改动不同，必须照做）

面板资产是 **`include_str!` 嵌进二进制的**，所以：

1. `cd Cellrix && cargo build`
2. 确认二进制**比资产新**：`ls -lT target/debug/cellrix-web web/assets/session.html`
3. 重启面板（当前是 harness 后台作业 `bash-18`；用 `pkill -f debug/cellrix-web` 后以**后台作业**重启，命令见该作业）
4. **对页面 grep 核实**（`/assets/...` 直接取是空的——资产是内联的，我踩过这个假阴性）：
   `curl -s http://127.0.0.1:8080/ | grep -c "<你新加的标识>"`
5. 跑回归网：`cd Cellrix/web && node tests/run_all.js`（应仍是 **10 suites green**；`all_views_test.js` 会对着真面板跑 55 条断言）

### 3.5 已知风险

- 改选择态会动 `session.html` 与 `script.html` 两处**共享状态**，容易只改一半
- `dom_contract_test.js` 断言「DOM 清单与源一致」——若新增元素，它可能变红。**别绕过它**，同步更新
- **不要**在这批里同时引入槽位（那是 N3）；一次只改一个变量

## 4. 待人类决策（不阻塞 N2 开工，但阻塞其后）

1. **批准 `Cellrix:ADR-0022`**（现为 Proposed，转 Accepted）
2. **N-014**：面板是否**永不需要**破坏性操作？若是，该条应**删除而非锚定**

## 5. 已知问题（登记在案，别当新发现）

- **`anaphase-helix/tests/stage_events.rs::deterministic_replay_same_clock_same_trail` 有抖动**：本会话出现 **4 次**单条失败、每次复跑全绿（弱机负载下两条 trail 的一条被截断）。**它是已知的测试可靠性问题**，不影响产品代码，但**会污染每一次"全绿"的可信度**。值得单独查一轮。
- `answer.delivered` 判据名实不符（检查的是工具边，不是"用户是否拿到答案"），**有意未改**。
- `layout_test.js` 跳过是**诚实**的（需 Chrome on :9222）。

## 6. 工作规矩（人类反复强调过，务必遵守）

1. **改之前先说计划、等确认**（"你把你计划告诉我,我确认之后才可以修改!"）
2. **回复用中文**
3. **命名红线**：写「**证轨**」，**绝不**写成「轨迹」
4. **活文档里不出现外部项目名**（参考做法可以，名号不留）
5. **每个功能都要有测试；用变异注入证明非空转**（"变异测试证明非空转"）
6. **不许静默提交错误数字**；更正**记录在案**，不重写历史
7. **绝不绕过 git hooks**（`[large]` 要写真实理由；ADR 首行/状态要能被 `tools/adr_head.py` 解析）
8. **陶土不许说成钻石**：无法论证的数字**标 `[ENG]` 或留空**，不填一个"看起来专业"的值

## 7. 产物索引

| 产物 | 路径 |
|---|---|
| 导航约束（N 系列 19 条 + 锚定表 + 主任务主张） | `Cellrix/docs/decisions/ADR-0022-panel-navigation-constraints.md` |
| hash 纯原语 | `Cellrix/web/assets/period_normalize.js`（`parseHash` / `buildHash`） |
| 回归网入口（含 N1 工具纪律 + 检查器） | `Cellrix/web/tests/run_all.js` |
| 建边决策（T5）与线上验收证据 | `helix-mind/docs/decisions/ADR-0043-write-path-edge-construction.md` |
| SA-Core 决策（D0/D1/D2/D3/D4/T6） | `helix-mind/docs/decisions/ADR-0042-sa-core-parameter-source-and-gating.md` |
| 生态 SSOT（本会话全过程） | `helix-mind/docs/helixECO/ECOSYSTEM.md`（v1.99 行） |

---

*本交接不含猜测：每条状态都对应一次本轮实测，每个"下一步"都标了验证路径与已知风险。*
