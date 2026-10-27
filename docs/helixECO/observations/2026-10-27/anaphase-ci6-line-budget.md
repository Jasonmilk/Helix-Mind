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

## 1. 超预算文件（8 个，全部 > 300 NOCL）

| 文件 | NCLOC | 目录（机械，非模块语义） | CI-6 豁免 | 判据现状 |
|---|---|---|---|---|
| `src/bind.rs` | 302 | `src` | jason / 2026-10-18 | 已过期（due 2026-10-18） |
| `src/hitl.rs` | 505 | `src` | — / — | **超预算且无任何豁免** |
| `src/rails.rs` | 370 | `src` | TBD / 2026-10-16 | 已过期（due 2026-10-16） |
| `src/pipeline/mod.rs` | 301 | `src/pipeline` | — / — | **超预算且无任何豁免** |
| `src/session_events/convergence.rs` | 489 | `src/session_events` | — / — | **超预算且无任何豁免** |
| `src/session_events/gc.rs` | 376 | `src/session_events` | — / — | **超预算且无任何豁免** |
| `src/session_events/query.rs` | 317 | `src/session_events` | — / — | **超预算且无任何豁免** |
| `src/adapters/mind.rs` | 395 | `src/adapters` | TBD / 2026-10-16 | 已过期（due 2026-10-16） |

⇒ **3 份豁免到期后不再豁免任何东西**；**5 份从来没有任何豁免**。
两个数相加 = 8，正好是检查器报的 over-budget 条数（这是交叉核对，不是巧合论）。

## 2. 同一次运行里另外三条读数（不在这 8 个文件里，但同属 CI-6）

| 项 | 读数 | 说明 |
|---|---|---|
| 文件棘轮 `grown` | `ci/allowances.py 382 -> 397`（含 +5 fix 额度）· 另 1 文件 | **与日期无关**的红：代码长过去了。它证明"CI-6 一直是红的"这件事**不依赖豁免到期** |
| 目录棘轮 `dir_grown` | `ci/ 1787 -> 1804` | 同上 |
| 即将到期（未过期） | `src/adapters/http_reasoning.rs` due **2026-10-30**（3 天后）· `src/contract/mod.rs` 11-13 · `src/bin/up.rs` 11-13 · `src/main.rs` 11-27 · `src/run_cycle/mod.rs` 12-11 | 这 5 条由 `tools/expiry-census.py` 在**到期前**报出（检查器只在到期后 FAIL） |

## 3. 交给园丁的三个选项（ADR-0042 已定"400 = 分层边界，总量不增"，此处只列处置形态）

| 选项 | 适用条件 | 代价 | 判据 |
|---|---|---|---|
| **A 拆** | 文件里存在**两个及以上**可独立陈述的职责 | 动结构 + 重跑三件（c1/c2/c3） | 拆后逐文件 ≤ 300，且 `dir_grown` 不增 |
| **B 调阈值/分层** | 职责**单一**，只是这类文件的自然尺寸大（如 `session_events/*` 是事件种类的枚举体） | 改 `ci/baseline.toml` 的 budget 或为该类文件立"tooling 层"式例外 | 例外必须写**为什么这类文件天然大**，不许只写"允许 500" |
| **C 重签豁免** | 只是还没到该拆的时候 | **最便宜也最危险**：给自己放假 | 必须带新 `due` + `owner`，且 `owner` 不许写 TBD |

**执行器的意见（不代替裁决）**：3 份过期豁免的 `owner` 是 `TBD`，说明**当初签的时候就没有人**。
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
