# anaphase CI-6 行预算 · 8 文件枚举表（交园丁裁决）

**日期**：2026-10-27 ｜ **性质**：只枚举，不裁决 ｜ **关联**：`register/KNOWN_ISSUES.md` 的 **K32-a** · `docs/decisions/ADR-0042-400-line-debt.md`
**为什么在这里**：本表服务的是"8 个文件该拆还是该调阈值"这个**语义判断**。语义判断归园丁，执行器只负责把判断的输入面压到最小——人就看着这 8 行做决定，不必自己去翻代码。

## 0. 这两张表由命令产出，不是手抄

```bash
cd anaphase-helix
python3 ci/check_line_budget.py            # → 8 个超预算文件 + 各自的 NCLOC
python3 ci/check_line_budget.py --json     # 只有 {"exit": 1}，无机器可读明细 ⇒ 明细只能取原文行
```
明细是从上面那次运行的**原文行**里取的（`        src/bind.rs  302` 形态），豁免信息取自
`anaphase-helix/ci/baseline.toml` 的 `[[waiver]]` 段（只取 `check = "CI-6"` 的 8 条）。

**具名局限**：本表的"目录"是**路径机械切分**，不是"模块职责"。真正的职责边界要读代码——
那正是需要园丁判断的部分，执行器不做。

## 1. 8 个文件 —— **口径：文书失效（豁免到期/无豁免），不是债务全集**

**本表叫什么名字，决定读者以为它覆盖什么。** 这张表的入选条件是「超 300 **且**豁免不生效」，
所以它漏掉了那些「超 400 但豁免还没到期」的更大文件：

```bash
python3 ci/check_line_budget.py          # → 8 file(s) over 300 NOCL with no waiver
```

| 口径 | 分母 | 读数 |
|---|---|---|
| 本表（文书式失效） | 超 300 且无有效豁免 | **8 个** |
| 债务全集（>400，ADR-0042 的分层边界） | `src/` 下非测试代码、NCLOC > 400 | **8 个 · over_total 1471**（见 §5.3） |

**两张名单的交集只有 2 个**（`hitl.rs` · `convergence.rs`）：

```bash
# >400 全集（实测）
main.rs 1035 · run_cycle/mod.rs 743 · bin/up.rs 545 · hitl.rs 505
contract/mod.rs 502 · convergence.rs 489 · http_reasoning.rs 449 · config.rs 403
# 本表 8 个
bind 302 · hitl 505 · rails 370 · pipeline/mod 301 · convergence 489 · gc 376 · query 317 · mind 395
# ⇒ 交集 = {hitl.rs, convergence.rs}
```
剩下的 6 个 >400 文件（`main.rs` 1035 · `run_cycle/mod.rs` 743 · `bin/up.rs` 545 ·
`contract/mod.rs` 502 · `http_reasoning.rs` 449 · `config.rs` 403）**全部有未到期豁免或走 DATA-ONLY 标记**
（`baseline.toml` 的 `[[waiver]]` due = 11-27 / 12-11 / 11-13 / 11-13 / 10-30，`config.rs` 走 `[[data_only_churn]]`）
⇒ **它们不在这张表里，但它们在债务里，而且其中两个（1035 / 743）是最大的。**

⇒ **按这 8 个裁 A/B/C，等于先处理文书问题，把最大的债继续往后推。** 这一点必须在裁决前讲明。

## 1.1 超预算文件（8 个，全部 > 300 NOCL）

