# 计划：接通 `one tape, many targets`（批次 1–5）

> **状态**：批次 1 待做，批次 2–5 未开始。  
> **前提**：本计划的**事实**均已查实（下方逐条标注文件与行号），不是推断。

## 一、根本诊断：一个病，重复了八次

今天所有缺陷是**同一个病**：**同一事实多份推导**。

| 现象                                               | 违背                    | 事实                         |
| ------------------------------------------------ | --------------------- | -------------------------- |
| 证轨自己 `fetch('/api/events?job_id=')`              | **极致复用** —— 第二份数据源    | `prove_track.js:124`       |
| `subscribers` 与 `targets` 并存                     | **极致复用** —— 同一问题两份实现  | `assembly.js:168` / `:171` |
| `flush()` 消费 `subscribers`，`activeTargets()` 无人用 | **按需驱动** —— 机制空置      | `assembly.js:328` / `:289` |
| DOM 契约清单是手抄快照                                    | **0 硬编码** —— 事实的第二份副本 | `docs/dom-contract.md`     |
| 用 `grep`／断言检查"有没有传 stream"                       | **物理事实优先** —— 在检查推导   | —                          |
| `job_id` 派生自输入内容                                 | **确定性 ≠ 稳定**          | `K15`                      |

**疫苗两年前就配好了**：`ADR-0018` 开篇 `one tape, many targets`（`assembly.js:3` 也写着）。**我们一直没接上它。**

## 二、已查实的事实（不要重新推断）

| 事实                                                                  | 位置                                              |
| ------------------------------------------------------------------- | ----------------------------------------------- |
| `"One tape, many targets"`                                          | `Cellrix/web/assets/assembly.js:3`              |
| `var targets = {}`（name → { name, active }）                         | `assembly.js:168`                               |
| `var subscribers = []`（函数列表）                                        | `assembly.js:171`                               |
| `var dirty`（tape 自上次 flush 后有变）                                     | `assembly.js:172`                               |
| `subscribe(fn)` 入队；**首个订阅者且已有 watermark 时立即推一次并清 dirty**            | `assembly.js:307`                               |
| `flush()` ⇒ **消费 `subscribers`**，返回订阅者数                             | `assembly.js:328`                               |
| `activeTargets()` ⇒ 返回活跃 target **名**（排序）                           | `assembly.js:289`                               |
| `snapshot()` ⇒ **只含 watermark + digest，不含 tape**（D5）                | `assembly.js:295`                               |
| `register` / `activate` / `deactivate` 已实现**且有测试覆盖**                | `assembly_test.js:333`、`acceptance_test.js:179` |
| `loadPeriodToChat` ⇒ `mergeChain` ⇒ **一次 normalize**                | `session.html`                                  |
| 证轨入口 `__proveTrackLoad(jobId, meta, stream)`；**无 stream 时自己 fetch** | `prove_track.js:132` / `:142` / `:150`          |

## 三、目标架构

```
mergeChain(...)  →  tape.feed(merged)  →  flush()
                                            │
                    ┌───────────────────────┼───────────────────────┐
              chat target              proveTrack target        export target
              （始终 active）          （解锁才 active）        （按需 active）
                    └──────── 同一个 snapshot，同一份数据 ────────┘
```

**证轨不再"被传参"，它是 target。导出不再是另写 route，它是 target 的序列化。**

## 四、批次（**顺序是杠杆，不是清单**）

⚠️ **批次 2 之前必须有"构造锁"**：只把 `flush()` 的出口换到 `activeTargets()`，
而 `register` 仍是工厂之外的**自由构造**，target 就能在任意处被 `new` ——
**构造本身就是第二份推导的入口**。那样会把 1 份推导变成 **N+1 份**。

| 批次 | 动作 | 为什么在这个位置 |
|---|---|---|
| **0** | ✅ **HANDOFF 最小版入库** | 决定「下一轮是否存在」 |
| **1** | **定义 Canonical Snapshot + 投影算子**；**ADR-0018 细化 D5** | 见下「前置」 |
| **2** | **`register(name, factory)` ＋ `activate` 作为唯一构造点** | **构造锁** |
| **3** | `flush()` 出口迁移到 `activeTargets()`；`subscribers` 退役（**语义要搬家**，见下） | 出口 |
| **4** | 证轨**注册为 target**（fallback **结构上无处落脚**）＋ 真链 80 事件断言 ＋ 变异 | 此时 fallback 自然消失 |
| **5** | 导出 —— **Pull-only** | 见下 |
| **6** | 三分布局（右证轨面板，收起 = `deactivate`） | 布局 |
| **7** | 密码解锁 = `activate` 的前置条件 | 真锁 |

### 前置（批次 1）：`snapshot()` 现在撑不起任何视图

