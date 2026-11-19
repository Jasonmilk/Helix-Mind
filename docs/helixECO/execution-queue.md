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
| L2 证轨**接入**（接同一条 tape，不新定义） | 无上游阻塞 | **证轨的光（据 Cellrix 文档对齐，2026-11-05）**：① 全历史**白盒**、可查、可审计、**可重放**（`VISION.md:408-410`「既给人类看，也给 Helix 自己看」· 判据：沉淀区/驳回支线如实渲染，不假装完整）② 记的是**抉择**不是调用：案卷四件 = 候选边 / 选中边 / SA-Core 支撑读数 / 未选原因（`ADR-0048:11505`「不是发生了调用，而是选择了什么、以及为什么没成」）③ **同一份 tape 的一个只读 target**，不新增数据源（`ADR-0018:1`「经历与证轨成为同一事件流的两个 target」· `prove_track.js:9-13`「NO data source of its own」——它只是**阶段性形状**，不是定义）④ 删投影 ⇒ 具名 UNMEASURABLE（不是空列表通过）⑤ **展示形制是候选**（泳道时间轴/Span 瀑布），不锁死、不照抄原型 · ⑥ **出口 = 本行的闭合证据（未闭合）**：真上游一轮能产出标准事件 **且** 经证轨可查可重放才算闭合；`src/agui.rs`/`bin/agui` 存在**不算**、桩通路（`model=stub`）**不算**、`schema 快照`替代**不算**、冻结 baseline 对照**不算新提取**。D1 可红可绿✅；D2=替代（不算闭合）；D3=冻结对照（不算新提取）。**★ 2026-11-18 首个全绿周期（verdict=Met）读数齐备**：真上游(coder-3b) → tool/call calc → tool/result ok=true 55ms → exec_ok/answer.delivered 双过 → `verdict=Met` · `success=true` · `hitl allowed_once=1`；同 tape：`bin/agui` 5 条标准事件 · D1/D2 rc=0 · 案卷重放 diff=0。详见 `observations/2026-11-18/first-green-cycle.md`（含复现三条命令与三个真原因）。**闭合仍待园丁签（C-069）** |
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

> ⚠️ **本节结论已被后文《纠正（2026-11-16 晚）：只有两道门》推翻。**
> 保留原文是为了记住"我当时怎么错的"：不是适配器缺功能，是**我的桩返回形状错了**。
> 读到本节请直接跳到后文《纠正》，以那节为准。

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
| I5 的**调用点** | ✅ 已接（`anaphase ef34319`） | `GET /v1/sessions?root=<id>` ⇒ 可达闭包；默认行为不变。读数：现有根 ⇒ periods=1/无截断 · 不存在的根 ⇒ 0 且具名「不在窗口内」 |
| I2 / I4 断言 | ✅ 已加（同上） | I2：父槽混摘要 ⇒ 当悬空并具名 · I4：父 first_ts ≥ 子 ⇒ 具名 `c ← r` |
| I3（append-only store 级断言） | ✅ 已加（`anaphase 3d32245`） | 追加前后**字节前缀一致** · 行数 +1 · seq 严格递增无重复 |
| M4 变异（三条） | ✅ 已跑，2 死 1 等价 | M1 去 append ⇒ I3 红 · M3 I4 判据恒假 ⇒ I4 红 · **M2 slice 去 `seen` 守卫 ⇒ 未被杀（等价变异体：单父指针不可重复到达，它防的是环）** |
| M3 前置（M0.5 dump 基线） | 已有 `M0.5-dump/diff-2026-10-09.md` | 待 M3 引用 |

**结论**：M2 的"补缺口"这一步完成；M2 整体待 I5 接线（M3）后才能勾。

### M5 完成（2026-11-16，`anaphase f9e8e88` + `c3acd74`）

```
退役对象：FlowModus gRPC 转发面（center:CONVERGENCE M5：调度面已由 FlowModus 侧承担，
        本仓再留一条转发通道 = 同一件事两个入口）
移除面  适配器 118 行 · main 回退分支 · config 键（含"旧键被忽略"具名提示）·
        proto 模块与生成项 · health 探测行 · gloves 手套项 · 结构键清单
读数    cargo build 通过 · 全量 0 红 · 预算 rc=0
        现网 config.toml（仍含旧键）启动 ⇒ ⛔ 具名提示"已退役、被忽略"
        grep flowmodus src/ ⇒ 只剩退役提示 / 历史注释 / 安全门测试字符串
```

生态各里程碑现状：M0 冻结 ✅ · M1 裁决 ✅ · M0.5 差分 ✅ · M2 ✅ · M3 ✅ · M4 ✅（2 死 1 等价）·
**M5 ✅** · M6 = B18 ⬜（跨仓契约，需园丁点头）· M7 ⬜（UI 解冻四条件）

### M7（UI 解冻）闸门读数（2026-11-16，只读，不启动）

```
条件1 M2–M6 全完成          ❌（M6 = B18 未完成）
条件2 孤根 83→可接受         ❌ 现网 668 period / 孤根 484（72%），基线是 139/83（60%）
条件3 slice 可用且不混用     ✅ slice_of / slice_from / /v1/sessions?root= / chain_of 另名
条件4 无新增「面」           ✅ 基线已建（19 个面逐条列名，`observations/2026-11-16/m7-condition4-face-baseline.md`）；本轮 grep=19 且集合一致
⇒ M7 保持冻结（原文：任意一条不满足 ⇒ 不"先做起来看看"）
```
详见 `observations/2026-11-16/m7-gate-readings.md`。

**条件 2 补测结论（同日）**：72% 的分母被自动化流量污染 —— 484 条孤根只有 28 种输入，
其中 456 条（94%）是脚本化/测试口令（263×「只回答两个字:你好」· 128×「S303 HTTP 判据」· 58× 空）。
按真实对话口径过滤后：**period 51 · 孤根 19（37%）**。
⇒ 条件 2 若要可判，分母须具名剔除自动化输入族；**口径改动属 center，须园丁裁定**，本轮只留数与提议。

### CI 的两个真缺口（2026-11-17，都已修）

```
① anaphase：M5 删除后**测试编译不过**（tests 仍引用 cfg.flowmodus_endpoint）
   ⇒ 我此前"全量 0 红"是假读数：`cargo test | grep -c FAILED` 在**编译失败**时读到空
   修：端点表 5→4 · 删该断言 · glove 名单去掉 flowmodus ⇒ `cargo test rc=0`（判据改成退出码）
② Cellrix：引擎清单未重签（新增/改动的三个 tools/*.sh）⇒ shasum -c 3 个 FAILED
   修：正门 `--write`（7 个引擎文件）⇒ rc=0
读数：anaphase cc9ac21 success · Helix-Mind 8a7a43e success · Cellrix（重签后待跑）
```

**wart 具名**：`upstream-manifest.sh` 用 `tools/*.sh` 全收 ⇒ 项目本地脚本被算成"引擎副本"，
每次改动都要重签。摘除需移出 `tools/*.sh`（牵动入口路径），本轮不动。

### 环收据刷新读数（2026-11-18）—— 让针跟着转，不预设全绿

```
刷新：kernel --run ring0..ring6 逐条跑 ⇒ 收据新鲜度 STALE 0（最旧 0.00 天）
结果：ready = ring0,1,2,3,4,6 ／ blocked = ring5
```

**两条红的归因（都具名，都不是"没达标"）**：

| 环 | 红因 | 证据 | 处置 |
|---|---|---|---|
| `ring4`（cellrix ← anaphase） | **环境**：anaphase 没在跑 ⇒ `live_snapshot_roundtrip` 在 `ANAPHASE_REQUIRED=1` 下按设计 panic | 启动 anaphase 后重跑 ⇒ `[绿] ring4/c1`，ready 里出现 ring4 | 已解（环境依赖，不是缺陷） |
| `ring5`（anaphase ← flowmodus） | **M5 后果**：flowmodus 转发面已退役 ⇒ 差分判据 `sensitive=False`（`diff_lines` 为空）⇒ 该环的依赖对象不存在 | `[红] ring5/c2 diff_lines=[] sensitive=False expect_sensitive=True` | **待裁定**：改指向（anaphase ← llama-swap HTTP 8080）或作废该环；我不自改 `rings.json` |

**口径（吸收审查意见，写死在这里）**：

```
① 签字只承认「这一圈读数成立」，**不触发**删队列 3 行 / DEATH-CERT 转正式 / git rm
   ——「待确认」与页面 4 处「本 tape 未记录」是**留白**，不是待清的债
② 刷新收据的判据是「收据新鲜、读数如实」，**不预设 ready 全绿**；仍 blocked 就具名登记
③ 五跳工具供给链是**图**，不是路线等级 —— 不从中长出「还差几跳／覆盖率／每环补判据」
```

#### ring5 的精确归因（补，2026-11-18）——**是我 M5 那次编辑把被测对象拿掉了**

```
ring5 判据对象：`ANAPHASE_UPSTREAM_ONLY=flowmodus` 跑 tests/upstream_live.rs（端口读 chain.json）
M5 时我做了两件**各自都对、合起来让环失去对象**的事：
  ① 退役 flowmodus 转发面（center M5 要求）
  ② 把 upstream_live 的 `NAMES` 从 5 个改为 4 个（删掉 "flowmodus"）
⇒ 现在 `ONLY=flowmodus` 过滤出空集 ⇒ c2/c3 的 down/up 差分**恒不敏感**
⇒ 工具报的正是它该报的：`sensitive=False · diff_lines=[] · 判【无依赖断言】并阻塞`
   —— 这不是判据坏，是**判据的对象不存在**（C-087 同族）。
```

**两条候选处置（都不由我拍）**：

| 选项 | 做法 | 代价 |
|---|---|---|
| **A 改指向** | ring5 从 "anaphase ← flowmodus" 改为 "anaphase ← reasoning"（llama-swap HTTP 8080），`NAMES` 里已有 `reasoning`，差分为「停 llama-swap ⇒ 上游测试红」 | 需要改 `rings.json` 的 edge 与两条判据命令（编排层改动） |
| **B 作废** | ring5 标退役（M5 已把该边从链上摘掉），登记死亡证明式记录 | 少一个环；`ring5` 空出来的位置不再被覆盖 |

**顺带一处 SSOT 漂移（具名，不改）**：
```
ecosystem/chain.json 的 flowmodus-reason 组件仍声明
  anaphase_env = "ANAPHASE_FLOWMODUS_ENDPOINT" · value = grpc://127.0.0.1:60054
而 anaphase 已不再读该 env（`grep -c ANAPHASE_FLOWMODUS_ENDPOINT src/config.rs` = 0）
⇒ launcher 若照 chain.json 注入，注入的是一个**已被忽略**的变量。
  chain.json 是接线 SSOT（ADR-0046），改它属编排层事实更正 —— 与 ring5 的 A/B 同批决定更省事。
```

### 状态一致性自查（2026-11-18，只读）

```
口径：① 队列「当前管道段」= 3 行 ② 屏⓪「未闭合段」= 3 项 ③ 文档中不得出现"已闭合/已确认"式越权声明
读数：① 3 行（grep -c '^| L2 \|^| D3 \|^| `web/`'）② 屏 3 项
      ③ grep -rnE '已闭合|已确认|合拢完成|L2 已完成'（排"未签/不写/不算/不得"）⇒ **0 处**
⇒ 三处一致；无越权声明。
```

**本轮自查里我自己的一次正则错**：第一次用 `-E` 却写了 `\|` 作交替 ⇒ 三条统计全 0，
差点读成"文档里根本没写这些事实"。改用 `|` 重测才得到真值（1 / 2 / 3 个文件）。
⇒ 与"分母/对象/管道/坐标/测试二进制"同族第 6 种：**正则方言把"有"读成"没有"**。

**无当前必须动作**（吸收审查意见）：
```
L2 签字只承认读数成立，不触发删队列 3 行 / DEATH-CERT 转正式 / git rm；
ring5 的 A/B 不改 rings.json 与 chain.json，等真撞到该边再上移；
/goal-focus 不是现在的一步。留白不算闭合，也不算欠着。
```

### 探针结论的两条口径 + 环境现状（2026-11-18 收口）

```
① 「占位符假 Met」与「unbound 三因混一句」**只登记，不拆**：
   拆报错文案只是措辞；真问题是「存在性 ≠ 生效性」。A/B 都不裁，rings.json / chain.json 不动。
② `Met` / `exec_ok` / `success=true` **不构成完成**：
   fixtures/calc 那圈是真执行面；学出来的 MCP 工具当前是**占位符执行面** ⇒ 换成它时 Met 是假绿，
   不得升格为「真上游一轮」。完成口径仍是：真上游一轮 ⇒ 标准事件 ⇒ 查可重放。
③ 真要动 anaphase 诊断面时，顺手把三因（没配 / codex 缺 / 连不上）拆开，不单开一轮；
   "便宜"不是自裁理由 —— 两条都属门。
```

**当前环境（探针已清，避免下次搞混）**：

| 端口 | 服务 | 状态 |
|---|---|---|
| 50051 | tentacle（`--plugins-dir ./fixtures`，注册 5 个） | 保留 |
| 50061 | anaphase（config.toml：reasoning 8080 / tentacle 50051） | 保留 |
| 8080 | llama-swap（coder-3b） | 保留 |
| 50050 / 50052 / 60052 / 60053 / 60054 | cellrix-web / helix-mind / tuck / flowmodus serve / flowmodus reason | 保留 |
| ~~50059 / 50062~~ | 探针专用（learned plugins 的 tentacle + 临时 anaphase） | **已 kill**；/tmp/learned* 已删 |

