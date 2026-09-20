# HANDOFF 2026-09-16 — 会话交接点

> **新会话从这里开始。**（本文件与 `HANDOFF.md`（09-08 历史快照）是**两份不同文件**；
> 那份是历史快照，本文件是当前交接点。）
> 工作区根 `/Users/jason/Doubao/chats/Jasonmilk/`（固定，不按日期分目录）。
> **只给当前状态与指向权威文档的指针** —— 细节不复述。

## 六仓 HEAD（**截至 2026-09-16 12:34**）

> ⚠️ **本表会立刻过时 —— 以 `git log --oneline -1` 为准，本文档不追。**
> 写死 hash 就是「事实的第二份副本」；这里保留只是为了说明**当时**的起点。

| 仓库 | HEAD |
|---|---|
| Cellrix | `02e6ac5` |
| Tuck | `5557bfd` |
| anaphase-helix | `69cc136` |
| helix-mind | `0c1ee57` |
| FlowModus | `547b795` |
| phyt-DNA | `f1331cf` |

> ⚠️ 2026-09-16 下午起六仓有**未提交改动**（Human confirmation before commit，未 commit）：
> anaphase-helix（rs 分支）6 改 + 1 新增；Cellrix（rs2）6 改；helix-tentacle（rs）2 改（README/PLAN 文档）。
> 见文末「本轮待提交」。

## 一句话现状

**2026-09-16 下午两条主线（用户临时插入 + 裁决后落地）**：

1. **Tentacle web_search 可用性（anaphase 侧，已物理闭环）**：
   - 三条猜测全部推翻：web_search.js 本体**完全可用**（Bing HTML 端点 + curl + 正则，**零 key**，不依赖 scraper feature）；真正卡点是 anaphase `Call.expect` 是**必需字段**，模型 6/6 漏写 ⇒ 解析期静默降级 NoToolNeeded ⇒ LLM 原始 JSON 透传成"泄漏"。
   - **P0-G**（`expect: Option<Expect>` + `#[serde(default)]` + None→ok + diag 留痕事件）+ **P0-D-1**（`calls schema mismatch` → fail-closed `TransitionCondition::Failure`）已落；两验证支点补齐：①fence/structured 默认 ok 是代码事实（contract/mod.rs:196/254）；②事件库 30/30 tool/call 的 expect 全为 `"ok"`（Numbers/Rate/Text 从未使用）。
   - 判据测试 `tentacle_live_web_search_p0g.rs`（真实二进制 + 缺 expect 计划，3 连）**3/3 通过**——修好 expect = 搜索恢复（此前预期"修好 expect ≠ 搜索恢复"被实测修正）。helper（free_port/spawn_real_tentacle/connect_tentacle）提升复用。
   - 用户裁决：**双模否决**（不加 web_search_api——两个搜索工具重蹈 P0-B；key 模等真需求）；渐进式觅食**不接线**但已在 PLAN 标注"已实现、未接线、原因"；README 构建命令失真已修正；ProcessTool cfg `"{}"` 硬编码入 PLAN 待办。

2. **Cellrix 4a 面板几何重构（panel-geometry-contract Q1/Q3/Q4/Q5）**：
   - **1440×813 下 layout_test 13/13 全绿**（checker 第三栏常驻 x=1103 w=317、主区 779 不重叠；transcript gap 0；input 视口内；整页不滚）；全回归 55/0 + 59/0 + run_all 8 套。
   - **800px 窄屏 8/5**（before 7/6；整页滚动已修；side 压缩 120px；判据 A 加 <1180px 模态守卫）；剩余窄屏 chat 单列 input 差 ~15px（Q5 退化带，已登记）。
   - DOM 事实：`.e-insp` 是 `#s-main` 直接子级 ⇒ 第三栏做在 `#s-main` 上（grid 两列）；判据 A 从"打开让宽"改为"常驻不重叠"（对齐契约 Q1/Q3-③，含 ≥1180px 守卫 + aria-modal 校验）。

## ⚠️ 未修的（**别被绿灯误导**）

**用户报「第二问之后记录消失」→ 查出两个独立症状：**

- **A（已修・真链验收）**：`seq` 每个 turn 从 0 重开 ⇒ 装配层按全局 `seq` 去重 ⇒
  **第 2 轮起的事件全被当重复拒收**。修法：**`gseq` 在读取边界一次性赋值**
  （`period_normalize.js`）。真链验收：`run-7efbf0f8` **55 事件 / 5 轮 / 0 拒收**。
