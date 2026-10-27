# phyt-DNA adopt 版本普查（生态级 · 2026-10-28）

> 谁点的：园丁点名（生态级）。要回答的一句话是「生态共用一套方法论」这句话**有没有被验证**。
> 判读纪律：所有读数都是本次现跑的命令输出，命令与输出行在 §6；凡写「无」的地方必须先有 `= 0` 的命中数。
> 采集时 HEAD：anaphase-helix `082bc83`(rs) · helix-mind `9979a2a`(rs) · Cellrix `cd1660c`(rs) · FlowModus `9359a8f`(rs) · Tuck `71feaec`(rs) · helix-tentacle `2554ebf`(rs) · phyt-DNA `97c2640`(v2)。

---

## 0 · 一句话结论

**各仓对 phyt-DNA 是「引用」，不是「副本」**——唯一持有引擎副本的是 Cellrix（3 个文件，其中 2 个已本地分叉、滞后上游 369 行）。
**v1 已归档、且 v1 树里没有 `tools/`**（引擎是 v2 才有的东西）⇒ 「某仓 phyt-DNA 版本旧」这个说法在多数仓根本不成立：它们连引擎都没有。
真问题是另一条：**文档声明面有 44 个文件写「方法论 v1.0」（= 指向归档线），而真正在跑的是 v2** ⇒ 这是**可见性缺口**（声明面与执行面分家），不是版本债。

按园丁给的三档判读落位：**「有仓引用了但从不跑 ⇒ 那是静默，比版本旧严重」为主 + 「有副本 ⇒ 副本仓才是真债」一处（Cellrix）**。没有落到「全是引用 + 都在跑 ⇒ 只补可见性」这一档，因为「都在跑」不成立。

---

## 1 · 三问三答

### 1.1 各仓是 adopt 引用 还是 vendored 副本

| 仓 | 形态 | 机械证据（见 §6 命令） |
| --- | --- | --- |
| anaphase-helix | **引用**（并把 phyt-DNA 检出进工作区） | 引擎文件名 0 个；CI `checkout … repository: Jasonmilk/phyt-DNA, ref: v2, path: phyt-DNA`（`ci.yml:36`），用途只是 ⓪·3 工作区自检 |
| helix-mind | **引用（真调用）** | 引擎文件名 0 个；`tools/todo-ready.sh:228` 真跑 `bash "$ROOT/../phyt-DNA/tools/ci-matrix.sh"`，另读 `../phyt-DNA/VISION.md` 的 `git hash-object`、`PLAN/RNA` 行数、`decisions/` 清单、`ledger/hits-2026.jsonl` |
| **Cellrix** | **副本（vendored，且已本地分叉）** | `tools/validate.sh` · `tools/xray.sh` · `tools/upstream-manifest.sh` + 指纹 `tools/UPSTREAM.sha256`（3 行）；副本文件自己头注写「环境适配的副本」 |
| FlowModus | 引用（文档级） | 引擎文件名 0 个；提及 phyt-DNA 的文件 7 个 |
| Tuck | 引用（文档级） | 引擎文件名 0 个；提及 18 个 |
| helix-tentacle | 引用（文档级） | 引擎文件名 0 个；提及 8 个 |

副本源自哪一版（用 blob 反查，不是猜）：`validate.sh` = phyt-DNA `d9a16a0`（2026-10-12，「作用域收窄 · 锚点三态具名跳过 · SELF 自检 · mktemp 备份 + trap」）· `xray.sh` = `9d3d86f`（2026-10-12）· `upstream-manifest.sh` = **与 v2 HEAD 逐字相同**。
滞后量：`validate.sh` 之后上游该文件还有 **21** 次提交、`xray.sh` **1** 次；内容差异 **369 行**（`xray.sh` 28 行）。

### 1.2 v1 → v2 是加性还是破坏性？v1 是否 EOL？

