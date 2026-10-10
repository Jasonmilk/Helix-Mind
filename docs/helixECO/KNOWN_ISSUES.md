# Helix 生态既有缺陷登记表（KNOWN_ISSUES）

> **性质**：跨仓缺陷的**唯一**登记处。**⚠️ 它同时是人类与 agent 的"笔记本"** ——
> **`为什么还没修` 那一列区分【待裁决】（要你）与【没人做】（要我）**，这就是两者的分工。
> **读它的地方**：`RNA.md` 的"必读"表与各仓入口（本会话教训：它一直存在，而**没人被指向它**）。不是待办清单，不是 ADR，不是 DEPRECATE。
> **为什么单独一份**：这些缺陷跨仓（涉及 anaphase / Cellrix / helix-mind），
> 塞进任一单仓都会变成"归属错误的文档"；放在工作区根又不在任何 git 仓里 —— **等于可丢失**。
> 故紧邻生态 SSOT（`helix-mind/docs/helixECO/ECOSYSTEM.md`）存放。
>
> **登记规则**：
> 1. 一条缺陷 = 一个**已被物理核对确认**的事实（附文件 + 行号 + 核对日期）。
> 2. 未核对的猜测**不登记**（宁可空着，不猜）。
> 3. 修复后**不删除**，转入 §2 已修（历史永不删除，只前进不回头）。
> 4. 每条必须写清「为什么它现在还没修」—— 是**待裁决**还是**没人做**，两者不同。
> 5. ★ **修好 ≠ 守住。转入 §2 已修时，必须同时注明【回归判据】**（哪条闸门 / 哪个测试 / 哪条命令守它）。
>    **没有回归判据的"已修"，是纸面富贵 —— 它会复发而无人知。**
>    （人类 2026-10-09：*"守住发现与修复的成果！否则这些坑和发现你会再走无数次。"*
>    这条规则本身就是那句话的机械化：**修复留下一条能红的判据，才算守住。**）

---


## 0. 审查跟进（**收件箱**：每轮收到审查 ⇒ **动手前先在此逐条登记**）

> **为什么有这一节**（人类 2026-10-10 reviewer 的判决，我认）：
> *"失效点不在执行，在收件：把关活在会话里，不落进清单。"*
> 实测：上一轮审查新增 **八件**，其中三件（K19-1 的 +1 点名 · redrate 样本量分层 · 静默家族索引）**我做了**，
> 但**没有一件进入清单** ⇒ 于是我据"我记得的那份"宣布"清单已清空" ⇒ **那句话不成立**。
> **⇒ 报告规矩（2026-10-10 加入）**：**报告 = 状态增量 + 收据；零增量不占轮次。**
> **类型首行**：报告的**第一行**必须就是这句（"报告（状态增量+收据）"），让人一眼知道这是增量报告而非复述。
> **工件生命周期**：**半页裁决之后 ⇒ 缩成一行 + 指针，不再重贴**（已裁决的工件不再占版面）。
> **修订不重写**：新信息**进旧清单** ⇒ **编号不动、状态动**（本表所有 R/K/D 均按此维护）。
> 来源：上一份报告逐字相同地发了两遍、零增量 —— 那不构成两轮。**同一条也约束本表自身**：
> 每轮必须给出**新的状态行**（哪怕只是把某件从 ⬜ 改成 ◐），否则不算引用。

> **⇒ 收件规则（本节的用法）**：**每轮收到审查后、动手之前，先把把关逐条登记（一行一件）**；
> **登记本身是每轮的第一个动作**。**落盘之后，"已清空"才有资格说。**

| ID | 把关条目 | 状态 | 证据 / 指针 |
|---|---|---|---|
| **R1** | **通道契约**：`stderr` = 协议通道（机器读）· `stdout` = 人眼通道 —— 进引擎设计规范 | ⬜ **未做** | 教训来源：announce 打 stderr 污染闸门违规文本 ⇒ CI 第 4 步假红（`red-lifecycle` ③ 收据） |
| **R2** | **绿的三种腐败形态**：① 没跑就说 ② 跑了读错 ③ 跑了不够 ⇒ 进 `green-is-a-sample` + **"绿必须带 run 收据"** | ⬜ **未做** | 与 `red-lifecycle` 三段同源 ⇒ 合并入卡（勿新开条目） |
| **R3** | **轻件清单落盘**（轻件的定义就是"清单不带着就走丢"） | ◐ **本节即落盘** | 本表 + 下方 R4-R8 |
| **R4** | **Cellrix 最小 CI + 六仓 CI 矩阵** | ⬜ **未做** | 现状：Cellrix **无 CI**（实测 `(no runs)`）；anaphase/phyt-DNA 有且绿 |
| **R5** | **`red-lifecycle` ③ 精化"红的保真"** | ⬜ **未做** | 与 R1 同源（假红 = 不保真） |
| **R6** | **K19-1 的 +1 判据点名** | ✅ **已做（未登记）** | `KNOWN_ISSUES` K19 行：+1 = `execution_failure_leaves_a_named_ledger_row`（**非 K19-1 专属**） |
| **R7** | **redrate 3/3 是存在性证明，不是红率** | ✅ **已做（未登记）** | `phyt-DNA/tools/redrate.sh` 头部"样本量分层" |
| **R8** | **静默家族索引**（四形态 + 判别式） | ✅ **已做（未登记）**；**新增第五形态**见 R9 | `DIAGNOSIS-METHODS.md#silent-family` |
| **R9** | **静默家族第五形态：定义层静默** —— 闸门存在、**判据不存在**（`check:` 缩进错 ⇒ 抽取为空 ⇒ 从未运行 ⇒ 绿照常输出） | ✅ **已入册（2026-10-10）** | `DIAGNOSIS-METHODS.md#silent-family` 第 ⑤ 形态 |
| **R11** | **机械化登记表**（三张卡 → 锚点链接式登记表）—— 上上轮提出，**至今未做** | ⬜ **未做** | 现状：`DIAGNOSIS-METHODS.md` 只有锚点，**没有从卡到落点的登记表** |
| **R10** | **"不是所有纪律都该成为闸门"** ⇒ 进引擎设计规范：强迫一切成闸门会**制造腐化**；探针能区分**闸门 vs 声明**是免疫系统的**分型能力** | ⏳ **待入册**（进 phyt-DNA 引擎设计规范） | 收据：`hard: false` 的声明类 ADR 配夹具 ⇒ 探针报"夹具无效／闸门已腐化" |


### §0-a · **每轮引用表（ID · 状态 · 收据）** —— 报告头部就贴这张，不贴即视为未引用

> **为什么单列一表**（reviewer 2026-10-10 第一刀）：*"落盘只是一半，**每轮引用才是另一半**。"*
> 实测：上一轮 §0 的四件（R2/R4/R5/R11）连状态行都没有，而 K23 的收据连续两轮缺席
> ⇒ **制度的敌人不是反对者，是下一个有趣的问题**（我跳了前四步去碰 K15）。

| ID | 状态 | 收据（可核） |
|---|---|---|
| `R1` | ✅ 已做 | `phyt-DNA/docs/PROTECTION.md`「引擎设计规范 · 通道契约」 |
| `R2` | ✅ 已做 | `DIAGNOSIS-METHODS.md#green-is-a-sample`「绿的三种腐败形态」+ **绿必须带 run 收据**（命令·计数·分母·模式） |
| `R3` | ✅ 已落盘 | 本节 + §0-a 引用表 |
| `R4` | ◐ **已交付（回本已发生）** | ✅ **Cellrix 最小 CI**（Rust 硬门 + 面板顾问读数；`.github/workflows/ci.yml`）· ✅ **六仓 CI 矩阵**（`phyt-DNA/tools/ci-matrix.sh`，首跑读数：**3 有 / 3 无**）· ★ **首次回本**：Cellrix CI 的首个 run **揭出 2 个潜伏红**（⇒ **K28**） |
| `R5` | ✅ 已做 | `DIAGNOSIS-METHODS.md#red-lifecycle` ③「红的保真」：假红/从不红/不可归因 ⇒ **红必须可归因** |
| `R6` | ✅ 已做 | K19 行：484 里 +1 = `execution_failure_leaves_a_named_ledger_row`（**K26 判据，非 K19-1 专属**） |
| `R7` | ✅ 已做 | `phyt-DNA/tools/redrate.sh` 头部「样本量分层」 |
| `R8` | ✅ 已做 | `DIAGNOSIS-METHODS.md#silent-family` |
| `R9` | ✅ 已入册 | 同上，**第 ⑤ 形态：定义层静默** |
| `R10` | ✅ 已入规范 | `phyt-DNA/docs/PROTECTION.md`「不是所有纪律都该成为闸门」 |
| `R11` | ✅ **本轮完成** | `DIAGNOSIS-METHODS.md`「落点登记表」——每张卡的落点 + **机械化程度** |
| `R12` | ⬜ **新（D5 揭出）** —— ★ **"485 · 0 红"自 2026-10-10 起【停牌】**：那是**本地读数**，而**真 CI 首红未读** ⇒ 在读到之前，不得以它作为"绿"的依据（**不是悄悄摘牌——悄悄摘牌是小静默**） | **六仓的真判据 CI 覆盖率 = 4/6**：`phyt-DNA ✅ · Tuck ✅ · Cellrix ★红(K28) · FlowModus ★红(K29)`；
**★ `anaphase` 与 `helix-mind` 的"绿"来自 `Graph Update`（依赖图更新，过期且非判据）⇒ 它们的【测试没有 CI】**。
判据 = `bash phyt-DNA/tools/ci-matrix.sh` 一屏里**没有 ⚠️非判据**的仓数 = 6/6 |
| `K23` | ✅ **自治结案** | 三问过尺（不改行为面/可逆/有判据）⇒ **不改名**；改为"Tuck 实施序列化时加一处映射 + 一条能红判据" |


