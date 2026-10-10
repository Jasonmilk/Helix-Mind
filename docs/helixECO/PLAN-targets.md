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
| **1** | **定义 Node 一次定完**（`kind` / `payload` / **identity** / `ord`）；**ADR-0018 细化 D5** | **1b 与 1d 合并**（见下）；判据分层（见下） |
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
**在全系统发生且仅发生一次**，产出**不可变**快照。

⚠️ **投影算子放 target 侧，不放 snapshot 侧**（两份外援在此相反，本计划采此裁决）：
- ❌ `snapshot.projectConversation()` —— **assembly 里写死三个 project 方法**
  ⇒ **每加一个视图就要改 assembly** ⇒ **把耦合搬进了核心层**
- ✅ **project 是 target 自己的纯函数**：输入 Node 流，输出视图数据；
  **assembly 只保证「Node 流唯一且已解释完」**

> 区分点很干脆：**Lens = 「我给你准备好视图」**（核心层知道有哪些视图）；
> **project = 「你自己算」**（核心层不需要知道）。**采后者。**

### 批次 1 的真实工作量：给 fold 补一层解释

**类型锁与证轨直接冲突**：锁要求 Node **不暴露可供分支的 type**（否则 target 能重新解释）；
而证轨**就是要区分** token / 工具 / 检查 —— **它需要分支**。

**解法：fold 输出语义字段，不输出 type 字符串。**

```
❌ node.type = 'tool/call'   ← target 可以 switch，能重新解释 ⇒ 违锁
✅ node.kind = 'tool'        ← 语义已定型，target 只能 select
   node.payload = { name, args, durationMs }
```

⇒ **批次 1 的主要工程量在这里**：把 **12 个事件类型的 `data` 解释成语义化 payload**。
**不是给 `snapshot()` 加几个字段** —— 是**给 fold 补一层解释**。
不写清这条，批次 1 会停在「snapshot 加字段」，然后发现**证轨仍然拿不到东西**。

⚠️ **`kind` 词表与 `type → kind` 映射必须进 `event_family.js`（契约层）。**

若写在 fold 里，它会与 `event_family.js` 的 `TYPES` / `KNOWN_TYPES` **并列** ⇒
**第三份类型事实** ⇒ **今天刚断言的「`EF.TYPES` 值集 == `KNOWN_TYPES`」当天就被打破**。

⇒ **契约层持有类型事实；fold 只做查表解释，不自带映射。**

**且 `interpret` 本身必须是查表，不是 `switch`**：
12 个 per-type 函数同样是硬编码，只是搬进了契约层。
⇒ payload 形状以 **`PAYLOAD_MAP` 声明**（`payload 字段 → 源字段 + 修饰`），`interpret` 退化为遍历该表。
✅ 已实现，且断言「`PAYLOAD_MAP` 覆盖全部类型」＋「契约不导出 per-type 函数」。
这样才能说：**fold 里没有硬编码的类型知识 = 0 硬编码 + 单一来源。**

### `snapshot()` —— **改，不是加**

**不留第二个出口。** 新增 `canonical()` 会让 `snapshot()` 与 `canonical()` 并存 ⇒
**又一份推导**（今天反复出现的那件事）。

按本计划自己的原则：**搬家，不是加线。** 现有调用方**迁移**到新的 `snapshot()`。

### Node 的 identity 与 ord 必须分离

```
node = jobId#gseq   ← 把身份与顺序焊死
```
⇒ 同一事件**从 root 读是 `B#10`、从中间片读是 `B#0`** ⇒ **违反确定性**。

⇒ **identity 锚「文件内可自证的量」；`gseq` 降为纯 `ord`，不进 id。**

### 批次 1：**Node 形状一次定完（1b 与 1d 合并）**

原顺序是 1b（产出 `kind`+`payload`）→ 1c（snapshot 撑得起）→ 1d（identity 分离）。
**这会留下一段中间态**：Node 已存在，但 id 仍是焊死的 `jobId#gseq`。
**若 1c 期间有人接了渲染，就会落在旧 id 上。**