| 文件 | NCLOC | 目录（机械，非模块语义） | CI-6 豁免 | 判据现状 | 在 >400 全集？ |
|---|---|---|---|---|
| `src/bind.rs` | 302 | `src` | jason / 2026-10-18 | 已过期（due 2026-10-18） || ✅ 是（+2） |
| `src/hitl.rs` | 505 | `src` | — / — | **超预算且无任何豁免** || ✅ 是（+105） |
| `src/rails.rs` | 370 | `src` | TBD / 2026-10-16 | 已过期（due 2026-10-16） || — 否（−30） |
| `src/pipeline/mod.rs` | 301 | `src/pipeline` | — / — | **超预算且无任何豁免** || — 否（−99） |
| `src/session_events/convergence.rs` | 489 | `src/session_events` | — / — | **超预算且无任何豁免** || ✅ 是（+89） |
| `src/session_events/gc.rs` | 376 | `src/session_events` | — / — | **超预算且无任何豁免** || — 否（−24） |
| `src/session_events/query.rs` | 317 | `src/session_events` | — / — | **超预算且无任何豁免** || — 否（−83） |
| `src/adapters/mind.rs` | 395 | `src/adapters` | TBD / 2026-10-16 | 已过期（due 2026-10-16） || — 否（−5） |

⇒ **3 份豁免到期后不再豁免任何东西**；**5 份从来没有任何豁免**。
两个数相加 = 8，正好是检查器报的 over-budget 条数（这是交叉核对，不是巧合论）。

## 2. 同一次运行里另外三条读数（不在这 8 个文件里，但同属 CI-6）

| 项 | 读数 | 说明 |
|---|---|---|
| 文件棘轮 `grown` | `ci/allowances.py 382 -> 397`（含 +5 fix 额度）· 另 1 文件 | **与日期无关**的红：代码长过去了。它证明"CI-6 一直是红的"这件事**不依赖豁免到期** |
| 目录棘轮 `dir_grown` | `ci/ 1787 -> 1804` | 同上 |
| 即将到期（未过期） | 见下表 5 行 | 由 `tools/expiry-census.py` 在**到期前**报出（检查器只在到期后 FAIL） |


## 2.1 5 个"即将到期"文件补 NCLOC —— 8 会变成 13（已实测，不是推算）

NCLOC 由检查器自己的计数器量得（`importlib` 载入 `ci/check_line_budget.py` 后调 `ncloc(text,"rust")`，
不手抄、不自算），四列并列：

| 文件 | due | NCLOC | 与 300 | 与 400 |
|---|---|---|---|---|
| `src/adapters/http_reasoning.rs` | 2026-10-30 | **449** | +149 | **+49** |
| `src/contract/mod.rs` | 2026-11-13 | **502** | +202 | **+102** |
| `src/bin/up.rs` | 2026-11-13 | **545** | +245 | **+145** |
| `src/main.rs` | 2026-11-27 | **1035** | +735 | **+635** |
| `src/run_cycle/mod.rs` | 2026-12-11 | **743** | +443 | **+343** |

⇒ **5 个全部已超 300**（没有一个是"到期了但其实不超预算"）。
⇒ 豁免一到期，它们同时进入 over-budget 名单：**8 → 13**。

## 2.2 交叉核对：我表里 8 个文件的 NCLOC 与检查器一致

| 文件 | 我表里 | 检查器 | 与 400 |
|---|---|---|---|
| `src/hitl.rs` | 505 | 505 | **+105** |
| `src/session_events/convergence.rs` | 489 | 489 | **+89** |
| `src/adapters/mind.rs` | 395 | 395 | −5 |
| `src/session_events/gc.rs` | 376 | 376 | −24 |
| `src/rails.rs` | 370 | 370 | −30 |
| `src/session_events/query.rs` | 317 | 317 | −83 |
| `src/bind.rs` | 302 | 302 | −98 |
| `src/pipeline/mod.rs` | 301 | 301 | −99 |

⇒ 逐字一致（这两列是同一计数器，同为机械读数）。

## 3. 交给园丁的三个选项（ADR-0042 已定"400 = 分层边界，总量不增"，此处只列处置形态）

| 选项 | 适用条件 | 代价 | 判据 |
|---|---|---|---|
| **A 拆** | 文件里存在**两个及以上**可独立陈述的职责 | 动结构 + 重跑三件（c1/c2/c3） | 拆后逐文件 ≤ 300，且 `dir_grown` 不增 |
| **B 调阈值/分层** | 职责**单一**，只是这类文件的自然尺寸大 | 改 `ci/baseline.toml` 的 budget 或立例外 | 例外必须写**为什么这类文件天然大**，不许只写"允许 500"。**★ 对 `hitl.rs`(505) / `convergence.rs`(489) 不适用**——见 §5：它们越过的是 ADR-0042 的 400 **分层边界**，不是 300 预算 |
| **C 重签豁免** | 只是还没到该拆的时候 | **最便宜也最危险**：给自己放假 | 必须带新 `due` + `owner`，且 `owner` 不许写 TBD |