`assembly.js:295` 的 `snapshot()` **只含 watermark 与 digest**（D5 有意如此）。
**证轨拿到它渲染不出 token 明细；对话拿到它渲染不出一条消息。**

⇒ 若批次 3 只是"把这个 snapshot 推给 activeTargets"，**三个视图会饿死**，
然后**各自再去取一次数** —— 那份推导会以「target 化已完成」的名义复活。

**必须先有 Canonical Snapshot**：拓扑还原 / 时序排序 / 归一化
**在全系统发生且仅发生一次**，产出**不可变**快照，暴露**投影算子**而非 raw 遍历：

```
projectConversation()  → 消息序列
projectAudit()         → token / 工具 / 检查
projectExport()        → 可审查结构
```

**⚠️ 同时细化 `ADR-0018` 的 D5**（原文「target 不得扫描窗口」会被误读成禁止此事）：

| 给 target 什么 | 允许？ |
|---|---|
| raw tape（**可重新解释**） | ❌ |
| **fold 已解释完的 Node 流**（只能 select / sort / group） | ✅ |

这就是**类型锁**：Node 上**不暴露可供分支的 type** ⇒ 想重新解释**没有原料**。
**不写清这条，下一个会话会拿 D5 把 Canonical Snapshot 否掉。**

### 批次 3 的两个语义必须搬家（不是删除）

`subscribers` 不只是通知，它还承载两条验收：

| 语义 | 现在的位置 | 搬到 |
|---|---|---|
| **一次发布 per merged window** | `dirty` 标志 | target 侧（或留在 `flush` 内） |
| **首帧立即推一次并清 `dirty`** | `subscribe()` 首个订阅者 | **`activate()` 时立即推一次** |

**漏掉第二条，现有验收条 7 会红。** 这是**搬家**，不是「删掉一套机制」。

### 批次 5：导出是 Pull，不是 Push

| 视图 | 动力学 |
|---|---|
| 对话 | Reactive Push |
| 证轨 | Conditional Push |
| **导出** | **On-demand Pull** |

⇒ 导出**不注册进 `activeTargets`**：用户点击时 `generate(assembly.getSnapshot())`。
仍走**同一个 snapshot**、**不另起 fold**，所以"消灭第三份 normalize"照样达成；
且**避免每次 token 刷新都全量序列化**。**导出仍需同步上锁**（它能绕过视图锁）。

## 五、把「检查」换成「不可能」

> **检查一个约束，不如让违反它变得不可能。**

| 现在（检查）                 | 改成（不可能）                                            |
| ---------------------- | -------------------------------------------------- |
| `grep`／断言"有没有传 stream" | target **只能读 snapshot**，没有别的入口                     |
| 清单 + 纪律防漂移             | 跨资产 id **单一声明处**（`dom-contract.md` 已是生成物，下一步是单一来源） |
| 密码"渲染出来再藏"             | **未 activate = 不构建**（真锁）                           |
| **`grep -E` 承重判断**     | **用 Python 脚本** —— `re` **无 BRE/ERE 分裂**，坑从根上消失    |

## 六、纪律（今天用血换来的）

- **多分支匹配一律 `grep -E`，永不用 `\|`。** 今天因此踩空 **8 次**。
- **合成数据证明算法对；真实数据证明对的是它。** 两个 bug（`first_ts` 全局排序、  
  根判定）只有真链能照出来。
- **跳过的检查读起来像通过。** 缺失样本必须 **FAIL**，不能 SKIP。
- **测试通过 ≠ 做完**（见 `phyt-DNA` 验证纪律）。
- **写注释前先自问语言**：源码英文，沟通中文。

## 七、待办（不属本计划）

- **HANDOFF 脱敏迁移** —— 它**不在任何 git 仓**，而它是「新会话唯一上下文来源」。已挂一整天。
- **`K15` 会话身份 —— 修法已修正，勿照旧版施工**：
  **不要**做 `session_id`（容器锚）与 `job_id`（内容锚）**分离**。两家一致否掉：
  - 分离会新增一个**不是从内容派生、必须被写入并传播**的**第二真相源**；
  - 引入容器 ⇒ **成员表**与物理 `resume_from` 是**两份事实**，必然裂脑。

  **正解**：
  ```
  job_id = derive(输入 ‖ resume_from)      root 用 ∅
  会话    = root/head 指针（不落盘、不新增实体）
  ```
  ⇒ 同时解决 `K15`（无需会话实体）与「5 轮挤一个文件」（**非单射**）。
- **`limits=50` 静默截断**（`K14`）—— 平铺后洞更明显。
- **`respond()` 的 `reason` 只映射 200/404/500**，其他状态码显示 `OK`（`server.rs:107`）。