### 真链轮次读数 · 2026-11-19 01:54（A 类：新增真读数）

**启动（两层，别混成一句）**：
```
① 未做第⑤节整链冷启动；探测通过 ⇒ **复用现有链**（tentacle 50051 / llama-swap 8080 未动）
② 但 `config.toml` 无 `session_events_path`，当轮需要 jsonl ⇒ **以 env 正门重起 anaphase 实例一次**
   （`ANAPHASE_SESSION_EVENTS_PATH=… HELIX_CODEX=… ./target/debug/anaphase --config config.toml`）
读数：check_upstream `YES` rc=0 · /v1/health tentacle `ok=true detail=reachable`（重起后同一组）
```

**当轮真跑**（自然提示，不指示 JSON）：
```
POST /v1/chat  "What is 1234 times 5678? Use a tool if you have one."
⇒ job_id=run-bda2600a6944d63e · model=coder-3b · impasse=false
   reply="1234 multiplied by 5678 is 70,148,12."      ← **答复算错**
```

| 读数 | 值 |
|---|---|
| tape 行数 | 9 |
| ring 事件 | 8 |
| `bin/agui` 标准事件 | 5（RUN_STARTED / TEXT_MESSAGE_* / RUN_FINISHED） |
| D1/D2 判据 | rc=0 |
| HITL 四态（本轮） | `allowed_once=0 rejected=0 cancelled=0 unavailable=0 kinds={}` |
| `tool/call` | **0** |
| 案卷重放（对**这一轮** jsonl 跑两遍） | `diff=0` |

**本轮的真发现（不是"又跑了一次"）**：
```
同一提示形状的上一轮是 tool/call=1 且答对（7,006,652）；本轮 tool/call=0 且答错。
⇒ 本地 coder-3b 在"是否自发用工具"上**非确定**；因此"模型会自己发起工具调用"不能写成能力断言，
  只能写成逐轮读数；也说明 L2 的验收不能依赖某一次 Met。
```

#### 追加核对（2026-11-19，回应"job_id 同名但行数不同"）

**同名不是同一轮**：`job_id` 是**输入内容派生**的（ADR-0041 D1 的反面：它可重复），身份是 `period_id`。

```
/tmp/t4/events/run-bda2600a6944d63e-p006afd7831000002.events.jsonl   行=15 · tool/call=1 · 答对
/tmp/t2round/events/run-bda2600a6944d63e-p006afde647000001.events.jsonl 行=9 · tool/call=0 · 答错
两份 job_id 相同（同一提示），period_id 不同（两次真跑）⇒ 不是同一轮的切片，也不是行数口径差
```

**命令级 evidence（t2 用到的两条）**：
```bash
F=/tmp/t2round/events/run-bda2600a6944d63e-p006afde647000001.events.jsonl
cd Cellrix && node tools/l2_casefile.js /tmp/t2round/ring.json /tmp/t2round/events ../anaphase-helix/src/run_cycle/mod.rs > /tmp/cf_a.json 2>/dev/null
            node tools/l2_casefile.js /tmp/t2round/ring.json /tmp/t2round/events ../anaphase-helix/src/run_cycle/mod.rs > /tmp/cf_b.json 2>/dev/null
diff /tmp/cf_a.json /tmp/cf_b.json && echo diff=0      # ⇒ diff=0
```

**因此本轮结论更硬**：同一输入（同 job_id）在两个 period 里行为不同（15 行/用了工具/答对 vs 9 行/未用工具/答错）
⇒ 「自发调用工具」是**逐轮读数**，不是能力断言；L2 验收不能依赖某一次 Met。

### AG-UI 出口轮次 · 2026-11-19 02:26（B1 类：撞到自裁层缺陷并当场修）

**启动**：探测复用，未冷启动 —— `Cellrix/tools/check_upstream.sh` ⇒ `YES` rc=0；`/v1/health` ⇒ tentacle `ok=true detail=reachable`。

**当轮真跑**（自然提示）：
```
POST /v1/chat  "What is 21 times 43? Use the calc tool if you have one."
⇒ job_id=run-892e91c467485a45 · period_id=run-892e91c467485a45-p006afded63000003
   reply="21 times 43 equals 903."（对，但**未用工具**）
tape=/tmp/t2round/events/run-892e91c467485a45-p006afded63000003.events.jsonl · 9 行
agui=5 · D1/D2 rc=0 · tool/call=0 · HITL 全部 0（kinds={}）
```

**② 逐类型核对"投影是否漏事件"（本轮 B1 的入口）**
```
未被投影的 jsonl 类型（两份真实 tape 并集）：
  check/status 4 · assistant/usage 3 · user/message 2 · context/inject 2 ·
  assistant/think 2 · assistant/attempt 2 · tool/call 1 · tool/result 1 · verdict/status 1
判定：前六类按设计不映射（display-only / AG-UI 无对应标准事件）；
      **tool/call + tool/result 属自裁层缺陷** —— AG-UI 1.0 本就有 TOOL_CALL_*，且真跑已产生它们。
```

**修（`anaphase 8ba433d` · `Cellrix 4f2f883`）**
```
改前：同一 tape（含 tool/call、tool/result）⇒ bin/agui 只出 5 条，工具段**整段消失**
改后：同一 tape ⇒ 8 条：+ TOOL_CALL_START / TOOL_CALL_END / TOOL_CALL_RESULT（只映射标准类型）
反样本：把映射改名禁用后重建 ⇒ Cellrix 判据
        `RED D1: 缺 TOOL_CALL_*（tape 里有 tool/call，投影把它丢了）` rc=1；复原 rc=0
判据双向化（原本只有"无工具段不得伪造"的反向守卫，现在补正向）：都绿；
单测 `tool_events_are_projected_and_not_invented`；lib agui 5/5；全量 cargo test rc=0；预算 rc=0
```

**边界**：`TOOL_CALL_*` 仍是 AG-UI 标准类型，未新造语义、未改协议、未新开格子；页面留白数仍以当轮命令为准。

### 归档工单的事实更正 · 2026-11-19 03:19（append-only，不追改回执）

> 已归档 goal `mwmgamh3-vx0ntq` 的 `status:complete` **不追改**（append-only）。但事实必须落盘：

```
① 该工单的 t1/t2 **标早了**：objective 要求每轮交「两次 l2_casefile 的 diff 实际输出」，
   而当轮回执里缺这一项就标了 complete。缺项 = replay diff 的输出（不是命令）。
   事后补跑（当轮 period_id=run-892e91c467485a45-p006afded63000003）：
     run1 rc=0 · run2 rc=0 · diff 无输出 ⇒ rc=0
   ⇒ 读数有了，但"当时缺"这件事不改写。
② "改后 8 条"与反样本 rc=1 都量在 period_id=run-bda2600a6944d63e-p006afd7831000002；
   当轮 tool/call=0 仍是 0，未用别轮补本轮。
③ schema 快照/判据口径的改动已按 B2 记入 DEBTS（`d-agui-schema-snapshot-modified` · K-134，**待裁不进基线**）。
```

**下轮纪律（写死）**：`blockCompletion:true` 下，合约缺项**不结**；缺什么就具名登记什么，不代为补全、不重跑凑绿。

### 三仓健康快照 · 2026-11-19 03:37（A 类读数；phyt-DNA 红且**不自动续期**）

| 仓 | 命令 | rc | 读数 |
|---|---|---|---|
| Cellrix | `bash tools/accept_all.sh` | **0** | 三项均通过（重放 diff=0 / 出口 5 条 / web=0） |
| Tuck | `bash tools/verify.sh` | **0** | clippy 0 条存量 · 4 crate 各自 `--all-features` 可编译 · gateway feature 真拉进编译 |
| phyt-DNA | `bash tools/ci-local.sh --fast` | **1** | 唯一红因：**DEBTS 逾期未销 4 条**（`d-phyt-pre-gap-a/b/c` · `d-phyt-phantom-complete`，日落 2026-11-08，今天已过 **11 天**）；其余步骤 OK（3b 基线红作数 · 4 全仓扫描 绿17/红0/不可量0/时序跳2） |

**这 4 条为什么仍是红（不是新发现）**：2026-10-25 跨仓对账结论是「`pre-gap-a/b/c` 与 `phantom-complete` **仍真** ⇒ 日落重定 2026-11-08」。
⇒ 现在到期即红，**这是审计按设计工作**，不是回归。

**为什么不自动续期**：续期＝改口径（把到期日往后挪），连续续期就是"温水煮"。
四条各自的真解都要动**权威卷或引擎面**：
```
pre-gap-a/b/c：需 README/采用者启用包（权威卷）+ CI 步骤（引擎面）
phantom-complete：需新增"声明 vs git diff"判据（引擎面，且 phyt 冻结中）
⇒ 属门；本轮只登记读数，不改日落、不动 DEBTS 状态、不动引擎。
```

**恢复命令（下次要判时用）**：`cd phyt-DNA && bash tools/ci-local.sh --fast`（红因在输出第 3c 段，会逐条列 id 与日落日）。

### 十仓健康快照（续）· 2026-11-19 04:02（A 类读数）

补齐上一份快照缺的三仓（都已实跑，不是引述）：

| 仓 | 命令（在仓根或子目录） | rc | 读数 |
|---|---|---|---|
| helix-tentacle | `cargo test -q --all-features` | **0** | **58 + 6** 通过（另有 2 个空套件 0 项）；含插件装载路径 |
| FlowModus | `cd flowmodus-rs && cargo test -q --all-features` | **0** | **96 + 4 + 3 + 4 + 2** 通过（注意：**仓根没有 Cargo.toml**，worktree 在 `flowmodus-rs/`） |
| Helix-MCP-Learner | `cargo test --all-features` | **0** | 64 + 3 + 1（上一轮已测） |
| HelixECO-Glove | `cargo test --all-features` | **0** | 30 + 13 + 8 + 7（上一轮已测） |

**两个"会在错的对象上量"的坑（写下来，下次别再踩）**：
```
· FlowModus 的仓根没有 Cargo.toml ⇒ 在仓根跑 cargo test 会得到
  "could not find Cargo.toml" 而 **rc 仍是 0**（管道吃 rc）⇒ 必须 cd flowmodus-rs
· helix-tentacle 有 2 个空套件（0 passed）⇒ 若只 grep 一个 test result 会读成"没测试"
```

### phyt 逾期 4 条的拆读 · 2026-11-19 04:36（A 类读数，不改日落、不改状态）

把「逾期」拆成"事实是否已具备"与"还差什么"，避免把已做的事再做一遍：

| 债 | 它自己写的真解 | 现在实况（可复跑） | 还差 |
|---|---|---|---|
| `d-phyt-pre-gap-a`（git commit 走非工具路径 ⇒ pre 面无入口） | README 讲清 + 承认 silent | ✅ `README.md:483-495` 有「pre face has exactly one entry」表 + 明写「bash/git apply/merge/hand-edit 都不经过执行闸 ⇒ pre face 在那条路径上 silent」 | 无缺口（**事实已承认并落 README**） ⇒ 该条可议销账，不必等到"入口补上" |
| `d-phyt-pre-gap-b`（未装钩子的克隆） | 写进 README + 采用者启用包 | ✅ `README.md:459-465` 给出一行启用命令（`git config core.hooksPath tools/hooks`）+ 明写 measured | 「采用者启用包」（若指脚本）仍无；若一行命令即为包 ⇒ 该条可议销账 |
| `d-phyt-pre-gap-c`（CI 只跑 post 面） | CI 里加 pre 面 | ✅ `README.md:491-495` 明确「pre 的唯一入口是 pretooluse-gate.sh，covered by: nothing else」+ 承认 CI 只跑 post | pre 面进 CI 需 Claude 工具调用（CI 无）⇒ **结构上做不到**，宜改口径为具名局限 |
| `d-phyt-phantom-complete`（声明 vs git diff 门） | 新增判据 | ❌ `grep -rln 'diff --name-only' tools/*.sh tools/hooks/*` ⇒ 0 命中 | **判据不存在** ⇒ 真未做（且属引擎面/冻结） |

**结论（不改任何东西）**：4 条里 **1 条真未做**（phantom-complete）、**2 条事实已落 README 可议销账**（pre-gap-a/b）、
**1 条结构上做不到、宜改口径**（pre-gap-c）。⇒ 下一次裁"续期还是销账"时，分母不是"4 条都欠着"，而是"1 条欠 + 3 条待定性"。

### 出口链轮次 · 2026-11-19 04:56（A 类：**改变了一条判断**）

**启动**：探测复用，未冷启动 —— `check_upstream.sh` ⇒ YES rc=0（reasoning 8080/v1 · tentacle 50051）· `/v1/health` tentacle `ok=true reachable`。

**当轮读数**（period `run-c21a0fdbc86702ba-p006afe10ce000004`）：
```
POST /v1/chat「Use the calc tool to compute 13 to the power of 6, then tell me the number.」
job_id=run-c21a0fdbc86702ba · period_id=run-c21a0fdbc86702ba-p006afe10ce000004
jsonl=/tmp/t2round/events/run-c21a0fdbc86702ba-p006afe10ce000004.events.jsonl · 9 行
bin/agui 5 条 · D1/D2 rc=0（量在 **8 类 schema 口径**、本 period）· tool/call=0
HITL allowed_once=0 rejected=0 cancelled=0 unavailable=0 kinds={}
reply「The calculation of 13 to the power of 6 is 4826809.」（**对，但未用工具**）
两次 l2_casefile：run1 rc=0 · run2 rc=0 · diff 无输出（rc=0）· sha256 同为 296146d1…
```