⇒ **Node 的 `kind` / `payload` / `identity` / `ord` 一次性定完**；1c 只负责把它放进 `snapshot()`。

**前置（1d 要锚的量必须先被产出）**：`mergeChain` 现在补出 **`lineNo`**（**该行在所属文件内的行号**）。
没有它，1d 无处锚定 —— 合并之后每个事件在原文件里的行号就丢了
（`gseq` 是**合并后的位置**，是另一个量，且随起点变化）。

### 批次 1 的判据：**分两层写**（identity 一改，node id 必然变）

⚠️ 不能写「与当前实现一致」—— **1d 改的正是 node id**：

| | node id |
|---|---|
| 当前 | `rootJobId#37`（根 job + 全局位置） |
| 1d 之后 | `sourceLine#5`（来源文件 + 文件内行号） |

⇒ 「与当前一致」**自动不成立** ⇒ 下轮会以为改坏了，**或者反过来把 1d 悄悄跳过**。

| 层 | 判据 |
|---|---|
| **消息序列** | **与当前一致**（这是对的部分，不能动） |
| **node id** | **允许变化**，且**变化必须有断言**：**同一事件从不同起点读取 ⇒ id 相同** |

第二条正是「identity / ord 分离」要兑现的承诺 —— **它本身就是 1d 的验收**，
不该被「与当前一致」掩盖。

### 原判据（视图维度，仍然成立）

**两个视图分开写 —— 当前基准一个是好的、一个是坏的**

⚠️ **不能写「与当前实现一致」**：**当前对话视图是对的（80 事件），
而当前证轨视图是坏的（8 事件 —— 它自己 fetch 单 period，见 `prove_track.js:132/142/150`）**。
「与当前一致」对前者是**验收**，对后者是**把 bug 钉死**。

| 视图 | 判据 |
|---|---|
| **对话** | 与当前一致（当前已对：`run-7efbf0f8` ⇒ 55 事件 / 5 轮） |
| **证轨** | **真链 10 period ⇒ 80 事件**（**不是**与当前的 8 事件一致） |

> 这与「合成数据证明算法对」是同一个病：**拿一个错误的基准去验证，得到的绿是假的。**

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
| **一次发布 per merged window** | `dirty` 标志 | **留在 `flush()` 内** —— `dirty` 是 **tape 的物理状态**，搬到 target 侧 = **N 份状态** |
| **首帧立即推一次** | `subscribe()` 首个订阅者 | **`activate()` 时立即推一次**（这一条**才**搬家） |

**漏掉第二条，现有验收条 7 会红。** 这是**搬家**，不是「删掉一套机制」。

### 每个批次的判据（**无反例不成证**）

| 批次 | 判据 |
|---|---|
| 1 | 对话视图与当前一致；**证轨视图 80 事件**（见上） |
| **2** | **在 `activate()` 之外构造 target ⇒ 失败**（没有入口） |
| **3** | 未 `activate` 的 target **不收到任何调用**（含首帧） |
| 4 | 真链 10 period ⇒ 80 事件全接受、0 拒收；变异：去掉 `reverse` 顺序断言必须红 |
| **7** | **未解锁 ⇒ 证轨零 DOM 节点 + 零内存对象**（**不是 `display:none`**） |

### 批次 4：形状（**不是行数**）

> **行数估计一旦进计划，就会被当成事实执行** —— 「1c 要 20 行」实测 2 行，是同一个错。
> 所以这里写**什么移动、什么塌掉**，不写数字。

