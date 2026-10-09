# HANDOFF · 入口（人或 agent 读的第一份）

> **一句话入口**：*"读 `helix-mind/docs/helixECO/HANDOFF.md`"* —— 读完就能进入工作状态。
> **为什么有这个文件**：人类说"我也需要笔记本，否则我会忘记"；agent 说"开工前不查就会重复发现已有物"。
> **两者是同一件事**：**一个被指向的入口**。它不是 PLAN（阶段计划）、不是 GROWTH（年轮）、
> 不是 KNOWN_ISSUES（缺陷册）、不是 decisions（规则）—— 它只回答五件事，**每次会话结束前由 agent 更新**。
>
> **⚠️ 它自己也会腐**：所以「下一件」与「等你」必须与 `KNOWN_ISSUES.md` 的 **待裁决** 行一致（那是唯一真源）。

---

## ① 现在在哪

| 仓 | 分支 | HEAD |
|---|---|---|
| anaphase-helix | `feature/convergence-M0M7` | `e26f24c` |
| helix-mind | 同上 | `b2b25b7` |
| Cellrix | 同上 | `b0df057` |
| FlowModus | 同上 | `6600ccd` |
| Tuck | 同上 | `812c966` |
| phyt-DNA | `v2` | （见 `git -C phyt-DNA rev-parse --short HEAD`） |

**生态 SSOT**：`ECOSYSTEM.md` · `ports.json` · **`KNOWN_ISSUES.md`（缺陷与裁决）** · 本文件。

## ② 等你（**这一节就是你的待办**）

| id | 要你决定什么 | 在哪说明 |
|---|---|---|
| **K16** | **给「做」装门**（想/说/做）：Tuck 实现 `security/gate` + anaphase 装配 + 配置。**三项都改行为**，已设计分阶段 M0–M4 | `KNOWN_ISSUES.md` K16 · `THINK-SAY-DO.md` · `D8-PLAN.md` |
| **K18** | CI-144 的 `vendored` 类型要不要加一致性判据（**通用向，先标记**） | `KNOWN_ISSUES.md` K18 |
| **K19** | `verdict` 一族改名（一物两名 + 同 crate 两个 `GateVerdict`） | `KNOWN_ISSUES.md` K19 |
| rail | rail 匹配门槛是否收紧（单字命中即得分）· rail 命中是否该 `calls.clear()` | `rail-bypass-findings-2026-10-09.md` |

## ③ 已有什么（别重复发明）

| 想做什么 | 先看这里 |
|---|---|
| 加一条闸门 | `bash tools/validate.sh --index`（**闸门清单，引擎生成**）+ `decisions/README.md` |
| 改 phyt-DNA 的规矩 | `RNA.md` 的「先查再写」· `README.md` 的 Workflow 四步 |
| 跨仓协作 | **`KNOWN_ISSUES.md`**（跨仓缺陷唯一登记处） |
| 端口 | `ports.json`（SSOT）+ Cellrix 的 `web/tests/port_table_test.js`（**已覆盖面板端点**） |

## ④ 怎么验（**跑不了的东西不能声称**）

```bash
cd phyt-DNA && bash tools/validate.sh --probe-all     # 12 条闸门的三态
cd phyt-DNA && bash tools/ci-local.sh                 # 逐字复现 CI（推送前必跑）
cd Cellrix  && node web/tests/run_all.js              # 面板套件（注意：同环境也可能 FLAKY）
```

## ⑤ 下一件（agent 的建议，人可推翻）

**K16**（给「做」装门）—— 契约、布局、落点都已摸清（`D8-PLAN.md` 的 M0 已完成：基线 `gate=none` 已确认）。
**但从它开始前请先读 `D8-PLAN.md` 的三个坑**（跨三处同时改 / 装门=改行为 / 门是 fail-closed）。

---

**更新规则**：会话结束前更新 ①（hashes）与 ⑤（下一件）；② 只在 `KNOWN_ISSUES.md` 变更时同步
（**它是唯一真源，本文件不另立一份**）。
