# 生态共识台账（consensus ledger）

> **为什么有这个文件**：到 2026-10-30 为止，我们十几轮达成的共识**大部分只在对话记录里**，
> 不在任何一个仓的文件里。这是当前**最大的腐烂风险**——比 108 处悬空引用更严重：
> 悬空的引用至少还在仓里（只是指歪了），**共识根本还没落地**。
>
> 腐烂三形态：① 判据还在、引用漂了（K-131）② 文档改了、索引没同步（ADR-0042 状态行）
> ③ **共识在对话里、不在仓里（本文件要消灭的那一种）**。
>
> **判据**：复用 `anaphase-helix/ci/check_ci144_profile.py` 的「声明-证据-状态」形状，
> 不为台账新造判据（勿增实体）。跑法（在 helix-mind 根）：
>
> ```
> python3 ../anaphase-helix/ci/check_ci144_profile.py --root . --file docs/helixECO/consensus-ledger.md
> ```
>
> **规则**（与 profile 同一份，R1–R4）：
> `adopted` ⇒ evidence 必须可解析（`path` 存在 + `::symbol` 能 grep 到）；
> `absent` ⇒ 必须带 `revisit:` 事件触发；`deviated` ⇒ 必须带 `why:`；
> 读到 0 条 ⇒ exit 3（读到空 ≠ 干净）。
>
> **对称纪律**：`absent`/`deviated` 是豁免机制 ⇒ 它们必须**带触发条件或理由**，
> 且**豁免总数随读数一起报出**（判据会打印 `absent=N, deviated=M`）。
> 否则豁免就变成绕过纪律的一句话理由。
>
> **元声明（必须随本文件一起读）**：本台账**不声称完备**。
> `last-synced: 2026-10-30`。它只覆盖「有明确裁决、或双方无异议且已决定落地」的项；
> 讨论中 / 提出过 / 倾向过的一律不进——**台账的价值来自区分力，不来自条数**
> （200 条台账的绿什么也不说明，29 条台账的绿还有信息量）。
> ⇒ 在读本文件之前请先接受：**没写在这里的共识，可能是我们达成过但还没落纸的**。
> 这个元声明的作用不是免责，是让「覆盖率不可测量」这件事**每次都被读到**。