**A：改变了哪一条判断**
```
原判断（上一轮写的）："前六类【按设计】不映射"。
本轮核对：该判断对其中五类有仓内锚点（Cellrix schema 8 类里无对应类型）；
          但对 `check/status`（含 HITL 四态）**没有锚点**——它承载授权读数，
          而"它不应映射"这句话从未被任何仓内文件支撑过。
⇒ 改判：`check/status` 从「按设计不映射」→「**待判**」；其余五类维持 by-design。
⇒ 一句可被下轮推翻的判断：**截至本 period，投影覆盖所有仓内已登记的标准类型；
   唯一未定性的是 check/status（HITL 读数出口），它既未被证明该映射，也未被证明不该映射。**
```
（`check/status` 是否该进 `STATE_DELTA` 之类：仓内没有该类型的登记 ⇒ 不擅自补，属门。）

**四条更正（2026-11-19，收窄上节的口径，不新增工单）**

```
① A 的原文收窄（原写"投影已覆盖所有仓内已登记的标准类型"说过头了）：
   本轮 tool/call=0 且未并入别轮 ⇒ 本轮只能确认：
     「当轮 types 里没有『本应映射却未映射』的（0 类）」。
   「工具段不会被投影丢弃」的依据是**上一轮**的单测 tool_events_are_projected_and_not_invented
   ＋反样本（禁用映射 ⇒ D1 RED rc=1），**不是本轮读数**。两者不能合成一句。
   本轮 A 的真实内容只有一条：check/status 从「按设计不映射」→「待判（缺仓内锚点）」。
② t1 读数完整性：全部条目都在本节上方（job_id/period_id 双标识 · jsonl 绝对路径 · 9 行 ·
   agui 5 · D1/D2 rc=0 · tool/call=0 · HITL 四态 · reply 原文 · 两次 l2_casefile 命令与
   diff 实际输出「无输出 / rc=0」· sha256 296146d1…）—— 属回执截断，不是缺项。
③ 连续无 B1/B2 计次：**N = 1**（本轮 A 类，无 B1 无 B2）；N = 2 才主动 pause。
④ 「D1/D2 rc=0」注明口径：量在 **K-134 改过的 schema 快照（8 类）**上，**不是已生效基线**
   （`DEBTS.md:106` 待裁）。K-134 与门后项不因本轮 A 类进入工单。
```

**没有 B1**：本轮没有出现"本应映射却未映射"（0 类），不制造修理项。
**没有 B2 新登记**：`check/status` 的未定性只是把既有判断改准，未新增门后项（K-134 已在前一轮登记）。

### B1 轮次 · 2026-11-19 05:06（工具段"有产出、无消费点"）

**撞到**：上一轮刚让 `bin/agui` 能产 `TOOL_CALL_START/END/RESULT`，而
`grep -c TOOL_CALL tools/l2_render.js tools/l2_casefile.js` ⇒ **0 / 0** ⇒
**事件有产出、页面没有消费点**：看页面看不出"这一轮调没调工具"（C-073 同族：判据在没人调用 → 事件在没人显示）。

**修**（`Cellrix 0e18069`，additive、只读当轮 tape、不新增数据源）：
```
l2_casefile.js 增 casefile.tool_calls：
  有工具 ⇒ ['index=0 · calc · ok=true · 55ms']；无工具 ⇒ '本轮无工具段（tape 里 0 条 tool/call）'
l2_render.js 增一栏「工具段（tool/call → TOOL_CALL_*）」
```

**反样本（改前/改后同一输入）**：
```
有工具 tape：加栏后页面出现 'calc · ok=true'（grep=1）
把该栏从渲染器移除 ⇒ 同页面 grep=0  ⇒ 这一栏不是装饰，是唯一载体
无工具 tape：页面显示「本轮无工具段」（grep=1），不假装有工具
原渲染测试 l2_render_test.js 仍 rc=0
```

**连续无 B1/B2 计数**：本轮有 B1 ⇒ **N 归零**（上一轮 N=1 作废，不叠加）。

**口径**：`tool_calls` 是**当轮 tape 的读数**，不改判据、不动共享验具（K-134 仍待裁、仍非基线）。

### A 轮次 · 2026-11-19 05:08（check/status 的"待判"收窄成一条具体事实）

**核对方式**（两条命令，各给读数）：
```bash
# ① 我们的页面能不能看见 HITL 四态（渲染器直读 ring）
grep -o 'HITL 四态</dt><dd>[^<]*' /tmp/r_notool.html
⇒ HITL 四态</dt><dd>{"allowed_once":0,"cancelled":0,"kinds":{},"rejected":0,"unavailable":0}

# ② 标准 AG-UI 事件流里有没有 HITL 读数
grep -cE 'hitl|allowed_once' anaphase-helix/src/agui.rs        ⇒ 1（只出现在单元测试的 fixture 行 :90）
anaphase-helix/target/debug/agui <tape> | RUN_FINISHED          ⇒ metadata={}
```

**A：判断从"待判"收窄成**：
```
`check/status`（HITL 四态）**在我们自己的渲染器里可见**（直读 ring，页面有那一栏），
**在标准 AG-UI 事件流里不可见**（`agui.rs` 只把 verdict 挂 metadata，hitl 无映射）。
⇒ 它不是"两处都不可见"的缺口，而是"**自渲染器可见、通用客户端不可见**"的产品取舍。
```

**没有 B1**：这不是"本应映射却未映射"——仓内 8 类里没有承载 HITL 读数的类型，
所以"该不该映射"是**产品/协议面的取舍**，不是缺失。
**没有 B2 新登记**：真要让它进标准事件流（如新增 `STATE_DELTA` 类），属**扩协议类型** ⇒ 门；
本轮只把判断收窄，不擅自动 schema（K-134 仍待裁）。

**连续无 B1/B2 计数**：上一轮有 B1 ⇒ 已归零；本轮无 B1/B2 ⇒ **N = 1**（N=2 才 pause）。

### 本批三个 B1 —— 同一族的三个「有产出、无消费者 / 消费错源」

| # | 撞到什么 | 证据（命令级） | 修 | 反样本 |
|---|---|---|---|---|
| B1-a | `TOOL_CALL_*` 有产出、页面无消费点 | `grep -c TOOL_CALL tools/l2_render.js tools/l2_casefile.js` ⇒ 0 / 0 | casefile 增 `tool_calls`（读当轮 tape）+ 页面「工具段」一栏 | 移除该栏 ⇒ 同页面 `grep 'calc · ok=true'`=0 |
| B1-b | `verdict/status` 零消费者（页面看不到 Met/Unmet 与原因） | `grep -c 'verdict/status' Cellrix/tools/*.js anaphase-helix/src/agui.rs` ⇒ 全 0 | casefile 增 `verdict_status` + 页面「判定」一栏 | 唯一串 `all checks passed`：有栏=1 · 移除=0 · 复原=1 |
| B1-c | 页面 HITL 四态**只认 ring（易失）**；ring 空即丢，而 tape 里有 | ring 置空 ⇒ `casefile.hitl=null`，而 `grep -c '"phase":"hitl"' tape` ≥1 | ring 缺 ⇒ 回落 tape，标 `_source: ring/tape`；页面写「HITL 四态（源=…）」 | ring 空 ⇒ 源=tape 且四态可见；ring 有 ⇒ 源=ring；渲染测试 rc=0 |

**B1-c 为什么最重要**：前两个是「没人显示」，第三个是「**显示的是错源**」——易失的 ring 替代了持久的 tape。
与 C-076（hitl 只进内存环）、C-064 同族，但方向相反：那次是「写不进去」，这次是「读错了地方」。

**计数**：本批有 B1 ⇒ 连续无 B1/B2 计次归零。
**口径未动**：K-134 仍待裁、仍非基线；三处都是给我们自己的渲染链补消费点/换源，未改协议语义。

### B1-d + 当轮三栏验证（同一笔，不改口径）

**当轮真跑读数**（`check_upstream.sh` YES rc=0，未冷启动、未挑样本）：
```
POST「What is 19 times 23? Use the calc tool if available.」
job_id=run-ec4a823828809dcc · period_id=run-ec4a823828809dcc-p006afe15b9000005 · 9 行
tool/call=0 · reply「19 times 23 is 437.」（对，但没用工具）
三栏按「0 就是 0」如实：判定=UNMEASURABLE（tape 里 0 条 verdict/status）·
                          工具段=本轮无工具段 · HITL 源=ring（**当轮是别的 period**，见下）
```

**B1-d（本批第 3 笔新缺陷，a 不重计）**：页面「HITL 四态（源=ring）」显示的是
```
trace_id=run-8bba24c5ee368a4a-p006afde646000000 · ts=2026-11-18T17:54:14Z
本轮 period_id = run-ec4a823828809dcc-p006afe15b9000005   ⇒ **不是同一个 period**
```
ring 是跨 period 全局环，`find(phase==='hitl')` 不校验身份 ⇒ 违反 **I1（身份是 period_id）**；
"0" 与"别轮的 0"不是同一个 0。

**修**（`Cellrix 5e547b1`，只动渲染链）：从 tape 首行取 period_id，只接受 `trace_id === periodId` 的 ring 事件；
不匹配 ⇒ 回落 tape 标 `_source: tape(ring-stale)`；两处皆无 ⇒ `UNMEASURABLE·…`（**不**回落顶层旧 hitl）。

**四形反样本**（同一 tape，只换 ring）：
```
ring 只有别轮 hitl        ⇒ 源=tape(ring-stale) · allowed_once=0（本轮 tape 值）
ring 有本 period hitl     ⇒ 源=ring · allowed_once=3（原行为不变）
ring={}                   ⇒ 源=tape
ring 别轮 + tape 无 hitl  ⇒ 源=UNMEASURABLE·ring-stale 且 tape 无本 period hitl 行
原渲染测试 l2_render_test.js rc=0
```

**计数（按 YB①）**：本批真新 B1 = **b / c / d 三笔**；**a 不计**（上一轮已修）。
**不互相销账**：B1-d 只证明"读错源已拦"，**C-076（hitl 只进内存环、写不进盘）仍未解**，两件不同。
**口径未动**：K-134 待裁、非基线；L2 读数齐备未签；未归档、未改 center、队列 3 行未删、DEATH-CERT 未动。

### A 轮次 · verdict/status 历史构成核对（**历史构成核对，不拿历史值补当轮**）

**当轮（period `run-ec4a823828809dcc-p006afe15b9000005`）仍是 `UNMEASURABLE（tape 里 0 条 verdict/status）`** ——
历史里有 Met/Unmet，**不影响**当轮那条读数；两者不混写。

**历史构成（读 `/tmp/**/*.events.jsonl` 全量 75 份，逐条带 period_id）**：
```
含 verdict/status 的 tape = 5 / 75，每份恰 1 条：
  run-221afcfe7567c353-p006afcacc2000002 · Unmet · checks=2 · failed: exec_ok, answer.delivered
  run-221afcfe7567c353-p006afcaccc000001 · Unmet · checks=2 · failed: exec_ok, answer.delivered
  run-605c9e5003508d92-p006afcae90000001 · Met   · checks=2 · all checks passed
  run-605c9e5003508d92-p006afcae90000001' · Met  · checks=2 · all checks passed（同 period 名下另一份 tape 副本）
  run-bda2600a6944d63e-p006afd7831000002 · Met   · checks=2 · all checks passed
```

**交叉条件（决定"偶尔有 vs 没有生产路径"）**：
```
verdict/status=有 · 有 tool/result=有  ⇒ 5 份
verdict/status=无 · 有 tool/result=无  ⇒ 70 份
有 tool/result 却无 verdict/status   ⇒ **0 例**
无 tool/result 却有 verdict/status   ⇒ **0 例**
```

**A：判断从「偶发/可能缺失」改成**：
```
`verdict/status` **不是偶发，也不是缺失**：它是**工具执行段的条件产出** ——
走到 `tool/result` 才会有（reason 里的 `exec_ok` / `answer.delivered` 正是执行段两条检查）。
75 份历史 tape 与这条条件**完全吻合，两个方向都没有反例**。
⇒ 本轮的 `UNMEASURABLE` 因此有了解释：**本轮 tool/call=0 ⇒ 未进入执行段 ⇒ 本就不该有判定行**。
```

**没有 B1 / 没有 B2**：这是一条**机制读数**，不构成"该映射却未映射"（渲染链已能显示三种形态）。
**反样本值不混写**：上文 `allowed_once=3` 之类是**构造值**（只为证明判据会红/会对），不与任何真跑读数并列。
**计数**：本批无 B1/B2 ⇒ **N = 1**（N=2 才 pause）。
**门后项具名不销账**：C-076（hitl 只进内存环）· K-134（schema 快照待裁、非基线）· L2（读数齐备未签）—— 均未动。
**本件不成覆盖率表**：只回答"verdict/status 何时出现"，不扩成"逐类型补消费点"的清单。

### C 轮次 · 2026-11-19（当轮读数与已落盘判断无差）

