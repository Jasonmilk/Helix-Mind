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