- **EOL：是，逐字。** `README.md:21`：「**Authoritative: the `v2` branch (default).** `v1` is kept as historical archive only.」；`docs/V1-TRIAGE.md:11`：「本表经园丁一次性批准，**v1 归档结案，"不再回头"生效**」。
- **形态：模板 → 实例，加性为主，无消费者故无破坏。** v1 树递归 **21** 个文件（顶层 7 项：`README*` + `docs/` + `template/`），v2 递归 **187** 个（+ `decisions/` `tools/` `ledger/` `fixtures/`）。文件集差（排序后 `comm`，不猜 rename）：**仅 v1 15 · 共有 6 · 仅 v2 181**；共有的 6 个里内容变了 4 个（`.gitignore` `README.md` `README.zh-CN.md` `docs/PROTECTION.md`），未变 2 个（`LICENSE` `NOTICE`）。
- v1 独有的 15 个全在 `template/` 下：13 个正文（`DEPRECATE/DNA/GROWTH/PLAN/RNA/SPEC/VISION.md` + `decisions/ADR-0001-template.md` + `spec/{architecture,contract,philosophy,position,safety}.md`）+ 2 个 `.gitkeep` 占位。v2 把它们**并入仓根并改写**（`README.zh-CN.md:125`「v2：仓库根就是模板（原先的 `template/` 已并入根目录）」；实测 7 卷差异 43/172/163/122/335/298/143 行）⇒ 名字消失、内容搬到根。
- **没有任何仓引用 `phyt-DNA/template`：6 仓各 0 命中** ⇒ 搬迁无消费者，破坏性为 0。
- 因此「不升 v1 的仓」这一判读**不适用**：v1 没有引擎，"留在 v1" 的仓其实只是"引用了旧 README 的话"。

### 1.3 各仓是否真的在跑 phyt-DNA 工具

| 仓 | 跑了吗 | 证据 |
| --- | --- | --- |
| helix-mind | **跑了读数工具，没跑判据引擎** | `tools/todo-ready.sh:228` `bash ../phyt-DNA/tools/ci-matrix.sh`（**读数**：CI 矩阵，本仓报告头 7/7 ✅ 是它的输出）；`validate.sh`（**判据引擎**）在本仓调用点 = **0** |
| Cellrix | **跑了副本的校验器，没跑副本的引擎** | CI `ci.yml:45` `bash tools/upstream-manifest.sh`；`validate.sh` / `xray.sh` 在 `.github/` `tools/*.sh` `package.json` 里的 grep 命中 **2 行、全部是注释**（`upstream-manifest.sh:4` 说明 + `:13` 用法示例）⇒ **执行调用点 = 0** |
| anaphase-helix | **没跑**（只检出） | CI 对 phyt-DNA 的唯一用途是 `[ -d "$d/.git" ]` 工作区自检 |
| FlowModus / Tuck / helix-tentacle | **没跑** | CI 里 phyt-DNA 相关行 = 0 |

---

## 2 · 五条真问题（排序按 ROAD-11 §P0-④ 修正：**谎报 > 沉默**）

> 排序原则（2026-10-28 修正）：**正在谎报 > 只是沉默 > 可复现性**。旧排序把 P1（44 文件写归档线，沉默）放在 P2（宣称"与上游一致"，谎报）之前，是错的。
> 新增 §2.0：园丁 ROAD-11 §P0-③ 的读数——**判据引擎在生态里的执行调用点 = 0**，它比四条问题更根本，所以排在最前。

### P0 · 判据引擎 `validate.sh` 的生态执行调用点 = **0**

| 仓 | 跑的是 | 是判据引擎吗 | 执行调用点 |
| --- | --- | --- | --- |
| helix-mind | `ci-matrix.sh` | 否（**读数**工具） | 0 |
| Cellrix | `upstream-manifest.sh` | 否（**自签指纹**） | 0（`validate.sh` 的 2 处命中全在注释里） |
| anaphase / Tuck / FlowModus / helix-tentacle | 无 | — | 0 |
| phyt-DNA 自己 | `tools/ci-local.sh` | 是 | **19**（纯自证） |

复跑：`for r in <6 仓>; do grep -rn 'validate\.sh' $r --include='*.sh' --include='*.yml' --include='*.toml' --include='*.rs' --include='*.js' | grep -v "^$r/tools/validate\.sh:" ; done` ⇒ 只剩注释命中。

⇒ 「生态共用一套方法论」的真状态：**声明 44 个文件 · 引擎调用 0**。这是同一个母题在**采纳面**的形态（根 README 三处"已清理" · `over_total` 写了没实现 · K22 判据绿无人调用 · 本条）。
**不造仪器**（触发判据未达），只让这个数字可数：它随新 adopt 变化，命令在本节。

## 2 · 四条真问题（按严重度，全部带可复跑命令）

### P1 · 文档声明面 44 个文件写「方法论 v1.0」，而 v1 已归档、且没有引擎