### §0-d · **D 系列引用表**（审查输出的编号，与 R/K 同法：每轮逐条引用）

| ID | 状态 | 收据 |
|---|---|---|
| **D1** 三份半页内联呈报 | ✅ | 上一份报告正文（K28/K16/K11 各两选项 + 后果 + 判据） |
| **D2** 空读数 fail loud（**两半**） | ◐→✅ | **真空那半**：已打印「(无就绪项)」并说明它 ≠「全做完了」；**数据源缺失那半**：`[ -f ]` + `except OSError ⇒ exit 2 + stderr 报路径`（本轮复核确认在位） |
| **D3** `filter-before-sort` 入卡 + 红测 | ✅ | `DIAGNOSIS-METHODS.md#filter-before-sort`（含**第一体验者机制**作红测） |
| **D4** 三件落地（prio 推导 / `--done` 顾问 / 读数进报告头） | ✅ 大部 | `todo-ready.sh`；**未能全绿的一处**：报告头第二腿（`--ready`）需手动贴 ⇒ 见 R12 的"读数进报告"制度化 |
| **D5** 矩阵"过期收据"刀 | ✅ **本轮**（**且它揭出一处更大的**） | `ci-matrix.sh` 加两列（**⚠️过期** / **⚠️非判据**）；**实测**：**anaphase 与 helix-mind 的"绿"来自 `Graph Update`（过期且非判据）** ⇒ **它们的测试没有 CI** ⇒ 见 **R12** |
| **D6** D2 的另一半 | ✅ | 同上（`except OSError` 分支：`exit 2` + stderr 报路径） |
| **D7** R4 可结 | ✅ | R4 → **已结**（交付物 3/3：Cellrix/FlowModus/Tuck 的 CI 已建、读数可得）；**红是 K28/K29 的账**，不再由 R4 挂 ◐ |


| **D8** | **读数二连断**（我只贴矩阵，不贴读数与 D/R 表 ⇒ 每轮靠"我记得贴" ⇒ 一定漏） | ✅ **由 D9 结构性修复** | 诊断：**幸存模式即诊断——矩阵最有趣，读数最无聊**；代价当场可见：**"你定其一"让自治的 K15 也去等裁决**（指南针不在，工人就等人指方向） |
| **D9** | **加 `--report-head`：一条命令打印完整报告头骨架**（读数 + CI 矩阵 + D/R/K 桩）；**D 刀收到即入登记册，一行一条——刀活在机器里，不活在会话里** | ✅ **已落地并首跑** | `helix-mind/tools/todo-ready.sh --report-head`（读数 + 矩阵 + 三套编号的桩，全部**生成**而非手写） |
| **D10** | **首红的解释学**：红未读之前**没有故事**；新 CI 首红**第一嫌疑是 CI 自己**（棘轮种在当前值了吗？工具链对吗？）—— **别把 Cellrix 剧本提前套在 anaphase 头上**（拿叙事代替读数 = "凭想法写 pattern"长在解释层） | ✅ 姿势已立 | 见下方"轻件①"的停牌句与 K28/K29 的凭据标注 |


### §0-e · **光的分类（2026-10-10，实测读数）** —— 加冕 ≠ 废黜

> **背景**：reviewer 提 vision 机制时命我"**先读 + 分类**"：另外 7 份 `VISION.md` **不是引用光的叶子，是另外的灯**；
> **按引用图扫描恰好会漏掉它们**。⇒ 下面是一次**实测分类**（读到词形为止，不猜）。
> ⚠️ **本分类不修改任何 `VISION.md` 的内容**（reviewer 明令"**内容一个字不动**"）。

| 类 | 文档 | 它是什么 | 佐证（实测） |
|---|---|---|---|
| **方法之光（王）** | `phyt-DNA/VISION.md` | **朝向"怎么长"** —— 方法论的光 | 27 行 · 标题 **"VISION · 种子"** · 首句 **"让项目像植物一样生长，而不是像机器一样被建造"** |
| **目标之光（七盏子光）** | `helix-mind/docs/VISION.md` · `Cellrix/docs/VISION.md` · `anaphase-helix/docs/VISION.md` · `FlowModus/docs/VISION.md` · `Tuck/docs/VISION.md` · `helix-tentacle/docs/VISION.md` · `lodestone-md/docs/VISION.md` | **各自朝向"朝哪去"** —— 项目级 | **词形完全一致**："**某某 愿景索引**"（7/7）· 各有版本号（v1.0 ~ v3.1） |

**⇒ 结论**：生态里有**两类光**（**方法之光 × 目标之光**）—— 这是**分层，不是重复** ⇒
**"给王加冕"= 声明这个分层**（在**登记册**里声明），**而不是**去改七盏子光的内容；
**七灯若愿意引用方法论的光，加一行 `上位光@<哈希>` 即可——那是可选的"称臣"，不是强制的"废黜"。**

**⇒ 扫描范围（mechanized）**：`grep -rl 'phyt-DNA/VISION.md' <各仓>` ⇒
**实测只有 `phyt-DNA/**` 引用它**（RNA/PROTECTION/ADR/README）⇒ **光的变更只影响朝向它的叶子**（按需加载用在愿景上）。

| **D11** | **声明与收据必须可对账**：我写"**五仓全部落地**"而只有**四份收据** ⇒ 按自家纪律：**声明与收据不符，声明不成立** | ✅ **已修补** | **补 Tuck 的第五份收据**（`Tuck/fe1ba1a`，它也把失败尾部写进 job summary）⇒ "五仓"成立。**长期纪律**：**"全部/五仓/皆已"类声明逐项配收据**（与 D5"CI 对 HEAD"同族：这次绑的是**声明数字与收据数**）。★ 同刀应收两件：**自捕①的修复收据 = `helix-mind/ba34893`**；**SYNC 行的口径须写明**（"本轮有变的仓"） |
| **D12** | **红也需要故事**：矩阵五个红、四个有注，唯独 **Tuck（本轮唯一从 ✅ 翻 ★红 的仓）零解释** ——**翻转是信息量最大的事件，恰是注释盲区** | ✅ **已修（且真相更大）** | **直查 API ⇒ Tuck 双 run 皆 `success`** ⇒**那次翻转从未发生**（是我的矩阵把 in-progress 标成 ★红）⇒ 修：**未完成 ⇒ 显式「运行中」**；**长期：矩阵对状态翻转（绿↔红）显式标注** |
| **D13** | **生成器 fail loud（D2 移植）**：`--report-head` 内部任何段挂 ⇒ **整体非零 + stderr 报哪段**，**绝不允许半截报告头静默流出** | ✅ **已落地并过变异测试** | 判据：注入错（`PHYT_ISSUES` 指向不存在文件）⇒ **exit=2 + 具名"找不到登记册"** ⇒ **它尖叫了** |

