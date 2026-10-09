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

## 1. 未修

| # | 缺陷 | 位置（事实基线 2026-09-15） | 为什么还没修 | 关联 |
|---|---|---|---|---|
| **K3** | `run_cycle.rs:1052` 把 `reasoning_mode`（`"left_brain"` 模式标签）当 `model` 写进 `traces/reasoning.jsonl`，与 `assistant/reply` 的 physical model（ADR-0036）**不是同一事实** | anaphase-helix `src/run_cycle.rs:1052` | **用户明确要求"再议"**（2026-09-14），故冻结不动 | `anaphase:ADR-0036` |
| **K4** | ADR 编号**跨仓撞号且不同义**：`anaphase:ADR-0016`（编排哲学）vs `Cellrix:ADR-0016`（证轨资产解耦）；`anaphase:ADR-0017`（CI-144 传输层）vs `Cellrix:ADR-0017`（资产语言） | 两仓 `docs/decisions/` | 编号是**生态共享序列**，不能回改。缓解办法已落地：**跨仓引用一律仓名限定**（见各 ADR 头部） | 本表 §3 |
| **K11** | `assistant/usage`（计量事件）被 `prove_track.data.js::derivePeriodUsage` 期待，但**不在 `anaphase:ADR-0026` D2 词表中** ⇒ 装配层（按词表校验）会**拒收**它，该原语恒 `null`。**前提修正（2026-09-15）**：原记录「真实事件流亦从未出现」**不成立** —— `assistant/usage` 自 2026-09-14 17:19 起就在真实流中（`run-66c96eca` / `run-ee6cbd83` / `run-fbb7890b` / `run-ddaf2e59` 均含，09-15 `run-8bba24c5` 亦有，seq 3，data `{chars,model}`）⇒ **是词表落后于实现**，不是实现缺失。**危险面坐实**：装配层对未知 kind 静默丢弃（见 K13），计量事件进装配路径即消失无痕 | `Cellrix/web/assets/prove_track.data.js:168` ／ `anaphase-helix/docs/decisions/ADR-0026-session-event-stream.md` §D2 ／ `Cellrix/web/assets/event_family.js` | 需裁决：① **词表补 metering 事件（协议扩展，推荐 —— 生产者已在发）**，或 ② 明确「计量不进事件流」并移除 Cellrix 的期待 | `K13` |
| **K5** | ECOSYSTEM.md 自述为生态 SSOT，但其内部存在**多份互相打架的组件清单**（目录树 / 项目状态总览 / 架构图 / 快速入口） | `helix-mind/docs/helixECO/ECOSYSTEM.md` v1.94（390 行） | **先收敛 SSOT 本身**，再谈投影一致 —— 否则会收敛到一个自相矛盾的源 | — |
| **K15** | **会话身份缺失**：`job_id = derive(输入内容)` ⇒ **锚在「内容」上** ⇒ 内容每次提问都变 ⇒ **每次都是新文件** ⇒ **会话永远只有一轮**（一问一答，无连续会话）。注意区分：**确定性 ≠ 稳定** —— 确定性只保证「同一输入→同一 id」，稳定性要求「同一会话→同一 id」；锚在内容上则**确定性满足、稳定性为零** | `anaphase-helix/src/contract.rs`（`derive_job_id`） | **未修**。正解：**`session_id`（容器锚）与 `job_id`（内容锚）分离** —— 见 `ADR-0020`。**L0**（Cellrix 视图层沿 `resume_from` 合并成连续问答流）是**临时缓解，须标注待 L1 退役**；**L1**（anaphase 提供 `session_id`，同会话 append 而非新建文件）**跨仓提案** |
| **K14** | **`/api/sessions` 的 `limit` 会静默截断**：磁盘有 **91** 个 period，API 只返回 **50** ⇒ **41 个不进 DOM**。两个后果：① **跨边界的链其 `+N` 只是下界**（老链会被再次报低，形态与 F16 同）；② **对话累积后老记录持续掉出窗口** ⇒ 将来会以「又消失了」的形式复发 | `anaphase-helix/src/main.rs:169`（`unwrap_or(50)`）／`Cellrix/web/src/routes.rs:118`（`?limit=50`） | **未修。已核对（91 vs 50）**。留痕两处：跨边界链的 `+N` 标注为**下界**；考虑改为不受 limit 影响的算法或分页加载 |