**执行器的意见（不代替裁决）**：3 份过期豁免里 **2 份 `owner` 是 `TBD`**（`rails.rs` / `mind.rs`），第 3 份是 `jason`（`bind.rs`）——即**多数当初签的时候就没有人**。
⇒ 无论选 A/B/C，**先把 owner 落到人**；否则下一轮到期还是无人复核，这只是把同一个日期往后推。

## 4. 这件事与环读数的关系（已实测，不是推理）

```bash
cargo test --all-features --no-fail-fast        # 失败清单 2 条（含 the_line_budget_check_runs_and_passes）
cargo test --test upstream_live --all-features  # rc=0，1 passed
# 两份失败清单交集 = 0 行
```
⇒ **环判据与 CI-6 无关**，7 环 READY 不是"红底上的读数"。
`the_line_budget_check_runs_and_passes` 是 CI-6 的**第二条读数**（在 `cargo test` 里再判一次），
所以 CI 的 ① 测试步骤红、② 行预算步骤被 skip——两处红同一根因。

## 5. "400" 的性质：是**分层边界**，但**无人看守**（两条实测）

### 5.1 原文（ADR-0042 自答，不需要我替它拍）

- `docs/decisions/ADR-0042-400-line-debt.md:1` 标题即「**400 行红线**债务清偿」
- `:512`「D17：行数不是复杂度的代理 —— 采用三层信号，**400 保留为分层边界**（2026-09-20，**实测后写入**）」
- `:545`「**400 行回答的是导航问题**（我能不能找到东西）；CC 回答的是理解问题」
- `:560`「⇒ 400 行是「**发现与结构化**」机制，不是「复杂度」机制」
- `:563`「D17.4 **为什么驳回「把 400 降级为 warning」**」
- `:492`（D16）「⇒ 门禁改用 **`over_total` = Σ max(0, NCLOC − 400)，只减不增**」

⇒ 性质 = **(a) 分层边界**。B（调阈值）**不能**用在 `hitl.rs`(505, +105) 与
`convergence.rs`(489, +89) 上：调 300 那一档改的是预算，不是边界；要动边界必须 supersede ADR-0042/DNA。

### 5.2 但 D16 说要换成的 `over_total` 闸门，**在代码里不存在**

```bash
grep -rn 'over_total' --include=*.rs --include=*.py --include=*.toml . | grep -v '^./docs/'
#   ci/baseline.toml:403  （注释）
#   ci/baseline.toml:405  （注释）
#   ⇒ 实现 0 处
```

实测当前值（用检查器自己的 `iter_sources` + `is_test_path` + `ncloc`，排除测试代码）：

```
>400 文件数 = 9 · over_total = 1775
  src/main.rs                1035 (+635)
  src/run_cycle/mod.rs        743 (+343)
  ci/check_line_budget.py     704 (+304)
  src/bin/up.rs               545 (+145)
  src/hitl.rs                 505 (+105)
  src/contract/mod.rs         502 (+102)
  src/session_events/convergence.rs 489 (+89)
  src/adapters/http_reasoning.rs    449 (+49)
  src/config.rs               403 (+3)
```
仅 `src/` 口径（去掉 `ci/` 那 304）：**over_total = 1471**。

ADR-0042 记录的读数是「`over_total` 已由 **1365** 降至 **1090**（门禁口径，5 个文件）」。
⇒ 若按同口径比较，现在是 **1471（src 仅）/ 1775（含 ci）**，**高于 1090**。
**口径差异必须如实标**：ADR 写那句时 `ci/` 尚未纳入预算（`iter_sources` 的 docstring 说
checkers 是后来才计入的），所以能直接比的是 **1471 vs 1090**。

