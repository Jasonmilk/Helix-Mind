# ⚠️ phyt-DNA 远端 v2 vs 本地目录：**本地是旧世代快照**（差点推错）

> 2026-10-09 ｜ 人类给了远端 `github.com/Jasonmilk/phyt-DNA`（分支 v2）+ 本机 SSH key ｜ **未推任何东西**

## 一、克隆成功，差距立刻现形

```
git clone --branch v2 git@github.com:Jasonmilk/phyt-DNA.git /tmp/phyt-dna-clone   → exit 0
远端 v2 最近提交：8538876 fix: .claude/ 移出根目录 → examples/claude-code/ …
```

| | **远端 v2**（真） | **本地 `phyt-DNA/`**（旧） |
|---|---|---|
| 方法论文件位置 | **根目录** `VISION.md` `DNA.md` `RNA.md` `SPEC.md` `PLAN.md` `GROWTH.md` `DEPRECATE.md` | `template/` 下 |
| `tools/` | `validate.sh` `check-baseline.sh` `claim-check.sh` `spec-lint.sh` **+ `baseline.sha256`** | 同样 4 个，**无基线** |
| `fixtures/` | **6 个闸门**（PLAN-must-exist · DNA-is-immutable · GROWTH-max-3 …） | **1 个** |
| `ledger/` | `README` · `hits-2026.jsonl` · **`appeals-2026.jsonl`** | 前两个 |
| `README` | 英文 + **`README.zh-CN.md`** + badges | 单份 |
| `docs/` | `PROTECTION.md` | `PROTECTION.md` |

**⇒ 结论：本地 `phyt-DNA/` 是一份**旧世代**快照（`template/` 子目录布局），不是 v2 的工作副本。**
**⇒ 幸好我按纪律先克隆比对 —— 直接 push 会把旧布局推上去。**（这是"提交推送 phyt-DNA"这条指令救回来的一次。）

## 二、但我的回流**不是白做**（对远端 v2 仍是新的）

```
grep -cE "DECISIONS_DIR|零闸门|probe-all|counter.sh" /tmp/phyt-dna-clone/tools/validate.sh  → 0
```

**⇒ 远端 v2 的 `validate.sh` 里**没有**：路径声明式（P2）· 零闸门具名（P3）· `--probe-all`（P11）·
反例夹具 `counter.sh`（P11）。**⇒ 我的回流对 v2 确实有内容。**

## 三、⚠️ 远端有一个我该遵守的既有机制

```
tools/baseline.sha256      ← 资产基线
tools/check-baseline.sh    ← 校验 decisions/ 与 tools/ 未被就地篡改
```

**⇒ 改完 `decisions/` 或 `tools/` 必须刷新基线**（否则 `check-baseline` 红）。
**⇒ 这正是我在 P4 里说"远端已有一半"的那一半 —— 它是**既有资产**，不是我要新增的。**

## 四、正确的做法（下一步，**尚未执行**）

在**真仓**（`/tmp/phyt-dna-clone`，分支 v2）上工作，把我的回流**移植**到正确位置：

| 我的改动（旧位置） | 应移植到（远端 v2） |
|---|---|
| `template/tools/validate.sh`（P2/P3/P5/P11） | **`tools/validate.sh`**（根） |
| `template/ledger/README.md`（P6/P7） | **`ledger/README.md`** |
| `template/fixtures/README.md`（P5/P11 counter） | **`fixtures/README.md`** |
| `template/decisions/README.md`（P1 双契约） | **`decisions/README.md`** |
| `template/VISION.md` 那一笔 | **`VISION.md`** |
| `docs/MULTIMETER.md`（新） | **`docs/MULTIMETER.md`** |
| `README.md` §The second axis | **`README.md` + `README.zh-CN.md`（两份都要）** |
| `docs/PROTECTION.md` §第二根轴 | **`docs/PROTECTION.md`**（远端版本不同，须**追加**而非覆盖） |

**然后**：`bash tools/check-baseline.sh --update` 刷新基线 → 提交 v2 分支 → push。

**⚠️ 待确认（我不擅动）**：
1. 直推 `v2` 还是**开分支 + PR**？（`v2` 看着是主线）
2. 本地那份旧 `phyt-DNA/` 是否应删除或改为指向远端的克隆？（避免下次又改到旧快照）
3. `README.zh-CN.md` 的中文措辞由我拟（可审）

**⇒ 本文件只做比对与计划，未改远端、未推任何东西。**