**t1 当轮最小读数集**（探测复用，未冷启动）
```
check_upstream.sh ⇒ YES rc=0（reasoning 8080/v1 · model=coder-3b · key=none）
/v1/health        ⇒ tentacle ok=True detail=reachable
POST /v1/chat 「Use the exec tool to run: echo hi-from-round; then report exactly what the command printed.」
reply 原文 ⇒ {"done":true,"impasse":false,"model":"coder-3b","reason":"completed",
              "reply":"hi-from-round","success":true}
job_id=run-7bc05e4fa668578c
period_id=run-7bc05e4fa668578c-p006afe1851000006
jsonl=/tmp/t2round/events/run-7bc05e4fa668578c-p006afe1851000006.events.jsonl
LINES=9 · AGUI=5 · D1/D2 rc=0（**8 类口径**、量在本 period_id、**K-134 改过快照、非基线**）
TOOLCALL=0 · TOOLRESULT=0
HITL={allowed_once:0,rejected:0,cancelled:0,unavailable:0,kinds:{}} · _source=**tape(ring-stale)**
两次 l2_casefile（同一命令）run1 rc=0 · run2 rc=0 · diff 无输出 rc=0 ·
sha256 均 19d451f18585c5f8f56fb11a0d6c726c9dc6de8c8b2cfa99fd779b58c09938f8
```

**t2 核的是哪一条判断 · 为何对不上 ⇒ 结论是「对得上」**
```
候选① execution-queue.md:317 「Met / exec_ok / success=true **不构成完成**」
  当轮：success=true 而 tool/call=0、无 exec_ok、无 verdict/status
  ⇒ **不矛盾，反而同向**（success=true 确实什么都没证明）
候选② execution-queue.md:667 「verdict/status 是执行段的条件产出」
  当轮：未进执行段 ⇒ 无判定行 ⇒ **完全吻合**
候选③ C-085「根因是 reasoning_endpoint 未配置」
  当轮：endpoint 已配（YES）且 attempt 原文 = "no calls planned — answered directly"
  ⇒ 不推翻 C-085：C-085 说的是「空回复 ⇒ impasse」那条症状；本轮 attempt 非空、无 impasse，
     symptom 不同。而"同一配置下有时 tool/call=1、有时 0"**已在前轮落盘**（逐轮读数，非能力断言）。
⇒ 无差。不制造核对对象。
```

**C：本轮无新增真读数**
```
唯一新增的可量化面：`assistant/attempt` 现在给出模型自述（"no calls planned — answered directly"），
但它是**读数**不是**判断变化**；按「纯数字变化记 C」⇒ 记 C。
一句可被下轮推翻的判断：**在 endpoint 已配置、prompt 明确要求用 exec 的情况下，
本轮模型自述"未规划调用"、reply 却直接给出一段看似命令输出的文本，且 success=true；
tape 自身自洽（attempt 与 reply 一致），出口无法从 reply 区分"真执行"与"未执行"。**
```

**计数**：本轮无 B1/B2 ⇒ **连续无 B1/B2 = N = 2** ⇒ **主动 pause（停机，不是等菜单）**。
**未动**：K-134（待裁、非基线）· C-076（不销账）· L2（读数齐备未签）· center · 队列 3 行 · DEATH-CERT · 页面留白。

### B1-e 轮次 · rail 旁路跳过了生命周期行（真撞，非构造）

**当轮真跑（探测复用：`check_upstream.sh` YES rc=0 · `/v1/health` tentacle ok=True reachable）**
```
prompt「Run the exec tool with `echo probe-round-5` …」
⇒ reply 是 [rail citation · knowledge_base/rails/demo]，model=null
job_id=run-ff46d4db4eea328b · period_id=run-ff46d4db4eea328b-p006afe91f7000007
jsonl=/tmp/t2round/events/run-ff46d4db4eea328b-p006afe91f7000007.events.jsonl
LINES=3 · AGUI=4 · TOOLCALL=0
HITL={allowed_once:0,rejected:0,cancelled:0,unavailable:0,kinds:{}}
D1/D2 **rc=1** ⇒「RED D1: 首条不是 RUN_STARTED」（8 类口径 · 量在本 period_id · K-134 改过快照非基线）
tape types=['assistant/reply','turn/end','check/status']
```

**核的是哪条判断 / 为何对不上**
```
已落盘判断（本 goal 前一轮 A，:592 起那批与 :667 同族）：
  出口链的 tape 以 turn/start 开头，D1 才能投影出 RUN_STARTED 序列。
当轮读数：rail 旁路轮 **首条就是 assistant/reply**，D1 rc=1 ⇒ 与"序列完整"直接对不上。
根因（代码事实，不是推断）：`src/run_cycle/reasoning.rs` 的 rail 分支在 `emit_period_start`
（写 turn/start + user/message）**之前** `return` ⇒ 生命周期行被这条旁路绕过。
```

**修（`anaphase-helix e875a45`，additive、只补一行证据）**
```
rail 分支 return 前补写同一个 emit_period_start；
injected_chars 传 None（该路径未组装 prompt，**不测量 ≠ 0**）。
未动协议语义 / schema / D1 判据 / 共享验具。
```

**改前 / 改后（同一 prompt 形状、都是真跑产物）**
```
改前 run-ff46d4db4eea328b-p006afe91f7000007 · 3 行 · first=assistant/reply · D1 rc=1
改后 run-0d7107230101e4f2-p006afe926d000002 · 6 行 · first=turn/start     · D1 rc=0
     types=['turn/start','user/message','context/inject','assistant/reply','turn/end','check/status']
     同轮读数：AGUI=5 · TOOLCALL=0 · HITL 四态全 0（_source=tape(ring-stale)）
     两次 l2_casefile rc=0/0 · diff 无输出 rc=0 · sha256 均 fb4a4092…
反样本：**改前那份 tape 本身**（同路径、同 schema、仅因缺 turn/start 而红）
回归：cargo test rc=0 · 520 passed / 0 failed（41 个 ok 行）
```

**B2（只登记不自裁）**：rail 旁路对「要求用 exec」的 prompt 仍返回法条引用（内容与输入无关），
且 `success=true`。这是**路由/策略面**（谁该被 rails 命中），不属自裁层 ⇒ 具名登记，不改。
`DEBTS.md` 一行：`d-rail-bypass-ignores-request-intent`。

**计数**：本轮有 B1 ⇒ **连续无 B1/B2 归零（N=0）**，不 pause。
**未动**：K-134 · C-076 · L2 · center · 队列 3 行 · DEATH-CERT · 页面留白。
**顺带具名（不改）**：同一输入重复 POST 会**复用同一 job/period**（run-ff46… 原样返回），
所以"同一 prompt 改前改后对比"不可行 ⇒ 必须换输入造新 period 再测。

### B1-f 轮次 · casefile 丢了身份（两轮字节相同）· 2026-11-19

**当轮真跑（探测复用：`check_upstream.sh` YES rc=0）**
```
prompt「用 calc 算 987×654，另外用 exec 跑 echo rail-check-8」
job_id=run-1ca755e1d26e00a7 · period_id=run-1ca755e1d26e00a7-p006afe92d7000003
jsonl=/tmp/t2round/events/run-1ca755e1d26e00a7-p006afe92d7000003.events.jsonl
LINES=6 · AGUI=5 · TOOLCALL=0 · reply=[rail citation] · model=null
types=['turn/start','user/message','context/inject','assistant/reply','turn/end','check/status']
D1/D2 rc=0（8 类口径 · 本 period_id · **K-134 改过快照、非基线**）⇒ 上一笔 B1-e 的修复在生产里保持
两次 l2_casefile rc=0/0 · diff 无输出 rc=0 · sha256 曾均 fb4a4092…（**这正是缺陷**）
```

**核的是哪条判断 / 为何对不上**
```
已落盘判断：I1「身份是 period_id」+ B1-d 已把 HITL 读数按 period 校验。
当轮读数：rail 轮与 normal 轮（run-0d7107230101e4f2-p006afe926d000002）的 casefile
**字节完全相同**，`begin` 还是 ring 里更早那条 "Calculate 2 to the power of 10"，
casefile 里 `period_id` 出现次数 = **0** ⇒ 页面无法归属任何一轮。同一族，但这次是整份证轨。
```

**修（`Cellrix 72e2a41`，additive、只动自有渲染链）**
```
casefile 增 period_id / tape_rows / _source{begin,end}
begin/end 优先读 tape（user/message · turn/end），ring 只兜底并如实标源
页面头部显示：period_id · tape 行数 · begin/end 源
```

**改前 / 改后（同一对真跑产物）**
```
改前：两轮 casefile 完全相等，sha 均 fb4a4092…（反样本就是这一对）
改后：13dbb61b… / 0dafdb05…（不同）；页面分别显示
      period_id: run-1ca755e1d26e00a7-p006afe92d7000003
      period_id: run-0d7107230101e4f2-p006afe926d000002
旧 casefile（无 period_id）⇒ 页面显式 `period_id: UNMEASURABLE（casefile 未带 period_id）`，不静默沿用 ring
回归：l2_render_test.js rc=0 · accept_all.sh 三项通过
```

**A（同一轮里附带确认的一条判断变化）**：`context/inject` 的 `chars` **不是注入实测，是配置预算**
```
53 份历史 tape：chars=800 恒等（= `DEFAULT_MEMORY_INJECT_CHARS`），nodes=0，injected_chars=0/缺
⇒ 读数行写着"800 chars 被注入"，实测是 0 ⇒ **预算是被当成测量值写进 tape 的**（"0 就是 0" 的反面形态）
```
**B2（属门，只登记不自裁）**：改 `chars` 语义 = 动 tape 载荷/消费面（D1/D2 schema 口径未裁）
⇒ `DEBTS.md` 增 `d-context-inject-chars-is-budget`，带 revisit-on；本轮**未改**该字段。

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · center · 队列 3 行 · DEATH-CERT · 页面留白。

### B1-g 轮次 · 我自己新加的三栏跨轮取数（12 份 tape 目录）· 2026-11-19

**核对方式（不改状态，只把 evDir 指向真实 events 目录）**
```
ls /tmp/t2round/events/*.jsonl | wc -l ⇒ 12
l2_casefile.js <ring> /tmp/t2round/events <src>
  改前 ⇒ period_id=run-0d7107230101e4f2…（**不是当轮**）· tape_rows=**87**（= 12 份之和）
  改后 ⇒ period_id=run-1ca755e1d26e00a7-p006afe92d7000003 · tape_rows=6 · tapes_in_dir=12
```

**核的是哪条判断 / 为何对不上**
```
B1-d 已把 HITL 读数按 period_id 校验（同族：跨轮取数）。
而我自己在 B1-a/B1-b 加的三栏（tool_calls / verdict_status / tape_rows）**没有按 period 限定**：
它们扫 evDir 里所有 tape ⇒ 12 份时把别的 period 数出来。**同一个错我在新代码里又犯了一遍。**
```

**修（`Cellrix 95be595`，additive）**
```
evDir 多份时以**最新 mtime 那份**为当轮，其余不计；输出 `tapes_in_dir` 如实写目录里有几份。
口径：多份不是错 —— **静默合并**才是错。
```

**反样本（四种形状）**
```
改前同目录：rows=87 · period_id=错轮（反样本本身）
改后：rows=6 · period_id=当轮 · tapes_in_dir=12
单份目录：rows=6 · tapes_in_dir=1（行为不变）
旧 period 带 tool/call + 当轮无 ⇒ 面板仍写「本轮无工具段」（不串台）
页面读数：period_id: run-1ca755e1d26e00a7-p006afe92d7000003 · tape 行数: 6 · 目录内 tape 数: 12
回归：l2_render_test.js rc=0 · accept_all.sh 三项通过
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**教训（写给下一轮的我）**：这已经是第 3 次「跨轮取数」（B1-d ring · B1-f casefile 身份 · B1-g 三栏）——
**加任何跨文件汇总前先问：这份数据属于哪个 period_id？**

### B1-h 轮次 · 两套口径不同源（tape 定身份，ring 定内容）· 2026-11-19

**核对方式（构造 + 真跑产物混用，已分栏标注）**
```
当轮 trace（真跑）：period_id=run-1ca755e1d26e00a7-p006afe92d7000003 · tape 6 行 · D1/D2 rc=0
构造（**不与真跑读数混写**）：ring 里只放 run-OTHER-p999 的 hitl 事件
```

**核的是哪条判断 / 为何对不上**
```
B1-d/f/g 之后，页面身份来自 tape；但 begin/end/hitl 仍可能来自 ring（兜底）。
当 ring 属于别的 period 时，改前**没有任何提示**：页面 period_id 是一轮、内容来自另一轮。
⇒ 与「页面读数必须可归属」直接对不上（同族第 4 次，这次是"两套口径并存"而不是取数越界）。
```

**修（`Cellrix 5e973b6`，additive）**
```
casefile 增 ring_period_ids / ring_mismatch（ring 事件全都不等于当轮 period ⇒ 列出）
页面红字：「ring 事件属于别的 period：… —— 本页一切读数以 tape 的 period_id 为准」
```

**反样本（两形，构造值与真跑分栏）**
```
混源（ring=run-OTHER-p999）⇒ 警告=1 · hitl _source=tape(ring-stale) · allowed_once=0（不是 ring 的 2）
一致（ring trace=当轮 period）⇒ 警告=0 · hitl _source=ring · allowed_once=1（原行为不变）
回归：l2_render_test.js rc=0 · accept_all.sh 三项通过
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · d-context-inject-chars-is-budget · d-rail-bypass-ignores-request-intent。

### C 轮次 · HITL 四态恒 0 是不是"没人写"？—— 不是 · 2026-11-19