**⇒ 结论**：`只减不增` 这个不变式**没有被执行**，因此它**已经在涨**，而且涨的时候没有任何东西报出来。
这不是"豁免到期才会红"的问题——**这条根子从来没红过，因为它没有实现**。

**对裁决的意义**：那 2 个 >400 的文件（`hitl.rs` / `convergence.rs`）不是"预算超 5 行要不要放宽"，
而是"**一条明确写下的不变式，实际无人执守，且已反向走了 +381**"。选择 B 在语义上等于
承认"这条边界不自守，可以继续涨"——那是改 ADR，不是调一个数字。

### 5.3 `+381` 现在是**实测**，不是推断（口径已对齐）

在 ADR 写入 1090 的那个 commit（`8c99738`，`docs(adr): D17 …`）上，用**同一份**检查器计数器重算：

```bash
git archive 8c99738 | tar -x -C /tmp/ana8c     # 不改本仓，不建 worktree
cd /tmp/ana8c && python3 -c "…iter_sources/is_test_path/ncloc…"
```
```
8c99738 上：>400 文件数=6 · over_total=1337 ｜ 仅 src/: 文件数=5 · over_total=1090
    src/main.rs           941 (+541) · src/run_cycle/mod.rs 653 (+253)
    src/bin/up.rs         545 (+145) · src/contract/mod.rs 502 (+102)
    src/adapters/http_reasoning.rs 449 (+49) · ci/check_line_budget.py 647 (+247)
```

⇒ **`仅 src/` 口径 = 1090、5 个文件** —— 与 ADR 那句「已由 1365 降至 **1090**（门禁口径，**5 个文件**）」
**逐字吻合**。⇒ 口径**已验证可比起**（ADR 当时 `ci/` 已纳入 `iter_sources`，但它 647 > 400，若含它则是 1337，不是 1090 ⇒ 1090 就是 src 口径）。

**逐项分解 +381**（同一计数器，同一口径）：

| 项 | 变化 | 增量 |
|---|---|---|
| `src/main.rs` | 941 → 1035 | **+94** |
| `src/run_cycle/mod.rs` | 653 → 743 | **+90** |
| `src/hitl.rs` | ≤400 → 505 | **+105**（新越界） |
| `src/session_events/convergence.rs` | ≤400 → 489 | **+89**（新越界） |
| `src/config.rs` | ≤400 → 403 | **+3**（新越界） |
| `bin/up.rs` 545 · `contract/mod.rs` 502 · `http_reasoning.rs` 449 | 未变 | 0 |
| **合计** | 1090 → **1471** | **+381** |

⇒ **老文件涨了 184；两个新越界文件（`hitl.rs` / `convergence.rs`）贡献了 194。**
⇒ 也就是说：**ADR 定下"400 是分层边界"之后，又有 3 个文件越过了它，而没有任何东西当时叫过一声。**
这正是 K-129 的实证形态——不是"到期才红"，是"这道边界从来没有门"。

**一处口径备注（如实标）**：`src/config.rs` 在 300 预算里走的是 `[[data_only_churn]]`（DATA-ONLY 标记，不参与 300 判决）；
ADR 的 400 口径只用 `iter_sources` + `is_test_path`，不排除 data-only ⇒ 本表按 ADR 口径把它算进去（+3）。
若按 300 预算的口径排除它，则是 **1468**。两个数都写出来，避免口径再次分叉。

**另一处 ADR 内部不一致（顺带发现，未改 ADR）**：同一份 ADR 里 `over_total` 有三个值——
`:496` 表的「1365 → **1158**（−207）」、`:566` 的「1365 → **1090**」、`:578` 的「此前记 1082，本轮实测 **1090**」。
`8c99738` 上实测 = 1090 ⇒ 与 `:566`/`:578` 一致，`:496` 的 1158 是**另一次拆分后的中间读数**（口径/时点不同）。
⇒ 记在这里，供园丁裁决时知道"1090 是哪个数"，不擅自改 ADR。
