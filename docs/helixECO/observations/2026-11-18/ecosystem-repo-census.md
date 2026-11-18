# 生态仓普查（2026-11-18）—— 包含两个此前不在图上的仓

> 为什么有这份：`ECOSYSTEM.md` 的 §0.1 更新在等批准（`d-ecosystem-md-stale-42d`），
> 但**新仓已经在本地**。不登记它们，下一次接手的人（或我）会以为工具层只有 fixtures。
> 本文件只做**指针式普查**，不改 `ECOSYSTEM.md`（那是方向层）。

## 全部本地仓（`~/Developer/Jasonmilk`）

| 仓 | 角色（一句话） | 门面 / 端口 | 本轮实测 |
|---|---|---|---|
| `anaphase-helix` | 编排循环 · 会话 DAG · HITL 门 | `/v1/health` :50061 | 全量 `cargo test` rc=0 |
| `helix-mind` | 生态办公室（docs/helixECO）· 核心 | 观测点 :50052 | 屏 rc=0 · 台账判据 rc=0 |
| `Cellrix` | 证轨/面板侧 · 契约与观测 | 面板 :50050 | accept_all rc=0 |
| `Tuck` | 审计链（哈希链）· 唯一 LLM 门 | `/v1/chat/completions` :60052 | tuck-audit 7/7（含三种篡改） |
| `FlowModus` | 供应商路由 · 费率 | serve :60053 · reason :60054 | `supplier list` 3 家（agnes / local-llama / mock） |
| `helix-tentacle` | 工具执行（确定性通道） | gRPC :50051 | 带 `--plugins-dir ./fixtures` ⇒ 注册 5 个工具 |
| `phyt-DNA` | 方法论引擎（不提供方向） | — | ci-local 12 项 |
| `FlowModus` 之外的 **`Helix-MCP-Learner`** | **MCP Server → CI-144 工具定义**（只学不做） | 无服务端口（CLI/库） | `cargo test` **64 + 3 + 1 全绿** |
| **`HelixECO-Glove`** | **原生适配层**（`EcoGlove` trait，由 Tentacle 加载；执行） | 无服务端口（库） | `cargo test` **30 + 13 + 8 + 7 全绿**（`adapters/tentacle`） |
| `helix-tentacle` 之外的历史仓 | — | — | — |

## 为什么这两个仓重要（对当前这条链的意义）

```
现状：我们跑的 calc / numbers 都来自 helix-tentacle/fixtures（夹具）。
真实工具供给链应当是：
   MCP Server（外部世界已结构化）
      ↓  Helix-MCP-Learner        ← 把它翻译成 CI-144 工具定义（raw→staging→stable/rejected）
      ↓  HelixECO-Glove           ← 具体平台的执行适配（macOS/Linux/…），实现 EcoGlove
      ↓  helix-tentacle           ← 调度 + 加载 + 执行
      ↓  anaphase                 ← 编排与取证
⇒ 现在这条链**只接了最后一跳（fixtures 夹具）**；前两跳已有仓、有测试，但没有接进我们的周期。
```

## 三条可核读数（本轮实跑，不是引述）

```
Helix-MCP-Learner  cargo test --all-features ⇒ 64 passed / 0 failed（+ 3 + 1 其它套件）
HelixECO-Glove     cargo test --all-features ⇒ 30 passed / 0 failed（+ 13 + 8 + 7 其它套件）
tentacle           带 --plugins-dir ./fixtures ⇒ 「扫描 5 个，成功注册 5 个」
```

## 指针（下一步从哪看）

```
· 工具定义从哪来      Helix-MCP-Learner/README.md「Positioning」段；状态机 raw/staging/stable/rejected
· 谁执行             HelixECO-Glove/README.md「Maximum Decoupling」表：Gloves only execute；Tentacle schedules
· 本机已跑通的整链     observations/2026-11-18/first-green-cycle.md（verdict=Met 那圈）
· 复现命令            Cellrix/docs/observations/acceptance-entrypoints.md 第 ⑤ 节
```

**不做**：不把这两个仓写进 `ECOSYSTEM.md`（方向层，等 §0.1 批准）；不新增判据；不接它们进周期（那是下一步的评估，不是本轮）。