- **B（已取消・非 bug）**：`resume_from` 续接链聚合成一张卡 ⇒ 用户以为记录丢了。
  正解：**取消折叠、列表平铺**，`resume_from` 退回协议层。**数据从未丢失**。

**同轮完成**：`mergeChain`（合并多 period，**一次 normalize 由结构保证**）；
**L0** 点开即整链（标注**临时**，待 `session_id`）；证轨**不再自取数据源**；
**两道 git 闸门**（gate1 首行 / gate2 diff）**双向验证 + 已装八仓**；
`ab_verify.sh` **断言能力而非端口**；`phyt-DNA` 写入**验证纪律**；
`Cellrix/docs/dom-contract.md`（改布局前必读，**已可执行校验**）。

## ⚠️ 未修的（**别被绿灯误导**）

| 症状 | 现状 | 证据 |
|---|---|---|
| **证轨的用户症状** | **未修** | **从对话进入证轨** ✅ 80 事件（对）；**独立打开证轨视图** ❌ **仍是单 period 碎片** |
| **两份实现共存** | **过夜了** | `prove_track.node.js`（新版，有测试）与 `prove_track.data.js` 的消费半部（**证轨实际在跑它**） |

**⇒ 测试里有一条是红的，且它说的就是这件事**：
```
FAIL  the trajectory view drives the Node layer
FAIL  the naive legacy wiring is gone
```
**⇒ 那两条红是如实状态，不是回归。** 3b-1 的六条绿证明的是**新层能用**，
**没有说任何人在用它** —— 那是「服务端 55」那个错的另一层。
**⇒ 3b-2 / 3b-3 落地后它才转绿。**

## 权威文档 —— 读这些，不读本文件

| 主题 | 位置 |
|---|---|
| **下一步计划（批次 0–7）** | `helix-mind/docs/helixECO/PLAN-targets.md` |
| 生态 SSOT | `helix-mind/docs/helixECO/ECOSYSTEM.md` |
| 跨仓缺陷登记 | `helix-mind/docs/helixECO/KNOWN_ISSUES.md` |
| DOM 契约（改布局前必读） | `Cellrix/docs/dom-contract.md` |
| 工作日志 | `.workbuddy-ai/memory/2026-09-16.md` |

## 批次 4 已落 —— 3b-3 + 4b 合并完成（Cellrix `02e6ac5`）

**证轨连续性接通。** 证轨不再是「另一个取数口」：**它是这笔 tape 的 target**，
壳层 `loadWindow` 只读一次链并喂 tape，对话渲染 events，证轨读 `snapshot.nodes`。
`__proveTrackLoad` 的 `stream` 形参**已删**，证轨自己的 `/api/events` 取数**已删**。

**真链证据（单条命令内起栈→探→停）**：独立打开证轨（不选经历）= 整条链；
该数由 e2e **自己从 API 重算**，不问应用要 ⇒ e2e **55 passed / 0 failed**；
服务端页面逐字等于资产 ⇒ **56 passed / 0 failed**；回归网 **7 套件全绿**。

**切换前用「两层逐字段对拍」查出并已修的 6 类缺陷**（全部有实测，不是读代码读出来的）：

| # | 缺陷 | 真因 |
|---|---|---|
| 1 | 4 个模板里出现游离 `—`（context / turn-end / tool-result / verdict） | **`opt` 键被写成了 `{slot}`** ⇒ 先填 em dash 再追加真值；而 `—` 与「诚实缺失」同形 ⇒ 看不见 |
| 2 | check 标签恒为字面量 `gate` | 应为 payload 的 `gate` 值（真值 `hard`） |
| 3 | tool/result 面板显示 **digest** 而非 outcome | 审查面板要看的是产物本身 |
| 4 | context 面板丢节点命中列表 | `detailOf` 少了 context 分支 |
| 5 | reply 行丢周期总量与可展开正文；reasoning/plan 丢 gap 时长 ⇒ `LLM TIME` 恒 `—` | 用量从「被筛过的流」推导，而 metering 恰不在其中 |
| 6 | metering 变成行（每链多 10 行）且重定义所有时长 | 旧的「不成行」决定没被搬过来 |