| **D14** | **仪器也会造假 · 判读要有真值源 · 翻转要先验证**（reviewer 2026-10-10 第二次认账；标本：其四项嫌疑全建在**我的矩阵捏造的红**上） | ✅ **已入册并落地** | **三条**：① **压缩成绿/红的工具总会在某输入下造假**（in-progress / 取数失败）⇒ **三态是下限；取数失败不映射到任何态**（已修：`ci-matrix.sh`）② **源头优先**：任何翻转**先花一次调用验真**（本次：直查 API ⇒ Tuck 双 run 皆 `success`），**再**生成 differential —— **未验证数据上的鉴别 = 精致噪音**③ **信任根具名**：CI⇒API · 命令输出⇒本地重跑 · 方向⇒人 ⇒ **守卫链收敛于信任根 + 便宜抽查，不无限回退**（**谎报变贵、抓获变便宜**）。★ 由此补上第三句纪律：**绿要收据 · 红要故事 · 翻转要验证**。 |
| **D15** | **报告头是报告头**：本轮报告正文**没有报告头**（矩阵/光/倒计时/能做全不在，只活在豆包转录里）⇒ **生成器是好的（豆包转写了它的输出），缺的是【组装】**；而**审查类报告恰恰最该带**。且没有它，**连 USAGE 是否 +1 都无法对账** | ◐ **本轮补** | **长期：CI 矩阵是报告头【必出段】**（由 `--report-head` 生成） |
| **D16** | **本地读数只能带【停牌警示】出现；撤牌须显式具名** —— `485` 从 SYNC 行**整段消失**，未见撤牌声明（"悄悄摘牌是小静默"） | ◐ **本轮补** | **规则：撤牌是一次声明，不是一次省略** |
| **D20** | **红的故事与分类需抽样验证**（**视野差异原则**）：全链（Pi 分类 → 卡 → 豆包执行 → 判据）**共享 line-0 镜头**，唯一断点是**审计换镜头**（全文 grep + 读源码 + 开文件）⇒ **分类/验收/审计不得同视野**（**两个盲区相同的守卫 = 一个守卫**）：**① 红的故事是假设**（开一个文件 = 最小证伪）**② 分类驱卡前必须人眼抽样** **③ 卡模板加【异常标记】槽**（执行者对输入的意外特征**如实上报、不行动** —— L3 首战四条真异常、零顺手处理 ⇒ **从原则变机制**） |
| **D21** | **审查者门（即日）**：报告**缺生成报告头（含矩阵）⇒ 审查只审那一项** —— 矩阵**第三次消失且与审计轮重合**（D18 定律第三数据点）；**而 anaphase CI（K15 真判据）正坐盲区**⇒ **矩阵消失的那轮，恰是对最需要的东西盲飞的那轮** |
| **D22** | **修复审计双问**：不止验**判据**（测试过没），还验**意图**（设计要的吗）—— `failover_e2e.rs` 自己的注释就是**意图收据**，把"提交模板"从"可能绕过"变成"**确认是设计要的**"。**判据问行为 · 意图问方向；判据绿 + 意图错位 = 绿色的绕过（比藏红更隐蔽）**。收据源：测试注释 · ADR · docs。**审计经济**：**声明性主张亲验 · 执行性主张归 CI**。**交付协议显形**：执行者**本地 commit（不 push）** + 报告 ⇒ 资深侧审后推（写进卡模板） |
| **D23** | **写盘不得裸奔**（2026-10-10 自伤事故）：`open(p,"w")` **打开即截断**，异常一抛文件停 0 字节，而 `git add -A` 会把灾难提交（实测毁 3 笔才发现）⇒ **一律 `tools/safe_edit.py`**：**原子写**（临时文件 + `os.replace`）+ **写后自检**（最小字节 + 必备锚点 + 回读一致），自检不过**原封不动**。**判据**：让它拒写一次（锚点故意写错 ⇒ 文件不变） |

| **K30** | ★ **`docs/decisions/` 有 20/36 份【不是 ADR 形状】**（首行 `- **决策日期**…` 而非 `# ADR-…`），且全是**另一套命名**（`00NN-slug.md`：`0013`–`0020`、`0044`）⇒ **形式层的一名两物**（与 K23/D12-D13 同族）。**分类（Pi 已做，机械判）：名不像 ADR**；**修法 = 批量补头**（头从文件名机械派生）。**边界**：别顺手重编号（K4 的地盘）· 别把"移出"当默认。**判据**：`bash tools/check-adr-heads.sh` exit 0 · 36/36。★ **机械补头 = 豆包 L2 第一枪**（实验与还债同一动作） | `helix-mind/docs/decisions/` | ⏳ 分工：Pi 分类 ✅ → **豆包补头**（冻结：每文件**恰 +1 行** prepend）→ 脚本判 → Pi 审 diff |
| **K31** | ★ **外部定时变更：`ubuntu-latest` → Ubuntu 26 = 2026-10-19**（**9 天**）⇒ 今天绿的两仓可能纯因环境转红 | 六仓 workflows | ✅ **已处置（不只是登记）**：**六仓全钉 `ubuntu-24.04`**（anaphase 7249217 · helix-mind d795172 · Cellrix 880cf30 · FlowModus 18c7624 · Tuck 611f80a · phyt-DNA ec246b7）。**下一窗**：24.04 自身退役日 —— 同样主动预演 |
| **K32** | ★ **anaphase 的 CI 是【古红】—— 从第一次有 CI 起就没绿过**（**独立于 K15**）：run 系谱实测 **最近 10 个 run 全为 `failure`**（`773a5a1`/`732e1e9`/`775edd0`/`7249217`/`434ee57`/`4f58c21`/`c853eaf`/`6471ab3`…）⇒ **钉 24.04 前后都红 · K15-A 前后都红** ⇒ **K15 的改动没有引入它**。失败步骤固定为 **【① 测试（本仓判据）】**（② 行预算被 skip）；annotations 只给 `exit code 101`。**已排除四条**（各带实测）：① gitignore 工件 ② `Cargo.lock` 已跟踪 ③ toolchain（钉 1.98.1 后仍红）④ 装法/YAML。★ **豆包在 FlowModus 验过的范式**（移走本地 gitignore 文件）**在 anaphase 已否证**⇒ 剩下必须**读 run 日志**（网页 job summary 可读；API 的 `output.summary` 为 null）。**判据**：任一 run 转绿，或读到具名根因。**它就是最初的 Ⓑ 项，从 K15 摘出** | `anaphase-helix/.github/workflows/ci.yml` | ⏳ **待读日志**（不是"没做"，是**读不到**：logs 403 / artifact 401） |
| **K33** | ★ **共用工作树 ⇒ 并发必须串行化**（人类一问揪出）：实测豆包的 `2cc82f4` **就在 `feature/convergence-M0M7` 上、其父是我的 `18c7624`** ⇒ **同一个仓/分支/工作树** ⇒ **同一仓同时刻只能有一个 agent 动它**（否则测试跑在混合状态上；这次是侥幸，不是设计）。**做法**：交付协议显形 —— **执行者本地 commit（不 push）· 资深侧审后推** | 全部仓 | ✅ **纪律已立** |
## 1. 未修

| **K29** | ⚠️ **与 K28 同一凭据阻塞**（不是"没做"，是"**读不到**"）—— **FlowModus 的 CI 红，根因未确立**：R4 新建 CI 后首个 run 失败于 **Rust 硬门**（`cargo test --all-features`）—— 而**本地全 ok**。**已读事实**：`flowmodus-rs/Cargo.toml` 有 `prost-build`/`tonic-build` 且 `build.rs` 调 `tonic_build::configure()` ⇒ 构建期需 `protoc`；本地有 `protoc 36.2`，而 ubuntu runner 默认没有 ⇒ **已加 `apt-get install protobuf-compiler` 且该步成功**。**但 Rust 门仍红** ⇒ **假设是必要但不充分**。**读不到日志**（CI logs 需 admin 403；artifact 下载亦 401 ⇒ **需凭据**）⇒ 已把失败输出改为 **artifact 上传 + 末尾 120 行打印**（**让下一轮能读**）。 | `FlowModus/.github/workflows/ci.yml` | ⏳ **待查**（下一步：下一轮 run 后**读 artifact**，而不是继续猜；候选：apt 的 protoc **版本过旧** ⇒ 需装新版 protoc）。**判据**：FlowModus CI 转绿（`ci-matrix.sh` 一屏可见）。 | `R4`（CI 的第一次回本）· `K28`（同族：CI 揭出的潜伏问题） |


| **K28** | ★ **CI 揭出的潜伏破损（Cellrix）**：`cargo test --all-features` 有 **2 个红**，**本地与 CI 都可复现** —— `boot::tests::boot_output_is_byte_identical_to_the_legacy_mechanism`（"T1a 必须是纯重构，输出须逐字节相同"，**首个差异 @ 字节 163000**：新输出**内联了 `panel_tree`**（`<script>/* panel_tree — DAG NA…`），而期望仍是**占位符 `__PANEL_TREE__`**）· `boot::tests::graph_order_matches_the_legacy_sequence`。**★ 它"红着却没人知道"的原因正是：Cellrix 此前【没有 CI】（R4 之前实测 `(no runs)`）。** 相关提交 `76b8d25 feat(web): 第 0 步 —— 投影资产进清单/替换表/占位符` ⇒ **变更是有意的，像是"变更落了、它的逐字节测试没同步更"**。 | `Cellrix/web/src/boot.rs:312/50` | ⏳ **升级（需裁决）**：**"哪一边是对的"是行为面问题**（boot 输出确实变了）⇒ 按三级守则：**不擅自改测试**（会"祝福"一个可能错的输出）· **不擅改代码**。**两个选项**：① 变更是有意的 ⇒ **更新那两条测试的期望**（并在提交信息里具名"为什么逐字节契约作废"）② 输出确实漂了 ⇒ **修代码**。**回归判据**：修完后 `cargo test --all-features` **0 红**（Cellrix CI 的首个绿 run）。 | `R4`（CI 的第一次回本）· `K25`（同族：红着没人知道） ⇒ ✅ **收据**：修复在 **`Cellrix 21ab6d0`**（`[large] fix(boot): K28 …… 金样 + 两守卫`）⇒ "本地已修"**有家**。 ⇒ ✅ **收据**：修复在 **`Cellrix 21ab6d0`**（金样 + 两守卫），且实测 **`21ab6d0` 是 `880cf30` 的祖先（in）** ⇒ 修复**确实在 CI 跑的 HEAD 里**；**而 Cellrix CI 仍红 ⇒ 红是【另一个】**（本地 18 组 0 红）⇒ 须重开/另立。 |


