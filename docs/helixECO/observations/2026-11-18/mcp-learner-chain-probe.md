# 探针：MCP-Learner → Tentacle → anaphase 这条链（2026-11-18）

> 只读探测，不改任何仓。目的：验证刚登记的两个仓里，**前两跳到底通不通**。

## 一、学定义这一跳：通

```
Helix-MCP-Learner：`cargo run -- learn --command python3 --args tests/mock_mcp_server.py --output /tmp/learned --name mock-filesystem`
⇒ ✅ Learning complete · Tools: 4 · Stable 4 / Staging 0 / Rejected 0
产物：4 个 *.manifest.json + 一个共用执行器 `mcp_proxy.js`
Tentacle 加载：`--plugins-dir /tmp/learned/stable` ⇒「扫描 4 个，成功注册 4 个」
```

## 二、执行这一跳：**不通（且会给假绿）**

用 anaphase 真跑一轮（`tool = mock-filesystem.create_file`）：

```
assistant/attempt  planned calls: mock-filesystem.create_file
tool/call          {"tool":"mock-filesystem.create_file","expect":"ok"}
tool/result        {"ok":true,"duration_ms":30,
                    "outcome":"{\"data\":{\"message\":\"MCP Proxy placeholder —
                              replace with tentacle-transport-mcp in production\",\"params\":{…}}}"}
verdict/status     {"status":"Met","checks":2,"reason":"all checks passed"}
turn/end           success=true · verdict=Met · reply "hello from helix"
但：目标文件 /tmp/mcp_demo.txt **不存在**
```

⇒ **`mcp_proxy.js` 是占位符**（它自己的返回里写着 "placeholder — replace with tentacle-transport-mcp in production"）。
⇒ 学出来的 manifest 能过检查、能执行、能 `Met`，但**没有真正调用 MCP Server**。
⇒ 这是「存在性 ≠ 生效性」的活样本：`ok:true` + `Met` 都是**占位符给的**，不是工具真的做了事。

**对验收口径的影响**：用 fixtures 的 `calc`（真算）那圈 Met 是真；但**若换成学出来的 MCP 工具，Met 是假绿**。
⇒ 结论：这条链的"第二跳"目前只有壳；要接真，需要 `tentacle-transport-mcp`（属新语义面/行为面，须裁定）。

## 三、顺带查到的一处**静默降级**（anaphase 侧）

```
现象：tentacle_endpoint 已配（50059，健康检查 detail=reachable），但 run_cycle 报
      "execution pipeline unbound: no tentacle endpoint configured"
真因：pipeline 只在 `PipelineConfig::from_codex(...)` 成功时才装配（main.rs:1053-1057），
      而 codex 路径是 **cwd 相对**的 `knowledge_base/fixture-codex.json`；
      在错误 cwd 下启动 ⇒ `Warning: failed to load fixture-codex` ⇒ pipeline=None ⇒ 报"没配 endpoint"
读数：同一条命令加 `HELIX_CODEX=<绝对路径>` 后 ⇒ 该警告 0 次，pipeline 装配成功（tool 真的执行了）
⇒ 缺陷不是 unbound 本身，是**报错文案把三种原因混成一种**：
   ① 真没配 endpoint ② codex 缺（cwd 错） ③ 配了但连不上
   （与 C-058「三件事混成一件」同族；修文案属可逆小改，但会动提示语义 ⇒ 先登记不擅自改）
```

## 四、边界

```
本轮未改任何仓的代码；anaphase 用的是临时 config（/tmp/cfg_learned.toml，端口 50062/50059），
真 config 仍是 50061/50051。探完即弃。
```