**做法上的一条要点**：`validate()` 在**装载期**跑，把第 1 类缺陷变成**不可能状态**
（槽位无法解析 / 槽位与 opt 同名 / kind 两表都没登记 ⇒ 模块拒绝加载）。
守卫带**阳性对照**：给 `validateTables` 喂坏表，必须抛。

**新增结构事实（写 ADR 前先看这里）**：
- `prove_track.render.js` = **表**（`LANE_OF` / `SUMMARY` / `KIND_NOTE` / `STATUS_OF` / 校验器）；
  **`SUMMARY` 就是行集** —— 没有条目 = 不成行；`NOT_DRAWN` 显式声明例外（metering）。
- 会话项字段**只命名一次**：`cls / lane / detail / term / kindNote / fields / ord / ts`；
  `seq`/`time` 无人读 ⇒ 已删。
- Node 带 **`source`**（与 `lineNo` 并列）：turn 表头因此能写出该轮来自哪一段；
  `node === source + '#' + lineNo` 是断言，不是约定。

## 本侧待办（不需要跨仓裁决）

| # | 事项 | 为什么在本侧 | 判据 |
|---|---|---|---|
| **L1** | **合并两个启动脚本**：`Cellrix/web/src/bin/up.rs` 与 `Cellrix/web/tests/start-panel.sh` 各自知道「怎么起栈」 | 今天全天在治的病（一份事实两个来源）换到启动层又长一遍：`subscribers` vs `targets` 已治，**`up` vs `start-panel.sh` 未治**；这次排查为它付了十轮 | 只有一处知道「怎么起栈」；再加一个参数不会只落到一半 |
| **L2** | **能力断言改运行期**：live 检查时断言 `list_tools() > 0` | 现在是文本断言，挡不住「插件目录空了 / 挂错路径 / manifest 坏了」 | 断言「能做」，不是「在听」 |

**注**：`start-panel.sh` 已补 `--plugins-dir`（`3cb437e`），`ab_verify.sh` 已加文本守卫（含反向自检）。
上面两条是把这件事做成**结构上不可能再犯**，而不是「这次补上了」。

## 未修（已登记，别让它消失）

| # | 事项 | 实测 | 修法与判据 |
|---|---|---|---|
| **U1** | **一个周期里第二次 LLM 调用的时长不可见** | `run-24135d386e8e8452`：span **8.0s**，行上 wait 合计 **4.6s** ⇒ **3.4s 不属于任何行** | `|span − Σ可见wait| ≤ 500ms` 可作为断言；根因在 anaphase（第二次调用的记录与归属），不是本侧渲染。**导出头部已把差值打印出来**，不再让它沉默 |
| **U2** | **REPLY 行的 token 是"该次调用总量"（含重发上下文），列名却只写 tokens** | 同段 total **2531**（prompt 2414 + completion **117**） | 列名已改为 **`call tok`**；若要更细，按 call 归属（本侧已有 `usageBySource`，可按 metering 与其后第一个已绘制行配对） |
| **U3** | **命名事故只做了"遮蔽"这一类**（`naming_test.js`）；另一类**做不成扫描**：`state.chatJobId = …` 落到 `id="state"` 的命名访问，与**故意的** `mode.textContent = …` 静态同形 | 闸门 55 → 46/1 | 结构性解法是**禁命名访问**（shell 里一律 `getElementById`）——那是对既有可用代码的重构，不是一条断言。在此登记 |

## 下一步

**批次 0–4 已完成**（4 见上）。**批次 5–7 见 `PLAN-targets.md`。**

**用户侧还剩 4 项诉求**（架构杠杆已接通，不会再跑掉）：
**导出证轨 / 三分布局 / 密码解锁 / Tentacle 搜索**。
用户原话：**「做完这一次切换，请立刻转向导出与布局。」**

**核心判断**：接上 `ADR-0018` 的 **`one tape, many targets`**。
现状不是「接口空置」，是**两套机制并存**：`flush()` 消费 `subscribers`，
而 `activeTargets()` **无人调用**（`assembly.js:171` / `:289` / `:328`）。
**批次 2 是杠杆** —— 接通后，每加一个视图从「写一套取数逻辑」降到「注册一个 target」。

## ⚠️ 两条已修正的结论（**别照旧版施工**）

