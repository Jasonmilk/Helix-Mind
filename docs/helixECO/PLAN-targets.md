# 计划：接通 `one tape, many targets`（批次 1–5）

> **状态**：批次 1 待做，批次 2–5 未开始。
> **前提**：本计划的**事实**均已查实（下方逐条标注文件与行号），不是推断。

## 一、根本诊断：一个病，重复了八次

今天所有缺陷是**同一个病**：**同一事实多份推导**。

| 现象 | 违背 | 事实 |
|---|---|---|
| 证轨自己 `fetch('/api/events?job_id=')` | **极致复用** —— 第二份数据源 | `prove_track.js:124` |
| `subscribers` 与 `targets` 并存 | **极致复用** —— 同一问题两份实现 | `assembly.js:168` / `:171` |
| `flush()` 消费 `subscribers`，`activeTargets()` 无人用 | **按需驱动** —— 机制空置 | `assembly.js:328` / `:289` |
| DOM 契约清单是手抄快照 | **0 硬编码** —— 事实的第二份副本 | `docs/dom-contract.md` |
| 用 `grep`／断言检查"有没有传 stream" | **物理事实优先** —— 在检查推导 | — |
| `job_id` 派生自输入内容 | **确定性 ≠ 稳定** | `K15` |

**疫苗两年前就配好了**：`ADR-0018` 开篇 `one tape, many targets`（`assembly.js:3` 也写着）。**我们一直没接上它。**

## 二、已查实的事实（不要重新推断）

| 事实 | 位置 |
|---|---|
| `"One tape, many targets"` | `Cellrix/web/assets/assembly.js:3` |
| `var targets = {}`（name → { name, active }） | `assembly.js:168` |
| `var subscribers = []`（函数列表） | `assembly.js:171` |
| `var dirty`（tape 自上次 flush 后有变） | `assembly.js:172` |
| `subscribe(fn)` 入队；**首个订阅者且已有 watermark 时立即推一次并清 dirty** | `assembly.js:307` |
| `flush()` ⇒ **消费 `subscribers`**，返回订阅者数 | `assembly.js:328` |
| `activeTargets()` ⇒ 返回活跃 target **名**（排序） | `assembly.js:289` |
| `snapshot()` ⇒ **只含 watermark + digest，不含 tape**（D5） | `assembly.js:295` |
| `register` / `activate` / `deactivate` 已实现**且有测试覆盖** | `assembly_test.js:333`、`acceptance_test.js:179` |
| `loadPeriodToChat` ⇒ `mergeChain` ⇒ **一次 normalize** | `session.html` |
| 证轨入口 `__proveTrackLoad(jobId, meta, stream)`；**无 stream 时自己 fetch** | `prove_track.js:132` / `:142` / `:150` |

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

## 四、批次（**批次 2 是杠杆点**）

| 批次 | 动作 | 判据 |
|---|---|---|
| **1** | **证轨 target 化**：删 fallback ＋ **真链 80 事件断言** ＋ 变异 | 真链 10 period ⇒ 80 事件全接受、0 拒收 |
| **2** | **`flush()` 消费 `activeTargets()`**；`subscribers` 退役 | `activeTargets()` 非空 ⇒ 只推活跃 target；未 activate ⇒ **不构建** |
| **3** | **导出 target**（复用批次 2，**不是新 route**） | 查表法：`job_id` 只作索引键；**默认完整导出**；`redact` 可选 |
| **4** | **三分布局**（右证轨面板，收起 = `deactivate`） | e2e **skip 数不得增加**（对照 `dom-contract.md`） |
| **5** | **密码解锁 = `activate` 的前置条件** | **fail-closed**（失败 = 一块都不显示）；**导出同步上锁**；**只留时间不留凭据** |

**批次 2 的价值**：它把"每个视图各自取数"变成"加一个 target"。
之后每加一个视图，成本从**写一套取数逻辑**降到**注册一个 target**。

## 五、把「检查」换成「不可能」

> **检查一个约束，不如让违反它变得不可能。**

| 现在（检查） | 改成（不可能） |
|---|---|
| `grep`／断言"有没有传 stream" | target **只能读 snapshot**，没有别的入口 |
| 清单 + 纪律防漂移 | 跨资产 id **单一声明处**（`dom-contract.md` 已是生成物，下一步是单一来源） |
| 密码"渲染出来再藏" | **未 activate = 不构建**（真锁） |
| **`grep -E` 承重判断** | **用 Python 脚本** —— `re` **无 BRE/ERE 分裂**，坑从根上消失 |

## 六、纪律（今天用血换来的）

- **多分支匹配一律 `grep -E`，永不用 `\|`。** 今天因此踩空 **8 次**。
- **合成数据证明算法对；真实数据证明对的是它。** 两个 bug（`first_ts` 全局排序、
  根判定）只有真链能照出来。
- **跳过的检查读起来像通过。** 缺失样本必须 **FAIL**，不能 SKIP。
- **测试通过 ≠ 做完**（见 `phyt-DNA` 验证纪律）。
- **写注释前先自问语言**：源码英文，沟通中文。

## 七、待办（不属本计划）

- **HANDOFF 脱敏迁移** —— 它**不在任何 git 仓**，而它是「新会话唯一上下文来源」。已挂一整天。
- **`ADR-0020 会话身份`** —— `session_id`（容器锚）与 `job_id`（内容锚）分离（`K15`）。
- **`limits=50` 静默截断**（`K14`）—— 平铺后洞更明显。
- **`respond()` 的 `reason` 只映射 200/404/500**，其他状态码显示 `OK`（`server.rs:107`）。