> **闭合清单的第五个元素（2026-10-09 新增）：清单迁移本身就是闭合的一部分。**
> K25/K26 结案后条目仍留在「未修」里，正是缺了这一步 ⇒ 审计才"偶发抓谎"。
> **⇒ 从此：ADR 落笔 ⇒ 同笔或紧邻一笔更新本清单**（审计从"偶发抓谎"变"流程免疫"）。
> **迁移必须带收据**：条目 + 修于哪个 commit + 判据绿在哪条测试。**带收据的迁移是记账，不带收据是洗白。**

> **剩余"需定性"条的定性标准（三问）**：① **位置**（哪一行/哪个函数）② **判据**（怎么证明它已好/仍坏，**能红吗**）③ **进不进序列化**（若进 ⇒ 改它就是**改行为面/指纹**，需单独授权；纯内部 ⇒ 可跨仓同步提交）。
> ⇒ 三问齐 ⇒ 归入「待裁决」或「没人做」；三问不齐 ⇒ 先补测量，再分类。


> **债务审计（2026-10-09，工具产出）**：本表共 **17** 条，其中 **✅ 已修待迁移 2 条**（K25 · K26，见 §2）⇒ **真·未修 15 条**。
> **分类（按"为什么还没修"的第一列）**：**⏳ 待裁决（需人点头）= 2 条**（K19 一物两名 · K23 跨仓一物两名）· **没人做（agent 可自行推进）= 3 条**（K17 并发 · K20 编排两份 · K21 clippy 存量）· **其余 10 条**需按行读原文定性（K3/K4/K5/K11/K14/K15/K16/K18/K22/K24 —— 其中 K18/K22/K24 已具备回归判据，形态上是"已修待迁移"）。
> ⇒ **"做完了吗"由本清单回答，不由感觉回答。**
> **再审计（2026-10-10，逐条判读）**：✅ 已修待迁 **5** 条（K18/K22/K24/K25/K26）· ◐ 部分 **4** 条（K4/K16/K19/K20）·
> ⏳ 待裁决 **3** 条（K3 冻结/K11/K23）· ○ 没人做 **6** 条（K5/K14/K15/K17/K21 + K20 剩余）
> ⇒ **真·未修 ≈ 9 条（6 没人做 + 3 待裁决）**，其中 **K15/K14/K11 正挡在 DSH 对话界面与证轨的必经路上**（见 `STATUS-2026-10-10.md` §3）。


| # | 缺陷 | 位置（事实基线 2026-09-15） | 为什么还没修 | 关联 |
|---|---|---|---|---|
| **K3** | `run_cycle.rs:1052` 把 `reasoning_mode`（`"left_brain"` 模式标签）当 `model` 写进 `traces/reasoning.jsonl`，与 `assistant/reply` 的 physical model（ADR-0036）**不是同一事实** | anaphase-helix `src/run_cycle.rs:1052` | **用户明确要求"再议"**（2026-09-14），故冻结不动 | `anaphase:ADR-0036` 〔deps=— · prio=— · class=人类冻结〕 |
| **K4** | ADR 编号**跨仓撞号且不同义**：`anaphase:ADR-0016`（编排哲学）vs `Cellrix:ADR-0016`（证轨资产解耦）；`anaphase:ADR-0017`（CI-144 传输层）vs `Cellrix:ADR-0017`（资产语言） | 两仓 `docs/decisions/` | 编号是**生态共享序列**，不能回改。缓解办法已落地：**跨仓引用一律仓名限定**（见各 ADR 头部） | 本表 §3 ⇒ **同族新例（2026-10-09）**：**文档级撞名** —— `helix-mind/docs/helixECO/IP-LEDGER.md`（20 行·生态 IP 保护台账）与 `phyt-DNA/docs/PROTECTION.md`（16974 B·闸门设计规范）**是两个不同的东西共用一个名字** ⇒ 两份文件头部各写明分工（问题域不同，不是同一份）。 〔deps=— · prio=low · class=自治〕 |
| **K11** | `assistant/usage`（计量事件）被 `prove_track.data.js::derivePeriodUsage` 期待，但**不在 `anaphase:ADR-0026` D2 词表中** ⇒ 装配层（按词表校验）会**拒收**它，该原语恒 `null`。**前提修正（2026-09-15）**：原记录「真实事件流亦从未出现」**不成立** —— `assistant/usage` 自 2026-09-14 17:19 起就在真实流中（`run-66c96eca` / `run-ee6cbd83` / `run-fbb7890b` / `run-ddaf2e59` 均含，09-15 `run-8bba24c5` 亦有，seq 3，data `{chars,model}`）⇒ **是词表落后于实现**，不是实现缺失。**危险面坐实**：装配层对未知 kind 静默丢弃（见 K13），计量事件进装配路径即消失无痕 | `Cellrix/web/assets/prove_track.data.js:168` ／ `anaphase-helix/docs/decisions/ADR-0026-session-event-stream.md` §D2 ／ `Cellrix/web/assets/event_family.js` | 需裁决：① **词表补 metering 事件（协议扩展，推荐 —— 生产者已在发）**，或 ② 明确「计量不进事件流」并移除 Cellrix 的期待 | `K13` 〔deps=— · prio=**high** · class=**升级** · blocks=证轨〕 ⇒ ★★ **2026-10-10 重核（reviewer：'谁还按词表校验？'）**：**该前提在代码层已过时** —— `anaphase-helix/src/session_events/types_and_stream.rs:82` 有 **`EventType::Usage => "assistant/usage"`** ⇒ **它已经是事件词表的成员**（词表的载体是代码里的 `EventType` 枚举，不是 ADR 的散文）⇒ 故"不在词表中 ⇒ 被拒收"至少有一半不成立。**下一步（D 裁决后仍按 ① 执行）**：① 核对 `ADR-0026` D2 的**文本**是否落后于代码（若是 ⇒ 更新 ADR 文本，属文档面）② **端到端重核拒收者**：Cellrix 装配层（`web/assets/event_family.js`）/ `prove_track.data.js` 的实际判定 ⇒ **判据 = `derivePeriodUsage` 从 `null` 变有值**（可在面板读数或 `run_all.js` 的相关判据上看到）。 |
| **K5** | ECOSYSTEM.md 自述为生态 SSOT，但其内部存在**多份互相打架的组件清单**（目录树 / 项目状态总览 / 架构图 / 快速入口） | `helix-mind/docs/helixECO/ECOSYSTEM.md` v1.| **K15** | **会话身份缺失**：`job_id = derive(输入内容)` ⇒ **锚在「内容」上** ⇒ 内容每次提问都变 ⇒ **每次都是新文件** ⇒ **会话永远只有一轮**（一问一答，无连续会话）。注意区分：**确定性 ≠ 稳定** —— 确定性只保证「同一输入→同一 id」，稳定性要求「同一会话→同一 id」；锚在内容上则**确定性满足、稳定性为零** | `anaphase-helix/src/contract.rs`（`derive_job_id`） | **未修**。正解：**`session_id`（容器锚）与 `job_id`（内容锚）分离** —— 见 `ADR-0020`。**L0**（Cellrix 视图层沿 `resume_from` 合并成连续问答流）是**临时缓解，须标注待 L1 退役**；**L1**（anaphase 提供 `session_id`，同会话 append 而非新建文件）**跨仓提案** 〔deps=— · prio=**high** · class=自治 · blocks=DSH 对话界面〕 ⇒ ★★ **2026-10-10 先查纠正（原行把读者指向了错的 ADR）**：**真正的相关决策是 `ADR-0006`（会话即经历——Episode 边界，Proposed→Active 2026-09-05 用户批准）**；`ADR-0020` 是"事件轨迹持久化"，与此无关。**实测**：`contract::derive_episode_id` **已实现**（`contract/mod.rs:406` + 单测 `:561`）、**已在被使用**（`reflection.rs:296` 的 provenance `{episode_id}#{step}`）。⇒ 故 K15 的真问题**不是"要发明 session_id"**，而是：**容器锚（episode）已存在/已批准/已实现，但文件命名的键是 `period_id`（每提问一个）而不是 `episode_id`（每会话一个）**。★ 另：**"session" 与 "episode" 是同一概念的两个名字 ⇒ 归入 K19/K23 家族**（本仓内一物两名），不新开条目。**自治/升级分界（reviewer 2026-10-10 三）**：**判据 + 观察态 = ① 自治**（不改行为，可直接推、做完报）；**把文件命名键从 `period_id` 改为 `episode_id` = ② 升级**（**落盘命名模式是持久化接口 ⇒ 行为面**）⇒ 升级段须带**半页**：旧 `{period_id}.events.jsonl` 的**迁移策略** + **下游读取者清单**。〔deps=— · prio=**high** · class=自治（自治段）＋升级（落盘段） · blocks=DSH 对话界面〕 ⇒ ★★ **迁移案三补（2026-10-10，实测）**：**(a) gist 锚**：`gist_of` 写 `{period_id}.gist`（`convergence.rs:362`）⇒ **同一个锚问题 ⇒ 同一答案**。**(b) 回滚**：改法是**加法** ⇒ 回滚 = 停止写新形态 + 回退读取顺序 ⇒ **旧 `{period_id}.events.jsonl` 始终是权威副本 ⇒ 无数据丢失**。**(c) API（实测，且它阻止了一次破坏性变更）**：`main.rs:274` 接受 `id`/`job_id`/`period_id`，而 `:268-269` 自述「`id` 是诚实的名字；`job_id` 与 `period_id` 仍被接受，**以便不在一夜之间打断调用方**」⇒ **API 的 `id` 现在就是 period_id** ⇒ 若顺手改它的语义 = **破坏性跨仓变更**（Cellrix 读 `id`）⇒ **正确形态：ADDITIVE（新增 `episode_id` 字段），不动 `id`**。 ⇒ ★★ **2026-10-10 先查（第二次"不用突变"）**：**锚【已存在且已测】** —— `session_events/query.rs:132` 的 **`conversation_id`**（:421 由 roots 填充；`query_tests.rs:347/349` 判据："**根命名自己 · 续接继承根**"）⇒ **K15 的真缺口是"文件/视图仍以 `period_id` 为键"，不是"缺锚"**。**两条路已写成 ADR**（`anaphase:ADR-0053`，Proposed 待裁决）：**A 纯视图消费（推荐：不改持久化接口 · ADDITIVE · 无需迁移）** vs **B 改文件键（行为面）**。★ 判据：A ⇒ DSH 侧栏同一 `conversation_id` 的 N 个 period 显示**一行**；B ⇒ `k15_target_one_episode_lands_in_one_file` 取消 `#[ignore]` 后转绿。 ⇒ ★★ **2026-10-10 判据换尺**（K32 摘出后）：**K15 不靠"job 绿"**（那会与古红绑死），而靠 **"run 日志里 `k15_a_route_groups_periods_by_conversation` 通过"** ⇒ 本地已绿（486 用例 0 红）。**附件三 ✅ 已落**（`anaphase-helix` 本笔）：`k15_attachment3_chain_coverage_is_observed_not_forced` —— **观察态**（`≥ 34/103` 基线，记时点 2026-10-10）⇒ **不逼它全绿**（链残缺是数据的性质，写"必须 100%"会造出永远红且数据侧不可修的仪器）；**两侧实测**：绿 = 打印 `35/103`；红 = 拆一节链 ⇒ `8/103 < 34 ⇒ panic（并具名"去看 resume_from 的写出侧"）`。★ 判据当场抓出我自己的 fixture 算错（32 < 34 ⇒ 绿不了）——**它不当橡皮图章**。 |的 N 个 period 显示**一行**；B ⇒ `k15_target_one_episode_lands_in_one_file` 取消 `#[ignore]` 后转绿。 |
| **K14** | **`/api/sessions` 的 `limit` 会静默截断**：磁盘有 **91** 个 period，API 只返回 **50** ⇒ **41 个不进 DOM**。两个后果：① **跨边界的链其 `+N` 只是下界**（老链会被再次报低，形态与 F16 同）；② **对话累积后老记录持续掉出窗口** ⇒ 将来会以「又消失了」的形式复发 | `anaphase-helix/src/main.rs:169`（`unwrap_or(50)`）／`Cellrix/web/src/routes.rs:118`（`?limit=50`） | **未修。已核对（91 vs 50）**。留痕两处：跨边界链的 `+N` 标注为**下界**；考虑改为不受 limit 影响的算法或分页加载 〔deps=— · prio=**high** · class=自治 · blocks=DSH 对话界面〕 |