---
| **K16** | **「想 / 说 / 做」之间没有门**：`main.rs` **从不装配** security gate（grep `with_security_gate` 为空）；`config.toml` 无该配置；**Tuck 也没有 `security/gate` 端点**（grep 为空）⇒ 提示注入让 LLM「想」出 `rm -rf` 时，**「做」这一步没有门**。**契约与纪律其实都在**（`security.rs` 的 `SecurityGate`/`GateVerdict`、`adapters/security_gate.rs` 的 `HttpSecurityGate`、`pipeline/mod.rs:171` 的 I7「没装门 ≠ 门通过了」、`run_cycle/mod.rs:722`「拒绝是具名的行」）—— **缺的只是三处接线** | `anaphase-helix/src/main.rs`（无装配）／`anaphase-helix/config.toml`（无配置）／`Tuck/crates/`（无端点） | ⏳ **待人类批准**（三项都改行为面）：① Tuck 实现 `security/gate` ② `main.rs` 装配 ③ 配置项。**已设计分阶段（M0 只读 ✅ / M1 Tuck 端点 / M2 装配观察态 / M3 翻转执行态 / M4 判据）**，见 `helix-mind/docs/helixECO/THINK-SAY-DO.md` 与 `D8-PLAN.md`。**⚠️ 门是 fail-closed ⇒ 装了它 "Tuck 挂了 = Helix 不思考"**（`run_cycle/mod.rs:671` 自己写着） | `I7` · `THINK-SAY-DO.md` |
| **K17** | **并发三处坑**（异步/高并发/队列是人类的长期目标）：① **每请求都跑一遍 `build_agent`**（内含 `build_identity_block`：读文件 + 连 Tentacle 取工具清单，`main.rs:902`）⇒ 高并发下的延迟/资源放大器；② **`tokio::mpsc::unbounded_channel` ×2**（`main.rs:593/608`）⇒ **无背压**；③ **无 `semaphore`/`rate_limit`/`concurrency_limit`** ⇒ 无减速阀，且"打满"没人具名 | `anaphase-helix/src/main.rs` | **没人做**（不急）。**形状是对的**：每请求一份全新 `AgentLoop`（`main.rs:512/548/692`）⇒ 无共享可变 agent、无状态串扰，且 47 处 `lock().unwrap()` 的爆炸半径被限制在单请求。⇒ 要做的是给它**加背压 + 上限（具名）**，不是重设计 | `D11` |
| **K18** | CI-144 边界上的 `vendored` 类型会漂（anaphase `src/ci144/` 钉在 Cellrix `cellrix-protocol` 的旧 commit 21d13d4 上） | 2026-10-09 | **回归判据：`Cellrix:cargo test -p cellrix-transport --test ci144_anaphase_live -- --ignored`**（需 `ANAPHASE_BIN`）—— **真协议、真二进制、真往返**（handshake → Manifest → snapshot push → action round-trip）。**实测当前通过**（`ok. 1 passed`）⇒ **漂移风险真实但未发生**。★ 同时修掉它"**未测量 = 通过**"的那一环：原写法在缺 `ANAPHASE_BIN` 时**静默 return ⇒ 报绿**；现改为**具名失败**（panic 带 `UNMEASURED: …` 与跑法）。**⇒ 不需要新加"静态类型比对"**（那要上 Rust 解析器 = 开越野），**已有的是更强的那个**（行为级），缺的只是"它别静默通过" |
| **K19** | **同一个名字指两个东西**：`verdict` 一族 5 处定义 —— 一物两名（`LedgerRecord::Verdict` 账本裁定 vs `PeriodVerdict` 周期结束），且 **同一 crate 内两个 `GateVerdict`**（`security.rs:58` 安全策略决定 vs `run_cycle/safety_gate.rs:52` 执行前检查结果，语义完全不同）⇒ 读者会误读 | `anaphase-helix/src/{security.rs,run_cycle/verdict.rs,run_cycle/safety_gate.rs,ledger/mod.rs}` | ⏳ **待裁决**（改名属重构）：建议把 `safety_gate` 那个改为 `ToolGateOutcome` 之类 | `D3` |
| **K20** | **编排有两份**：`pipeline::run()` 串六阶段，而**活路径**是 `run_cycle` + `Reflection` 自己调 `execute_calls`/`record_evidence`。⇒ 查"裁定为何没写"时读 `pipeline/mod.rs` 会**读错文件**（本会话实证：连猜四次全错，因为活路径根本不走它） | `anaphase-helix/src/pipeline/mod.rs` vs `src/run_cycle/{mod.rs,reflection.rs}` | **没人做**（大重构）。**低成本那一半可先做**：在 `pipeline::run()` 上**加一行注释指向活路径** | `D5` |

| **K21** | **Tuck 存量 clippy 警告**（实测 2026-10-09：`cargo clippy --workspace --all-features` 有若干条，主要是**测试模块里的 `unused import`**）。**`-D warnings` 不是本仓标准**（其 PLAN 写的是"clippy **新文件**零警告"）⇒ 整仓 `-D warnings` 会把既有的测试模块 lint 判成 error（假红） | `Tuck/crates/tuck-core/src/{config.rs,file_store.rs,audit_query.rs,credential.rs}` 等 | **没人做**（低优先）。**已落地处置**：`Tuck/tools/verify.sh` 的自证入口按**本仓标准**判：报告存量（具名）+ **只对真正的 `error` 判红** ⇒ 它仍然**能红**（不为装饰）。⚠️ 计数必须排除 cargo 的汇总行（"generated N warnings"），否则会报出不可归因的数（本步第一版就把 10 说成 40） | `Tuck/tools/verify.sh` |

| **K22** | PreToolUse 执行闸对真危险动作 fail-open（写 DNA.md / rm -rf 均放行） | 2026-10-09 | **回归判据：`phyt-DNA/examples/claude-code/hooks/hook_test.sh`**（9 例，含 3 条 ★回归：rm -rf ⇒ 2 · git push --force ⇒ 2 · 写 DNA.md ⇒ 2；以及 1 条 safety：`--force-with-lease` ⇒ 0）。**修法二分（诚实）**：**路径型**（权威卷 `DNA.md`/`VISION.md`/`decisions/**`）由 `ADR-20261009-dangerous-action-shapes-must-be-blocked`（`timing: pre`）拦；**字符串型**（命令形状）由 hook 自身的 **L0.5** 判 —— 因为 **glob 模型是路径型的，表达不了命令**。**⚠️ 第一版教训（已记入该 ADR）：用 `applies-to:["**"]` 想兼管命令 ⇒ 撞上引擎既有的"路径不存在 ⇒ fail-closed" ⇒ `**` 匹配一切 ⇒ **写任何新文件都被拦**（全拦，不是形状判据）；是 `hook_test` 在推送前抓住的。 |


## 2. 已修

> **规则第 5 条**：每行必须注明【回归判据】——没有回归判据的"已修"是纸面富贵。


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
