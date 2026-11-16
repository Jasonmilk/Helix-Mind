# 执行队列（已裁未做）—— 与台账 + pending 组成三件套

| 文件 | 只记 | 判据 |
|---|---|---|
| `consensus-ledger.md` | **已裁已落地**项（落点 + 判据 + 状态） | `check_ci144_profile.py` |
| `pending-decisions.md` | **未裁**项（含阻塞谁） | 待裁为 0 才可声称"无未裁" |
| **本文件** | **已裁但未做**（含依赖与完成判据） | 同 id 出现在两处 ⇒ 红；本文件清空 ⇒ 全部落地 |

⇒ 三者互斥且并集 = 全体。**"已裁未做"过去两边都不装**，所以 11 条裁决只做了 3 条、其余只活在对话里（A4 时效三档已被弄丢过一次）。

> **当前格**：合拢已签（`governance/signoff.toml`）→ HITL 观测/就绪评估态（**不是**实现应答者注册）
> **起跑**：真实任务；撞到不适应 ⇒ 先入册带机械 revisit（记录 ≠ 立刻修）；可逆可观测 ⇒ 当场修带 mutation，不可逆（购买/转账/动历史）⇒ 先验 hazard、不试错
> **回来判据**：真实授权需求出现 / 某类撞门 ≥50（**前置已落**：hitl 行镜像进落盘流（C-076/C-078）；累计与 cap 边界待真实流量举证）
> **界面三层（2026-11-05 并入坐标，不新建文件）**：`L1 = Cell-Manifest 索引薄侧栏（不建模板库、默认进最近项目、项目自报即出现）` · `L2 = 选中后才开的 AG-UI additive 出口（只映射真实骨架里有的 RUN_STARTED / TEXT_MESSAGE_* / RUN_FINISHED；metadata **只挂 verdict**（criteria/trace 不进 metadata；验收拆 D1/D2/D3 三层）；CUSTOM 只做自家扩展；TOOL_CALL_* 等到真走工具段的周期再长）` · `L3 = 可删本地偏好（栏宽/展开/最近项目，不写回 manifest 或账本）`。切换属导航层，**不进 AG-UI**（interrupt/resume 是 run 内语义，混用会被客户端误认成对当前 run 的答复）。
> **脱落触发条件（不是闹钟，是事实）**：任何标了「待脱落」的东西，**触发条件写在自己的死亡证明里**；条件满足 ⇒ 出 `docs/deaths/DEATH-CERT-*.md`（8 字段）+ 留腐殖质 + 写受体，然后才删代码。不满足 ⇒ 坐标里只留一行指针，不预排。
> **不阻塞**：队尾三条（`risk_tier` 跨仓 · 代码→文档映射 · `upstream-manifest` 空分母）+ 两条依赖人（`EXPECTED-RED` 到期换基线 · signoff 的 successor 阶段）；不新开批，不出现「等你签/等你拍/无法推进」

`last-synced: 2026-11-05`（逐条核过；读数以 §0 三条命令为准，本文件不复述）

## 队列（按依赖排，不按发现顺序）