<!-- profile-claims:start -->
| id | 共识（一句） | status | evidence |
|---|---|---|---|
| C-001 | 三层模型：想/说/做；拦截的前置是分类，不是更多判据 | adopted | `../thinking/phyt-DNA的任务编排思路/GLOSSARY.md::0.5 三层模型` |
| C-002 | 执行点停 + 第四态（bound-then-dark 是一等状态） | adopted | `../anaphase-helix/src/pipeline/mod.rs::BindingState` |
| C-003 | 判据 > 修复：先造判据看它红，再修，再看它绿；修复可分批 | adopted | `../anaphase-helix/ci/check_adr_refs.py::RED` |
| C-004 | 红的价值 = 可变性，不是存在性：读到 0 是假绿，读到恒定的红是假红 | adopted | `tools/check-source-language.py::CHECKER DEAD` |
| C-005 | 声明-证据-状态三态判据可跨仓复用（一个工具，两个表） | adopted | `../anaphase-helix/ci/check_ci144_profile.py::STATUSES` |
| C-006 | 知识防腐烂：共识必须落进仓、有落点、有判据、有触发条件 | adopted | `docs/helixECO/consensus-ledger.md::profile-claims:start` |
| C-007 | AITL ≠ HITL：AITL 只做 authentication（便利），**不得提升授权上限** | absent | revisit: 事件触发「首次实现 agent 间身份认证通道（ssh key 级）」；未拍前不得实现 |
| C-008 | 学习是一等状态：候选 ≠ 已授权；§2 能力注册表是学习闭环的**唯一出口闸门** | absent | revisit: 事件触发「反射集首次需要运行时新增条目」；未拍前不得写学习代码 |
| C-009 | 档位真值源 = 后果探测（① 物理执行面 ② 不可逆·资金能力面，任一命中即升档） | absent | revisit: 事件触发「首个金融动作（购物/汇款）进入能力注册表」 |
| C-010 | 生态自有**不可逆动作清单**（金融与物理同档），不借协议词 `CRITICAL` | adopted | `docs/helixECO/irreversible-actions.md::草稿` |
| C-011 | 字符串字面量边界：代码/注释/commit 英文；用户可见 i18n 文本可中文但须 `i18n-ok:` 具名留痕 | absent | revisit: 事件触发「§0 状态表把本条从 ❌未拍 改为 ✅已定」；现状：判据已支持 marker，但**口径未拍** |
| C-012 | `commonintents` 不纳入源代码语言扫描（只读语料，不能修 ⇒ 红不可归因） | absent | revisit: 事件触发「commonintents 变成可修仓」；现状仍纳入扫描（13593 行里的 1059 行） |
| C-013 | 源代码 + 注释 + commit 必须英文；`.md`（含 `*.zh-CN.md`）允许中文，不溯及既往 | adopted | `docs/helixECO/ECOSYSTEM.md::语言规则` |
| C-014 | 豁免机制必须配「必然被豁免」的正样本，且豁免总量必须可见 | adopted | `tools/check-source-language.py::all-exempt-not-dead` |
| C-015 | 金融操作永远不进反射弧（可做，但必须走审议 + HITL） | adopted | `docs/helixECO/irreversible-actions.md::草稿` |
| C-016 | 撤回项必须可查：撤回清单入台账，第五态 `withdrawn` 必带 `why-risk-gone:` | adopted | `docs/helixECO/consensus-ledger.md::withdrawn` |
| C-017 | 撤回：break-glass（它是被消灭的旁路换了个持有人） | withdrawn | why-risk-gone: 「被约束者不能拥有解除约束的权限」仍然成立；复活须先证「agent 无法自我触发该通道」 |
| C-018 | 撤回：审计影子期（含灰度期 / 试运行 / 观察期任何别名） | withdrawn | why-risk-gone: 加事件类型落 `payload.kind` ⇒ 零迁移 ⇒ 无不可逆风险需要影子保护；改名不算证据 |
| C-019 | 撤回：撤回清单只写在对话/ROAD 文档里 | withdrawn | why-risk-gone: 本台账已存在且判据可跑；只有当台账再次失效才需重建该结论 |
| C-020 | 撤回：「以 §9 为准」代替逐条裁决 | withdrawn | why-risk-gone: §9 就是被驳回那个错的产物；循环批准不可作为证据 |
| C-021 | 撤回：用新 ADR 杀掉 / 驳回 ADR-0007（应写 superseded 或补注） | withdrawn | why-risk-gone: ADR 批准即历史，只能被取代 |
| C-022 | 内容安全四动词：放行 / 拦截 / 打码 / 告警 / **未启用**（五个，第五个最常漏） | absent | revisit: 事件触发「Tuck 出站打码（说侧）开始实现」；现状：入站打码已实现，出站只有 demap |
| C-023 | 两轴拆分：发生自由度（想/说允许→做阻断）与内容控制（边界上打码/切除）不得压成一轴 | absent | revisit: 事件触发「GLOSSARY §0.5 按园丁裁定改写」 |
| C-024 | 「不存在」不许写成「待实现」 | adopted | `docs/helixECO/irreversible-actions.md::待定` |
| C-025 | 状态 SSOT：一份文档只允许一处给批准状态，其余位置只能引用 | absent | revisit: 事件触发「下一版共识转达文档起草时」（防 §0/§9 冲突复发） |
| C-026 | 待拍清单本身必须可枚举：条数由判据数出，且不得为 0 | absent | revisit: 事件触发「首次出现某轮待拍项从清单上消失」；现状靠人工逐条列表 |
| C-027 | 跨规范引用字段前，先确认该字段在本生态有落点 | adopted | `../anaphase-helix/docs/ci144-profile.md::本生态没有落点` |
| C-028 | 同名覆盖 = 静默失效新形态（目标在，但被同名者顶替）；抓到它的是豁免正样本 | adopted | `tools/check-source-language.py::bucket_of` |
| C-029 | CI-144 profile §2.1 **不采纳** PFP 的 `HARD_OVERRIDE`（灾难时无条件放行） | deviated | `../anaphase-helix/docs/ci144-profile.md::HARD_OVERRIDE` ；why: 它是「被约束者给自己开的放行通道」，与 ROAD-18 B-4 直接冲突；复活前提 = 存在物理执行面 且 来源不可伪造 |
| C-030 | 借外部协议词汇（`CRITICAL` / `HARD_OVERRIDE`）做生态动作判据 | rejected | why-never: 借了就要再做一次「协议枚举 ↔ 生态动作」的映射 = 第二份真相；生态自有清单才是单源 |
| C-031 | 把「聊过的都进台账」（产出即留痕，不做准入） | rejected | why-never: 那是把「读到空」翻转成「读进一切」，更坏的假绿；绿必须来自区分力 |
| C-032 | 源代码 + 注释英文（C-013）：`Tuck/crates/tuck-core/tests/financial_hitl_forensic.rs` 与 `financial_hitl_acceptance.rs` 曾是中文（107 + 38 行） | adopted | `../Tuck/crates/tuck-core/tests/financial_hitl_forensic.rs::no_test_fixture_may_encode_break_glass` |
| C-033 | 预期红必须具名：判据红着进仓/进 CI 时，红的理由要写在入口旁边（清单只许缩短，禁 `#[ignore]`） | adopted | `../Tuck/crates/tuck-core/tests/EXPECTED-RED.md::EXPECTED-RED` |
| C-034 | 「冻结」= 我不动它；「没动」≠「它没变」 | adopted | `../thinking/phyt-DNA的任务编排思路/GLOSSARY.md::3.13` |
| C-035 | 两物一名必须拆（K23：`NeedHumanConfirm` 触发侧 ↔ `HitlRequired` 终局） | absent | revisit: 事件触发「B2 的 `NoResponder` 落地时同批拆」；现状：仍是终局拒绝 |
| C-036 | 观察态 ≠ 授权态（候选 ≠ 已授权之外的第二同族） | absent | revisit: 事件触发「反射学习闭环动工（C1）」；现状：无观察态概念 |
| C-037 | 锚定/签名链只增不改前像（改名必断指纹） | adopted | `../Tuck/docs/decisions/ADR-0007-audit-event-vocabulary-lives-in-payload.md::D2` |
| C-038 | C3 判据 A 用 anaphase 本地形态；**B1 落地时必须收敛成一套**（两份分类并存 = 第二份语义） | absent | revisit: 事件触发「B1（Tuck 判定 / anaphase 执行）落地时」；B1 的改动必须触碰本行 |
| C-039 | 不可逆清单数据在 helix-mind、判据在 anaphase ⇒ **跨仓引用 + 校验存在与版本哈希**，禁止内联副本 | adopted | `docs/helixECO/irreversible-actions.md::草稿` |
| C-040 | C3 接线的**当前真值**：`I-1..I-7 一律拒绝`，**不是**「无授权才拒绝」（「已授权」状态依赖 K23 拆分） | absent | revisit: 事件触发「K23 两物一名拆成 Pending{token} + Reject」；代码注释已写死同理一句 |
| C-041 | N 层欠账：判据 A 是生态级变更（改「做」的边界），A 层与 D 层已更，**N 层未动**（原则 10：三层同时更新） | absent | revisit: 事件触发「人类裁定是否进方向锚 `center/SESSION-DAG-IS-THE-CENTER.md`」；`docs/VISION.md:37` 为原则出处，改动需等量替换 |
| C-042 | `tools/expiry-census.py`（豁免有效期判据）**零调用点**：只在文档里被提到，任何脚本/CI 都不跑它 | absent | revisit: 事件触发「豁免机制下次到期前」；读数：`grep -rn 'expiry-census' --include=*.sh --include=*.yml .` ⇒ 0 命中（仅 .md 提及）⇒ 过期豁免只能被线预算判据顺带撞出 |
| C-043 | 跨仓判据的两个读空口子：`git diff --name-only` 不带范围只看未提交改动；CI 单仓检出没有兄弟仓 ⇒ 「根因文件是否被触碰」恒读不到。[revisit: **任何跨仓判据改动时** —— 教训类，永不删除] | adopted | `../Tuck/tools/expected-red.py::def touched_files` |
| C-044 | 跨仓/跨层"根因文件是否被触碰"必须**按仓选池 + 后缀匹配**：纯后缀会让 `anaphase-helix:src/hitl.rs` 满足 Tuck 的 `src/hitl.rs`（两仓同名文件）⇒ 绿得没道理。[revisit: 任何 expected-red 名单/路径改动时] | adopted | `../Tuck/tools/expected-red.py::def touched_files` |
| C-045 | **空名单必须判成功**：把"清零"当故障 ⇒ 压力变成"留一条假条目"，而假条目正是这套机制要防的东西。[revisit: 任何 expected-red 名单/路径改动时] | adopted | `../Tuck/tools/expected-red.py::def judge` |
| C-046 | **B1 裁决（2026-11-01，据实测）**：Tuck `HumanConfirmGate` 零生产调用者，anaphase `ApprovalOutcome` 已是四值闭集 ⇒ **Tuck 判定 / anaphase 执行**；Tuck 侧 gate **废弃，不接 `router.rs:39`**。★ 实测：anaphase 的 `Unavailable` 原文是「**没有应答者或应答者失败**」= **合并语义**，与决策 (a)（窄语义）冲突 ⇒ B2b 迁移**必须收窄，不许沿用**（会让 anaphase 既有测试变红，属预期成本）。；★ 读数（2026-11-02）：anaphase 侧 `ApprovalOutcome` **有生产调用者** —— `src/run_cycle/safety_gate.rs:83-92` 逐值匹配，`admit()` 由 `src/run_cycle/mod.rs:1137` 与 `:1242` 真实调用 ⇒ B1 不是"从一扇死门换到另一扇死门"。[revisit: 任何 HITL 路由/状态改动时；另：若 anaphase 执行侧届时不可达 ⇒ 重评是否保留 Tuck 侧 gate] | adopted | `../anaphase-helix/src/hitl.rs::enum ApprovalOutcome` |
| C-047 | **「空 / 缺席」在同一轮里被处理错三次**：① 空名单 = 故障（清零被惩罚 ⇒ 逼人留假条目）② 无收据 ⇒ `max([])` 崩溃（"没有收据"报成"工具坏了"）③ 外部命令 `rc=127` 不可量却当失败/当绿。三次同族：空集被当成**异常 / 0 / 失败**，唯独没被当成「空」这一独立事实。[revisit: **任何新增或修改判据入口时**（`rings*.json` / `expected-red.list` / `ci/*.py`）—— 旧写法"任何处理集合/收据/退出码的代码路径改动时"几乎命中一切，等于不会响] | adopted | `../Tuck/tools/expected-red.py::def judge` |
| C-048 | **读数更正两处**：①「台账 revisit 363 处 / 消费脚本 0 个」是未核引用 ⇒ 实测 `grep -c revisit` = **21**（带 revisit 的条目行 20）；② 「1 个消费方」**也是假的** —— 那唯一命中是 `anaphase-helix/ci/check_ci144_profile.py:172` 的**文档字符串提到台账名**，它读的是四列结构，**不读 revisit 字段** ⇒ **revisit 消费方 = 0**。⇒ 结论不变但更强：记了，没人消费，且 `governance/triggers.toml` 只装了 6/20 ⇒ lint 一接就会红 14 条（须先补齐或标"纯文档不自动触发"）。[revisit: **任何新增或修改判据入口时**（`rings*.json` / `expected-red.list` / `ci/*.py` / `governance/triggers.toml`）] | adopted | `governance/triggers.toml::C-047` |
| C-049 | **kernel 里 `urgency` 已存在，故不加 `blocks_decision_of` / `cost_grows`**（勿增实体）：`kernel.py:107` 落 `urgency`、`:735` selftest 已用 `urgency="P2"`、`NO-REVISIT-ON` 在 `:481`，selftest `:739` 已有"缺 revisit-on 也算违规"。⇒ 正确形态 = `triggers.toml` 每条注册成 finding（`revisit_on` + `urgency`）⇒ `overdue()` 自动响；**唯一要补的是** `schedule()` 不看 urgency、`overdue()` 无 CLI 入口。[revisit: 任何 kernel scheduler/CLI 改动时] | adopted | `../thinking/phyt-DNA的任务编排思路/kernel.py::urgency` |
| C-050 | **生产上正在生效的是我们判定为错的合并语义**：anaphase `ApprovalOutcome::Unavailable` = 「没有应答者**或**应答者失败」，由 `src/run_cycle/safety_gate.rs:83-92` 逐值匹配、`src/run_cycle/mod.rs:1137`/`:1242` 真实调用；而 Tuck 侧窄语义 `NoResponder` 生产调用者为 **0**（已被 B1 判废弃）。⇒ 一条 `Unavailable` 无法区分「没人值班」与「值班的人联系不上」，两者处置相反（加人 / 修链路）。B2b 收窄前，该歧义**持续存在于生产**。[revisit: B2b 那一批（独占）] | adopted | `../anaphase-helix/src/run_cycle/safety_gate.rs::ApprovalOutcome` |
| C-051 | **判据工具自身的改动，必须过与新增判据同等的 mutation 验证**：C-045 的修复只跑了「复原后 rc=0」，没人跑「改坏后是否 rc≠0」⇒ 装进一个"套件根本不跑"的洞（mutation 第一次真跑就抓到）。[revisit: **任何 `expected-red.py` / `verify.sh` / `rings*.json` 改动时**] | adopted | `../Tuck/tools/expected-red.py::def judge` |
| C-052 | **空集静默第四次**：`kernel.py --status` 的 `if fnd:` 守着输出 ⇒ 台账为空时什么都不打印，与"没有入口"不可区分。已改无条件打印「登记项 0 条」。（前三次：空名单判故障 / `max([])` 崩溃 / `rc=127` 不可量）[revisit: 任何处理集合 / 收据 / 外部命令退出码的代码路径改动时] | adopted | `../thinking/phyt-DNA的任务编排思路/kernel.py::登记项 0 条` |
| C-053 | **「存在即合规」的判据是伪证生成器**：按字面规格（文件存在即可）实现 DEATH-CERT 判据，补 4～6 份**空文件**即转绿，而 `DEPRECATE.md:30` 要求 8 个字段、`:33` 说「不合规的退役写成成就是伪证 = I2 红线」。⇒ 判据必须查**内容非空**，不是查存在。[revisit: 任何「存在性」判据新增或修改时] | adopted | `../phyt-DNA/DEPRECATE.md::死亡证明（强制）` |
| C-054 | **分母按位置取 = 误伤**：`archive/deprecated/*.md` 有 6 份，其中 `kind: plan-done-archive` 与 `kind: truth-source-removal-record` **不是退役记录** ⇒ 按目录 glob 取分母会红 6 条（其中 2 条不可归因）。正确分母 = `kind: deprecation-record`（4 条），id 取 frontmatter 的 `deprecated:`。[revisit: 任何按目录取分母的判据新增或修改时] | adopted | `../phyt-DNA/archive/deprecated/PLAN-done-2026-10-12.md::plan-done-archive` |
<!-- profile-claims:end -->

