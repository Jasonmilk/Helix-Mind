# HANDOFF — 生态交接点

> 更新时间：2026-09-08（ADR-0034 空回复修复完成）
> 用途：跨会话恢复上下文的权威快照。先读本文件 → ECOSYSTEM.md（SSOT）→ 目标项目 PLAN.md/GROWTH.md。
> 维护：每次交接时更新本文件（覆盖式，保持单页、准确、不冗余）。

## 0. 当前物理状态（已核对）

| 服务 | 端口 | 状态 | 进程 |
|---|---|---|---|
| anaphase-helix | 50061 | ✅ RUNNING（含 ADR-0034 新二进制） | target/debug/anaphase |
| cellrix-web | 8080 | ✅ RUNNING | target/debug/cellrix-web |
| tuck | 60052 | ✅ RUNNING（Bearer tk-local-gate） | target/debug/tuck |
| helix-mind-cli | — | ✅ RUNNING（config: .helix/mind/config.toml） | helix-mind-cli run |

浏览器入口：`http://127.0.0.1:8080/`（WebUI + TUI 同构，TUI 经 `up` 选择 [2]）。
白盒核对路径：`.helix/events/run-*.events.jsonl`（会话事件，权威）、`.helix/traces/reasoning.jsonl`（推理正文）、`Tuck/gateway-audit.jsonl`（审计链）。

## 1. 各仓库对齐状态（2026-09-08 物理核对，全部已推送）

| 仓库 | 分支 | HEAD | 说明 |
|---|---|---|---|
| anaphase-helix | rs | e4adc88 | ADR-0030（SSE done 确定性）+ ADR-0034（空回复防护）；测试 240 全绿 |
| Helix-Mind | rs-dev | d6787b5+ | ECOSYSTEM v1.88；P0-P10 全通 |
| Cellrix | rs2 | b921f92 | ADR-0029：proxy EOF grace + fold primitive + think/check/outcome render |
| Tuck | rs | （刚清理推送） | 移除 gateway-audit.jsonl 跟踪（运行时产物，*.jsonl 已 ignore） |
| helix-tentacle | rs | a94584e | calc + web_search fixture |
| HelixECO-Glove | main | 8901a4c | P4-T1 完成，45 测试 |
| Helix-MCP-Learner | main | 3b98a67 | 核心完成 |
| phyt-DNA | main | e7bfbbf | PROTECTION v1.3（IP 条款 + spec headers） |

全生态测试：1558（Anaphase 240 / Cellrix 341 / Tuck 369 / Mind 118 / BIND-19 142 / Tentacle 153 / Glove 45 / MCP-Learner 50 / FlowModus 83）。

## 2. 最近完成（已推送，勿重复）

1. **ADR-0030**：SSE 事件序运行时焊死——终态通道 oneshot→mpsc（oneshot 重复 poll panic 曾导致 done 行丢失 → 浏览器只见裸 JSON）。
2. **ADR-0034**：回答被思考吞掉——reasoning 模型思考与回答共享 token 预算（deepseek-v4-flash 已知行为）。三层修：预算 2048→8192（config）+ `empty_reply_retries` 有界直答重试 + attempt `empty` 诚实标记。实测同问题回复落地（think 5816 + attempt 103，empty=false）。

## 3. 未完成事项（优先级排序）

### P0 待实施（方向已认可，未动手）
1. **判据拆分**：`exec_ok`（工具层 expect ok==true）拆出 `answer.delivered`（交付层 expect echo==true）——"工具成功 ≠ 任务完成：答案必须回传到人类"。当前判据声明与隐含检查不符（名实不符）。
2. **参数名 schema 锁定**：`expr`→`expression` 漂移毁掉 outcome_sha 复演对合——`list_tools` 丢弃了 proto Manifest 的 `parameters_schema` 字段（补字段 6），参数名由 tool schema 锁定，模型不得自由发挥。
3. **outcome_sha 只哈希业务产物**：现对 `{"ok":true,"result":"262144"}` 整体哈希，把协议字段 ok 也算入——协议层一变复演就误报漂移。只 hash `262144`。
4. **CONTEXT 恢复分隔符 + resume 段**：上次 `L3·03b55c83 0.5 liquid · L1·0d0a73d4 0.5 liquid` 有分隔可读，这次 `L3·0.5 c59cbf6b liquidL3·0.5 71b29c32 liquid` 挤成一团且 resume 段消失（承接上文的对话更需要 resume）。

### P1 待拍板
5. **hard FAIL 补救策略**：当前 hard gate 拦下后 `done=true` 静默结束——拦住但没处理。候选：重试/转人工/僵局标记（impasse）。需用户拍板。
6. **结晶闭环自动化**：同类 Unmet≥2 → 自动生成判据候选（胶体态）→ 人工确认 → 升 Tuck hard 规则（0-token 拦截）。当前只有手动 `POST /v1/crystallize`。

### P2 已裁定"记下以后再修"
7. **自检四色语义化**：黄=启动未联通 / 绿=连通 / 红=错误 / 灰=未检出（cellrix 面板仍二元 ✅/❌）。
8. **rails demo 导航词法误命中**：本地 config 已关闭 rails（不在路径上，不阻塞）。

## 4. 下一步建议（按你的节奏）

1. **优先 P0-1/2/3**（判据诚实三件套）：一次提交让 Engram 账本自洽——"判据能解释自己、参数名被 schema 锁定、哈希只盖业务产物"。这是上次审查的核心结论，做完 run-1dc862 就成了最好的 demo。
2. **P0-4**（CONTEXT 可读性）顺手同批修。
3. **P1-6**（结晶）值得认真做——"审计是结晶的矿源"是 Helix 差异化（0-token 拦在前面），但需先定人机确认 UI（Cellrix 侧）。
4. **P2** 按用户裁定暂缓。

## 5. 约束提醒（每次开工必守）

- 哲学八条：极致解耦/按需加载/按需驱动/极致复用/极致节能/物理事实优先/确定性优先/0硬编码。
- 文档先子后父：ADR → PLAN → GROWTH → README → ECOSYSTEM，全部推 GitHub，commit 关联 ADR。
- README 必更新（badge 测试数同步）；ECOSYSTEM 是 SSOT，项目状态变必须同步。
- 零行业词汇（测试/字段只用 tool/args/expect/numbers/rate/text/fixture/mock/schema）；拒绝正则、拒绝补丁思维、第一性原理。
- DSH/Cherry 只作灵感，不借命名（轨迹=印痕 Engram）；内部命名简短准确省 tokens。
- 运行时产物（*.jsonl 审计/事件）不进 git。
