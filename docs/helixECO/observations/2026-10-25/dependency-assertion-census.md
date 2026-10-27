# 依赖断言普查（六条边）· 2026-10-25

> **只读普查。** 6 仓起点工作树干净（helix-mind / Cellrix / anaphase-helix / helix-tentacle / Tuck / FlowModus `git status --porcelain` = 0）。本文是**唯一写入**；未 commit、未 push。
> **修订说明（第 2 版 · 审计后重写）**：第 1 版有两处 ② 假「无」（`anaphase←helix-mind`、`anaphase←helix-tentacle`），根因是**我的搜索 head-limited**（`| head -5` 把同文件后面的断言截掉了）与我**声称查过 `Cellrix/src`/`Cellrix/tests`（两目录根本不存在）**。第 2 版逐条按审计员给出的 `file:line` 实读原文重写；①②③全部重算。

| 边（下游 ← 上游） | ① 本仓（**下游**）自证命令 | ② 上游 fail-closed 断言（`file:line` + 原文） | ③ 断掉上游的手段 |
|---|---|---|---|
| **cellrix ← anaphase** | `cd Cellrix && cargo test --no-fail-fast`（**下游**自证；本轮未实跑） | **有（但默认不跑）**：`Cellrix/transport/tests/anaphase_live.rs:13` = `assert!(client.health_check().await.unwrap(), "Anaphase not reachable at {base}");` —— 该测试 `:9` = `#[ignore = "requires live Anaphase with cap_http_enabled"]` ⇒ **断言上游可达，但 `#[ignore]`，默认 `cargo test` 不执行**（活体测试）。查过：`Cellrix/transport/tests/**`·`Cellrix/web/tests/**`·`Cellrix/web/src/**` | `--anaphase-endpoint http://127.0.0.1:1`（`Cellrix/web/src/bin/up.rs:126` 的旗标）；或不启动 anaphase |
| **anaphase ← helix-mind** | `cd anaphase-helix && cargo test --no-fail-fast`（**下游**自证；本轮未实跑） | **有（默认跑）**：`anaphase-helix/tests/mind_integration.rs:418` = `assert!(result.is_err(), "连接失败必须返回 Err 而非 panic");`（测试 `:415` `fn grpc_adapter_connect_failure_returns_err()`，`:417` 用 `http://127.0.0.1:1` 死端口）；同文件 `:450` = `assert!(result.is_err(), "server 下线后 query 必须返回 Err（非 panic）");` ⇒ **连接失败 / server 下线都必须 Err** | `export ANAPHASE_MIND_ENDPOINT=http://127.0.0.1:1`（仓内既有环境变量，见 `src/config.rs:566`） |
| **anaphase ← helix-tentacle** | `cd anaphase-helix && cargo test --no-fail-fast`（**下游**自证；本轮未实跑） | **有（默认跑）**：`anaphase-helix/src/health.rs:251` = `assert_eq!(v["ok"], false);` 与 `:258` = `assert_eq!(tentacle["ok"], false);`（测试 `:240` `fn endpoint_reachable_vs_refused()`；`:246` 指向本地 listener，`:249` drop 后必须 false）；另有 `anaphase-helix/src/gloves.rs:278` = `assert_eq!(g.status("tentacle"), Some(GloveStatus::Unavailable));`（`:275` 指向 `http://127.0.0.1:9`）| `export ANAPHASE_TENTACLE_ENDPOINT=http://127.0.0.1:1`；或不启动 helix-tentacle |
| **anaphase ← tuck** | `cd anaphase-helix && cargo test --no-fail-fast`（**下游**自证；本轮未实跑） | **有（默认跑）**：`anaphase-helix/src/health.rs:205` = `assert!(gate_ok(&c).is_err());`（测试 `:202` `fn gate_configured_unreachable_blocks()`，`:204` = `c.tuck_endpoint = Some("http://127.0.0.1:1".into()); // closed port`）| `export ANAPHASE_TUCK_ENDPOINT=http://127.0.0.1:1` |
| **cellrix ← flowmodus** | `cd Cellrix && cargo test --no-fail-fast`（**下游**自证；本轮未实跑） | **无**。查过：`Cellrix/transport/tests/**`（含 `anaphase_live.rs`）· `Cellrix/web/tests/**`·`Cellrix/web/src/**`（`bin/up.rs` 有 `flowmodus-serve`/`flowmodus-reason` 的**启动装配** `:464`/`:465`，但**没有任何测试断言它不可达时失败**）。另注：`Cellrix/src`、`Cellrix/tests` **不存在**（第 1 版误写为"查过"） | 不启动 `flowmodus-serve`(60053)/`flowmodus-reason`(60054)（`up` 链报 `Missing`）；无"断言型"手段 |
| **helix-mind ← helix-tentacle** | `cd helix-mind && cargo test --workspace --no-fail-fast`（**下游**自证；CI step ③ 已绿） | **无（且代码层无该依赖）**。查过：`helix-mind/crates/*/src/**`·`helix-mind/crates/*/tests/**` ⇒ 含 `tentacle` 的 `.rs` **0 个**、`Cargo.toml` 无该依赖；唯一相关是 `crates/helix-mind-core/src/config.rs:449` 的**端口撞号注释**（不是依赖）| 无（**不适用**）|

## 汇总

- **有断言的边数：4** —— `cellrix←anaphase`（**`#[ignore]` 活体，默认不跑**）· `anaphase←helix-mind` · `anaphase←helix-tentacle` · `anaphase←tuck`
- **值为「无」的边数：2** —— `cellrix←flowmodus` · `helix-mind←helix-tentacle`（后者代码层无依赖）

## 具名局限（不许假装全覆盖）

- ② 的关键词面 = `is_err` / `assert_eq!(…false)` / `Unavailable` / `closed port` / `127.0.0.1:1` / `127.0.0.1:9` / 上游名；**用自定义错误类型或超时间接表达"不可达"的测试仍可能漏**。第 1 版就是被 `| head -5` 截断而漏掉同文件的断言 —— 本轮已改为**逐条实读 `sed -n`**。
- ① 中**已实跑**的只有 FlowModus（109 passed，另有本次普查前记录）与 helix-mind（CI step ③ 绿）；其余 4 条为下游仓工作区标准命令，**本轮未实跑**（只读普查，不为此跑长构建）。
- **默认跑 vs `#[ignore]` 已在 ② 逐条标注**：4 条"有断言"里，`cellrix←anaphase` **默认不执行** —— 这是本表最重要的一条：**"有断言"不等于"cargo test 会跑到它"**。

---

**末行汇总（完成判据）**：**有断言的边数 = 4** · **值为「无」的边数 = 2**

