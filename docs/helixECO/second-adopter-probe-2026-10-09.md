# 第二个采用者：在**全新目录**上从零走一遍（通用性验证）—— 结果抓出我三个 bug

> 2026-10-09 ｜ 人类："直接跑过去提交通用版，实现复用性" + "别让文档落后" ｜ **已跑通**

## 一、做法（真正的"第二个采用者"，不是重跑 Cellrix）

```bash
cp -r phyt-DNA/template/. /tmp/phyt-adopt-probe/     # 全新目录，只用模板自带的东西
cd /tmp/phyt-adopt-probe
DECISIONS_DIR=decisions FIXTURES_DIR=fixtures LEDGER_DIR=ledger bash tools/validate.sh --probe-all
```

**⇒ 关键是"全新"**：它没有 Cellrix 的任何历史（不同的 ADR 路径、不同的闸门、没有我后来加的东西）。

## 二、★★★ 结果：抓出**3 个**我回流里引入的 bug

| # | bug | 为何 Cellrix 里不现形 |
|---|---|---|
| 1 | **P2 路径改造漏了一处 `docs/decisions/`** ⇒ `check_of` 读错文件 ⇒ 报 **"闸门已腐化"（假红）** | Cellrix 的 ADR 就在 `docs/decisions/`，**恰好蒙对** |
| 2 | **`decisions/README.md` 被当成一个闸门**（我在 P1 文档里放的 YAML 示例含 `hard: true`） | Cellrix 的 README 没有那种示例 |
| 3 | **`$out（`** —— 变量名后紧跟**全角括号**，bash 把它吞进变量名 ⇒ **`unbound variable`** | Cellrix 的对应提示语恰好没踩 |

**⇒ 已全部修**：① 路径改完；② **闸门只认 `ADR-*.md`**（不是"文档里出现 `hard: true` 就算"）；
③ **`$var` 后紧跟非 ASCII 一律写 `${var}`**（机械扫了一遍，2 处）。

## 三、修好后的读数（模板开箱可用）

```
心跳 RED — 检出: PLAN.md 内容过少或已被清空（它是 must-read）
[具名] 无反例夹具 fixtures/ADR-20261009-PLAN-must-exist/counter.sh —— 该闸门只证了'该红的红'，没证'不该红的不红'
probe-all: 跑了 1 个闸门
exit=0
ledger: {"gate_id":"ADR-20261009-PLAN-must-exist","verdict":"pass","kind":"probe", …}
```

**⇒ 四件事同时成立**：闸门**真的红**（理由正确）· 缺反例**被具名**（P11 的机制）·
探针**写账本**（闭环）· `exit=0`（探针成功 = 闸门活着）。
**⇒ 模板对第二个采用者**开箱可用** —— 这是"通用 + 复用"的第一份实证。**

## 四、新增第 32 条（**中文代码库的真陷阱**，机械可查）

**第三十二条 · `$var` 后紧跟 CJK/全角字符，必须写 `${var}`。**
`"$out（…）"` 会被 bash 读成变量 `out（` ⇒ **`unbound variable`**（而报错信息里那个乱码
恰好把原因藏起来）。**判据**：扫 `\$[A-Za-z_][A-Za-z0-9_]*(?=[^\x00-\x7F])` ⇒ 命中即须加花括号。
（对以中文写注释/提示的项目，这是**零成本、可机械检查**的一条。）

## 五、人类要求"提交推送 phyt-DNA" —— **做不到，原因如下（需你定）**

```
ls -d phyt-DNA/.git           → 无
git -C phyt-DNA status        → fatal: not a git repository
git check-ignore -v phyt-DNA  → （不被忽略，只是从未入库）
```

**⇒ phyt-DNA 是一个**纯目录**，没有版本库 ⇒ 我无法 commit/push。** 三个选项（你选）：

| | 选项 | 代价 |
|---|---|---|
| **a** | `git init phyt-DNA` + 首次提交（**需要一个远端 URL** 才能 push） | 需要一个仓库地址 |
| **b** | 把 `phyt-DNA/` 移进某个已有仓（或作为 submodule） | 改目录结构 |
| **c** | **保持现状**：内容以 `Cellrix/docs/BACKFLOW-phyt-DNA-v2.md` 为准、时点以 Cellrix 的 commit 为准 | **无 git 历史**，但**内容与追溯都有**（本册已写明） |

**⇒ 我倾向 c（现状可用）或 a（若你有远端）。** 但**我不会擅自 `git init`** —— 那会改变你的目录性质。