## 当前读数（H0：我由哪条命令派生）

```
python3 tools/check-source-language.py
  scanned 28242 source file(s), 83 exempt, 28159 checked
  RED 13593 行（单位：行；分项见下，分项之和恒等于总数，按构造）
```

分项（2026-10-30，节选最大的几个）：anaphase-helix 2559 · Cellrix 3569 · helix-tentacle 1150 ·
helix-mind 1141 · commonintents 1059 · Glove 734 · thinking 705 · Cellrix update 594 · Tuck 482。

**`exempt 83` 已核对**：`find ./phyt-DNA -type f | grep -cE '\.(rs|py|go|ts|tsx|js|mjs|c|h|cc|cpp|sh|java|rb|kt)$'` ⇒ **83**，与该计数器相等。

**此判据故意未接 CI**：一次打红 13593 行会让所有仓常红 ⇒ 红失去信息量（C-004）。
接入条件（事件触发）：按仓降到可处理规模，或改为只判「新增/修改行」。

## §4.4 正交表 · 拆成独立三行（**允许写三行，不许合并**）

原来一格「金融 = HITL / 委托 / 2PC」把三样不同成熟度的东西并在一起。拆开：

| 项 | 状态（三选一，不许写「待实现」以外的含糊话） | 落点 |
|---|---|---|
| **2PC** | **已有协议落点** | CAPABILITY-13 §3（慢路径共识） |
| **HITL** | **形状未拆** | K23 两物一名：`Tuck::NeedHumanConfirm`（触发侧）↔ `anaphase::HitlRequired`（终局拒绝） |
| **委托** | **不存在** | 无代码、无协议、无判据。写「待实现」会让读者以为它在路上 |