| 项 | 现状 | 变成 |
|---|---|---|
| **`KIND_CLASS` + `classOf`** | — | **进契约层**（`event_family.js`，与 `KIND_OF` 同处）—— **新增一个表 + 一个查表函数** |
| **`summarize`** | **11 个 `if (t === '…')` 按协议名分派** | **塌成查表 + 填值**（摘要模板与字段声明进 `KIND_CLASS`）—— **删多于加** |
| **`statusOf`** | 同 | **读 `payload`** —— **删** |
| **`buildSession`** | `TYPES[e.type]`（2 处）+ 吃原始事件 | **签名改 `nodes`** |
| **`prove_track.data.js:45` 的 `TYPES`** | **第四份类型词表** | **删除**（它变成契约层的 `KIND_CLASS`） |

#### 🔴 `KIND_CLASS` 必须进契约层，**不进证轨**

「**每一类语义属于哪个轨道**」是**类型事实**，不是视图事实。
放进 `prove_track.data.js` ⇒ **从「第四份词表」变成「第五份」**，只是换到证轨名下。
⇒ **今天刚把第三份（`KIND_OF`）收进契约层，不能在同一个坑里再挖一次。**

| 项 | 归处 |
|---|---|
| **`KIND_CLASS` / `classOf`** | **`event_family.js`**（契约层） |
| `summarize` / `statusOf` / `buildSession` | **证轨**（视图层，纯投影） |

⇒ **这样证轨层只有投影、没有类型知识** —— 类型锁才算真的锁住。

**判据**：`grep -En "turn/|tool/|check/|verdict/" web/assets/prove_track*` **必须为零**
—— **证轨里不出现任何协议名字符串。**

#### 🔴 `summarize` 必须塌成查表，**不是换成 kind 的 11 个 if**

若从 11 个 `if (t === '…')` 变成 11 个 `if (k === '…')` ⇒ **只是换了字符串，形态没变** ⇒
**与刚做对的 `interpret` 数据驱动化自相矛盾。**

⇒ 正确形态：**`KIND_CLASS` 里声明每类怎么摘要**（模板串 + 取哪些 `payload` 字段），
**`summarize` 退化为查表 + 填值**。

**判据**：`prove_track.data.js` 里 **per-kind 分支数 = 0**
（与契约层那条「不导出 per-type 函数」是同一守卫。）

#### 🔴 4a 的判据**不能写「与当前一致」**

当前证轨是**按协议名分派**的实现，而且**它此刻根本没被喂过正确的 Node 流**
（这正是批次 4 要修的）⇒ **没有稳定基准**。
⇒ 这就是 1e 那次踩过的坑：**拿错误的基准去验收。**

**正确判据**（用**真链 10 period ⇒ 80 事件**跑证轨）：

- **事件数 = 80**
- **每一类 `kind` 都被归类**（**无 `unknown`**）
- **`summarize` 对每种 kind 产出非空摘要**

⇒ 这是「**它撑得起**」的证明，不是「**它没变**」的证明。

### 批次 4 的隐藏前置：**tape 按 job 缓存（不是单例）**

`session.html` 每次 `loadPeriodToChat` 都 `create()` ⇒ **每次新 tape** ⇒
**注册的 target 每次都丢**（**批次 7 的密码锁解锁后换个经历就掉了**）。

**⇒ 采「按 job 缓存」`Map<jobId, tape>`**：切回同一经历**直接命中**，且天然隔离。

| 方案 | 判读 |
|---|---|
| 单例 + `reset` | ❌ 切回去要重放，丢失增量状态 |
| **按 job 缓存** | ✅ **切回命中 + 隔离** |

⚠️ **缓存要有上界**（LRU 或简单上限即可，**别过度设计**）。
**判据**：**切 job 不串；切回命中。**

### 纪律：批次 4 **不重跑脚本**

半改状态下的那个脚本**已被验证过一次是错的**。**手工改四处**
（`buildSession` / `prove_track.js` / `session.html` / `script.html`），
**每改一处跑一次测试** ⇒ **与「ADR 禁用脚本全局替换」是同一条纪律。**

### 批次 4 的两半步（各可独立提交）