`phyt-DNA v1` 的 README 自述 `Version: v1.0`（`git show origin/v1:README.md` 第 3 行）⇒ 兄弟仓里那句「所属方法论：phyt-DNA v1.0」在字面上指的就是**归档线**。按仓（排除 `archive/`）：

| 仓 | 声称 v1 的文件 | 声称 v2 的文件 |
| --- | --- | --- |
| anaphase-helix | 4 | 1（`ci.yml:36`） |
| helix-mind | 4 | 3（`STATUS-2026-10-10.md` · `gen/BRANCH-MAP.md` ×2） |
| Cellrix | 15 | 2 |
| Tuck | 15 | 0 |
| helix-tentacle | 6 | 0 |
| FlowModus | **0** | **0** |
| **合计** | **44** | **6** |

性质：**不是版本漂移，是可见性缺口**。这 44 个文件里没有一个是"引擎调用点"（引擎调用点只有 2 处：helix-mind `todo-ready.sh:228`、Cellrix CI `ci.yml:45`），它们全是**七卷的头注**——即"我是谁、我跟谁一套方法"的声明。声明指向归档线，执行跟着 v2，两者从未对账。
FlowModus 的 0 是独立问题：**0 声明 = 无契约**，不是"跟随最新"。

### P2 · Cellrix 副本是真债，且它的指纹是「自我一致性」而非「上游对齐」（**零成本那一半已做，2026-10-28**）

> 已做（自治档，不需授权）：成功文案 `OK: 引擎与上游一致` → `OK: 引擎副本与本仓自签指纹一致（…）—— 不证明与上游一致`；文件头注改写为"证明什么/不证明什么"；CI 步骤名 `引擎与上游一致（R4 附带：防分叉）` → `引擎副本未漂（自签指纹；≠ 上游对齐）`。Cellrix commit 见 `d-cellrix-vendored-engine`。
> 顺带发现：旧 `tools/UPSTREAM.sha256` 里**有一条自指条目**（`tools/upstream-manifest.sh` 自己）——脚本 `FILES=$(ls tools/*.sh | grep -vE '^tools/(upstream-manifest)\.sh$')` 明确排除自己，但清单是三行（含自己）⇒ **改文案 = 改指纹 = 自检当场红**。重签后 4 条（补上原先漏掉的 `determinism.sh` / `emit-failures.sh`），差异在提交信息里具名。
> 命名局限：这份指纹**无法在"不重签"的前提下修正文案/注释**——仪器把自己的维护成本记成了漂移。

- 副本与 v2 HEAD 差 **369 行**；同一文件头注在 2026-10-09 自述「与上游已差 **274** 行」⇒ 18 天涨 95 行，**漂移在加速而不是收敛**。
- 更硬的一层：`tools/upstream-manifest.sh` 的 `--write` 用 `shasum $FILES > tools/UPSTREAM.sha256` 把**副本自己的** sha 写进去，校验分支跑 `shasum -c` ⇒ 它证明的是"副本没被再改过"，**永远不能证明"副本与上游一致"**；而它的成功文案逐字写「`OK: 引擎与上游一致`」。
- 这属「文案宣称的比机制做到的更多」，与 `#[ignore]`（读起来像覆盖率）、"三件齐"里混 N/A 同族。**现在不修**（改 CI 行为面 = 升级档，需授权），登记见 §5。

### P3 · anaphase CI 把 phyt-DNA 钉在**浮分支** `ref: v2`

`ci.yml:36` 钉的是分支名不是 SHA ⇒ 同一条 commit 在不同日子检出不同的 phyt-DNA。目前无害（用途只是工作区自检），但一旦将来有人把 phyt-DNA 的判定接进 CI，就变成「读数不可复现」。与 `BRANCH-MAP` 记 HEAD 的纪律相反。

### P4 · 唯一"真跑引擎"的调用点没有版本钉

helix-mind `tools/todo-ready.sh:228` 直接跑 `../phyt-DNA/tools/ci-matrix.sh`（工作区里的那一份）。它不钉版本、也不校验上游指纹 ⇒ 同一句命令在不同工作区给出不同矩阵。这一条**是我每天在用的东西**（报告头的矩阵段由它出），目前靠"工作区里就是最新 v2"这个习惯，不是靠机制。

---

## 2.5 · 「phyt-DNA 引擎冻结」的范围澄清（园丁 P1-⑤）

**结论：冻结的是我给自己下的操作约束（本轮不改 phyt-DNA 引擎），不是上游停止演进。**

