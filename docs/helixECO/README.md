# docs/helixECO —— 目录契约（**"新文件放哪"的唯一说明**）

> **2026-10-25 立。** 定这条契约的原因很具体：这里曾**平铺 94 个文件**（2026-10-09 一天的会话产物与权威卷混放，
> 谁该读哪个、哪些能删全靠记忆）⇒ 一次没有引用完整性检查的批量删除造成 11 条断链。
> **结论：目录本身要有契约，否则"整理"永远只能靠人重做一遍。**

## 一眼看懂（先读这张表再动手）

| 位置 | 放什么 | 判据（可复核） |
|---|---|---|
| **根（恰好 3 个）** | `ECOSYSTEM.md`（生态总图 · **入口**）· `DEBTS.md`（欠债账本）· `README.md`（本契约） | **根上不许再放别的** |
| `canon/` | **权威引用件**：他处/他仓按名引用，或代码 SSOT | 见下「消费者」表；改动属"动权威" |
| `register/` | **登记册**：`KNOWN_ISSUES.md` · `IP-LEDGER.md` | 只增不改；条目带 ID |
| `gen/` | **生成物**：`BRANCH-MAP.md` | 只由 `tools/branch-map.sh` 写；`--check` 过期即红；**禁手编** |
| `center/` | **中心命题与收敛计划**：`SESSION-DAG-IS-THE-CENTER.md` · `CONVERGENCE-M0-M7.md` | 改它 = 动**方向**，须人裁定 |
| `proposals/` | **提案与裁决**（`PROPOSAL-*` · `RULING-BRIEF-*`） | 带 `revisit-on`；**冻结条款必须写解冻条件** |
| `handoff/` | **交接与状态快照**（`HANDOFF.md` · `STATUS-<日期>.md`） | 只增不改；新快照另起文件 |
| `usage/` | **使用日志**（append-only）：`USAGE-candidate-tools.md` | 一行一条：`时间 · 命令 · 读数 · **行动变化**` |
| `observations/<YYYY-MM-DD>/` | **会话产物**（probe/verdict/dump/finding；**非权威**） | 按日期分目录；**不进根、不进 canon** |

## 命名（三条，够用）

1. **权威/生成物**：`UPPER-KEBAB.md`（`ECOSYSTEM.md` · `BRANCH-MAP.md` · `AITL-CONTRACT.md`）
2. **带日期的文档**：`UPPER-KEBAB-YYYY-MM-DD.md`（提案/裁决/状态快照）
3. **会话产物**：`lower-kebab-YYYY-MM-DD.(md|json)`，放在 `observations/<YYYY-MM-DD>/`
   · **禁**：中文文件名（历史遗留的两个已改名：`center/SESSION-DAG-IS-THE-CENTER.md` · `center/CONVERGENCE-M0-M7.md`）；`M0.5-dump` 这类**层级点号**名。

## ★ 消费者（**搬任何文件前，先看这张表**）

| 文件 | 谁按路径读/写它 |
|---|---|
| `register/KNOWN_ISSUES.md` · `usage/USAGE-candidate-tools.md` · `proposals/PROPOSAL-workitem-dag.md` · `canon/DIAGNOSIS-METHODS.md` | `tools/todo-ready.sh`（报告头与顾问模式） |
| `gen/BRANCH-MAP.md` | `tools/branch-map.sh`（**写**）+ `--check`（读） |
| `canon/ports.json` | `crates/helix-mind-core/src/config.rs`（端口 SSOT） |
| `canon/AITL-CONTRACT.md` | phyt-DNA `ADR-20261009-dangerous-action-shapes-must-be-blocked` 正文 |
| `center/*` | `ECOSYSTEM.md` §3（中心命题） |
| `DEBTS.md` | `tools/todo-ready.sh` + phyt-DNA 的 DEBTS 行（跨仓指路） |

⇒ **改路径 = 同一批改上表所有消费者**（工具内的路径字符串 + Rust 注释 + 他仓正文），否则就是断链。

## 归档与删除（**不靠记忆**）

- **历史全树**：tag `helixECO-before-cleanup-2026-10-25`（清理前 94 文件原样）⇒
  `git show helixECO-before-cleanup-2026-10-25:docs/helixECO/<名>` 可取回任何一份。
- **删文件前**：`git grep -F '<文件名>'` 全仓扫引用；引用 > 0 ⇒ **同批改指针**，否则别删。
  （待 phyt-DNA 的 `dangling-pointer-must-not-exist` 门落地后，这一步由门守；在那之前靠本条。）

## 变更记录

| 日期 | 变更 |
|---|---|
| 2026-10-25 | 立契约；16 个文件从根搬入 8 个子目录；两个中文名改为 UPPER-KEBAB；消费者 6 处同步；交叉引用 12 份文档改写；`branch-map` 生成物路径迁到 `gen/` |
