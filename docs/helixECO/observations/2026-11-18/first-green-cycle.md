# 第一个全绿周期（verdict=Met）—— 怎么复现、卡在哪三处、下一步看哪

> 状态：**读数齐备，未签**。按 `C-069`（守卫不自签）：闭合由园丁签，本文件只交可复跑的读数。

## 一、这一圈成立的三件事

```
① 真上游一轮：本地 llama-swap / coder-3b（非桩、非快照、非冻结 baseline）
② 标准事件：bin/agui 产出 5 条（RUN_STARTED / TEXT_MESSAGE_* / RUN_FINISHED）
③ 证轨可查可重放：同一 tape 跑两遍 casefile diff=0；D1/D2 判据 rc=0
```

## 二、tape 原文（14 行里的关键 6 行）

```
tool/call     {"tool":"calc","index":0,"expect":"ok"}
tool/result   {"ok":true,"duration_ms":55,"outcome":"{\"ok\":true,\"result\":\"387420489\"}","outcome_sha":"62d7a55f"}
check exec_ok          actual=ok=true
check answer.delivered actual="result produced, delivery confirmed at tool edge"
verdict/status         {"status":"Met","checks":2,"reason":"all checks passed"}
turn/end               success=true · verdict=Met · model=coder-3b
check(phase=hitl)      {"allowed_once":1,"rejected":0,"cancelled":0,"unavailable":0}
```

## 三、三个真原因（各自带证据，避免下次重踩）

| 症状 | 真因 | 修法 | 证据 |
|---|---|---|---|
| `tool/call` 恒 0 | 模型回 ```` ```json ```` 围栏包着的计划，而 `parse_tool_fence` 只认 ```` ```tool ```` + `name(args)` | `contract::parse_json_fence`（多认一种围栏，不放宽 `Call` schema）+ 反样本「围栏里只有答案⇒Err」 | 修后 `assistant/attempt: planned calls: numbers` |
| 执行 `gRPC NotFound` | tentacle 启动**没带 `--plugins-dir`**（`chain.json` 的 note 写着这件事） | 重启：`tentacle --transport grpc --grpc-port 50051 --plugins-dir ./fixtures` ⇒「扫描 5 个，成功注册 5 个」 | tool/result `ok=true`、55ms |
| `verdict=Unmet` | 用了 `numbers`（固定序列夹具），它不返回业务结果 | **换工具不换判据**：用 `calc`（返回 `{ok,result}`） | `exec_ok actual=ok=true` |

## 四、复现（三条命令；本机已跑通）

```bash
# 1) 本地推理（OpenAI 兼容，8080；coder-3b 在 ~/llama-swap/config.yaml 里）
llama-swap --config ~/llama-swap/config.yaml &          # /v1/models 可用即可

# 2) 工具通道必须带插件目录（否则注册 0 个工具 ⇒ gRPC NotFound）
cd /Users/jason/Developer/Jasonmilk/helix-tentacle
./target/debug/tentacle --transport grpc --grpc-port 50051 --plugins-dir ./fixtures &

# 3) anaphase 的本地配置（config.toml 是 gitignored，不改仓）
#    [anaphase]
#    reasoning_endpoint = "http://127.0.0.1:8080/v1"
#    tentacle_endpoint  = "http://127.0.0.1:50051"      ← 取自 ecosystem/chain.json 的 anaphase_value
cd /Users/jason/Developer/Jasonmilk/anaphase-helix
ANAPHASE_SESSION_EVENTS_PATH=/tmp/cycle cargo run -q --all-features --bin anaphase -- --config config.toml &
curl -s -X POST http://127.0.0.1:50061/v1/chat -H 'content-type: application/json' \
  -d '{"message":"Reply with ONLY this JSON in a ```json fence: {\"calls\":[{\"tool\":\"calc\",\"args\":{\"expression\":\"9**9\"},\"expect\":\"ok\"}]}"}'
```

**注意（会误导的地方）**：
```
· 端口一律取 ecosystem/chain.json（ADR-0046 唯一真源），不要凭记忆
· /v1/health 对 reasoning 报 "not configured" 是按 config 字段名判，端点生效路径可能不同
· 50061 上若有旧 anaphase 实例，它会用**自己的** session_events 目录应答 ⇒ 先清场再起
```

## 五、下一步（按依赖，不按偏好）

```
1. 把这一圈固化成第二份 fixture tape（fixtures/l2-tape-…) ⇒ accept_all.sh 覆盖真上游样本
2. M7 条件 1 仍差 M6/B18；条件 2 待园丁定分母口径（见 m7-gate-readings.md）
3. L2 行：读数齐备，等园丁签（C-069）；签之前不删行、不写「已确认」
```
