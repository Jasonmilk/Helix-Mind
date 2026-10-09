# 重复 / 疏漏登记册（2026-10-09）

> 人类提醒：*"项目多次合拢和重构，所以可能会有重复的功能，或者疏漏，你需要严谨一点，有需要就使用 x-ray 协助。"*
> **⇒ 本册 = 扫描结果 + 证据 + 状态。**（与 `BACKFLOW-phyt-DNA-v2.md` 同体例：发现时登记，不靠记忆。）

## D1 · 两个 `up`（人类提过的老病，确认存在）

```
Cellrix/web/src/bin/up.rs          二进制 1.6MB（10-01）
anaphase-helix/src/bin/up.rs       二进制 5.0MB（10-09）   ← 我一直在跑的是这个
```

**⇒ 两份 launcher 实现。** **⚠️ 未判定**：哪一份是权威？另一份该退役还是该保留（例如一个只管面板、一个管全栈）？
**⇒ 需人类一句话**（这属于"两个真相"⇒ 与 ports.json 同族）。

## D2 · ★ `ports.json` 声称"唯一真源"，而代码里有 13 处端口字面量

```
ports.json 自述： "SSOT for Helix ecosystem ports. Every component's port lives HERE and nowhere else."
实测（在 Jasonmilk/ 下扫描 Cellrix/web/src）：
  Cellrix/web/src/bin/up.rs:1011  60052
  Cellrix/web/src/bin/up.rs:569   50051
  Cellrix/web/src/bin/up.rs:582   50051
  Cellrix/web/src/config.rs:17    60053
  Cellrix/web/src/config.rs:9     50061
  Cellrix/web/src/main.rs:34      50050
  Cellrix/web/src/main.rs:111     50050
  Cellrix/web/src/main.rs:206     60052
  Cellrix/web/src/main.rs:213     60052
  Cellrix/web/src/main.rs:296     60052
  Cellrix/web/src/main.rs:299     60053
  Cellrix/web/src/routes.rs:304   60053
  …（共 13 处）
```

**⇒ **SSOT 与代码直接矛盾**。** 本会话已修过其中一处（`up.rs` 的 `WEB_PORT_DEFAULT: 8080 → 50050`）——
**⇒ 说明这条路确实会漂。** **⚠️ 未做**：把这 13 处改为从 SSOT 读（或至少加一条闸门：代码里的端口字面量必须在 SSOT 里）。

**⚠️ 我的失误记录**：第二次扫描我从 `anaphase-helix/` 里跑、路径写成 `Cellrix/web/src`
⇒ 路径错 ⇒ **空结果**。**⇒ 空结果不能当作"没有重复"**（第一次的 13 处是在正确目录下量的）。

## D3 · 「verdict」一族：**5 处定义**，其中**两个同名不同物**

| 位置 | 名字 | 是什么 |
|---|---|---|
| `ledger/mod.rs:14` | `VerdictStatus` | **账本裁定**（Met/Unmet） |
| `run_cycle/verdict.rs:64` | `PeriodVerdict` | **周期结束**（HTTP 体那个） |
| `run_cycle/verdict.rs:24` | `EndReason` | 结束原因（Completed/Impasse/UndefinedTransition/CycleCapExhausted） |
| `security.rs:58` | **`GateVerdict`** | **安全闸门的决定**（策略层：Pass/Reject/HitlRequired/HardOverride） |
| `run_cycle/safety_gate.rs:52` | **`GateVerdict`** | **工具执行前两道检查的结果**（Cleared/Refused(TransitionCondition)） |

**⇒ 两件事：**
- **一物两名**：`LedgerRecord::Verdict` 与 `PeriodVerdict` 都叫"verdict"却指不同的量
  ⇒ **查一条失败时必须先问"哪个 verdict"**（本会话已实际卡过此处）
- **★ 一名两物**（本次新发现）：**同 crate 内两个 `GateVerdict`**，语义完全不同
  （一个是安全策略决定，一个是执行前检查结果）⇒ **读者会误读**；Rust 模块系统能区分，**人会混淆**

**⚠️ 未做**：改名（属 `decisions/` 之外的代码重构）—— **建议：把 `safety_gate` 那个改为 `ToolGateOutcome` 之类**，
但**这是重构决定，不擅动**。

## D4 · CI 第 3 步**复制**了引擎的闸门发现逻辑（已修一半）

```
tools/validate.sh 的 gates()            ← 引擎的发现逻辑（我已加 ADR-*.md 限制）
.github/workflows/phyt.yml step 3       ← ★ 自己又写了一遍 for id in $(ls decisions/*.md …)
```

**⇒ "第二份清单"（A5）**：我修 `gates()` 时，CI 那份**不经过它**。
**⚠️ 未做**：让 CI 第 3 步直接用 `--probe-all`（那就只剩一份发现逻辑）。
**⚠️ 但注意**：`--probe-all` 会**跳过**无夹具的闸门 ⇒ 需要先决定"无夹具算不算失败"（P11 说：算 `unproven`）。

## D5 · 编排有两份（本会话早些发现，登记以免遗忘）

```
pipeline::run()                       六阶段
run_cycle + Reflection                活路径（自己调 execute_calls/record_evidence）
```

**⇒ 查"裁定为何没写"时，读 `pipeline/mod.rs` 会读错文件**（本会话实证：连猜四次全错）。

## 优先级（我的判断）

| # | 事项 | 为什么 |
|---|---|---|
| **1** | **D2**：端口 13 处 ⇒ 加一条闸门（代码里的端口必须在 SSOT 里）| **机械可查、可红**、且 SSOT 已被直接矛盾；本会话已修过一处 ⇒ 证明会漂 |
| 2 | **D4**：CI 用 `--probe-all`（去掉第二份发现逻辑） | 同一类；且顺手决定"无夹具"的语义 |
| 3 | **D1 / D3**：`up` 与 `GateVerdict` | **需人类裁决**（谁的权威 / 要不要改名） |
| 4 | **D5**：编排合一 | 大重构，且**不紧急**（已有记录，查错时先看它即可） |