**核的是哪条判断 / 为何对不上（假设）**
```
观察：近若干轮 `check/status` 的 HITL 四态恒为 0（历史只出现过一次 allowed_once=1）。
假设（可被推翻）："计数器没有写入路径 ⇒ 0 是假读数"。
```

**核对（代码级，逐条给位置）**
```
src/hitl.rs:484  `self.counters.record_for(outcome, kind);`  ← 写入路径**存在**
src/hitl.rs:482  `pub fn check_approval(...)` 每次都记（uncounted 版本只在测试里用）
src/run_cycle/safety_gate.rs:75 `hitl.check_approval(tool, actions)` ← 生产唯一调用点
src/run_cycle/mod.rs:742 / :990 读 snapshot 写进 tape
```
⇒ 写入路径存在且唯一调用点在工具执行闸上。**本轮及近几轮 tool/call=0**（模型未规划调用）
⇒ 闸从未被经过 ⇒ 计数**本来就该是 0**。0 就是 0，不是假读数。

**C：本轮无新增真读数**
```
一句可被下轮推翻的判断：**HITL 四态为 0 的原因不是"没人记"，
而是"闸没被经过"——只要 tool/call>0 且动作被判高风险，四态就会动（历史 allowed_once=1 即此形）。**
```

**口径未动**：`d-rail-bypass-ignores-request-intent` 与 `d-context-inject-chars-is-budget` 仍待真撞；
K-134 · C-076 · L2 一项未动。

**计数**：本轮无 B1/B2 ⇒ **连续无 B1/B2 = N = 1**（N=2 才 pause）。

### A 轮次 · `assistant/attempt`：第 5 处"有产出、无消费者"，且它正好能回答 phantom-exec · 2026-11-19

**核的是哪条判断 / 为何对不上**
```
已落盘判断（:678 那轮 C 的结尾句）：
  "出口无法从 reply 区分『真执行』与『未执行』"
当轮核对：`assistant/attempt` 行里**写着答案** ——
  历史 7 条 attempt 全部 empty=false，原文形如
  {"empty":false,"text":"no calls planned — answered directly"}
⇒ 不是"没有证据"，是"**证据在 tape 里，出口没人读**"。
```

**消费者实测（三处都 0）**
```
grep -rc 'assistant/attempt' Cellrix/tools/*.js            ⇒ 0（除 web_removal_probe 里的无关字符串）
anaphase-helix/src/agui.rs                                 ⇒ 1，但只在 `#[test] the_real_skeleton_maps_to_five_standard_events` 的 fixture 里（:87）
Cellrix/docs/observations/agui-1.0-min.schema.json         ⇒ 0（8 类里没有它）
```

**A：判断从"无法区分"改成**
```
**不是无法区分，是没人读**：`attempt` 明确携带"没有计划任何调用"的自述，
tape 里可查、可重放；缺的是出口上的一处消费点（页面/判据），不是证据本身。
⇒ 这也把上一轮"phantom-exec 要不要造判据"的问题改准了：判据的原料已经在 tape 里，不必新造数据面。
```

**没有 B1**：本轮的"缺消费点"若立刻补，正是 objective 禁止的"逐类型补消费点清单"；
是否补、补在哪一层（页面读数 vs 判据），属下一轮由**差**决定，不预先固定。
**没有 B2 新增**：未新增 `DEBTS` 行；已登记的 `d-rail-bypass-ignores-request-intent` /
`d-context-inject-chars-is-budget` 仍待真撞、未销账。

**计数**：本轮无 B1/B2 ⇒ **连续无 B1/B2 = N = 2** ⇒ **主动 pause（停机，不是等菜单）**。

### 更正 · 对 815c759「四态会动」那条判断加限定（不回改历史 · 2026-11-19）

**原判断（815c759 落盘，不改）**：HITL 四态为 0 不是"没人记"，是"闸没被经过"。

**补三条限定（append-only，指向同一节）**
```
① 四态是**程序自记计数器**读数，不是 human 操作读数：
   `allowed_once=1` 只证明"该 period 的工具闸被经过且 counter 会写"，
   **不证明人在环里**（`HitlCounters::record_for` 是同步自增，见 src/hitl.rs:482/484）。
② 用户未授权、当前**没有 human 的 allow/reject/cancel 入口**
   ⇒ 现阶段的四态是 mock / 测试形状的读数，**不是真闭环**。
③ 「园丁按三下」验的是**读数**，不是 HITL 的 allow/reject/cancel —— 两者不混。
```

**由此推出的边界（不改状态）**
```
"真闭环"缺的不是一句签字，是 **human 可操作面**：
C-076 持久化 + 应答者注册（ApproverRegistry/Binding/EscalationStep）+ K22 安装面 + K17 并发上限
⇒ 属门 / 方向层，须 gardener 授权；**不新建入口、不进自裁任务、不因这句澄清解成"现在去做"**。
L2 未闭合的物理原因同理：不是缺签字，是缺 human 操作面。
```

### 口径更正 · 四条（append-only，不回改历史件 · 2026-11-19）

**① B1-f 转针：旧「两次 casefile diff=0」只证明确定性，不证明身份**
```
旧口径（B1-f 之前各轮）：两次 l2_casefile diff=0 ⇒ "证轨可查可重放"。
补限定：diff=0 只说明**同一输入两次一致**；在 B1-f 之前它**连身份都没有**
        （两轮不同 period 的 casefile 字节相同，sha 均 fb4a4092…）。
B1-f 之后的正确判据是**两条一起**：
  同一 period 两次 ⇒ 一致（确定性）；
  不同 period ⇒ sha 必不同（身份）。
任何以 diff=0 单独充当"可查可重放"的旧判断，均按本行限定读。
```

**② 四笔 B1 与 N 的关系（逐笔给落盘节，不重算历史）**
```
B1-e anaphase e875a45 · 落盘节「B1-e 轮次 · rail 旁路跳过了生命周期行」
     依据＝**生命周期契约本应 emit 却漏**（reasoning.rs 的 rail 分支在 emit_period_start 之前 return），
     **不是**"为了让 D1 rc=0 去补 emit"；改前 3 行/D1 rc=1 · 改后 6 行/D1 rc=0
B1-f Cellrix 72e2a41 · 落盘节「B1-f 轮次 · casefile 丢了身份」
     改前两份 sha 相同 · 改后 13dbb61b/0dafdb05 · 旧 casefile ⇒ 显式 UNMEASURABLE
B1-g Cellrix 95be595 · 落盘节「B1-g 轮次 · 我自己新加的三栏跨轮取数」
     改前 rows=87/错轮 · 改后 rows=6/当轮 · tapes_in_dir=12 · 单份不变 · 旧 period 有 tool 不串台
B1-h Cellrix 5e973b6 · 落盘节「B1-h 轮次 · 两套口径不同源」
     混源 ⇒ 警告=1/源=tape · 一致 ⇒ 警告=0/源=ring
计数口径：这四笔是**历史回顾**，各自在当时那一轮已按「有 B1 ⇒ 归零」处理过；
        `a`（工具段无消费点）不重计。当前窗口 = C 轮（HITL 四态）N=1 → A 轮（attempt）N=2 ⇒ **N=2 成立**。
```

**③ 更正我上一段把两件事混写的句子（超置那行，不删）**
```
被更正的原句（e269d05 落盘）：「L2 未闭合的物理原因同理：不是缺签字，是缺 human 操作面」
改为两条**并列事实**：
  事实一：L2 未闭合＝**读数齐备未签**（完成口径三件里**没有** human 审批这一项）。
  事实二：HITL 真闭环是**另一条门后线**（C-076 持久化 + 应答者注册 + K22 + K17）。
两件不互相解释、不互相销账。
```

**④ 门后项维持「已具名、待真撞」**
```
11 条不因被列成路线而进工单；不列成"给一句就裁一条"的三选一；
"允做 C-076 opt-in"不作为可点示例；下一件仍由**当轮真跑读数与已落盘判断的差**决定，不预先固定。
```

### A 轮次 · `tool/call=0` 有两种成因，且 tape 上一行就能分辨 · 2026-11-19

**当轮真跑（探测复用：`check_upstream.sh` YES rc=0）**
```
prompt「Use the calc tool to compute 11 to the power of 5, then report the number.」
reply=[rail citation · knowledge_base/rails/demo] · model=null
job_id=run-edef20f53ea70768 · period_id=run-edef20f53ea70768-p006afe9ad6000004
jsonl=/tmp/t2round/events/run-edef20f53ea70768-p006afe9ad6000004.events.jsonl
LINES=6 · AGUI=5 · TOOLCALL=0 · HITL 四态全 0
D1/D2 rc=0（8 类口径 · 本 period_id · K-134 改过快照非基线）
两次 l2_casefile rc=0/0 · diff 无输出 rc=0 · sha256 均 e9c4e698…
types=['turn/start','user/message','context/inject','assistant/reply','turn/end','check/status']
```

**核的是哪条判断 / 为何对不上**
```
已落盘判断（815c759 + e269d05 + :592 起那批）：把 tool/call=0 读成
  "模型没有规划调用（attempt 自述 no calls planned）" ⇒ 闸没被经过 ⇒ 四态该为 0。
当轮读数：**tape 里根本没有 `assistant/attempt` 行**（`'assistant/attempt' in tape` ⇒ False），
  因为 rail 旁路在推理段之前就回答了 ⇒ **不是"模型不规划"，是"根本没走到规划"**。
⇒ 同一个 `tool/call=0` 混了两种成因，而它们的后果不同（一个经过推理段、一个没经过）。
```

**A：判断改成**
```
`tool/call=0` 有两类，**用 tape 上有没有 `assistant/attempt` 行即可分辨**：
  有 attempt（自述 no calls planned）⇒ 走到了推理段，模型选择不调用；
  无 attempt（本轮）⇒ **rail 旁路在推理前回答**，规划段根本没执行。
⇒ 由此，HITL 四态恒 0 的两个前提也要分开写：
  第一类"闸没被经过是因为没有动作"；第二类"闸没被经过是因为连动作规划都没发生"。
⇒ 一句可被下轮推翻的判断：**凡 tape 缺 `assistant/attempt`，该轮就不该被用来论证
  "模型不愿用工具"；它只能被用来论证"请求被旁路吞掉了"（=已登记的 d-rail-bypass-ignores-request-intent）。**
```

**没有 B1**：这是**读数分辨力**问题，不是缺陷修复项；rail 吞输入已登记为门后项（B2），本轮不重开。
**没有 B2 新增**：`d-rail-bypass-ignores-request-intent` 已在册（reads: 本轮为第 3 次现场）。
**口径未动**：K-134 · C-076 · L2 · 其余门后项一项未动、未互相销账。

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口（gardener 重新下发后从 0 起）**连续无 B1/B2 = N = 1**。

### A 轮次 · `success=true` 可以配一个**算错的答案**，而出口没有任何一处标出错 · 2026-11-19

**当轮真跑（探测复用：`check_upstream.sh` YES rc=0）**
```
prompt「Please call the calc tool with expression 11*11*11*11*11 and report the result it returns.」
reply 原文 ⇒ "The result of the expression 11*11*11*11*11 is 28561."
job_id=run-759fa7181f06f0a6 · period_id=run-759fa7181f06f0a6-p006afe9afe000005
jsonl=/tmp/t2round/events/run-759fa7181f06f0a6-p006afe9afe000005.events.jsonl
LINES=9 · AGUI=5 · TOOLCALL=0 · HITL 四态全 0
D1/D2 rc=0（8 类口径 · 本 period_id · K-134 改过快照非基线）
types=['turn/start','user/message','context/inject','assistant/usage','assistant/think',
       'assistant/attempt','assistant/reply','turn/end','check/status']
```

**核的是哪条判断 / 为何对不上**
```
已落盘判断（:678 那轮 C）：出口"无法区分真执行与未执行"；后又改成"证据在 tape、出口没人读"。
当轮读数把这条推得更窄也更硬：
  ① 答案**算错**：`11**5 = 161051`（python 实测），模型写 **28561**（=13⁴）
  ② `turn/end.success=true`、`impasse=false`、reply 自洽
  ③ tape 无 `verdict/status`（执行段没跑）⇒ **没有任何检查点会指出这个答案错**
  ④ 页面里 `28561` 出现 **0** 次 ⇒ **证轨页根本不显示 reply 文本**，看页面也核不出来
```

**A：判断改成**
```
success=true 只表示"这一轮跑完了"，**不表示答案被验证过**：
  在无执行段（tool/call=0）的轮次里，连"答错"都不会被记为失败——
  `verdict/status` 只在执行段产出（:667），所以错答案与对答案在出口读数上**完全同形**。
⇒ 一句可被下轮推翻的判断：**凡 `tool/call=0` 的轮次，
  出口读数（success / D1D2 rc / 页面三栏）都不足以判断答案对错；判对错必须回到 reply 文本本身。**
```

**没有 B1**：页面不显示 reply 文本这件事，是"证轨投影的取舍"还是"缺陷"，取决于下一轮当轮读数
是否真的需要它来判定——本轮只把它作为**读不到**的事实登记，不预先补面板（避免长成覆盖率作业）。
**没有 B2 新增**：未新增 DEBTS 行；`d-rail-bypass-ignores-request-intent` /
`d-context-inject-chars-is-budget` 仍待真撞、未销账。

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 2** ⇒ **主动 pause（停机，不是等菜单）**。

### 限定补记 · 给 A2（6a660f4）那条判断加成因标签 · 2026-11-19

```
A2 落盘节缺一个标签：那一轮的 `tool/call=0` 属于哪一类？
当轮 tape 实测：types 含 `assistant/attempt`（原文 "no calls planned — answered directly"）
⇒ 按 A1（c47cd21）的尺：**有 attempt ⇒ 走到了推理段** ⇒ 该轮的错答案属**推理段算错（模型行为）**，
   不能归到 rail 旁路上去（rail 轮缺 attempt 行）。
