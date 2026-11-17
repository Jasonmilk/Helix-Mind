# M7（UI 解冻）四条件 —— 只读读数（2026-11-16）

> 依据：`center/SESSION-DAG-IS-THE-CENTER.md:127-140`「UI 无主动启动权，四条件全满足才启动；
> 任意一条不满足 ⇒ M7 保持冻结，不'先做起来看看'」。本文件只报数，不启动。

## 四条件逐条

| # | 条件（原文摘） | 读数 | 判 |
|---|---|---|---|
| 1 | M2–M6 全部完成且物理验收通过 | M2 ✅（写路径 0/0 · I5 原语+调用点）· M3 ✅（I2/I3/I4/I5 各有断言）· M4 ✅（2 死 1 等价）· M5 ✅（转发面移除）· **M6 = B18 ⬜** | ❌ |
| 2 | 139 个 period 孤根数 83 → 可接受 | 现网 local store `.helix/events`：**668 period · 有父 184 · 孤根 484（72%）· 父非 id 形状 0** | ❌（72% 孤根，与"可接受"相反方向） |
| 3 | `slice` 在展示层可用，且与 `lineage_path` 不混用 | `slice_of`（convergence.rs:627）· `slice_from`（:704）· 路由 `GET /v1/sessions?root=`（main.rs:176）· `chain_of`（:716）另名保留 | ✅ |
| 4 | 无新增「面」（六条硬禁令之 2） | `anaphase` 路由计数 **19**（`grep -c '\.route("' src/main.rs`） | ⚠️ 需与基线比较（本轮未取基线，具名 UNMEASURABLE） |

## 结论

```
M7 保持冻结：条件 1（M6/B18 未完成）与条件 2（孤根 72%）明确不满足。
唯一已满足的工程条件是条件 3（slice 可用且与 lineage_path 分名）。
```

## 顺带一个真读数（与条件 2 相关，且是新一轮）

```
668 个 period 里 484 个是孤根（72%）—— 比 10-09 基线的 139/83（60%）更差。
原因待查（不属本文件范围）：多数是"单轮问答"天然无父，还是 resume 失败？
判据（下一轮可做）：抽样 20 条孤根，看是否带 resume 意图（user/message 里出现"继续/接着"）
```

---

## 补测（同日）：条件 2 的 72% 被**自动化流量**污染

```
孤根 484 条里只有 28 种不同输入：
   263 × 「只回答两个字:你好」
   128 × 「S303 HTTP 判据:不带锚点」
    58 × 空输入
     其余 35 条（hi / say pong / 你是谁 / calc 题 …）
⇒ 疑似脚本化/测试输入合计 456（**94%**）

按"真实对话口径"过滤后：period 51 · 孤根 19（**37%**）
```

**结论（口径问题，不是新缺陷）**：条件 2 的"孤根 83 → 可接受"若要可判，必须先**具名剔除**
脚本化输入族（上表前四类）。过滤前 72% / 过滤后 37% —— 两个数都真实，混用就是拿错分母。

**建议（不自行改条件 2 的口径）**：在 center 的 M7 条件 2 旁加一句"分母 = 去除已具名
自动化输入族后的真实对话 period"，由园丁裁定；本轮只在观测里留数与口径提议。

---

## 端到端复验（2026-11-17，M5 删除之后 —— 行而后知）

```
bash Cellrix/tools/accept_all.sh                      ⇒ rc=0
  ① 重放 diff=0 · sha faba6d6a81262328
  ② 出口 5 条 · D1/D2 rc=0
  ②b 页面缺口 2 处 · 2930 字节 · 反向检查通过
  ③ git ls-files web = 0 · test -d web ⇒ 1
bash Cellrix/tools/run_one_cycle.sh --config-upstream ⇒ rc=4
  UNMEASURABLE: config.toml 里没有 reasoning_endpoint ⇒ 无法接真上游（不是通过）
bash Cellrix/tools/run_one_cycle.sh（桩）             ⇒ rc=0 · impasse=False · model=stub
  ⇒ M5 删除 FlowModus 转发面**没有**伤到桩通路
convergence-screen.sh                                 ⇒ rc=0（分项判定达标）
anaphase cargo test --all-features                    ⇒ rc=0（M5 删除后全量仍绿）
```

**本轮的自我提醒（又是同一族）**：第一次读 `run_one_cycle --config-upstream` 的 rc 时写成
`bash … | tail -3; echo rc=$?` ⇒ 取到的是 `tail` 的 0，误报"期望 4 却得 0"。
去掉管道后 rc=4 正确。⇒ **rc 永远不能穿过管道读**（第 N 次，记在案上）。

---

## 文档判据全量跑一遍（2026-11-17，行而后知）