1. **`K15` 的修法**：**不要**做 `session_id`（容器锚）与 `job_id`（内容锚）**分离** ——
   两家一致否掉（会多出「不是从内容派生、必须被写入并传播」的第二真相源；
   且引入容器 ⇒ 成员表与物理 `resume_from` 两份事实，必然裂脑）。
   **正解**：**`job_id = derive(输入 ‖ resume_from)`**，root 用 ∅；
   **会话 = root/head 指针**（不落盘、不新增实体）。
   这同时解决 `K15`（无需会话实体）与「5 轮挤一个文件」（非单射）。
2. **导出**：**Pull-only** —— 用户点击时才 `generate(snapshot)`，
   **不常驻 `activeTargets`**（否则每次 token 刷新都全量序列化）。
   仍需**同步上锁**（导出能绕过视图锁）。

## 纪律（今天用血换来的）

- **多分支匹配一律 `grep -E`，永不用 `\|`** —— 今天因此踩空 **8 次**。
- **合成数据证明算法对；真实数据证明对的是它。**
- **跳过的检查读起来像通过** ⇒ 缺失样本必须 **FAIL**，不能 SKIP。
- **API 数是服务端事实，不是消费方事实。**
- **检查一个约束，不如让违反它变得不可能。**
- **写注释前先自问语言**：源码英文，沟通中文。
- **换数据源之前，先拿真数据把两层**「逐字段对拍」——本轮 6 类缺陷全部由此查出，
  其中 4 类**读代码看不出来**（游离 `—` 与诚实缺失同形；`LLM TIME` 恒 `—`；
  digest 冒充产物；metering 变行）。对拍脚本**不进仓**（它比较的那一侧即将消失），
  但**对拍这个动作必须做**。
- **「0 行」的根因常常是你喂错了输入端**，不是字段名 —— 本轮真因是
  **把 events 喂给了收 node 的层**。改字段名改了两次都没动它。
- **哨兵要喂正确的那个产物**：`verify_live` 被指向**冻结快照**（JS 跑完后的 DOM），
  于是「应用自己设过的 inline style」被读成「烧入坏了」⇒ **两条永久假红**，
  而永久假红会训练人忽略检查。现在它**拒收冻结快照并说明该喂哪个文件**。
- **一条自己算不出来的断言毫无价值**：e2e 的判据「整条链」由**测试自己从 API 重算**，
  不向应用要数 —— 否则应用自己的计数只会与自己一致。

## 本轮待提交（2026-09-16 下午，**Human confirmation before commit**）

> 全部改动已验证（测试绿），但按 phyt-DNA「no auto-commits」未 commit，等用户拍板。

**anaphase-helix**（分支 `rs`）——P0-G + P0-D-1 + 判据测试：
- `src/contract/mod.rs`（Option&lt;Expect&gt; + serde default + 2 回归测试）、`src/pipeline/mod.rs`（None→ok + diag 留痕）、`src/run_cycle.rs`（fail-closed）
- `tests/tentacle_live_web_search_p0g.rs`（新增，`#[ignore]` 手动跑）、`tests/common/mod.rs`（helper 提升）、`tests/tentacle_live.rs`（改复用）
- 全量 cargo test 0 失败；判据 3/3

**Cellrix**（分支 `rs2`）——4a 几何：
- `web/assets/base.html`、`components.html`（高度链/chat flex/ses-side clamp/窄屏 120px）、`flows.html`（fl-body）、`prove_track.css`（#s-main 双列 grid 第三栏 + e-scroll/e-vp/e-traj 高度链）、`web/tests/layout_test.js`（判据 A 修正 + 守卫 + PROBE）
- `.gitignore`（+node_modules/）；layout_test 13/13 + 55/0 + 59/0 + run_all 8 套

**helix-tentacle**（分支 `rs`）——文档（顺手项）：
- `README.md`（构建命令修正：主 bin 无 feature；tentacle-tools 才声明 bloom/wasm/js/runtime/scraper）、`docs/PLAN.md`（forager 未接线标注 + cfg "{}" 待办）

**helix-mind**：无代码改动（本 HANDOFF 在 git 仓内，若 commit 一并带上）。

**仍挂起（不阻塞提交）**：①800px 窄屏 chat 单列 input ~15px 出视口（Q5 退化带，登记）；②expect 30/30 全 ok ⇒ "从契约删除 Expect 字段"方向提出但**未批准**（现裁决 Option+默认+留痕）；③旧主线三裁决（ADR-0018 T5 / K11 metering / K13 D3 违约修复）。
