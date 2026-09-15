# Helix 生态既有缺陷登记表（KNOWN_ISSUES）

> **性质**：跨仓缺陷的**唯一**登记处。不是待办清单，不是 ADR，不是 DEPRECATE。
> **为什么单独一份**：这些缺陷跨仓（涉及 anaphase / Cellrix / helix-mind），
> 塞进任一单仓都会变成"归属错误的文档"；放在工作区根又不在任何 git 仓里 —— **等于可丢失**。
> 故紧邻生态 SSOT（`helix-mind/docs/helixECO/ECOSYSTEM.md`）存放。
>
> **登记规则**：
> 1. 一条缺陷 = 一个**已被物理核对确认**的事实（附文件 + 行号 + 核对日期）。
> 2. 未核对的猜测**不登记**（宁可空着，不猜）。
> 3. 修复后**不删除**，转入 §2 已修（历史永不删除，只前进不回头）。
> 4. 每条必须写清「为什么它现在还没修」—— 是**待裁决**还是**没人做**，两者不同。

---

## 1. 未修

| # | 缺陷 | 位置（事实基线 2026-09-15） | 为什么还没修 | 关联 |
|---|---|---|---|---|
| **K3** | `run_cycle.rs:1052` 把 `reasoning_mode`（`"left_brain"` 模式标签）当 `model` 写进 `traces/reasoning.jsonl`，与 `assistant/reply` 的 physical model（ADR-0036）**不是同一事实** | anaphase-helix `src/run_cycle.rs:1052` | **用户明确要求"再议"**（2026-09-14），故冻结不动 | `anaphase:ADR-0036` |
| **K4** | ADR 编号**跨仓撞号且不同义**：`anaphase:ADR-0016`（编排哲学）vs `Cellrix:ADR-0016`（证轨资产解耦）；`anaphase:ADR-0017`（CI-144 传输层）vs `Cellrix:ADR-0017`（资产语言） | 两仓 `docs/decisions/` | 编号是**生态共享序列**，不能回改。缓解办法已落地：**跨仓引用一律仓名限定**（见各 ADR 头部） | 本表 §3 |
| **K11** | `assistant/usage`（计量事件）被 `prove_track.data.js::derivePeriodUsage` 期待，但**不在 `anaphase:ADR-0026` D2 词表中** ⇒ 装配层（按词表校验）会**拒收**它，该原语恒 `null`。**前提修正（2026-09-15）**：原记录「真实事件流亦从未出现」**不成立** —— `assistant/usage` 自 2026-09-14 17:19 起就在真实流中（`run-66c96eca` / `run-ee6cbd83` / `run-fbb7890b` / `run-ddaf2e59` 均含，09-15 `run-8bba24c5` 亦有，seq 3，data `{chars,model}`）⇒ **是词表落后于实现**，不是实现缺失。**危险面坐实**：装配层对未知 kind 静默丢弃（见 K13），计量事件进装配路径即消失无痕 | `Cellrix/web/assets/prove_track.data.js:168` ／ `anaphase-helix/docs/decisions/ADR-0026-session-event-stream.md` §D2 ／ `Cellrix/web/assets/event_family.js` | 需裁决：① **词表补 metering 事件（协议扩展，推荐 —— 生产者已在发）**，或 ② 明确「计量不进事件流」并移除 Cellrix 的期待 | `K13` |
| **K5** | ECOSYSTEM.md 自述为生态 SSOT，但其内部存在**多份互相打架的组件清单**（目录树 / 项目状态总览 / 架构图 / 快速入口） | `helix-mind/docs/helixECO/ECOSYSTEM.md` v1.94（390 行） | **先收敛 SSOT 本身**，再谈投影一致 —— 否则会收敛到一个自相矛盾的源 | — |
| **K14** | **`/api/sessions` 的 `limit` 会静默截断**：磁盘有 **91** 个 period，API 只返回 **50** ⇒ **41 个不进 DOM**。两个后果：① **跨边界的链其 `+N` 只是下界**（老链会被再次报低，形态与 F16 同）；② **对话累积后老记录持续掉出窗口** ⇒ 将来会以「又消失了」的形式复发 | `anaphase-helix/src/main.rs:169`（`unwrap_or(50)`）／`Cellrix/web/src/routes.rs:118`（`?limit=50`） | **未修。已核对（91 vs 50）**。留痕两处：跨边界链的 `+N` 标注为**下界**；考虑改为不受 limit 影响的算法或分页加载 |

---

## 2. 已修

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

> **F6 附注（设计事实，不是缺陷）**：系统负载的降级实为**两层** —— Anaphase 的 `load_gate`（作用在 `budget_tier`）与 Mind 的 energy guard（作用在 `CognitiveMode`）。**作用对象不同，不是同一事实的重复来源**；但两者阈值分属两仓 config，**没有对齐机制**。将来若有人改其中一处，需知另一处独立存在。

---

## 3. 已落地的缓解措施（不是修复，是止血）

| 缺陷 | 缓解 | 状态 |
|---|---|---|
| **K4 撞号** | 跨仓引用一律**仓名限定**（`anaphase:ADR-0016` / `Cellrix:ADR-0016`）。已写入 ADR-0018 / 0039 / 0040 / 0102 的头部与参考节 | ✅ 已落地 |
| **K3/K5** | 登记在案（本表），不再依赖记忆文件保存 | ✅ 已落地（本表） |

---

*登记 ≠ 认领。修不修、什么时候修，另议。*
