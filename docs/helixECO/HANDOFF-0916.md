# HANDOFF 2026-09-16 — 会话交接点

> **新会话从这里开始。**（本文件与 `HANDOFF.md`（09-08 历史快照）是**两份不同文件**；
> 那份是历史快照，本文件是当前交接点。）
> 工作区根 `/Users/jason/Doubao/chats/Jasonmilk/`（固定，不按日期分目录）。
> **只给当前状态与指向权威文档的指针** —— 细节不复述。

## 六仓 HEAD（**截至 2026-09-16 12:34**）

> ⚠️ **本表会立刻过时 —— 以 `git log --oneline -1` 为准，本文档不追。**
> 写死 hash 就是「事实的第二份副本」；这里保留只是为了说明**当时**的起点。

| 仓库 | HEAD |
|---|---|
| Cellrix | `24d5a1f` |
| Tuck | `5557bfd` |
| anaphase-helix | `69cc136` |
| helix-mind | `0c1ee57` |
| FlowModus | `547b795` |
| phyt-DNA | `f1331cf` |

六仓**全部零未提交**。

## 一句话现状

**用户报「第二问之后记录消失」→ 查出两个独立症状：**

- **A（已修・真链验收）**：`seq` 每个 turn 从 0 重开 ⇒ 装配层按全局 `seq` 去重 ⇒
  **第 2 轮起的事件全被当重复拒收**。修法：**`gseq` 在读取边界一次性赋值**
  （`period_normalize.js`）。真链验收：`run-7efbf0f8` **55 事件 / 5 轮 / 0 拒收**。
- **B（已取消・非 bug）**：`resume_from` 续接链聚合成一张卡 ⇒ 用户以为记录丢了。
  正解：**取消折叠、列表平铺**，`resume_from` 退回协议层。**数据从未丢失**。

**同轮完成**：`mergeChain`（合并多 period，**一次 normalize 由结构保证**）；
**L0** 点开即整链（标注**临时**，待 `session_id`）；证轨**不再自取数据源**；
**两道 git 闸门**（gate1 首行 / gate2 diff）**双向验证 + 已装八仓**；
`ab_verify.sh` **断言能力而非端口**；`phyt-DNA` 写入**验证纪律**；
`Cellrix/docs/dom-contract.md`（改布局前必读，**已可执行校验**）。

## 权威文档 —— 读这些，不读本文件

| 主题 | 位置 |
|---|---|
| **下一步计划（批次 0–7）** | `helix-mind/docs/helixECO/PLAN-targets.md` |
| 生态 SSOT | `helix-mind/docs/helixECO/ECOSYSTEM.md` |
| 跨仓缺陷登记 | `helix-mind/docs/helixECO/KNOWN_ISSUES.md` |
| DOM 契约（改布局前必读） | `Cellrix/docs/dom-contract.md` |
| 工作日志 | `.workbuddy-ai/memory/2026-09-16.md` |

## 下一步

**批次 0 已完成**（本文件）。**批次 1–7 见 `PLAN-targets.md`。**

**核心判断**：接上 `ADR-0018` 的 **`one tape, many targets`**。
现状不是「接口空置」，是**两套机制并存**：`flush()` 消费 `subscribers`，
而 `activeTargets()` **无人调用**（`assembly.js:171` / `:289` / `:328`）。
**批次 2 是杠杆** —— 接通后，每加一个视图从「写一套取数逻辑」降到「注册一个 target」。

## ⚠️ 两条已修正的结论（**别照旧版施工**）

1. **`K15` 的修法**：**不要**做 `session_id`（容器锚）与 `job_id`（内容锚）**分离** ——
   两家一致否掉（会多出「不是从内容派生、必须被写入并传播」的第二真相源；
   且引入容器 ⇒ 成员表与物理 `resume_from` 两份事实，必然裂脑）。
   **正解**：**`job_id = derive(输入 ‖ resume_from)`**，root 用 ∅；
   **会话 = root/head 指针**（不落盘、不新增实体）。
   这同时解决 `K15`（无需会话实体）与「5 轮挤一个文件」（非单射）。
2. **导出**：**Pull-only** —— 用户点击时才 `generate(snapshot)`，
   **不常驻 `activeTargets`**（否则每次 token 刷新都全量序列化）。
   仍需**同步上锁**（导出能绕过视图锁）。

## 纪律（今天用血换来的）

- **多分支匹配一律 `grep -E`，永不用 `\|`** —— 今天因此踩空 **8 次**。
- **合成数据证明算法对；真实数据证明对的是它。**
- **跳过的检查读起来像通过** ⇒ 缺失样本必须 **FAIL**，不能 SKIP。
- **API 数是服务端事实，不是消费方事实。**
- **检查一个约束，不如让违反它变得不可能。**
- **写注释前先自问语言**：源码英文，沟通中文。