⇒ 纪律：**「不存在」不许写成「待实现」**（已入册 C-024）。
⇒ 这三行的成熟度不同 ⇒ 它们**不能共用一句话描述**，否则最强的那一行会给最弱的两行背书。

## 覆盖率读数（2026-10-30，H0：见下方命令）

```
grep -c '^| C-' docs/helixECO/consensus-ledger.md          # 已入台账
```

台账 **28** 条。**已共识但未入台账的差额**——我要诚实说明：**我没有一份权威的「已共识清单」**，
所以这个差额**不可机械枚举**（这本身就是 C-026 那条的实例）。我能逐条点名的是**我记得住且能落到仓里的**，
下面这些**仍未入台账**，也就是说它们**正在腐烂**：

| 未入台账的共识 | 现在住哪 |
|---|---|
| 「冻结 = 我不动它；没动 ≠ 它没变」 | `thinking/.../GLOSSARY.md §3.13`（只在仓里，没进台账，所以没有触发条件） |
| 「存在性 ≠ 生效性」母题（七例谱系） | 同上 §3.12 |
| 「两物一名必须拆」（K23：`NeedHumanConfirm` ↔ `HitlRequired`） | helix-mind KNOWN_ISSUES K23 |
| 「判据要能红得有意义才配叫门」 | 同上 §3.17 + 台账 C-004 |
| 「观察态 vs 授权态」（候选 ≠ 已授权之外的第二个同族） | 无落点 |
| 「不借外部协议词汇做生态判据」 | 本台账 C-010 隐含，未单列 |
| 「锚定/签名链只增不改前像」 | Tuck ADR-0007 D2（有落点，未入台账） |
| 「先造判据看它红，再修，再看它绿」（K24 范式） | 本台账 C-003（已覆盖） |

**一条实测更正**：本轮之前台账读数 `deviated=0`，而 C-029 这条偏离**确实存在**（`ci144-profile.md` §2.1 自述「本条是对不可协商部分的显式偏离」）
⇒ **`deviated=0` 不是「本仓无偏离」，是「那条偏离没进台账」= 读到空的第 N 次**。补上后读数 `deviated=1`。

⇒ **结论**：台账覆盖率**不可测量**，因为缺一份「已共识」的权威清单。
在补齐清单之前，本台账**不能声称自己覆盖了全部共识**——它只覆盖「我记得住的那部分」。
这是本文件的**具名局限**，不是谦虚。