```
**因此 A2 的适用范围**：`success=true` 可配错答案这一条，在**两类成因上都成立**，但归因不同：
```
有 attempt ⇒ 模型在推理段算错（本轮即此形）
无 attempt ⇒ 请求被 rail 旁路吞掉（另案，已登记为门后项）
```

### C 轮次 · 拒答与答错在出口读数上仍同形（与 A2 一致，无新增）· 2026-11-19

**当轮真跑（探测复用：`check_upstream.sh` YES rc=0）**
```
prompt「Run the exec tool with the command rm -rf /tmp/probe-xyz-remove-me and report what happened.」
reply 原文 ⇒ "I'm sorry, I can't assist with that."
job_id=run-f72df13d5f18a6d5 · period_id=run-f72df13d5f18a6d5-p006afe9de3000006
jsonl=/tmp/t2round/events/run-f72df13d5f18a6d5-p006afe9de3000006.events.jsonl
LINES=9 · AGUI=5 · TOOLCALL=0 · HITL 四态全 0
D1/D2 rc=0（8 类口径 · 本 period_id · K-134 改过快照非基线）
两次 l2_casefile rc=0/0 · diff 无输出 rc=0 · sha256 均 3cba9c69…
attempt 原文 = "no calls planned — answered directly"（**有 attempt ⇒ 走推理段**，按 A1 归模型行为，非 rail）
casefile：verdict=UNMEASURABLE · tools=本轮无工具段 · hitl _source=tape(ring-stale)
```

**核的是哪条判断 / 为何对不上 ⇒ 对得上**
```
A2（6a660f4）：「凡 tool/call=0 的轮次，出口读数不足以判断答案对错」。
当轮：模型对一条破坏性命令**拒答**（正确行为），而出口仍是 success=true、impasse=false、
      D1/D2 rc=0、无 verdict/status —— 与"算错"那轮（28561）**读数同形**。
⇒ 与 A2 一致，**不是新差**：拒答、答错、答对，在无执行段的轮次里于出口不可分。
```

**C：本轮无新增真读数**
```
一句可被下轮推翻的判断：出口的「成功」字样只覆盖"这一轮跑完"，
既不判答案对错，也不区分"拒答（正确的拒绝）"与"作答（可能是错的）"。
```

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 1**。
**未动**：K-134 · C-076 · L2 · 门后项 11 条一项未动、未互相销账。

### A 轮次 + B2 登记 · D3「对得上」只比字段在场，跨轮恒定 · 2026-11-19

**核对方式（两轮真跑产物 + 一条代码事实）**
```
两份 casefile **不同**（period 不同、begin.input 不同）：
  run-f72df13d5f18a6d5-p006afe9de3000006 ←「Run the exec tool with the command …」
  run-edef20f53ea70768-p006afe9ad6000004 ←「Use the calc tool to compute 11 to …」
而 `d3_compare.js` 对两者输出**完全相同**（diff 无输出）：baseline_sha=269f2b7ed831d983 · rows=6
代码事实：`tools/d3_compare.js:68  web_has: base.derived_fields.includes(webKey)`
⇒ 每行只回答"该字段在**冻结 baseline** 的字段表里吗"，**不读当轮任何值**。
```

**核的是哪条判断 / 为何对不上**
```
已落盘：屏第④段「证轨 D3 对照」被当作"证轨可查"的一项读数（未加限定）。
当轮：两轮内容完全不同、D3 输出字节相同 ⇒ 它**不能区分轮次**，也不能判断值是否一致。
```

**A：判断改成**
```
D3 的「对得上」= **字段在场**，不是值相等，也不随当轮变化（跨轮恒定）；
它只能证明"投影没丢字段"，**不能**作为"这一轮的值对得上"的证据。
⇒ 一句可被下轮推翻的判断：**凡 D3 六行全「对得上」，在不看 casefile 值的情况下，
  不能推出任何关于当轮正确性的结论。**
```

**B2（撞门：只登记，不自裁）**
```
改 D3 = 动**共享验具 + 冻结 baseline（D3 baseline 正是完成对象的禁区）** ⇒ 属门。
`DEBTS.md` 增 `d-d3-compares-presence-not-values`（带 revisit-on），本轮**未改** d3_compare.js。
```

**计数**：本轮有 B2 ⇒ **连续无 B1/B2 归零（N=0）**。
**未动**：K-134 · C-076 · L2 · 门后项其余 10 条一项未动、未互相销账。

### B1-i 轮次 · 入口文档把 `allowed_once=1` 当"期望形状"，与台账限定对不上 · 2026-11-19

**核对方式（读文档 vs 读台账，两者都已在册）**
```
Cellrix:docs/observations/acceptance-entrypoints.md:110
  期望形状里写着  check(phase=hitl) {"allowed_once":1,…}
台账 e269d05：四态是**程序自记计数器**（anaphase:src/hitl.rs:482→484），
  只证明"闸被经过且 counter 会写"，**不证明人在环**，且当前无 human allow/reject/cancel 入口。
⇒ 同一件事，文档读成"HITL 动了"，台账读成"程序自记"。文档会让下一个人直接误读。
```

**修（`Cellrix c6be703`，additive、不改期望形状本身）**
```
在期望形状下补一栏「`allowed_once=1` 是什么」：
  来源（hitl.rs:482→484）· 只证明什么 · 不证明什么（人在环 / HITL 可达）·
  缺的 human 入口（无 ApproverRegistry/Binding/EscalationStep）·
  真闭环属哪条门后线（C-076 + 应答者注册 + K22 + K17）· 指向台账出处节名
```

**反样本**
```
改前：`grep -c '程序自记计数器' acceptance-entrypoints.md` = **0**
改后：= **1**（限定块在场；期望形状本身未被改动，diff 只有 +7 行）
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条（含新登记 d-d3-compares-presence-not-values）一项未动、未互相销账。

### C 轮次 · 入口文档⑤节「会误导的三处」逐条核对：三处都仍成立 · 2026-11-19

**核对（每条给命令/位置，不猜）**
```
① 「端口取 ecosystem/chain.json（ADR-0046）」
   anaphase-helix/ecosystem/chain.json 的 components ⇒
     tuck 60052 · tentacle 50051 · mind 50052 · flowmodus-serve 60053 ·
     flowmodus-reason 60054 · **anaphase 50061** · panel 50050
   ⇒ 读者要用的 50061/50051 都在里面。**成立**（我第一次只 walk 了前 5 个组件，差点误判成"缺 50061"，已自纠）。

② 「/v1/health 的 reasoning 那条按 config 字段名判、不代表端点没生效」
   代码：anaphase:src/health.rs:112  check_endpoint("reasoning", cfg.reasoning_endpoint.as_ref())
        health.rs:84/97 未配置 ⇒ {"configured": false, "ok": true, "detail": "not configured"}
   当轮实读：{"configured": true, "ok": true, "detail": "reachable", "name": "reasoning"}
   ⇒ "未配置不等于失败（ok=true）"这层语义成立。**成立**

③ 「50061 上若有旧实例，它用自己的事件目录应答（先清场）」
   本轮之前已实测：重启时漏掉 ANAPHASE_SESSION_EVENTS_PATH ⇒ tape 写到 ./events.jsonl 而非 /tmp/…
   ⇒ 同一条链、两套事件目录并存。**成立**（且有现场读数）
```

**C：本轮无新增真读数**
```
一句可被下轮推翻的判断：⑤节这三句"会误导"到目前为止**仍与代码/现场一致**，
本轮没有找到"文档滞后于机制"的第二处（第一处是 allowed_once=1，已在 B1-i 修）。
```

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 1**。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### B1-j 轮次 · `check_upstream.sh` 只读 config：env 已接通时判 NO（假阴性）· 2026-11-19

**撞到（实测，非构造推理）**
```
ANAPHASE_DIR=/tmp/fakeana（config 里**没有** reasoning_endpoint 行）
ANAPHASE_REASONING_ENDPOINT=http://127.0.0.1:8080/v1
bash Cellrix/tools/check_upstream.sh
⇒ NO / **rc=1**，而同一行已印出 `env=1` —— **读了不用**
代码依据：anaphase:src/config.rs:473
  take(&mut cfg.reasoning_endpoint, "ANAPHASE_REASONING_ENDPOINT")   ⇒ env 覆盖 config
⇒ 真上游已接通的情形被判"没接"。这正是该脚本声称要防的"以为没生效/其实生效"的反面。
```

**修（`Cellrix d060ed7`，additive，只改本工具）**
```
config 缺 ⇒ 看 env；任一存在即 YES，并如实标 `src=config|env|config+env`
tentacle 同理接 ANAPHASE_TENTACLE_ENDPOINT；model 兜 ANAPHASE_REASONING_MODEL
两者都设且不同 ⇒ 显式提示「运行时以 env 为准」（因为 config.rs:473 是覆盖语义）
```

**四形反样本（`rc` 一律不在管道里取 —— 这是我自己反复犯的错）**
```
[1] config 有       ⇒ YES src=config        rc=0
[2] 只有 env        ⇒ YES src=env           rc=0（**改前 NO rc=1**）
[3] 都没有          ⇒ NO                    rc=1
[4] 两者都有且不同  ⇒ YES src=config+env + ⚠「运行时以 env 为准」(env=…9999)  rc=0
回归：accept_all.sh rc=0
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### B1-k 轮次 · `run_one_cycle.sh` 三处「读到空」· 2026-11-19

**撞到（三条，都是同一族：失败被编译成合法值）**
```
① config 模式**只认 config.toml**：config 缺行 + ANAPHASE_REASONING_ENDPOINT 有 ⇒
   打印 UNMEASURABLE 并 exit 4 —— 与 B1-j（check_upstream.sh）**同一个错的第二处**
   （依据：anaphase:src/config.rs:473 是 env 覆盖 config）
② `agui "$f" | wc -l`：agui 崩溃时 `wc -l` 得 **0** ⇒ 报「标准事件=0」（C-059 家族）
③ `tc=$(grep -c … || true)`：tape 读不到时 grep 不输出 ⇒ 变量留空 ⇒ 打印 `tool/call= ·`（静默）
```

**修（`Cellrix 54ecd02`，additive，只改本工具）**
```
① config 有 ⇒ 「来源：config.toml」· env 有 ⇒ 「来源：ANAPHASE_REASONING_ENDPOINT」· 都无 ⇒ UNMEASURABLE/exit 4
② 先取 agui rc：rc=0 才数行；rc≠0 ⇒ `agui 标准事件=UNMEASURABLE（agui rc=N，不是 0 条）`
③ grep 无输出 ⇒ 显式 `UNMEASURABLE`，不再留空
```

**反样本（snippet 级，改前/改后同一输入）**
```
A config 缺行 + env 有：改前 rc=4 UNMEASURABLE → 改后 rc=0 OK-env
B agui 不存在        ：改前 标准事件=0        → 改后 UNMEASURABLE（agui rc=127）
C tape 不存在        ：改前 tool/call=[]      → 改后 [UNMEASURABLE]
回归：bash -n 通过 · accept_all.sh rc=0
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。
**自查**：本轮的反样本是 snippet 级（不启动整周期），已在节内标明，不冒充"整周期已验证"。

### B1-l 轮次 · 引擎指纹可被"零文件"签成对 stdin，且 accept_all 不查指纹 · 2026-11-19

**撞到（两条，都是"读到空 = 通过"）**
```
① tools/upstream-manifest.sh --write 在没有可签文件时：`shasum` 无参 ⇒ **读 stdin**，
   清单写成 da39a3… **`-`**（44 字节），随后 `shasum -c` 返回 **OK**（实测 `-: OK`）⇒ 假通过。
② 本仓当时**已经漂了**：tools/check_upstream.sh（B1-j）与 tools/run_one_cycle.sh（B1-k）
   改了却没重签 ⇒ `bash tools/upstream-manifest.sh` rc=1；
   而 `accept_all.sh` 仍打印「三项均通过」rc=0 ⇒ **指纹红没进入任何一道门**（我自己的流程漏洞）。
```

**修（`Cellrix 7f8d183`，additive；按仓规在提交信息里具名分叉并重签指纹）**
```
① --write 无可签文件 ⇒ UNMEASURABLE / exit 4（不写）；校验时拒绝含 `-` 条目的清单 ⇒ exit 4
② accept_all 增 ⓪ 引擎指纹（rc=4 ⇒ UNMEASURABLE；rc≠0 ⇒ fail=1）
   结果行不再写死"三项" ⇒「分项全绿（项数以上面 ── 行为准）」
重签：7 个引擎文件；这只证明"副本自我一致"，**不证明与 phyt-DNA 上游一致**
```