- `git -C phyt-DNA log --oneline d9a16a0..v2 -- tools/validate.sh` ⇒ **21 次提交**（Cellrix 副本正是 `d9a16a0` 那一版），逐条看全是**引擎语义/自述算术**的修法：`hard:report` 分母四类 · 门面门改名 · 豁免名单从注入点派生 · 变量必须赋值断言 · 账本回写 trap 擦行根因 · probe-all 算术闭合 24+1=25 …… 没有一条是"未冻结区"。
- ⇒ 369 行差异里**混着引擎语义变更**，所以「同步副本」不能当成"拿一次更新"，它等于把 21 次引擎修法一次性引进来 ⇒ 必须授权（P2 的 A 方案），且要先能逐条说清引进了什么。
- 一句话：**「冻结」= 我不动它；「没动」不等于「它没变」。** 18 天里它变了 21 次，而副本一次都没跟上。

## 3 · 判读与处置建议（不自裁）

| 项 | 建议 | 档位 |
| --- | --- | --- |
| P1 44 文件写 v1.0 | **不逐个改文档**（44 处手改必漂，且它们是七卷头注 = 声明面）。改为在 `ECOSYSTEM.md` 加一列「本仓声明的方法论线 + 它现在指向哪」，由 `branch-map.sh` 形状的生成器产出；头注文本本身是否改写，等园丁定（涉及 5 仓七卷头注 = 权威卷） | 需授权（动权威卷） |
| P2 Cellrix 副本 | 三选一：(A) 同步上游并重算指纹（一次性，代价=复核 369 行差异）(B) 把 `upstream-manifest.sh` 的成功文案改成它真做的事（自我一致性），并另建"与上游对齐"的检查（需上游 SHA）(C) 注销副本，改成运行时引用 `../phyt-DNA/tools`（零安装自足性丢失） | 需授权（改 CI 行为面） |
| P3 浮分支 | 一行：`ref: v2` → `ref: <sha>` | 自治档（可逆、不改行为面、有判据） |
| P4 无版本钉 | 让 `todo-ready.sh` 打印 `phyt-DNA HEAD` + 上游指纹校验结果（只报数、不判红） | 自治档 |

---

## 4 · 明确不做

- 不逐个改 44 个文件的方法论声明（那是 5 仓权威卷头注，且手改必漂）。
- 不给任何仓新建"版本钉"闸门（先有可见性，再谈门；否则是为造仪器而造）。
- 不动 Cellrix 副本（同步 = 行为面变更；先要园丁点头 A/B/C）。
- 不动 phyt-DNA 引擎（冻结中）。

---

## 5 · 这份普查什么时候会变红（判据，可复跑）

1. 出现**新的副本仓**：`find <仓> -name 'UPSTREAM.sha256' -o -name 'validate.sh'` 在 Cellrix 之外命中 ⇒ 本表 P2 需扩。
2. Cellrix 副本漂移行数**下降**却没在提交信息里具名（说明有人静默改了指纹）：`diff` 行数 369 → 变小且 `git log -1 --stat tools/UPSTREAM.sha256` 无对应说明 ⇒ 红。
3. 声明面 `v1` 文件数从 44 **上升** ⇒ 红（有人在新的卷头注里继续写归档线）。
4. helix-mind `todo-ready.sh:228` 那行**消失** ⇒ 唯一"真跑引擎"的调用点没了，本表 §1.3 归零 ⇒ 红。
5. v1 分支被重新启用（`git ls-remote --heads origin v1` 之外出现新提交）⇒ 本表 §1.2 「EOL」失效 ⇒ 红。

---

## 6 · 收据（命令 → 输出行）