---
| **K16** | **「想 / 说 / 做」之间没有门**：`main.rs` **从不装配** security gate（grep `with_security_gate` 为空）；`config.toml` 无该配置；**Tuck 也没有 `security/gate` 端点**（grep 为空）⇒ 提示注入让 LLM「想」出 `rm -rf` 时，**「做」这一步没有门**。**契约与纪律其实都在**（`security.rs` 的 `SecurityGate`/`GateVerdict`、`adapters/security_gate.rs` 的 `HttpSecurityGate`、`pipeline/mod.rs:171` 的 I7「没装门 ≠ 门通过了」、`run_cycle/mod.rs:722`「拒绝是具名的行」）—— **缺的只是三处接线** | `anaphase-helix/src/main.rs`（无装配）／`anaphase-helix/config.toml`（无配置）／`Tuck/crates/`（无端点） | **进展 2026-10-09**：M0 ✅ · **M1 ✅**（Tuck `POST /v1/security/gate`，形状逐字对齐本仓契约；观察态；每次判定落一行审计，口径与 HTTP 响应同一套）· **M2 ✅**（anaphase 装配，`anaphase-helix 0a760f1`；地址由既有 `tuck_endpoint` 派生而非新增字段；**仍观察态**：Tuck 侧无准入表时回 `pass` 并**明说 `gate=none`**）· **M3/M4 ⏳ 待批准**。原待批准项 ①②③ **已全数落地**（③ 以派生替代新增配置项），见 `helix-mind/docs/helixECO/THINK-SAY-DO.md` 与 `D8-PLAN.md`。**⚠️ 门是 fail-closed ⇒ 装了它 "Tuck 挂了 = Helix 不思考"**（`run_cycle/mod.rs:671` 自己写着） | `I7` · `THINK-SAY-DO.md` 〔deps=授权（M3/M4 需人类一句话） · prio=**high** · class=**升级** · blocks=DSH 对话界面（安全收尾）〕 ⇒ ⏱ **观察态死线（2026-10-10）**：**两周 或 ≥50 条 `security_gate` 判定被记录**（先到者）⇒ 届时**带数据呈报**，在 ①（直接翻转）/ ③（带具名风险阈值翻转）里选。**读数来源可核**：Tuck `ledger/hits-2026.jsonl` 里 `source:"security_gate"` 的行数（或 anaphase 的 `gate` 事件行）⇒ **死线不是「以后」，是一个可数的量**。 |
| **K17** | ★ **实地样本（2026-10-09 K21 顺带发现）**：`Tuck/crates/tuck-core/src/file_store.rs:210` `self.credentials.lock().unwrap()` **held across an await** ⇒ 持 std Mutex 跨 await（阻塞执行器/可死锁）——**正是本条要治的那一类**。**并发三处坑**（异步/高并发/队列是人类的长期目标）：① **每请求都跑一遍 `build_agent`**（内含 `build_identity_block`：读文件 + 连 Tentacle 取工具清单，`main.rs:902`）⇒ 高并发下的延迟/资源放大器；② **`tokio::mpsc::unbounded_channel` ×2**（`main.rs:593/608`）⇒ **无背压**；③ **无 `semaphore`/`rate_limit`/`concurrency_limit`** ⇒ 无减速阀，且"打满"没人具名 | `anaphase-helix/src/main.rs` | **没人做**（不急）。**形状是对的**：每请求一份全新 `AgentLoop`（`main.rs:512/548/692`）⇒ 无共享可变 agent、无状态串扰，且 47 处 `lock().unwrap()` 的爆炸半径被限制在单请求。⇒ 要做的是给它**加背压 + 上限（具名）**，不是重设计 | `D11` 〔deps=— · prio=low · class=自治〕 |
| **K19** | **同一个名字指两个东西**：`verdict` 一族 5 处定义 —— 一物两名（`LedgerRecord::Verdict` 账本裁定 vs `PeriodVerdict` 周期结束），且 **同一 crate 内两个 `GateVerdict`**（`security.rs:58` 安全策略决定 vs `run_cycle/safety_gate.rs:52` 执行前检查结果，语义完全不同）⇒ 读者会误读 | `anaphase-helix/src/{security.rs,run_cycle/verdict.rs,run_cycle/safety_gate.rs,ledger/mod.rs}` | ⏳ **待裁决**（改名属重构）：建议把 `safety_gate` 那个改为 `ToolGateOutcome` 之类 | `D3` ⇒ **裁决简报已呈**：`RULING-BRIEF-K19-K23-2026-10-09.md`（四问齐：名字/出现面/波及面/建议 + 授权需求） ⇒ **K19-1 已修（2026-10-09，自动分流：三问全否 ⇒ 纯内部）**：`run_cycle/safety_gate.rs` 的 `GateVerdict` → **`ToolGateOutcome`**（判据：编译 0 error · 全套 **484 用例 0 红** · 跨仓引用仍 0。**收据点名**：484 里那个 +1 是 `execution_failure_leaves_a_named_ledger_row`（K26 的判据测试，**非 K19-1 专属**）⇒ 故 K19-1 自身的收据是"**编译绿 + 全套绿 + 旧名 grep 0**"；近名（`GateVerdict` 一族）以 `grep -w` 边界为准，不扩大。；三笔分账：定义+测试 / 调用点 / 本条清单同步）。**K19-2（公开的那个）建议不改**（生态契约名；线上词汇已由 `security_gate.rs:89-95` 字面量定死 ⇒ 改标识符无收益）。**K19-3（`LedgerRecord::Verdict`）进 JSONL（`record_type`）⇒ 动指纹 ⇒ 要动需单独授权，本轮不动。** 详见 `RULING-BRIEF-K19-K23-2026-10-09.md`。 〔deps=K19-3 需授权 · prio=low · class=升级（K19-3）〕 |
| **K20** | **编排有两份**：`pipeline::run()` 串六阶段，而**活路径**是 `run_cycle` + `Reflection` 自己调 `execute_calls`/`record_evidence`。⇒ 查"裁定为何没写"时读 `pipeline/mod.rs` 会**读错文件**（本会话实证：连猜四次全错，因为活路径根本不走它） | `anaphase-helix/src/pipeline/mod.rs` vs `src/run_cycle/{mod.rs,reflection.rs}` | **没人做**（大重构）。**低成本那一半可先做**：在 `pipeline::run()` 上**加一行注释指向活路径** | `D5` 〔deps=— · prio=low · class=自治〕 |