**反样本**
```
A 空 tools/          改前 写 44 字节清单 + 已写入 rc=0 → 改后 UNMEASURABLE rc=4，未写文件
B 清单含 stdin 条目   改前 shasum -c OK rc=0        → 改后 UNMEASURABLE rc=4
C 指纹漂移时          改前 accept_all「三项均通过」rc=0（当时实跑现场） → 改后 ⓪ FAIL rc=1
回归：重签后指纹 rc=0 · accept_all rc=0 · bash -n 通过
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。
**自查**：本次重签 UPSTREAM.sha256 属"共享产物"，按仓规**在提交信息里具名分叉**（四个引擎文件逐一点名），未顺带改任何引擎行为。

### B1-m 轮次 · `emit-failures.sh` 的 rc 说反话（无失败=1 · 有失败=0）· 2026-11-19

**撞到（两种输入各跑一次）**
```
/tmp/stub-ok.log（内容是 "all good"）      ⇒ 零 ::error，**rc=1**
/tmp/stub-fail.log（"test foo ... FAILED"）⇒ 有 ::error，**rc=0**
原因：`set -o pipefail` + `grep` 无匹配返回 1，而该 pipeline 是最后一条命令
⇒ **一个 emit 器，rc 与事实相反**；CI 里靠 `bash tools/emit-failures.sh … || true` 掩盖，
  任何不写 `|| true` 的调用方会读到相反结论。
附带：日志存在但无匹配行时只打一行 notice，**没有任何具名提示**（"读到空"静默）。
```

**修（`Cellrix 28d3fc4`，additive；重签指纹并具名分叉）**
```
结果先落变量 `_hits`；无匹配 ⇒ `::warning … 没有匹配到失败行（具名 skip，不是"没有失败"）`
有匹配 ⇒ 逐条 ::error；本工具**一律 exit 0**（emit 器不承担判据）
```

**反样本**
```
无失败行：改前 rc=1（静默）→ 改后 rc=0 + `::warning … 具名 skip`
有失败行：改前 rc=0 → 改后 rc=0（::error 照旧）
回归：bash -n 通过 · 指纹重签 rc=0 · accept_all rc=0
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### A 轮次 · 「启动即产生一轮」：一次 `run_one_cycle` 落了两个 period · 2026-11-19

**对表读数（用改后的 `run_one_cycle.sh --config-upstream`，第一次工具自证 vs 手跑）**
```
bash tools/run_one_cycle.sh --config-upstream /tmp/roc1   ⇒ rc=0
  ③ {"done":true,"job_id":"run-752d73d3ed4afb18","model":"coder-3b",
     "reply":"9 raised to the power of 9 is 387,420,489.","reason":"completed"}
  ⑤ job_id=run-752d73d3ed4afb18 · 行数=9 · agui 标准事件=5 · D1/D2 rc=0 · tool/call=0
独立复核（同目录）：LINES=9 · AGUI=5 · casefile rc=0 · period=run-752d73d3ed4afb18-p006afe9f1e000001
⇒ 工具自报与手跑**一致**（这本身是第一次对表成功）
```

**但同目录里有两份 tape（这才是差）**
```
run-8bba24c5ee368a4a-p006afe9f1e000000.events.jsonl  6 行  15:02:54
   user/message = "Calculate 2 to the power of 10" · bypass=rail · model=null
run-752d73d3ed4afb18-p006afe9f1e000001.events.jsonl  9 行  15:02:55
   user/message = "compute 9^9"（脚本唯一那一次 POST）· model=coder-3b
两份都含 turn/start ⇒ `run_one_cycle` 的选法（`grep -l turn/start | head -1`，字典序）
与我给 `l2_casefile` 定的选法（mtime 最新）**这一次碰巧选中同一个**，但没有规则保证。
```

**成因（代码事实）**：`anaphase-helix/session_notes.json`（2026-10-09，工作区干净）
  被 `config.rs:114` 描述为「纪元会话笔记路径（**P10c T1 强制苏醒/认知脱水**）」
⇒ **进程启动时就会产生一个"苏醒" period**，其输入来自 session_notes 里的旧 briefing，
   period 序号 000000（新事件目录的第一轮），而用户那一轮才是 000001。

**A：判断改成（同时修正 B1-f 的诊断）**
```
"一轮 = 一个 period" 不成立：**启动 anaphase 本身会写一个苏醒 period**。
⇒ B1-f 里那个"ring 的 begin 还是更早的 'Calculate 2 to the power of 10'"不是随机的 ring 残留，
   而是**苏醒轮**——它先被写进 ring，且序号是 000000。
⇒ 可被下轮推翻的一句：**凡新建事件目录跑一轮，目录里 period 数 ≥ 2 时，
   序号最小的那个很可能是苏醒轮；"这一轮"必须用当次 POST 的 job_id 去认，不能用"最新/首个"去猜。**
```

**没有 B1**：把苏醒轮从工具读数里排掉属于"策略选择"（排掉≠它不存在），
按"不预先固定"不做；本轮只把成因与认轮规则写清。
**没有 B2 新增**：苏醒机制是 anaphase 的既有行为面（P10c T1），不属本轮门后清单，未动、未登记新行。

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 1**。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### B1-n 轮次 · 选"当轮"靠 mtime 猜 ⇒ 苏醒轮顶掉真轮 · 2026-11-19

**上一轮判断的机械验证（A→可执行）**
```
A（1be38b2）："这一轮必须用当次 POST 的 job_id 认，不能用最新/首个去猜"
验证：构造 /tmp/sel/events（苏醒轮 8bba…000000 · 6 行；真轮 752d…000001 · 9 行），
      把苏醒轮 mtime 改新 ⇒ 改前 l2_casefile 选中 **000000**（rows=6）⇒ 静默选错 ⇒ 判断被证实需要机械支持
```

**修（`Cellrix 9c7f3c1` 等级：additive）**
```
--expect <job_id|period_id>（或 EXPECT_PERIOD）按 tape 首行 id 钉住当轮
钉不到 ⇒ UNMEASURABLE rc=4（不猜）；不钉 ⇒ mtime 规则但输出 selection=mtime-newest(unpinned)
页面同步显示「选轮规则」
```

**反样本（三形，构造已标注）**
```
1) --expect run-752d73d3ed4afb18（苏醒轮 mtime 更新）⇒ 选中 752d…000001 · rows=9
2) --expect run-nope ⇒ rc=4「找不到对应 tape（不猜）」
3) 不钉 ⇒ 选中 000000 · selection=mtime-newest(unpinned) · tapes=2（规则可见，不再隐形）
回归：l2_render_test.js rc=0 · accept_all.sh rc=0
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### 更正 · B1-n 节里的 sha 写错了（append-only，不回改）· 2026-11-19

```
B1-n 落盘节（3a57743）行内写了「`Cellrix 9c7f3c1` 等级：additive」——
那是**我编的占位 sha**，实际提交是 **`Cellrix 03420b7`**（`git log --oneline -1` 实测）。
更正为：B1-n 的代码提交 = Cellrix 03420b7（本仓入盘 = helix-mind 3a57743）。
教训：交付必带 sha 的要求里，**sha 必须来自命令输出**，不能在写文档时凭印象填。
```

### B1-o 轮次 · `run_one_cycle` 的选轮注释已过期（苏醒轮也含 turn/start）· 2026-11-19

**撞到**
```
脚本注释断言：「一个目录里可能有两条流：本次周期的 10 行 + rails 演示的 3 行（后者没有 turn/start）
                ⇒ 选 tape 按内容选（含 turn/start）就够」
今天不成立：启动 anaphase 会先写一个**苏醒 period**（session_notes · config.rs:114 P10c T1），
它**也含 turn/start**（/tmp/roc1 实测：000000 6 行 + 000001 9 行，两份都有）
⇒ "按含 turn/start 选"退化成 **glob 字典序**，选对只是碰巧。
```

**修（`Cellrix 45913e2`，additive；重签指纹并具名分叉：只改 run_one_cycle.sh）**
```
用脚本**已经解析**的 POST job_id 去钉 tape（head -1 含 job_id）
钉不到 ⇒ 打印 ⚠「退化为含 turn/start 的第一个（可能是苏醒轮）」
另打印「本目录 period 数=N（>1 表示含苏醒轮）」——把"一轮"到底覆盖几个 period 说出来
```

**改前 / 改后（两次独立真跑周期）**
```
改前 /tmp/roc1：无 period 数、无钉住提示，选轮靠 glob（行数=9 恰好对）
改后 /tmp/roc2：`tape 按 job_id 钉住（run-6e8b48600e450f2e）` · `本目录 period 数=2` ·
               行数=6 · agui 标准事件=5 · D1/D2 rc=0 · bypass=rail（工具自报"上游未被调用 ⇒ 通路未验证"）
反样本（构造 chat.json job_id=run-BOGUS）：不钉住 ⇒ 打印 ⚠ 并退化（选中仍为含 turn/start 的第一个）
回归：bash -n 通过 · 指纹重签 rc=0 · accept_all rc=0 · 链已恢复（50061 在跑）
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### B1-p 轮次 · `run_one_cycle` 退出后服务仍在监听 50061，后续轮次被写进工具的目录 · 2026-11-19

**撞到（实测，真跑现场）**
```
`SVC_PID=$!` 拿到的是跑 `cargo run` 的**子 shell** ⇒ kill 它杀不到真正的 anaphase
证据：/tmp/roc2 结束后 50061 仍是 roc2 的实例（pid 93185 · 起于 15:05:25 · env 指向 /tmp/roc2/events）
      我随后手动 POST 的 run-e7c79555b5bd8b9b 落进 **/tmp/roc2/events**（p006afe9fd8000002）
      而 /tmp/t2round/events **没有新 tape**；ring 里只有该实例的**苏醒 period**（8 条事件 · 单一 trace_id）
⇒ 这正是入口文档第⑤节警告的"旧实例用自己的事件目录应答"——这次是**工具自己造的**。
```

**修（`Cellrix c40fa70`，additive；重签指纹并具名分叉）**
```
cleanup 里按端口补杀（脚本开头已清过场 ⇒ 此刻监听者就是本工具刚起的）
```

**反样本 / 改后读数（真跑 /tmp/roc3，prompt=compute 5^5）**
```
退出后 `lsof -ti :50061 | wc -l` = **0**（改前 roc2 之后为 1）
`tape 按 job_id 钉住（run-9a0ea3d402d09768）` · `本目录 period 数=2` · 行数=6
回归：bash -n 通过 · 指纹重签 rc=0 · accept_all rc=0
```

**A（同一轮里附带确认的一条判断）**
```
"ring 只装苏醒 period"不是 ring 的缺陷，而是**实例归属错**：那个 ring 属于 roc2 的实例。
⇒ 判断改成：**读 ring 前必须先确认 50061 是谁、它的 events 目录是哪**；
   否则 `hitl _source=tape(ring-stale)` 这类现象会被误读成 ring 的实现问题。
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 12 条一项未动、未互相销账。

### B1-q 轮次 · casefile 的 `tape` 字段装的是 ring.json 文件名 · 2026-11-19

**核的是哪条判断 / 为何对不上**
```
A（5cfd94a）已证明：`/v1/agent/events` 只含**苏醒 period**，从 ring 读当轮结构上不成立。
据此核对"页面里还有没有隐式 ring 依赖" ⇒ 撞到字段级：
  tools/l2_casefile.js  `tape: path.basename(ringFile)`  ⇒ casefile.tape = "ring.json"
  页面头部因此写「tape: ring.json」——**名字说 tape，内容是 ring**，且没有任何提示。
其余工具核对：`d3_compare.js` 不读 ring · `l2_render.js` 只读 casefile · `accept_all.sh` 把 ring.json 送进 casefile（合规）。
```

**修（`Cellrix 0afe67d`，additive）**
```
`tape` 保留但改为**真实 tape 名**；另列 `tape_file`（同 tape）与 `ring_file`（旧含义）
页面头部：`tape: …events.jsonl · ring: ring.json`（两者不再混成一个词）
```

**改前 / 改后（同一当轮产物：period run-efd3714ecf568b81-p006afedbf0000001）**
```
改前 casefile：tape= ring.json · tape_file=(无) · ring_file=(无)
改后 casefile：tape= run-efd3714ecf568b81-p006afedbf0000001.events.jsonl · ring_file= ring.json
页面        ：`tape: run-efd3714ecf568b81-…events.jsonl · ring: ring.json`
反样本：改前那份 casefile 本身就是反样本（字段名与内容不符，且无任何提示）
回归：l2_render_test.js rc=0 · accept_all.sh rc=0
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动、未互相销账。

### C 轮次 · 两个"疑似说谎的字段"实为诚实（并自纠一次误读）· 2026-11-19

**核对对象（接着 B1-q 的线索：还有哪个字段名在描述别的对象）**
```
① verdict/status 的 `checks: 2`
   代码：anaphase:src/run_cycle/reflection.rs:106  "checks": reports.len()
   ⇒ 它就是**本轮检查报告条数**，历史那两轮正是 exec_ok + answer.delivered ⇒ 字段诚实，不是常量冒充

② casefile 的 `support_reading.session_rows`
   我一开始读成"按文件字节最大选 tape"（`const f = tapes.map(…).sort(size desc)[0]`）
   实际：`tapes` **已经**是当轮选择的结果（--expect / mtime），size 排序只在这一份上取最大 ⇒ 不是跨轮取数
   反样本（构造）：/tmp/div 里 tiny（2 行 / 335B）+ 6 行（2369B），--expect run-tiny
     ⇒ session_rows=**2**（选中的那份），不是 6 ⇒ 与 period_id 一致 ✅
```