```
git -C phyt-DNA ls-remote --heads --tags origin
  → refs/heads/v1  d41b7ed…   refs/heads/v2 97c2640…（无 tags）

grep -rIl 'phyt-DNA/tools' <仓>        # 引用形态
  → anaphase 0 · helix-mind 8 · Cellrix 0 · FlowModus 0 · Tuck 0 · helix-tentacle 0
    （helix-mind 的 8 个里有 1 个是真调用 tools/todo-ready.sh，其余 7 个是文档叙述）

find <仓> -name validate.sh -o -name xray.sh -o -name UPSTREAM.sha256
  → Cellrix: tools/validate.sh · tools/xray.sh · tools/UPSTREAM.sha256（其余 5 仓全 0）

git -C Cellrix hash-object tools/validate.sh → 212913b20239
git -C phyt-DNA log --all --find-object=212913b20239 -- tools/validate.sh
  → d9a16a0 …（最近一次出现在该提交）
git -C phyt-DNA rev-list --count d9a16a0..v2 -- tools/validate.sh → 21
diff <(git -C Cellrix show HEAD:tools/validate.sh) <(git -C phyt-DNA show v2:tools/validate.sh) | grep -c '^[<>]' → 369
  同法：xray.sh → 9d3d86f，之后 1 次提交，差异 28 行；upstream-manifest.sh → 与 v2 HEAD 同一 blob

grep -rIn 'phyt-DNA/template' <6 仓> → 全部 0

git ls-tree -r --name-only origin/v1 | wc -l → 21（顶层 7 项：.gitignore LICENSE NOTICE README.md README.zh-CN.md docs template）
git ls-tree -r --name-only v2 | wc -l → 187
comm -23 /tmp/g1 /tmp/g2 | wc -l → 15（全在 template/：13 正文 + 2 个 .gitkeep 占位）
comm -12 /tmp/g1 /tmp/g2 | wc -l → 6（.gitignore LICENSE NOTICE README* docs/PROTECTION.md）
comm -13 /tmp/g1 /tmp/g2 | wc -l → 181
  ※ 另用 git diff --diff-filter 复核得 13 D / 179 A，差 2 项是 git 的 rename 检测把两个 .gitkeep 折叠了
    ⇒ 本表以 comm（无 rename 猜测）为主口径

grep -rIl --exclude-dir=archive -E 'phyt-DNA[^)]{0,12}v1' <仓> | wc -l
  → anaphase 4 · helix-mind 4 · Cellrix 15 · Tuck 15 · helix-tentacle 6 · FlowModus 0（合计 44）
grep -rIl --exclude-dir=archive -E 'phyt-DNA[^)]{0,12}v2' <仓> | wc -l
  → anaphase 1 · helix-mind 3 · Cellrix 2 · Tuck 0 · helix-tentacle 0 · FlowModus 0（合计 6）

grep -rn 'phyt-DNA' <仓>/.github/workflows/*.yml
  → anaphase 2 行（checkout + 工作区自检）· 其余 5 仓 0 行
grep -n 'phyt-DNA' helix-mind/tools/todo-ready.sh → :228 bash "$ROOT/../phyt-DNA/tools/ci-matrix.sh"
grep -n 'upstream-manifest' Cellrix/.github/workflows/ci.yml → :45 run: bash tools/upstream-manifest.sh

sed -n '16,27p' Cellrix/tools/upstream-manifest.sh
  → FILES=$(ls tools/*.sh | grep -vE '^tools/(upstream-manifest)\.sh$') ; shasum $FILES > "$MAN"
    … shasum -c "$MAN" … printf 'OK: 引擎与上游一致\n'
```

---

## 7 · 具名局限

1. **「命中 ≠ 调用点」**：本表 §1.3 的「没跑」是按**执行调用点**判的，不是按 grep 命中判的（Cellrix 那 2 行命中都是注释）。反向的坑同样存在：`grep -c` 命中 0 只能说明「字面量不在那里」，不能说明「没在跑」（`bash ../phyt-DNA/tools/*.sh` 这类拼接式调用就会漏）。
2. **「提及 phyt-DNA」不等于「声明对齐」**：8/25/29/7/18/8 是按路径字面量统计的提及数，本表没有逐个判它是「叙述历史」还是「声明对齐」；只有那 44 个写「方法论 v1.0」的才算「当前主张」。
3. `v1` 的 **21** 个文件是用 `git ls-tree -r` 数的目录项，未展开到行；「v1 没有引擎」成立（`tools/` 不在 v1 树里），但 v1 的 `docs/` 与 v2 的 `docs/` 是否还能对应未做逐文件比对。
4. 44 这个数**排除了 `archive/`**（历史记录里的旧声明不算"当前主张"）；若把 archive 计入会更大。口径写在这里，改口径必须重跑命令。
5. 「在跑」只统计 CI 与 `tools/*.sh` 里的调用点，**不含人肉手工执行**（我本机就手工跑过 phyt-DNA 的 `ci-local.sh`）——人肉执行不可机械计数，本表不声称覆盖。
6. 副本溯源靠 `git log --find-object` 比对 **blob id**（不是 `shasum` 的十六进制）：若上游历史被改写（rebase/force-push 老提交），溯源会失效——phyt-DNA v2 目前无此类改写，但这是一条将来可红的假设。