```
anaphase  ci/check_adr_refs.py                                  rc=0
anaphase  ci/check_line_budget.py                               rc=0
helix-mind tools/check-adr-heads.sh                             rc=0
helix-mind tools/check-terms.py --naming                        rc=0
helix-mind tools/convergence-screen.sh                          rc=0
kernel     --overdue --triggers …                               rc=0
docgen     --check                                              rc=0
台账+队列  check_ci144_profile.py --root . --file … --queue …    rc=0（selftest 13/13）
```

**又一次"在错的对象上量"**：第一次跑台账判据时我写的是
`cd anaphase-helix && python3 ci/check_ci144_profile.py --root .. --file docs/helixECO/…`，
`--root ..` 指向工作区根、而 `docs/helixECO/…` 只在 helix-mind 下 ⇒ rc=1。
**那是我的调用错了，不是台账红了**；在 helix-mind 里用 `--root .` 跑 ⇒ rc=0 + selftest 13/13。
⇒ 判据的 rc 只有**在正确坐标下**才有意义（与前几轮"分母/对象/管道读 rc"同一族）。

---

## 「可审计」这条光的机械读数（2026-11-17）

VISION 说证轨要「可审计」；审计的篡改可见性住在 **Tuck 的哈希链**（`tuck-audit`），不在 anaphase。
本轮把它真跑了一遍：

```
cargo test -p tuck-audit --lib ⇒ 7 passed / 0 failed
  append_and_verify_roundtrip · deterministic_byte_identical · seq_recovers_from_tail
  tamper_edit_payload_breaks_chain · tamper_delete_row_breaks_chain · tamper_reorder_breaks_chain
  empty_chain_verifies
⇒ 三种篡改（改内容 / 删行 / 换序）各有具名测试，且都绿
```

**又一次"读到空"**：第一次跑 `cargo test -q -p tuck-audit | tail -4` 显示 `running 0 tests`，
我差点记成"这个 crate 没有测试"。实际是 tail 截到了**doc-test 那个二进制**（它确实 0 个）；
`--lib` 才是那 7 个。⇒ 与"分母/对象/管道/坐标"同族第 5 种形态：**读到的是另一个测试二进制**。

---

## M6 / C1 的前置：中心卷那条「Tuck 无 model 字段」需要更正（2026-11-17 读码）

`CONVERGENCE-M0-M7.md §4.3` 记：C1 的 `model/status` 两腿被两条缺陷卡住，其一是「Tuck 无 model 字段」。
读码结果比这句更精确：

```
Tuck 的审计记录 `tuck-audit::AuditRecord.payload` 是**不透明 JSON**（`crates/tuck-audit/src/lib.rs:106`）
   ⇒ 链本身不限制字段，model/status 无需改链结构即可承载
而且 `tuck-gateway/src/gov.rs:94/:112` **已经在写**：
   {"action":"access_deny","supplier":…,"model":…,"scope":…,"rule_id":…}
   —— 但只在 `verdict.effect == Deny` 这一支里写
```

⇒ 准确说法不是「Tuck 无 model 字段」，而是「**Tuck 只在拒绝类记录上带 model/supplier**」。
这对 M6 的影响是决定性的：它不是"加字段"，而是**要不要把记录条件从 Deny 扩到全量请求**——
前者是结构变更（门），后者是行为面变更（也是门），但两者代价差一个量级。

**结论**：M6 仍是门，但门的形状变了（从"加字段的契约变更"变成"扩记录条件的行为面变更"）。
本条只更正前提，不改任何代码；`center` 的 §4.3 措辞若要改，须人裁定（方向层）。

---

## ★ 第一个**真上游**一轮（2026-11-18，本地 llama-swap，非桩）

**怎么接上的**（全部本地，无需外部凭据）：
```
chain.json 说 Tuck 是 "the only LLM door" ⇒ 探 60052 /v1/chat/completions ⇒ 401（要求上级供应商 key）
FlowModus supplier list ⇒ local-llama（coder-3b，llama-swap @8080）· agnes-ai（外部，需网络）
≫ 选本地：llama-swap --config ~/llama-swap/config.yaml，8080 /v1/models 立即可用
anaphase config（本地，gitignored）：
   reasoning_endpoint = "http://127.0.0.1:8080/v1"
   tentacle_endpoint  = "http://127.0.0.1:50051"   ← chain.json 的 anaphase_value
```

**读数（真模型，不是桩）**：
```
POST /v1/chat "…compute 9^9…" ⇒ {"impasse":false,"model":"coder-3b",
  "reply":"The exact integer value of \(9^9\) is 387,420,489."}
tape: 9 行（turn/start · user/message · context/inject · usage · think · attempt · reply · turn/end · check/status）
/health: tentacle ok=true detail=reachable  ⇒ pipeline **Bound**
/health: reasoning ok=true detail=not configured ← 注意：健康检查按 config 字段名判，端点在却报 not configured
```

