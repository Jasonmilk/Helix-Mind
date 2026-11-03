# 执行队列（已裁未做）—— 与台账 + pending 组成三件套

| 文件 | 只记 | 判据 |
|---|---|---|
| `consensus-ledger.md` | **已裁已落地**项（落点 + 判据 + 状态） | `check_ci144_profile.py` |
| `pending-decisions.md` | **未裁**项（含阻塞谁） | 待裁为 0 才可声称"无未裁" |
| **本文件** | **已裁但未做**（含依赖与完成判据） | 同 id 出现在两处 ⇒ 红；本文件清空 ⇒ 全部落地 |

⇒ 三者互斥且并集 = 全体。**"已裁未做"过去两边都不装**，所以 11 条裁决只做了 3 条、其余只活在对话里（A4 时效三档已被弄丢过一次）。

`last-synced: 2026-11-03`（本轮逐条核过，读数写在各行；核不到的不标✅）

## 队列（按依赖排，不按发现顺序）

| id | 事 | 依赖 | 完成判据 | 状态 |
|---|---|---|---|---|
| Q-P0-2 | 修 `expected-red.py` R2 跨仓误报 | — | A2 的跨仓修法能正常转绿 | ✅ 已做（`repo:path` + 兄弟仓逐个查 + 不可见时具名 skip） |
| Q-P0-3 | 补测二：`is_high_risk_call` 对自己领域命中率 | — | 输出命中数 + 三分支分流 | ✅ 已做（见读数） |
| Q-P1-A3 | 两处 `Default` 投影 + 同批换掉漂移检测器 | — | 行为不变、既有测试 0 红、检测器可红 | ✅ `c0feb60` |
| Q-P1-A2 | `catastrophic_override` 默认 `Reject` | Q-P1-A3 · Q-P0-2 | 3 条 break-glass 判据转绿 | ✅ `0b54d8d` + `3489f4f` |
| Q-P1-A1 | 分离两轴 + 给 `lib.rs:513-519` 改名 | Q-P1-A2 | 9 红全绿、5 反样本仍绿 | ✅ `92ec69c`（KNOWN-LIVE 9→1） |
| Q-P2-1 | 判据清册 `criteria-inventory.md` + "没跑即 MISSING" | — | 遍历表即"跑全量" | ⬜ |
| Q-P2-2 | 产出即留痕（共识产生时落盘） | — | 新共识在产出当轮即进台账 | ⬜ |
| Q-P2-3 | `stale-after` 按批分组 | — | 到期能区分"忘了"与"排在后面" | ✅ 已落（`Tuck/tools/expected-red.py:48/:59` 解析表头 `since:`/`stale-after:`，到期仍有余项 ⇒ 判红） |
| Q-P3-A4 | 时效三档（形状先定死） | — | 三钟各自可设可测；撤销不依赖 TTL | 🟡 形状已裁（C-063）· 未实现：`anaphase-helix/src/hitl.rs:213` 注释即「本层拿不到 risk_tier」⇒ 分档做不了，等跨仓取用 |
| Q-P4 | 补测一：真实工具名矩阵 | — | 用例来自真实调用，不是编的 | ⬜ |
| Q-P5-B1 | Tuck 判定 / anaphase 执行分层 | — | 一套 HITL：Tuck 判定、anaphase 执行 | ✅ 已裁（C-046：Tuck gate 零生产调用者 ⇒ 废弃，不接路由） |
| Q-P5-B2b | 把 B2 四条判据迁到 anaphase 侧 | Q-P5-B1 | 见 `../../anaphase-helix/src/hitl.rs::enum ApprovalOutcome`（收窄与否以该枚举为准） | ✅ 已落 |
| Q-P5-B2 | 应答者注册（窄语义 + 映射表） | Q-P5-B1 | 见 `../../anaphase-helix/src/hitl.rs::enum ApprovalOutcome` · 注册器缺失判据 = `grep -rn ApproverRegistry anaphase-helix/src/` = 0 | ⬜ 未做 |
| Q-P6-C1 | 反射归「做」+ 学习一等状态（候选 ≠ 已授权） | — | ADR-0054 补段落；§2 学习出口闸门 | ⬜ |
| Q-P6-C2 | 档位真值源 = 后果探测（物理面 OR 资金面） | Q-P6-C1 | 声明与事实冲突时取高者可测 | ⬜ |
| Q-P6-M | 判据 A 对应 CONVERGENCE 的 M 几 | — | 写进 `center/CONVERGENCE-M0-M7.md` 任务列表 | ⬜ |

## 关于 kernel.py 的任务分配

`kernel.py` 现在排的是**环**（ring0..ring6 的 c1/c2/c3 判据），不是**这一份队列**。两者不要混：
- 环判据回答"生态现在通不通"，**已 7 环全绿**；
- 本队列回答"已裁的事做到哪一步"，**不依赖任何运行期状态**。

⇒ 若要让 `kernel.py` 承担任务分配，必须给它加一列 **`blocks`（本项阻塞谁）**，用现成的 `kernel.overdue()` 形态推优先级 —— 但那是**新工具**，在 A 组落地前不做（§2 停做清单第 1 条：不展开新发现）。
⇒ 现在的最小形态：本文件 + `Q-P0-x` 排序，人读即可。

## 队尾（本文件自身欠账，带机械 revisit）

| 条 | 判据 | 状态 |
|---|---|---|
| 代码路径 → 文档叶子映射缺口 | 无（形态未定，见 C-072 的撤回教训） | ⬜ 只登记不定形态；[revisit: `phyt-DNA` 面板仓建立 remote 且有人负责发布时 / 任何"代码反哺文档"提议] |
| 队列完成判据列的引用**无消费方** | 现状：`check_ci144_profile.py` 只扫台账 evidence，**不扫本文件** ⇒ 符号改名不会有人收到断链信号。⇒ C3 **降级**为「断链靠人点开时看见」，**不写成已验真**。 | ⬜ 若要升级：让现有 checker 顺带扫本列（本批不做，避免改判据后要跑三形+重基线）；[revisit: 下一次改 `anaphase-helix/src/hitl.rs::enum ApprovalOutcome` 时] |