| **K21** | **Tuck 存量 clippy 警告**（2026-10-09 起：cargo fix/clippy --fix 清掉可机械修的部分 **27→22**，测试 12/12 仍绿；余 22 条为风格级 + 1 条并发隐患已转 K17 + 1 条假警告已具名 allow）。**`-D warnings` 不是本仓标准**（其 PLAN 写的是"clippy **新文件**零警告"）⇒ 整仓 `-D warnings` 会把既有的测试模块 lint 判成 error（假红） | `Tuck/crates/tuck-core/src/{config.rs,file_store.rs,audit_query.rs,credential.rs}` 等 | **没人做**（低优先）。**已落地处置**：`Tuck/tools/verify.sh` 的自证入口按**本仓标准**判：报告存量（具名）+ **只对真正的 `error` 判红** ⇒ 它仍然**能红**（不为装饰）。⚠️ 计数必须排除 cargo 的汇总行（"generated N warnings"），否则会报出不可归因的数（本步第一版就把 10 说成 40） | `Tuck/tools/verify.sh` 〔deps=— · prio=low · class=自治〕 |


| **K23** | ★ **跨仓"一物两名"**：anaphase 的 `GateVerdict::HitlRequired(String)` 与 Tuck 的 `DecisionConfig::NeedHumanConfirm` 是**同一件事的两个名字**（其余三档同名同义：Pass/Reject/HardOverride）。⇒ 接线时必须写映射表，而**最容易漏的那一档恰是"需要人工确认"= 最该守的一档** | `anaphase-helix/src/security.rs:63` · `Tuck/crates/tuck-core/src/policy.rs:78` | ⏳ **待裁决**（统一名字 = 改跨仓契约；或先只登记 + 在 M1 的映射处写明这一档）。**已登记在 `K16-DESIGN-GROUNDED.md` §八** | `K16` · `K19`（同族：本仓内的一物两名/一名两物） ⇒ **裁决简报已呈**：同上文件；★ 量测缩小了严重度 —— **线上词汇其实已一致**（anaphase 按 "hitl_required" 解析），真实风险是"将来 Tuck 序列化时必须写对且**没有测试守着**"，故建议**不改名**、改为"一处映射 + 一条能红的判据"，并入 Tuck 实施序列化那次改动 ⇒ ✅ **自治结案（2026-10-10，用三级守则过尺）：不改名**（三问：不改行为面 · 可逆 · 有判据）⇒ 改为"Tuck 实施序列化时加一处映射 + 一条能红判据" ⇒ **移出待裁决**。 〔deps=— · prio=— · class=✅ 自治结案〕 |




| # | 缺陷 | 位置 | 为什么还没修 | 关联 |
|---|---|---|---|---|
记录点选在知道 tool/index 的那一层且**不需要 skip 条件**（证据链：唯一构造点 `:228` + 必带前缀 + 控制流不可达）·
判据两条且**实测能红**（具名行可见 = 撤掉⇒FAILED；反双记不变量 = 闸门场景 blocked 恰 1、ExecutionFailed 恰 0）·
全套 35 二进制/483 用例 0 红 · 账 +45 行（功能代价）· 见 `anaphase:ADR-0052` | `K25`（同族：静音仪器）· `K19`（一名两物）· `ADR-0051` |


### 自我分类（2026-10-10 · **用刚落地的三级守则**给"待裁决"逐条过尺子）

> 判据三句：**① 改行为面吗？（对外可观察的语义/格式/接口）② 可逆吗？③ 有能红的判据吗？**
> 三句全"否/可逆/有" ⇒ **① 自治**；任一为"是/不可逆/无" ⇒ **② 升级**；破坏性且影响他人 ⇒ **③ 禁行**。

| 条目 | ① 改行为面？ | ② 可逆？ | ③ 有能红判据？ | **裁决** | 行动 |
|---|---|---|---|---|---|
| `K23`（跨仓一物两名） | **否**（两个 Rust 标识符不同，**线上词汇已一致** `"hitl_required"`） | 可逆 | 有（映射点 + 一条判据） | **① 自治** | ✅ **直接结案：不改名**（Tuck 15 文件零收益）；改为"在 Tuck 实施序列化时加一处映射 + 一条能红判据" ⇒ **从"待裁决"移出** |
| `K11`（计量事件入词表） | **是**（装配层从**拒收**变**接收** ⇒ 下游可见性改变 ⇒ 证轨的 `null` 变有值） | 可逆 | 有 | **② 升级（需一句话）** | ⏳ 保持待裁决 · **半页已足**（生产者已在发 ⇒ 补词表 vs 明确不进，二选一） |
| `K19-3`（`LedgerRecord::Verdict` 改名） | **是**（改名会动 JSONL 的 `record_type` 指纹） | **历史账本不可逆** | 有 | **② 升级** | ⏳ 保持待裁决 · **建议不动** |
| `K3`（`reasoning_mode` 当 model 写进 traces） | — | — | — | **人类冻结** | ⛔ **不动**（你 2026-09-14 明确"再议"）⇒ 本表**不替人类解冻** |

## 2. 已修

| **K27** | **Tuck `tools/verify.sh` 的 gateway 步骤"名不副实 + 静音"**：名叫「gateway crate 参与编译」，命令却是 `cargo build --workspace --all-features`（**承诺一件事、做另一件事**，且该命令**不证明**任何关于 gateway 的事）；并 `>/dev/null 2>&1` ⇒ **红不带原因**。实测它报过一次 ★ RED 而同一命令手跑 exit=0 ⇒ **不可归因的红**（比红更糟：会教人重跑直到绿）。附带查明：`gateway` feature 属于 `tuck` 包（`-p tuck-gateway --features gateway` ⇒ does not contain this feature）。 | `Tuck/tools/verify.sh:57` | **已修（2026-10-09）** 改为 `cargo build -p tuck --features gateway` + 失败打印末尾 12 行。**回归判据**：改回丢 stderr 的写法 ⇒ 一旦再红又变得不可归因；判据 = 连跑 3 次全绿且 ★ RED 数为 0。 | `K24`（同族：入口/构建的判据必须名实相符）· `K25`（同族：静音仪器） |

> **规则第 5 条**：每行必须注明【回归判据】——没有回归判据的"已修"是纸面富贵。

| **K18** | CI-144 边界上的 `vendored` 类型会漂（anaphase `src/ci144/` 钉在 Cellrix 的旧 commit 上） | `Cellrix/transport/tests/ci144_anaphase_live.rs` | **已修（2026-10-09）** · **收据**：修于 `Cellrix:ead8968`；并修掉它"**未测量 = 通过**"的那一环（缺 `ANAPHASE_BIN` 时由**静默 return ⇒ 报绿**改为**具名 panic `UNMEASURED: …`**）。**回归判据** = 该测试（真协议/真二进制/真往返）；**实测 `ok. 1 passed`**。 | `K16` · `ADR-0050` |
| **K22** | PreToolUse 执行闸对真危险动作 fail-open（写 `DNA.md` / `rm -rf` 均放行） | `phyt-DNA/examples/claude-code/hooks/` | **已修（2026-10-09）** · **收据**：修于 `phyt-DNA:27b115b`（hook 加 **L0.5**（`rm -rf` 家族 + `git push --force ` 带词边界）+ `ADR-20261009-dangerous-action-shapes-must-be-blocked`（`timing: pre`））。**回归判据** = `examples/claude-code/hooks/hook_test.sh`（**9 例，含 3 条 ★回归**）；**实测 9 passed**，并已接进 `ci-local.sh` 第 5a 步。 | `K25`（同族：闸门必须能红） |
| **K24** | 单 crate 构建失败而 workspace 通过（`cargo build -p tuck-gateway --all-features` ⇒ E0425；workspace ⇒ 0 error） | `Tuck/crates/tuck-audit/src/lib.rs` | **已修（2026-10-09）** · **收据**：修于 `Tuck:9868161`（`pub type Signer` 补 `#[cfg(feature = "anchor")]` —— 它与那处 `use std::sync::Arc` 同属该 feature）；**判据落在 `Tuck:fdeb5a7`**（把一个"每个 crate 各自 `--all-features` 可编译"的检查加进 `tools/verify.sh`）。**回归判据** = 该步（先造判据看它红 ⇒ 再修 ⇒ 现 **OK(4 crate)**）。 | `K27`（同族：入口判据必须名实相符） |