| 半步 | 动作 | 判据 |
|---|---|---|
| **4a** | `KIND_CLASS`/`classOf` 进契约层；`summarize` 塌查表；`buildSession(nodes)` | 真链 80 事件；每种 kind 有非空摘要；**per-kind 分支 = 0**；**证轨零协议名字符串** |
| **4b** | 证轨注册为 target；删 `stream` 参数与 fallback | 真链 80 事件；**fallback 无处落脚** |

**⇒ 任何时刻中断，工作区都是干净可提交的。**

### 批次 4 · 3b 的设计（**已读清，未开工**）

**为什么不可分**：`summarize` / `statusOf` / `toolName` / `payloadOf` / `detailOf` /
`buildSession` / `derivePeriodUsage` 的**输入要一起从 `event` 换成 `node`**。
分开做就是「改签名不改调用方」—— 正是上轮回滚的那个半步。

#### 步骤 0 的分类结果（**有数字**）

| 类别 | 数量 | 去向 |
|---|---|---|
| **分支在协议名上**（`t === '…'`） | **23** | **塌成查表** |
| **分支在 payload 值上**（`d.ok` / `d.passed` / `d.status === 'Met'` / `d.empty`） | 嵌在 return 内 | **保留 ⇒ 抽具名谓词**（`isToolOk` / `isCheckPassed` / `isVerdictMet` / `isEmptyReply`） |

> **判据**：**kind 类 = 0**；**值类允许但必须具名**（可单测）。
> 一律塌会把「什么算通过」搬进数据表 —— 可读性归零、无法单测、改规则要改数据。

#### 分派与投影的分界

| 表 | 归处 | 内容 |
|---|---|---|
| **`KIND_CLASS`** | **契约层** ✅ 已就位 | `kind → cls`（**中性语义轴，单一字符串**） |
| **`LANE_OF`** | **视图侧（新）** | `kind → lane`（`model` / `input` / `tool`）—— **渲染概念，不进契约**（ADR-0019 §4） |
| **`SUMMARY`** | **视图侧（新）** | `kind → { tpl, fmt, opt, when }` —— **声明，非分支** |
| **`STATUS`** | **视图侧（新）** | `kind → status` —— **查表** |

#### 调用链（**两处必须同改**）

```
prove_track.js:121   buildSession(events, S.meta)   →  buildSession(nodes)
prove_track.js:122   derivePeriodUsage(events)      →  derivePeriodUsage(nodes)
prove_track.data.js  buildSession(events, meta)     →  buildSession(nodes)
```

**`meta` 不再需要**：`note` 是显示文案，而 Node 流已含全部所需。
**`deriveCoordinates` 不再需要**：`nodes` 已含 `turn` / `node` / `ord`。

#### 判据（**可机器验证**）

1. **`prove_track.data.js` 里 kind 类分支 = 0** —— 用
   `grep -cE "=== '[a-z]+/" ` 计数（值类谓词不计入）。
2. **证轨零协议名**：`grep -En "turn/|tool/|check/|verdict/|assistant/" prove_track*` **= 0**。
3. **真链 10 period ⇒ 80 事件**；**每类 `kind` 都被归类**（无 `unknown`）；
   **`summarize` 对每种 kind 产出非空摘要**。
4. **7 个新字段真实出现次数 ≥ 1** ✅ 已验证（`choice` 98/112、`empty` 74/109、
   `outcome_sha` 20/24、`index` 20/24、`verdict` 20/111、
   `cached_tokens` 33/33、`reasoning_tokens` 33/33、`model` 33/33）。

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

- **旧 `HANDOFF.md`（工作区根）：或删除，或脱敏后入库** ——
  批次 0 已完成：**新交接点 `HANDOFF-0916.md` 已在 git 内** ✅。
  但**旧文件仍在工作区根**，且**含敏感信息**（绝对路径 / 端口 / 端点 / key 线索），
  而它**不受版本控制**。⇒ 待办是**处理这个文件**（删或脱敏入库），不是"迁移"，
  否则下轮会以为批次 0 没做完。
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