| id | 事 | 依赖 | 完成判据 | 状态 |
|---|---|---|---|---|
| Q-P0-2 | 修 `expected-red.py` R2 跨仓误报 | — | A2 的跨仓修法能正常转绿 | ✅ 已做（`repo:path` + 兄弟仓逐个查 + 不可见时具名 skip） |
| Q-P0-3 | 补测二：`is_high_risk_call` 对自己领域命中率 | — | 输出命中数 + 三分支分流 | ✅ 已做（见读数） |
| Q-P1-A3 | 两处 `Default` 投影 + 同批换掉漂移检测器 | — | 行为不变、既有测试 0 红、检测器可红 | ✅ `c0feb60` |
| Q-P1-A2 | `catastrophic_override` 默认 `Reject` | Q-P1-A3 · Q-P0-2 | 3 条 break-glass 判据转绿 | ✅ `0b54d8d` + `3489f4f` |
| Q-P1-A1 | 分离两轴 + 给 `lib.rs:513-519` 改名 | Q-P1-A2 | 9 红全绿、5 反样本仍绿 | ✅ `92ec69c`（KNOWN-LIVE 9→1） |
| Q-P2-1 | 判据清册 `criteria-inventory.md` + "没跑即 MISSING" | — | 遍历表即"跑全量" | ⬜ |
| Q-P2-2 | 产出即留痕（共识产生时落盘） | — | 新共识在产出当轮即进台账 | ⬜ |
| Q-P2-3 | `stale-after` 按批分组 | — | 到期能区分"忘了"与"排在后面" | ✅ 已落（`Tuck/tools/expected-red.py:48/:59` 解析表头 `since:`/`stale-after:`，到期仍有余项 ⇒ 判红） |
| Q-P3-A4 | 时效三档（形状先定死） | — | 三钟各自可设可测；撤销不依赖 TTL | 🟡 形状已裁（C-063）· 未实现：`../../../anaphase-helix/src/hitl.rs::HitlCounters` 注释即「本层拿不到 risk_tier」⇒ 分档做不了，等跨仓取用 |
| Q-P4 | 补测一：真实工具名矩阵 | — | 用例来自真实调用，不是编的 | ⬜ |
| Q-P5-B1 | Tuck 判定 / anaphase 执行分层 | — | 一套 HITL：Tuck 判定、anaphase 执行 | ✅ 已裁（C-046：Tuck gate 零生产调用者 ⇒ 废弃，不接路由） |
| Q-P5-B2b | 把 B2 四条判据迁到 anaphase 侧 | Q-P5-B1 | 见 `../../../anaphase-helix/src/hitl.rs::enum ApprovalOutcome`（收窄与否以该枚举为准） | ✅ 已落 |
| Q-P5-B2 | 应答者注册（窄语义 + 映射表） | Q-P5-B1 | 见 `../../../anaphase-helix/src/hitl.rs::enum ApprovalOutcome` · 见 `../../../anaphase-helix/src/hitl.rs::HitlCounters` | ⬜ 未做（`grep -rn ApproverRegistry anaphase-helix/src/` = 0） |
| Q-P6-C1 | 反射归「做」+ 学习一等状态（候选 ≠ 已授权） | — | ADR-0054 补段落；§2 学习出口闸门 | ⬜ |
| Q-P6-C2 | 档位真值源 = 后果探测（物理面 OR 资金面） | Q-P6-C1 | 声明与事实冲突时取高者可测 | ⬜ |
| Q-P6-M | 判据 A 对应 CONVERGENCE 的 M 几 | — | 写进 `center/CONVERGENCE-M0-M7.md` 任务列表 | ⬜ |

## 关于 kernel.py 的任务分配

`kernel.py` 现在排的是**环**（ring0..ring6 的 c1/c2/c3 判据），不是**这一份队列**。两者不要混：
- 环判据回答"生态现在通不通"，**已 7 环全绿**；
- 本队列回答"已裁的事做到哪一步"，**不依赖任何运行期状态**。

⇒ 若要让 `kernel.py` 承担任务分配，必须给它加一列 **`blocks`（本项阻塞谁）**，用现成的 `kernel.overdue()` 形态推优先级 —— 但那是**新工具**，在 A 组落地前不做（§2 停做清单第 1 条：不展开新发现）。
⇒ 现在的最小形态：本文件 + `Q-P0-x` 排序，人读即可。

## 队尾（本文件自身欠账，带机械 revisit）

| 条 | 判据 | 状态 |
|---|---|---|
| 代码路径 → 文档叶子映射缺口 | 无（形态未定，见 C-072 的撤回教训） | ⬜ 只登记不定形态；[revisit: `phyt-DNA` 面板仓建立 remote 且有人负责发布时 / 任何"代码反哺文档"提议] |
| 队列完成判据列的引用**无消费方** | ✅ 已补消费方：`check_ci144_profile.py --queue` 解析本列（复用同一套解析，不新建工具）。三形实测：改坏路径 ⇒ `RED: 断链`；清空引用 ⇒ `RED: 读到空 0 条`；复原 ⇒ OK。接线：屏调用该命令时带 `--queue`。 | ✅ 已落 |

## 当前管道段（已裁未做；闭合由读数签）