| # | 缺陷 | 修于 | 出处 |
|---|---|---|---|
| **F1** | `base.html` 第 1 行残留 `        r#"` → DOCTYPE 不在首位 → quirks mode + 页面顶部渲染字面量 | 2026-09-14 | `Cellrix:ADR-0016` D5 |
| **F2** | 证轨状态栏三格恒 `—`（`TOKENS` / 缓存命中 / `TOK/S`）—— 无数据源，非未实现 | 2026-09-14 | `anaphase:ADR-0038` | **根因（2026-09-15 查明）**：不是「无数据源」这么笼统 —— `derivePeriodUsage` 判的是 `e.type !== 'assistant/usage'`，而该类型**不在 `anaphase:ADR-0026` 词表中，真实事件流也从未出现**（扫 5 个 `.helix/events/*.jsonl`），故恒返回 `null`。见 K11。 |
| **F3** | 测试临时目录竞态：`mod tests` 与 `mod query_tests` 共用 `anaphase-session-events-test-3/4` → 全量偶败 | 2026-09-14 | ECOSYSTEM v1.94 |
| **F4** | cellrix 手套空串遮蔽：`cellrix_endpoint` 配置为空时仍被当成显式端点，导致兜底被跳过、Native 手套恒暗 | 2026-09-14 | 抽 `cellrix_probe_target()` 单一决策点 |
| **F5** | SSE 终局行缺 physical model；前端 sender 槽位不唯一 | 2026-09-14 | `anaphase:ADR-0036` |
| **F6** | 能量降级阈值硬编码：`negotiate_mode` 内 `system_load > 0.9` / `latency_limit_ms < 100` / `token_budget < 100` 三个字面量**零 config 来源**；且与 Anaphase 侧 `cfg.high_load` 构成**同一事实（系统负载）的第二决策点** —— `anaphase:ADR-0039` D4 只裁了 Anaphase 侧，这里是它的盲区 | 2026-09-15 | ①阈值移入 `RetrievalConfig`（`high_system_load` / `min_latency_limit_ms` / `min_token_budget`，serde 可覆盖，**默认值 = 原字面量，行为等价**）②判定抽为具名纯函数 `energy_degraded(energy, cfg)`，可脱离 engine 单测 ③测试 14 → 17（新增 3，零回归）；变异测试（0.9→0.5）三条全失败证明非空转 |
| **F7** | ADR 索引文档过期：`docs/decisions/README.md` 只列 0001（实际 **36** 份），且命名规范写 `NNNN-<kebab>.md`（实际 `ADR-NNNN-…`）；状态栏也只承认两态，实际另有 Accepted / Proposed | 2026-09-15 | 索引重建为 36 行（标题 / 状态 / 日期**由脚本从各 ADR 头部提取，不手抄**）；命名规范改为实际格式；新增「编号缺口」说明（`0014` 待查、`0031`–`0033` 属 helix-mind、跨仓引用仓名限定） |
| **F8** | FlowModus `GROWTH.md` 9 条远超自定 ≤3；归档目录 `docs/growth-archive/` **不存在** ⇒ 归档规则无法执行 | 2026-09-15 | 记录 0–5 归档到 `docs/archive/growth/2026-09-06-rs-refactor-r0-r5.md`（**对齐生态三个仓既有范式**，不是原头部的自造路径）；GROWTH 留 3 条；头部规则路径一并修正 |
| **F9** | FlowModus PLAN 里程碑 R-4/R-5/R-6 标 ⏳，而 GROWTH 记录 5/6/7 标 ✅ —— 同一事实两个状态 | 2026-09-15 | 以 GROWTH 为准（它有验收数字）改为 ✅ 76 / 83 / 83 passed |
| **F10** | helix-mind `GROWTH.md` 4 条，超 ≤3 一条 | 2026-09-15 | 归档日期最早一条（2026-09-06 P10 认知工艺与生态深度集成）到 `docs/archive/growth/`；留 3 条 |
| **F11** | `ADR-0014` 曾疑为「编号被占用而文件缺失」—— **实为跨仓引用漏了仓名**：anaphase 的 `ADR-0015` / `ADR-0017` 裸引用「ADR-0014（Web 面板 / 驾驶舱 G2）」，指的其实是 `Cellrix:ADR-0014`（`cellrix-web`，浏览器白盒窗口），该 ADR **一直存在且 Accepted**；anaph
| **F12** | **K1** `adapters/mod.rs:48` 注释过期（写「P10b fills value_grade; empty until then」） | 2026-09-15 | 注释改为指向 `helix-mind-api/src/layer3.rs:374` 的真实回填（`format!("{grade:?}")` → `Low`/`Medium`/`High`），并注明仅 Noop 降级时为空；anaphase `cargo test --no-fail-fast` = **260 passed / 0 failed / 9 ignored** 复核通过 |
| **F13** | **K12** Cellrix 证轨视图 e2e「4 条既有失败」（`31 passed / 4 failed`） | 2026-09-15 | **根因 = 运行未传 `<job_id>`**：无 job_id 时行点击块被整体跳过 → 后续 3 条全空。**但「已结案」曾是过度乐观** —— 当时的修法只是**改文案**（`check(false, usage…)`），断言**仍然全红**，无人值守跑依旧失败。**2026-09-15 真正修复**（`3ae7422`）：①无 job_id 时**自动问运行中的面板要最新 period**；②**确无可测数据时 SKIP** 而非 FAIL，与 pass 分开计数。**31/4 → 49 passed / 0 failed / 1 skipped**；原先失败的 4 条现在 **PASS**（证轨 drive 路径真的跑到了：`lane block opens the inspector [on=true]`、`turn toggle flips aria-expanded [true -> false]`） |
| **F14** | **K13** **D3 违约**：装配层对未知 kind **静默丢弃** —— `accept()` 遇 `!isValidEvent(e)` 直接 `return false`，**无任何诊断计数** ⇒ 全被拒的流与**空流无法区分** | 2026-09-15 | 按 `(reason, type)` **计数** + **有界样本**（`REJECT_SAMPLE_MAX`），经 `rejections()` 暴露；`accept()` 的两条拒绝路径（`invalid` / `duplicate`）都记账。**⚠️ 原建议「暴露于 `digest()`」有误，已证伪**：digest 是 §3.9/§3.10 的比较对象，而拒绝计数**依赖投递历史**（重放会重计重复）⇒ 进 digest 会让**同一盘录像产生两个 digest**，破坏它要守的不变量。**诊断不是不变量**，故留在 `rejections()`。验收网当场抓住了这个错（`replay of the same window is idempotent` FAIL）|
| **F15** | **`seq` 每个 turn 从 0 重开** ⇒ 装配层按**全局 seq** 去重 ⇒ **第 2 个 turn 起的事件全部被 duplicate 拒收** ⇒ 同一问题只显示第一轮。这是用户「第二问之后消失」的一部分 | 2026-09-16 | `gseq` 在**读取边界一次性赋值**（`period_normalize.js`），去重键改用它；`turn` 只作显示分组、**不进 node id**。实测 `run-7efbf0f8` **55 事件 / 5 轮 / 0 拒收 / 55 唯一节点**；真实周期切片已入回归网（`bef41b7`） |
| **F16** | **续接链的列表表达**（三者叠加 = 用户「记录找不到」的主因）：① 根卡片不显示续接数 ⇒ **有 6 条续接的根与没有的长得一样**；② `+N` 口径错（报**直接子**，线性链恒 1，10 节点链报 `+2`）；③ 渲染只取**直接子** ⇒ **孙节点根本不在 DOM** | 2026-09-16 | `walkChain()` **唯一遍历**（BFS + `seen`），**计数与渲染同源**（避免「两份推导」）；根卡片显示整链续接数（`+9`）。9 条断言含 **BFS 顺序**、**计数与遍历一致**、环终止、内联变异 |
| **F17** | **哨兵被指向错误的产物**：`verify_live.py`（烧入校验）文档里被指向 **冻结快照**，而冻结快照是 **JS 跑完后的 DOM** —— 应用自己设过的 inline style （如 `#eEmpty` 的 `display:none`）与资产不再逐字相同 ⇒ 读成「烧入坏了」= **两条永久假红**。永久假红会训练人忽略检查，比没有检查更坏。**同类**：`coupling_audit.py` 在冻结快照上报 `__SNAPSHOT__` DEAD（桩里的赋值它不认），喂原始页时 24 个 `window.*` 全部有定义 | 2026-09-16 | ① `verify_live.py` **拒收**含 `OFFLINE SNAPSHOT` 的输入并说明该喂哪个文件；② `snapshot.js` 同时写出 `<out>.raw.html`（服务端原页）；③ 补上它从未覆盖的 `prove_track.node.js`。**实测**：原始页 56 passed / 0 failed，冻结快照 54 passed / 2 failed ⇒ 差异全部来自产物，不是页面 |