**⇒ 门① 与 门② 同时满足**（上游真跑 + pipeline Bound）—— 这是本项目第一次。

## 但工具段仍未走通：解析器比文档写的更严（三个形状都不认）

```
① 让模型直接回 JSON 计划 → 它回 ```json 围栏包着的计划
   attempt: "no calls planned — answered directly"  ⇒ 围栏没被 parse_tool_fence 接受
② 让它回 ```tool 围栏 + numbers({"expression":"9**9"})
   attempt: 同上 ⇒ 仍未解析
③ 三个形状下 tool/call 计数都是 0
```

**复现材料**（两条原文 reply）：
```
"```json\n{\"calls\":[{\"tool\":\"numbers\",...}]}\n```"
"```tool\nnumbers({\"expression\":\"9**9\"})\n```"
```

⇒ 结论：**工具段卡在"模型输出的形状"与"契约接受的形状"之间**，不是模型不会、也不是 tentacle 不通。
下一步（下一轮，不需要人）：读 `contract::parse_tool_fence` 的接受形状，与上面两条原文对差。

---

## ★★ 第一次端到端：计划 → 执行 → tool/result → verdict → HITL 计数非零（2026-11-18）

**环境**：llama-swap(coder-3b @8080) · tentacle 重启时带 `--plugins-dir ./fixtures`
（原进程 `--transport grpc --grpc-port 50051` 没带插件目录 ⇒ 注册 0 个工具 ⇒ 上一轮 gRPC NotFound）。
重启后 `插件扫描完成: 扫描 5 个, 成功注册 5 个`。

**tape（14 行）**：
```
turn/start · user/message · context/inject · usage · think · attempt(planned calls: numbers)
tool/call      {"tool":"numbers","index":0,"expect":"ok"}
tool/result    {"ok":true,"duration_ms":26,"outcome":"{\"series\":[1,2,3,4,5,6,7,8,9,10]}","outcome_sha":"e7878edb"}
check/status   exec_ok        actual=ok=false
check/status   answer.delivered  actual="no business result in tool return"
verdict/status {"status":"Unmet","checks":2,"reason":"failed: exec_ok, answer.delivered"}
reply "9**9 = 387420489" · turn/end success=false
check/status(phase=hitl) {"allowed_once":1,"rejected":0,"cancelled":0,"unavailable":0}   ★
```

**三条读数，各自独立**：
1. **工具段走通了**：`tool/call` 与 `tool/result` 首次出现，执行 26ms、`ok=true`。
2. **HITL 四态首次非零**：`allowed_once=1`（此前几十轮一直是 0）。
3. **verdict Unmet 是因为夹具工具**：`numbers` 的 fixture 返回固定 `series`，不认 `expression`
   ⇒ `exec_ok` 假、`answer.delivered` 假。**不是链路坏，是工具语义与判据不匹配。**

⇒ 这同时回答了两个长期问题：「授权段能不能被撞到」= 能（本次 allowed_once=1）；
  「工具段为什么一直 0」= 模型形状（已修 parse_json_fence）+ tentacle 无插件（已带目录重启）。

---

## ★★★ 第一次全绿端到端（verdict=Met，2026-11-18）

**换成会算的工具就没问题**——`fixtures/numbers.js` 是"固定序列"夹具（设计如此，不该为了过判据去改它），
`fixtures/calc.js` 才返回 `{ok:true,result:…}`。用 calc 跑同一句话：

```
tool/call     {"tool":"calc","index":0,"expect":"ok"}
tool/result   {"ok":true,"duration_ms":55,"outcome":"{\"ok\":true,\"result\":\"387420489\"}","outcome_sha":"62d7a55f"}
check exec_ok          actual=ok=true
check answer.delivered actual="result produced, delivery confirmed at tool edge"
verdict/status         {"status":"Met","checks":2,"reason":"all checks passed"}
turn/end               success=true · verdict=Met · model=coder-3b
reply                  'calc: {"ok":true,"result":"387420489"}'
check(phase=hitl)      {"allowed_once":1,…}
```

**配套三条验证（同一份 tape）**：
```
bin/agui 标准事件 = 5 条（RUN_STARTED / TEXT_MESSAGE_* / RUN_FINISHED）
D1/D2 判据 rc=0（线层 + 语义层）
案卷重放：同 tape 跑两遍 diff=0（可重放）
```

⇒ 「真上游一轮 ⇒ 标准事件 ⇒ 证轨可查可重放」三件事第一次**同时**成立。
仍按 C-069：闭合由园丁签，本文件只交读数。