| 段 | 依赖边 | 闭合判据 |
|---|---|---|
| L2 证轨**接入**（接同一条 tape，不新定义） | 无上游阻塞 | **证轨的光（据 Cellrix 文档对齐，2026-11-05）**：① 全历史**白盒**、可查、可审计、**可重放**（`VISION.md:408-410`「既给人类看，也给 Helix 自己看」· 判据：沉淀区/驳回支线如实渲染，不假装完整）② 记的是**抉择**不是调用：案卷四件 = 候选边 / 选中边 / SA-Core 支撑读数 / 未选原因（`ADR-0048:11505`「不是发生了调用，而是选择了什么、以及为什么没成」）③ **同一份 tape 的一个只读 target**，不新增数据源（`ADR-0018:1`「经历与证轨成为同一事件流的两个 target」· `prove_track.js:9-13`「NO data source of its own」——它只是**阶段性形状**，不是定义）④ 删投影 ⇒ 具名 UNMEASURABLE（不是空列表通过）⑤ **展示形制是候选**（泳道时间轴/Span 瀑布），不锁死、不照抄原型 · ⑥ **出口 = 本行的闭合证据（未闭合）**：真上游一轮能产出标准事件 **且** 经证轨可查可重放才算闭合；`src/agui.rs`/`bin/agui` 存在**不算**、桩通路（`model=stub`）**不算**、`schema 快照`替代**不算**、冻结 baseline 对照**不算新提取**。D1 可红可绿✅；D2=替代（不算闭合）；D3=冻结对照（不算新提取） |
| D3 对照 | 卡在 L2（需要对照物） | **有对象了（2026-11-05）**：`Cellrix/tools/d3_compare.js` 从 `web/assets/prove_track.js` 机检提取对照物（sha 269f2b7e · derived 15/helpers 4/renderers 3），落 `docs/observations/d3-baseline.json`；6 项逐条比较：4 项对得上 + 2 项**具名缺口**（候选选择/重复检测 = tape 未记录）。⇒ D3 未闭合的原因已收敛为「记录面缺字段」，补法走门。★ **D1/D2 已各有能红判据**：D1 删 turn/end ⇒ RED（末条不是 RUN_FINISHED）；D2 metadata 加未钉定键 ⇒ RED。D2 的"钉定 SDK"由 schema 快照 `agui-1.0-min/2026-11-05` **替代（具名，不假装等价）**。反样本：对照表缺失 ⇒ `UNMEASURABLE` rc=4；L2 静默缺项 ⇒ RED rc=1。**冻结声明**：`web/` 已删 ⇒ `--baseline` 现为 `UNMEASURABLE rc=4`，对照物冻结在 `269f2b7e`，**不算"新提取"**（要重新提取必须先恢复 web/，属回滚，需园丁开口） |
| `web/` **已删待验收**（原「待删」） | 卡在 D3 | **信息面已量（2026-11-05）**：`Cellrix/tools/web_removal_probe.js` 对 web 侧 15 个派生字段逐个查"重建需要哪类 tape 事件"，再与真实 events 的 type 集合对 ⇒ rebuildable 8 · UI 态 7（本就不是 tape 事实）· **缺 tape 来源 0** ⇒ 删 web 只丢渲染代码，不丢信息。反样本：空 events ⇒ UNMEASURABLE(4)；基线缺失 ⇒ UNMEASURABLE(4)。**边界**：信息面读数 0 丢失；**已于 2026-11-05 删除**（`Cellrix 725c66f`，1396→0，`git ls-files web \| wc -l` = 0）⇒ 现状态是「已删，**终局待园丁验收**」，DEATH-CERT 身份仍为「待确认」 |

> 闭合由**读数**签，不由交接语签：删行前把机检读数（命令 + rc + 关键行）追加进台账。

### L2 真跑撞出的缺口（2026-11-05，读数来自 `Cellrix/tools/l2_casefile.js`）

| 案卷四件 | 取到? | 读不到的**原因**（不是"没写代码"） |
|---|---|---|
| ① 选中边 | ✅ 5 条（`Perception --Success--> PreAssessment` …） | 来自 ring 的 `state` 迁移 |
| ② 候选边 | ❌ UNMEASURABLE | tape 只记**被选中**的那条；**兄弟条件未落盘** |
| ③ 支撑读数 | ✅ `by_from` + `verdict=null` + `session_rows=10` | — |
| ④ 未选原因 | ❌ UNMEASURABLE | tape 里**没有任何字段**记录"为什么没走另一条边" |