> **F6 附注（设计事实，不是缺陷）**：系统负载的降级实为**两层** —— Anaphase 的 `load_gate`（作用在 `budget_tier`）与 Mind 的 energy guard（作用在 `CognitiveMode`）。**作用对象不同，不是同一事实的重复来源**；但两者阈值分属两仓 config，**没有对齐机制**。将来若有人改其中一处，需知另一处独立存在。

---

## 3. 已落地的缓解措施（不是修复，是止血）

| 缺陷 | 缓解 | 状态 |
|---|---|---|
| **K4 撞号** | 跨仓引用一律**仓名限定**（`anaphase:ADR-0016` / `Cellrix:ADR-0016`）。已写入 ADR-0018 / 0039 / 0040 / 0102 的头部与参考节 | ✅ 已落地 |
| **K3/K5** | 登记在案（本表），不再依赖记忆文件保存 | ✅ 已落地（本表） |

---

*登记 ≠ 认领。修不修、什么时候修，另议。*

## K-127（待查，本轮不修）· 工具计划泄漏成答复 —— 是否 v1.97 回归？

**登记日期**：2026-10-09 ｜ **状态**：待查 ｜ **来源**：gist 判据实验的副产物

**现象（实测，非推测）**：21 轮链的第 21 轮追问「那个词是什么？只回那个词」，
事件流里的 `turn/end.reply` 是 **`{"calls":[{"tool":"numbers","args":{"series":"[旺财]"}},"expect":"ok"]}`**
—— 即**工具计划被当成答复交付**（`converge_enabled=off` 那一格是 ```json 围栏包着的 `web_search` 计划）。
另一格 reply 直接是 `Jason`。

**为什么值得单列**：ADR-0034 / P0-D-1 家族（"never leak the raw JSON as a reply"）修过一次，
`Reasoning` 侧早有该守卫。**要查的是：这是 v1.97 的回归，还是那条守卫没覆盖到的新切点**
（例如工具计划在 finalize 前就被判为"已交付"）。

**为什么本轮不修**：超出当前目标（会话收敛）范围；且它会污染任何"看 reply 判对错"的实验，
所以必须**登记**而不是略过。

**对实验的影响**：本轮判据因此改为**从事件流 `turn/end.reply` 读**，
而不是看 HTTP 返回的截断值 —— 口径已写进 `gist-criterion-2026-10-09.json`。

---

## 3. 给后来人的两条行为纪律（**不是给产品的**，是给操作者/agent 的）

> 依据：人类 2026-10-09 *"守住发现与修复的成果！否则这些坑和发现你会再走无数次"* +
> *"一跳出编译器之外，就要自己谨慎测并做好记录，让下回的你更轻松"*。

**D-1 · 验证与推送必须分成两次调用。**
本会话**三次同形**：把"跑 ci-local + 提交 + 推送"放在同一次调用里 ⇒ 红被刷过去 ⇒
**三次都是"看到红还是推了 / 没复验就推"**。⇒ 规矩：**先跑自证看绿（一次调用），再推送（另一次调用）。**

**D-2 · `KNOWN_ISSUES §2 已修` 的每行必须带【回归判据】**（登记规则第 5 条）。
**没有回归判据的"已修"是纸面富贵** —— 它会复发而无人知。
**⇒ 且回归判据的形态要按判据的对象选**：路径型可用 `--probe`；**对象是命令/任意字符串的那种用不了 `--probe`**
（glob 模型是路径型的）⇒ 用直接测试（例：`hook_test.sh`），并用 `probe: none` + `probe-why` **具名声明**。
| **K15** | **会话身份缺失**：`job_id = derive(输入内容)` ⇒ **锚在「内容」上** ⇒ 内容每次提问都变 ⇒ **每次都是新文件** ⇒ **会话永远只有一轮**（一问一答，无连续会话）。注意区分：**确定性 ≠ 稳定** —— 确定性只保证「同一输入→同一 id」，稳定性要求「同一会话→同一 id」；锚在内容上则**确定性满足、稳定性为零** | `anaphase-helix/src/contract.rs`（`derive_job_id`） | **未修**。正解：**`session_id`（容器锚）与 `job_id`（内容锚）分离** —— 见 `ADR-0020`。**L0**（Cellrix 视图层沿 `resume_from` 合并成连续问答流）是**临时缓解，须标注待 L1 退役**；**L1**（anaphase 提供 `session_id`，同会话 append 而非新建文件）**跨仓提案** 〔deps=— · prio=**high** · class=自治 · blocks=DSH 对话界面〕 ⇒ ★★ **2026-10-10 先查纠正（原行把读者指向了错的 ADR）**：**真正的相关决策是 `ADR-0006`（会话即经历——Episode 边界，Proposed→Active 2026-09-05 用户批准）**；`ADR-0020` 是"事件轨迹持久化"，与此无关。**实测**：`contract::derive_episode_id` **已实现**（`contract/mod.rs:406` + 单测 `:561`）、**已在被使用**（`reflection.rs:296` 的 provenance `{episode_id}#{step}`）。⇒ 故 K15 的真问题**不是"要发明 session_id"**，而是：**容器锚（episode）已存在/已批准/已实现，但文件命名的键是 `period_id`（每提问一个）而不是 `episode_id`（每会话一个）**。★ 另：**"session" 与 "episode" 是同一概念的两个名字 ⇒ 归入 K19/K23 家族**（本仓内一物两名），不新开条目。**自治/升级分界（reviewer 2026-10-10 三）**：**判据 + 观察态 = ① 自治**（不改行为，可直接推、做完报）；**把文件命名键从 `period_id` 改为 `episode_id` = ② 升级**（**落盘命名模式是持久化接口 ⇒ 行为面**）⇒ 升级段须带**半页**：旧 `{period_id}.events.jsonl` 的**迁移策略** + **下游读取者清单**。〔deps=— · prio=**high** · class=自治（自治段）＋升级（落盘段） · blocks=DSH 对话界面〕 ⇒ ★★ **迁移案三补（2026-10-10，实测）**：**(a) gist 锚**：`gist_of` 写 `{period_id}.gist`（`convergence.rs:362`）⇒ **同一个锚问题 ⇒ 同一答案**。**(b) 回滚**：改法是**加法** ⇒ 回滚 = 停止写新形态 + 回退读取顺序 ⇒ **旧 `{period_id}.events.jsonl` 始终是权威副本 ⇒ 无数据丢失**。**(c) API（实测，且它阻止了一次破坏性变更）**：`main.rs:274` 接受 `id`/`job_id`/`period_id`，而 `:268-269` 自述「`id` 是诚实的名字；`job_id` 与 `period_id` 仍被接受，**以便不在一夜之间打断调用方**」⇒ **API 的 `id` 现在就是 period_id** ⇒ 若顺手改它的语义 = **破坏性跨仓变更**（Cellrix 读 `id`）⇒ **正确形态：ADDITIVE（新增 `episode_id` 字段），不动 `id`**。 ⇒ ★★ **2026-10-10 先查（第二次"不用突变"）**：**锚【已存在且已测】** —— `session_events/query.rs:132` 的 **`conversation_id`**（:421 由 roots 填充；`query_tests.rs:347/349` 判据："**根命名自己 · 续接继承根**"）⇒ **K15 的真缺口是"文件/视图仍以 `period_id` 为键"，不是"缺锚"**。**两条路已写成 ADR**（`anaphase:ADR-0053`，Proposed 待裁决）：**A 纯视图消费（推荐：不改持久化接口 · ADDITIVE · 无需迁移）** vs **B 改文件键（行为面）**。★ 判据：A ⇒ DSH 侧栏同一 `conversation_id` 的 N 个 period 显示**一行**；B ⇒ `k15_target_one_episode_lands_in_one_file` 取消 `#[ignore]` 后转绿。 ⇒ ★★ **2026-10-10 判据换尺**（K32 摘出后）：**K15 不靠"job 绿"**（那会与古红绑死），而靠 **"run 日志里 `k15_a_route_groups_periods_by_conversation` 通过"** ⇒ 本地已绿（486 用例 0 红）。**附件三**：`34/103` 链覆盖率进观察态判据（待落）。 |
