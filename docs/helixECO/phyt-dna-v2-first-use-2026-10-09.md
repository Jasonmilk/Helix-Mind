# phyt-DNA v2 首次真实使用 —— 观察、已落地项、以及给 v2 的提议

> 2026-10-09 ｜ 人类：*"v2 第一次真实使用，以前是无约束的 v1，导致项目混乱生长"* ｜ **本轮未改 phyt-DNA 模板**

## 一、v1 无约束造成的**可观测后果**（本会话逐一撞到的）

| 观察 | 性质 |
|---|---|
| `docs/VISION.md` 停在 **v1.0**（96 行），而 v3.1（496 行）已存在 | 权威文本漂移 |
| **24 份 ADR 全是散文**：`hard:`/`applies-to:`/`check:` **各 0 份** | 判据**不可执行** |
| **`v12` / `DAG v5.0` 有文本未入库**（在工作区 `Cellrix update/`） | 被委派产物缺席 ⇒ "内容挤在一起" |
| **`L0 — TEMPORARY`** 注释从未退役（`script.html:133`、`mergeChain`） | 临时件永久化 |
| **`asset_parity_test` 用 mtime** ⇒ `touch` 即可骗绿 | 假绿判据 |
| **Cellrix 无 CI**；`ledger/`、`fixtures/`、`tools/*.sh` 全缺 | 三件本该机械化的纪律靠人守 |
| 面板**只有一个窗口且是血缘路径**（`script.html:143` 自注 L0 TEMPORARY） | 装错菜 |
| `docs/archive/VISION.v2.md` 声明保留但不存在 | 经历链断点 |

**⇒ 这不是八个独立毛病，是同一个：**判据没有被执行，所以没人发现它们。**v1 的"无约束"最终表现为"无人能回答'现在到底成不成立'"。**

## 二、本轮已落地（都在 Cellrix）

1. **`fixtures/ADR-0049-asset-parity/inject.sh`** —— 首个夹具（手工变异第一次机械化）
2. **`tools/validate.sh`**（承模板 + **零闸门具名 BLOCK**）
3. **`ledger/{README.md,hits-2026.jsonl}`**
4. **`docs/decisions/ADR-0049-asset-parity.md`** —— 首份「闸门式」ADR
5. **`run_all.js` 每次跑完自动写账**（`kind: scan`）—— 取代那张手写的 `CRITERIA-INVENTORY`

**实测**：probe→RED ✅ · 扫描→pass ✅ · override→留痕 ✅ · 连跑三次→账本三行（append-only）✅ ·
零闸门→exit 2 + 具名 ✅ · 钩子通过（`commit-exit=0`）✅

## 三、★ 给 phyt-DNA v2 的提议（每条都有本会话的真实反例可挂）

**P1 · ADR 需要"双契约"写法，并写进 v2 模板。**
Cellrix 的 pre-commit 钩子要求**首行是 ADR 头**；模板把元数据放**文件开头**的 front-matter ⇒
**首次提交被 REJECT**。已证明可共存：**H1（含 ADR 号）首行 + front-matter 紧随其后**。
⇒ 建议 v2 直接规定这个写法，否则**每个采用者都会撞一次**。

**P2 · 路径必须声明式，不得硬编码。**
模板用 `decisions/`（仓库根），Cellrix 用 `docs/decisions/` ⇒ 我不得不改脚本。
⇒ v2 应把 `decisions/`、`fixtures/`、`ledger/` 的**位置**做成一处声明（符合采用者自己的「0 硬编码」）。

**P3 · ★「零闸门」必须具名 —— 建议回流模板。**
`validate.sh` 在**未登记任何 `hard: true` ADR** 的项目上会**静默 pass**（`scanned:0`）。
这正是 v1 的病：**空集合上的"全部通过"是伪证**。我已补 `exit 2 + 具名理由`。
⇒ **这是本轮最该回流模板的一处。**

**P4 · 需要「散文 ADR → 闸门 ADR」的过渡策略。**
`check-baseline.sh` 会把**未登记文件判红** ⇒ 已有 24 份散文 ADR 的项目一上 v2 就全红。
⇒ v2 需要显式双轨期（例如 `legacy: true` 标注，逐步迁移），否则采用者只能两种坏选择：
不迁移（闸门空转）或一次性改写权威文本（违反"隐性化≠删除"）。

**P5 · `fixtures/<gate-id>` 的 id 一致性应有 lint。**
我把目录命名成 `asset_parity` 而非闸门 id，`--probe` 才报"缺 fixture"。
⇒ 建议 `validate --lint` 直接检查**目录名 = gate-id**（这是"注入目标 = 检测目标"的机械保证）。

**P6 · ★ 账本行必须带环境 —— 建议 v2 吸收。**
Cellrix 的裁决行带 `[E: cdp=… panel=… siblings=… jsdom=…]`，理由是 ADR-0048 §200：
**"没有环境的计数在跨 commit 时不可比"**。本轮实测确证：同一 commit，
`proven 55 → 77 → 80`、`held 31 → 1`、`red 2 → 8 → 6`，**只因为 `panel/cdp` 从 down 变 up**。
而模板的 ledger schema **没有环境字段** ⇒ 账本行**不可比**。

**P7 · ledger 的计数应结构化，不要只放 `note`。**
`kind` 区分了 task/scan/probe…，很好；但 `note` 是自由文本 ⇒ 机器无法比较两次运行。
⇒ 建议 `note` 之外加结构化字段（如 `{"counts":{"proven":80,"red":6}}`）。

**P8 · 「判据真跑」应当成一等公民。**
VISION v3.1 自己写着 DAG v5.0 是**"判据真跑"**，而 DAG 与 v12 **此前不在仓库里** ⇒ 声明无处可查。
⇒ v2 应把"被委派的产物必须存在于仓库"做成检查（否则"归入 v12"这类委派会**静默失效**，
而它正是本轮"内容挤在一起"的真因）。

## 四、本轮**未做**

- **未改 phyt-DNA 模板一个字节**（提议 P1–P8 尚未回流；改动共享模板影响所有采用者，应先经人类裁决）
- `claim-check.sh` / `spec-lint.sh` / `check-baseline.sh` 未移植（未跑通的不入库）
- 旧 24 份 ADR 未迁移（不动权威文本）
- 四条新红未定归属（`layout_test` / `hit_targets_test` / `s303_continuation_test` 报 `null` 测量值；
  `all_views_test` 单独跑 exit=0 ⇒ 环境/顺序）。**它们在我 P3 改动之后出现，但我的改动只涉及窗口作用域
  与一个调用点，未碰 CSS/布局/指针解析 ⇒ 我判断无关，但我**没有证明**，故记为未归属。**