⇒ 缺口在**记录面**（anaphase 的 state 事件只写 `from/condition/to`），不在展示面。
⇒ 补法属「新语义面」（要给事件加兄弟条件 / 未选原因字段）⇒ **走门**，本轮只登记。

### 门项：`候选边 / 未选原因` 是否进 tape（t4，未决，不自行补）

| 事实 | 读数 |
|---|---|
| anaphase state 事件形状 | `run_cycle/mod.rs:889` `{"from","condition","to"}`；`:916` 失败支 `{"from","condition","to":null,"undefined":true}` |
| 是否已有 `candidates` / `unselected` 字段 | 无（全仓 14 处同名命中均为无关代码：gc/retry/query） |
| 因此 L2 案卷两件 | 恒为 `UNMEASURABLE`（具名缺口，不填想象值） |

**为什么不自行补**：`未选原因` 只能来自模型自述 ⇒ 写进审计链＝让模型的话成为证据。
`ADR-0048:11505` 要的是"选择了什么、为什么没成"，但"为什么"必须由**人或确定性规则**给。
⇒ 补字段属改外部事件契约（新语义面）⇒ 门；本轮只登记，不动 anaphase。

### L2 的「出口」证据（未闭合；2026-11-05 桩上游真跑一轮）

| 项 | 读数 |
|---|---|
| 真跑一轮 | `job_id=run-8451798a882701d7` · `model=stub` · jsonl **10 行** · `bin/agui` 产出 **5 条**标准事件 |
| D1/D2 | `tools/d1_d2_check.js <tape>` ⇒ `rc=0`（D1 线层真绿；D2 是 schema 快照替代，**不算闭合**） |
| 四条反样本 | 缺 tape 目录 `rc=1` · 空事件流 `rc=1` · `metadata` 多键 ⇒ D2 `rc=1` · 删 `turn/end` ⇒ D1 `rc=1` |
| 一键入口 | `Cellrix/tools/accept_all.sh` ⇒ `rc=0`（正样本）；藏起 tape ⇒ `rc=1`（反样本） |
| 不算闭合的三件 | 桩通路（`model=stub`）· `schema 快照 agui-1.0-min/2026-11-05` · 冻结 baseline `269f2b7e` |
| **仍缺** | 真上游一轮（端点/凭据四问已列在 `Cellrix/docs/observations/real-upstream-checklist-2026-11-05.md`） |

### 出口链的新读数（2026-11-16，桩模式，非生产）

| 事实 | 读数 |
|---|---|
| 桩上游跑一轮（修好 env 传参后） | `impasse=False · model=stub · agui 5 条 · D1/D2 rc=0` |
| **上游返回 tool_calls 时本仓仍 `tool/call=0`** | ⇒「模型会不会发起工具调用」不是唯一条件，工具段另有装配门（具名，未处置） |
| rail 命中会绕过上游 | `bypass=rail` ⇒ 该轮 **上游未被调用**（读数会因此假"通"）——`run_one_cycle` 现已打印 |
| 曾经的真 bug | `env "$A=1 B=2"` 被当成单变量 ⇒ endpoint 丢失 ⇒ `builder error`；已改数组传参 |

**结论边界**：以上全部是**桩**读数，不构成「真上游一轮」；L2 仍保持未闭合。

### 工具段的两道前置（2026-11-16 读码确认，解释 tool/call=0）

```
① 上游必须给 tool_calls           —— 由 reasoning_endpoint 的模型行为决定
② execution pipeline 必须 Bound   —— pipeline/mod.rs:557 resolve_binding：endpoint 为空 ⇒ Unbound
                                     ⇔ config.rs:98 tentacle_endpoint / env ANAPHASE_TENTACLE_ENDPOINT
Unbound 时 run_cycle/mod.rs:317 ring_execution_refusal ⇒ "no tool call"（fail-closed，只读继续）
tool/call 事件只在**已执行的调用**上发射（run_cycle/mod.rs:1340，遍历 records）
⇒ ①②缺任一：tool/call 恒 0，HITL 门（挂在组装之后）都轮不到
```

**⇒ 要走到授权段，需要两个 endpoint 同时在位**：`reasoning_endpoint` + `tentacle_endpoint`。
（当前 config.toml 两者都没有；桩模式只能验①的"给了会怎样"，验不了②。）

### 第三道门（2026-11-16 实测，比"两道前置"更准）