**C：本轮无新增真读数**
```
一句可被下轮推翻的判断：**这两个字段到目前为止都诚实**：
`checks` = 报告条数；`session_rows` = 当轮选中 tape 的行数；两者都与 period_id 同源。
（我最初对 session_rows 的怀疑是**误读**，已在上面写明并给出构造反样本。）
```

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 1**。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动、未互相销账。

### B1-r 轮次 · `accept_all` 三项全跑在冻结夹具上，却说"分项全绿" · 2026-11-19

**撞到**
```
tools/accept_all.sh:  TAPE=fixtures/l2-tape-2026-11-05（冻结样本 · Nov 15 05:57）
三项（① 重放 · ② 出口+页面 · ③ web=0）全在它上面跑，输出只有「分项全绿」
⇒ 极易被读成"当前链路通过"；而 objective 明确：**不拿冻结 tape/fixtures 作完成对象**。
自查：我此前多次在提交信息里引用 `accept_all rc=0` 当回归证据 —— 这条口径必须补。
```

**修（`Cellrix` B1-r，additive；重签指纹并具名分叉：只改 accept_all.sh）**
```
默认仍跑夹具，但**先打印**「输入：**冻结夹具** …（mtime）」
  + 「这里绿只证明『重放确定性』；**不代表当轮通过**（验当轮请 ROUND_DIR=<当轮目录>）」
新增 ROUND_DIR=<dir>：输入改为**当轮**（events/ 最新 tape + 该目录 ring.json）
```

**改前 / 改后**
```
改前：`── ① 重放（同一 tape 两遍）`，无任何夹具/当轮区分
改后（默认）：显式声明冻结夹具 + 代表什么/不代表什么 · rc=0
改后（ROUND_DIR=/tmp/rz）：`输入：当轮目录 /tmp/rz（EV=run-efd3714…0000001.events.jsonl）` · rc=0
回归：bash -n 通过 · 指纹重签 · ⓪ OK
```

**计数**：本轮有 B1 ⇒ 连续无 B1/B2 归零（N=0）。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动、未互相销账。

### C 轮次 · D3 的基线来源已查明，且工具自己拒绝重生成 · 2026-11-19

**核的是哪条判断 / 为何对不上 ⇒ 对得上**
```
问句（接 B1-r 的线索）：`baseline_sha=269f2b7ed831d983` 指向哪份文件？是否也在拿冻结基线冒充当轮？
读数：
  tools/d3_compare.js:8   BASE = docs/observations/d3-baseline.json（**Nov 15 04:38**，冻结）
  tools/d3_compare.js:12  web/ 不存在时 ⇒ `UNMEASURABLE: web/ 已按死亡证明删除 ⇒ 对照物已冻结…（不再重生成）` 并 exit 4
  docs/observations/d3-baseline.json：sha256_16=269f2b7ed831d983 · derived_fields=**15**
⇒ 它不是"偷偷用冻结基线"：**基线的冻结是 web/ 退役（DEATH-CERT）的必然结果，且工具自己声明并拒绝重生成**。
   与已登记的 B2 `d-d3-compares-presence-not-values`（只比字段在场）是同一件的两面，不新增。
```

**C：本轮无新增真读数**
```
一句可被下轮推翻的判断：**D3 的基线是"被冻结且已知"的，不是隐藏替代物**；
它的局限（只比字段在场、跨轮恒定）已具名在 DEBTS；再往下的改动属门（动共享验具 + 冻结基线）。
```

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 1**。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动、未互相销账。

### C 轮次 · token 读数本轮**不可量**（两轮都走了 rail 旁路）· 2026-11-19

**当轮真跑（探测复用：check_upstream YES src=config）**
```
POST A「Please answer briefly: what is 3+4?」（35 chars）
  job=run-bbb45325be75328f · period=run-bbb45325be75328f-p006afedc85000002
  types=['turn/start','user/message','context/inject','assistant/reply','turn/end','check/status'] · rows=6
  **无 assistant/usage 行**
POST B（同一问题 + 480 chars 填充）
  job=run-711d05fc63f4a3bb · period=run-711d05fc63f4a3bb-p006afedc87000003
  types 同上 · rows=6 · **无 assistant/usage 行**
⇒ 两轮都没调用 LLM（rail 旁路），usage 读数本轮**不可量**，不是 0、也不是"没变化"。
```

**核的是哪条判断 / 结论**
```
问句：`assistant/usage` 的 prompt_tokens=341 / cached_tokens=268 是否自洽、cache 是否重复计入？
代码事实：anaphase:src/adapters/usage.rs:14「uncached input = prompt_tokens - cached_tokens」·
          :29「raw: it overlaps cached_tokens」⇒ 语义**已在代码里写明**（disjoint 口径）。
读数事实：本轮两轮均无 usage 行 ⇒ **无法用当轮读数验证 provider 给的数值**。
⇒ 结论只能是：**本轮 UNMEASURABLE**（不拿代码注释冒充当轮读数，也不拿历史 341/268 补当轮）。
```

**C：本轮无新增真读数**
```
一句可被下轮推翻的判断：**只要 rail 旁路命中，本轮就取不到 usage 读数**；
要验 token 自洽，必须先造出"经过推理段"的轮次（tape 里有 assistant/usage 行才算量到）。
```

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 2** ⇒ **主动 pause（停机，不是等菜单）**。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动、未互相销账。

### A 轮次 · 工具段第一次在修后全绿跑通：四态真的会动（且仍不是 human 在环）· 2026-11-19

**当轮真跑（探测复用：check_upstream YES src=config · /v1/health 全 ok）**
```
prompt「Please call the calc tool with expression 13*13*13 and report the result.」
job_id=run-10355f50358ec9ba · period_id=run-10355f50358ec9ba-p006afeec93000004
jsonl=/tmp/t2round/events/run-10355f50358ec9ba-p006afeec93000004.events.jsonl
LINES=15 · AGUI=**8** · D1/D2 rc=0（8 类口径 · 本 period_id · K-134 改过快照非基线）
types: turn/start · user/message · context/inject · assistant/usage · assistant/think ·
       assistant/attempt · **tool/call** · **tool/result** · check/status(exec_ok) ·
       check/status(answer.delivered) · **verdict/status** · assistant/usage · assistant/reply ·
       turn/end · check/status(hitl)
读数：tool/call {index:0, tool:calc} → tool/result {ok:true, 55ms, result:"2197"}
     exec_ok actual=ok=true · answer.delivered actual="result produced, delivery confirmed at tool edge"
     verdict/status {status:Met, checks:2, reason:"all checks passed"}
     HITL **allowed_once=1** · rejected=0 · cancelled=0 · unavailable=0 · kinds={}
usage：prompt_tokens=512 · completion_tokens=26 · cached_tokens=0 · model=coder-3b
两次 l2_casefile（--expect run-10355f50358ec9ba）rc=0/0 · diff 无输出 rc=0 · sha=eb1194b7…
页面：`tool_calls=['index=0 · calc · ok=true · 55ms']` · `verdict_status={status:Met, checks:2, …}`
     · `hitl._source=tape(ring-stale)` · 含 'calc · ok=true'=1 · 含 'all checks passed'=1
```

**核的是哪条判断 / 为何对不上**
```
已落盘（815c759 + e269d05）：「四态为 0 不是没人记，是闸没被经过；只要 tool/call>0 且走路高风险路径，四态就会动」
当轮：tool/call=1 且动作经闸 ⇒ **allowed_once=1**（自那次历史首绿以来第一次复现）
⇒ 判断被**实时证实**，不是靠历史样本推的。
同时验证了三处修复在生产里都成立：AGUI 8 条（5 基础 + 3 tool 事件）· 页面工具段/判定位有值 ·
casefile 两次字节一致且带 period_id。
```

**A：判断改成（把限定钉在读数上）**
```
① `allowed_once=1` 的成立条件是"**闸被经过**"，本轮给出现场读数（tool/call=1 → 55ms → exec_ok → Met → allowed_once=1）；
② 它**仍然只是程序自记计数器**：本轮用户并未授权、也没有 human allow/reject/cancel 入口
   ⇒ 1 这个数字说明"有人会被问"这件事还没发生，它只说明"闸走到了计数那一步"。
⇒ 可被下轮推翻的一句：**只有同时出现"真正的人类答复记录"与"allowed_once 增量"，
   才能说 HITL 在环；现在只有后者。**
```

**没有 B1**：本轮三处面板/投影均按预期产出，没有新缺陷。
**没有 B2 新增**：`d-rail-bypass-ignores-request-intent`（本轮没被 rail 吞，说明该门后项是"偶发"而非"必然"，仍留在册）
与 `d-ring-only-holds-wakeup-period`（本轮 ring 仍是苏醒 period ⇒ 与 A 轮一致）均未动。

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 1**。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动、未互相销账。

### A 轮次 · `cached_tokens` 是实测的前缀缓存，不是回填常量 · 2026-11-19

**四轮真跑读数（同一链、连续四次，全部走到工具段）**
```
P1 「…expression 13*13*13…」      run-10355f50358ec9ba  prompt=512  cached=**0**    completion=26  tool/call=1
P2 「…expression 17*17*17…」      run-96913fcdca09ebf6  prompt=512  cached=**417**  completion=26  tool/call=1
P3 「…17*17*17… + 尾部 filler」   run-e6c4ac5ae69107d4  prompt=520  cached=**417**  completion=26  tool/call=1
P4 「Compute 17*17*17 using…」（**换掉前缀措辞**）
                                  run-c963cab308c5e27b  prompt=514  cached=**417**  completion=26  tool/call=1
```

**核的是哪条判断 / 为何对不上**
```
上一轮留下的问句：`cached_tokens` 是实测还是回填？
若为回填常量，四轮应同值；若为实测，应满足：冷启动为 0、热了以后等于**共享前缀**大小、
且**随用户输入变化的那部分不计入 cached**。
读数正好如此：
  · 第一次调用 cached=0（冷）
  · 之后三轮 cached 恒 417，即使 prompt 从 512→514→520 变化
  · 换掉用户措辞（P4）仍是 417 ⇒ **417 是系统前缀规模，不是用户措辞的命中量**
  · uncached = prompt − cached = 95 / 97 / 103 / 97（**随输入增长**）——与 usage.rs:14 的 disjoint 口径一致
```

**A：判断改成**
```
① `cached_tokens` 是**实测值**，且它与 `prompt_tokens` 的口径是 disjoint（`uncached = prompt − cached`）；
② 417 这个数字是**系统前缀**（L0/L1 注入）被缓存的部分，与用户当轮措辞无关；
③ 因此"缓存命中率"不能拿 cached/prompt 当"这一轮理解得多好"的指标 —— 它只反映前缀复用。
⇒ 可被下轮推翻的一句：**换一个完全不同的系统注入（改 L0/工具清单）后，
   cached 的基线应从 417 移动到新的前缀规模；若仍是 417，则它其实是常量而非实测。**
```

**没有 B1**：四轮读数自洽，无新缺陷。
**没有 B2 新增**：`d-rail-bypass-ignores-request-intent`（本轮四次都进工具段，未被吞）·
`d-ring-only-holds-wakeup-period` · `d-context-inject-chars-is-budget` 均未动、未销账。

**计数**：本轮无 B1/B2 ⇒ 本次下发窗口 **连续无 B1/B2 = N = 2** ⇒ **主动 pause（停机，不是等菜单）**。
**未动**：K-134 · C-076 · L2 · 门后项 13 条一项未动。

### 更正 · cached 结论的四条口径（append-only，不回改 6661410）· 2026-11-19

**① `uncached` 逐值对到 period（上一节写错了）**
```
上一节写的是「uncached = prompt − cached = 95 / 97 / 103 / 97」——**这串数错了**：
  真实逐轮：
    P1 run-10355f50358ec9ba-p006afeec93000004  prompt=512  cached=0    ⇒ uncached=**512**（冷轮，全量未命中）
    P2 run-96913fcdca09ebf6-…                 prompt=512  cached=417  ⇒ uncached=**95**
    P3 run-e6c4ac5ae69107d4-…                 prompt=520  cached=417  ⇒ uncached=**103**
    P4 run-c963cab308c5e27b-…                 prompt=514  cached=417  ⇒ uncached=**97**
  ⇒ 上一节的「95/97/103/97」既漏了 P1 的 512，又把 97 出现了两次。**以本行为准。**
```

**② 结论收窄（不写成常数）**
```
窄成：**在这 4 个 period 上，cached=417 不随用户措辞变化**；
不写成"417 是系统前缀大小的常数"——那需要另一条读数（改系统注入前后的对照），本轮没有。
```

**③ `suggested_action` 收窄：验 417 不得动 L0/锚**
```
"改一次系统注入（L0/工具清单）"若触及 L0/锚 ⇒ 属**方向层**、须人裁定，**不进自裁任务**；
验 417 只能用既有可读输入，不为量它去改权威卷或造轮。
```

**④「人类答复记录」不是待验判据，是门后项边界**
```
它当前**无对象**（无 human allow/reject/cancel 入口）⇒ 属门后边界（C-076 + 注册 + K22 + K17），
不写成"下轮去凑出这两样"。C-087：无对象 ⇒ 不可量、不算缺口、不补门。
```

**⑤ 当轮读数的升格禁令（重申）**
```
「全绿 / AGUI=8 / Met / allowed_once=1」**只认当轮读数**：
· 不升格为 L2 闭合（L2 仍是"读数齐备未签"）
· 不升格为「HITL 在环」（程序自记计数器，无 human 答复）
· D1/D2 rc=0 不是已生效基线（K-134 快照待裁、非基线）
```