```
按"两道前置"配好（reasoning_endpoint=桩 · tentacle_endpoint=grpc://127.0.0.1:1 故意不可达）
真跑一轮读数：impasse=False · model=stub · **tool/call = 0** · 日志里没有 execution pipeline refused
⇒ 工具调用**根本没走到执行点** ⇒「配置了但不可达」这条分支本轮**没被触发**，不可作结论
⇒ 说明在"上游给 tool_calls"与"执行点"之间还有一道**装配门**（未命名、未定位）
```

**登记（不处置）**：第三道门 = tool_calls → 执行点 之间的装配。定位它之前，
「tentacle 可不可达」都只是静态配置读数，不能当作工具段走通或走不通的证据。

**顺带的真 bug**：`cp config.toml + 追加 reasoning_model` ⇒ `TomlError: duplicate key`（服务直接退出）。
⇒ 配置是 fail-closed 的（好事），但也说明"追加一行"这种操作必须先看原文件有没有该键。

### 第三道门已定位（2026-11-16 读码 + 实跑）

```
src/adapters/http_reasoning.rs:143   非流式只取 choices[0].message.content
grep -c tool_calls src/adapters/     = 0        ← 适配器**从未读取** tool_calls
实跑（桩返回 tool_calls）：日志 "empty reply … retry" → "Unstructured output (no calls plan)"
⇒ sig.calls 恒空 ⇒ NoToolNeeded；tool/call 恒 0
⇒ 与 tentacle_endpoint **无关**（那是第二道：执行点 Bound）
```

**结论（修正对外口径）**：当前实现下「上游会不会自己发起 tool_calls」**无法回答**——
tool_calls 在适配器层就被丢弃。要回答它，必须先让适配器解析 tool_calls（改行为面 ⇒ 需裁定）。

三道门（现在有名字了）：
```
① 适配器解析 tool_calls        缺 ⇒ calls 恒空（现状：缺）
② pipeline Bound（tentacle）   缺 ⇒ job 不组装、执行点 refuse
③ 执行点通过（HITL 安全门）    缺 ⇒ 不执行、无 tool/call 事件
```

### 纠正（2026-11-16 晚）：只有两道门，"适配器不读 tool_calls"是我的桩形状错

```
本仓契约（contract/mod.rs:92）：模型在 **content** 里回 JSON 计划
  {"calls":[{"tool":…,"args":{…},"expect":"ok"}]}
我此前的桩回 OpenAI 风格 message.tool_calls ⇒ 本仓从不读它 ⇒ 误判成"适配器缺功能"

把桩改成契约形状后同一轮真跑：
  [Reasoning] 1 tool call(s) parsed but pipeline UNBOUND (tentacle_endpoint missing) ⇒ job NOT assembled
  ⛔ tool calls parsed (n=1) but execution pipeline is unbound ⇒ not executed, no tool/call event
⇒ ① 解析成立（n=1）；② 缺 tentacle_endpoint ⇒ job 不组装（昨日新增的 warn 首次真触发）
```

**门只剩两道**：`reasoning_endpoint`（上游给计划）+ `tentacle_endpoint`（pipeline Bound）。
`accept_all` 预检已从三道改回两道，并删掉那条误导性 grep。

### 生态计划 M2 的进度（2026-11-16，不是我的新计划，是 center 里的 M2）

| M2 组成 | 状态 | 读数 |
|---|---|---|
| ADR-0041 写路径验收 | ✅ 新 store 实测 | 多组 `turn/start` 的文件 **0** · 时间戳非单调的文件 **0**（3 轮，`/tmp/m2/events`） |
| 缺口 I5（切片原语） | ✅ 已实现（`anaphase cd5a9ff`） | `slice_of(list, root)` 纯函数 · 正样本 r{a,b,a1} 不含 other 树 · 两个反样本（根不在窗口 / 悬空父）各自具名 |
| I5 的**调用点** | ⬜ 未接（按计划属 M3） | 本轮刻意不改读路径行为；M3 断言 I5 时才接线 |
| M3 前置（M0.5 dump 基线） | 已有 `M0.5-dump/diff-2026-10-09.md` | 待 M3 引用 |

**结论**：M2 的"补缺口"这一步完成；M2 整体待 I5 接线（M3）后才能勾。
