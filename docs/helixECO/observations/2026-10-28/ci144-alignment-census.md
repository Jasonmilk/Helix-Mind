# CI-144 协议族对齐普查（2026-10-28）

> 语料：`CommonIntents/` 全族 7 仓已 clone 到工作区（`commonintents/`，已在
> `tools/install-hooks.sh` 的 REPOS 名单内，I5 覆盖判据绿）。
> 本文只答三件事：**在不在栈上** · **差的是字段还是整层** · **落地顺序被什么卡住**。
> 所有读数附命令；凡未核的写未核。

## 0. 语料与复跑

| 仓 | 分支 | HEAD |
|---|---|---|
| `BIND-19` | `v2.0-alpha` | `2eaba74` |
| `CAPABILITY-13` | `main` | `19f92ee` |
| `INTENT-7` | `main` | `88e1465` |
| `INTENT-7-SECURE` | `main` | `0b1f178` |
| `PFP-xCF14` | `main` | `0875d18` |
| `SAP-xCF14` | `main` | `fad51a6` |
| `.github` | `main` | `fc8e81b` |

```bash
ls /Users/jason/Developer/Jasonmilk/commonintents
for d in commonintents/*/ ; do printf '%-22s %s\n' "$d" "$(git -C "$d" rev-parse --short HEAD)"; done
```

三条被转达的引文已逐条核对原文（**两处准确，一处需改措辞**）：

| 引文 | 原文位置 | 核对 |
|---|---|---|
| 「向 L0 控制器查询…L0 基因锁谱系哈希」 | `CAPABILITY-13/spec/CAPABILITY-13.md:51` 「Query the system's trusted **L0 Controller** (via the standardized L0 gRPC interface over local UDS)」 | ✅ 准确 |
| 「失败 ⇒ `0x05FA` 拒绝启动」 | 同文件 `:53` 「MUST immediately throw error code `0x05FA` (`INVALID_CONFIGURATION_SIGNATURE`), refuse to start」 | ✅ 准确 |
| 「`INTENT-7-SECURE` 自述不是强制组件」 | `INTENT-7-SECURE/README.md:3` 「**One possible implementation of BIND-19, not a mandatory protocol component.**」 | ✅ 准确 |

## 1. ① 先答「在不在栈上」（这条顺序是对的）

链路的线上编码，逐条实测：

| 链路 | 线上编码 | 是不是 CI-144 帧 | 证据 |
|---|---|---|---|
| anaphase ↔ Cellrix | `CIB/1.0` 握手 + 小端长度前缀 MessagePack | **是** | `anaphase-helix/src/ci144/mod.rs:1-13`（vendored `cellrix-protocol@21d13d4`，ADR-0017）· `src/main.rs:1276` `ci144::server::run_stdio` · 判据 `tests/ci144_vendored_consistency.rs`（14 个类型逐字段） |
| anaphase ↔ tentacle | **tonic gRPC / protobuf** `ExecuteToolRequest` | **否** | `anaphase-helix/src/adapters/tentacle.rs:77-87` |
| anaphase ↔ Tuck | HTTP/JSON `POST /v1/security/gate`（且该路由未挂载，见 K-133） | **否** | `anaphase-helix/src/adapters/security_gate.rs` · `Tuck/crates/tuck-gateway/src/router.rs:19` |

```bash
grep -rn 'fn execute_tool_with_labels' -A10 anaphase-helix/src --include=*.rs
grep -n 'run_stdio' anaphase-helix/src/main.rs
```

**⇒ 三点结论（其中两点修正了转达的判断）**

1. 「整条链路不在栈上」只在 **2/3** 条链成立。cockpit 链不但**在**栈上，还是生态里 CI-144 的**唯一**落点。
2. 而 cockpit 链是**进程内 stdio**（`run_stdio`，stdin/stdout），不是网络。stdio 链没有中间人，也没有长时重放窗口——**真正的重放面在 tentacle(gRPC) 与 Tuck(HTTP) 上，而那两条恰恰不在 CI-144 上**。
3. ⇒ 因此「防重放对齐 SAP-xCF14」的落地不是加字段，是**把这两条链换成 CI-144 帧（PFP + BIND-19）**＝一次传输层替换。授权档位与工作量都与此前的估法不同（见 §9）。

## 2. 最大差距不是防重放，是 §2 的整套能力授权层不存在

```bash
grep -rn 'capability_mapping\|signer_l0_hash\|0x05FA' \
  anaphase-helix/src anaphase-helix/config.toml.example Tuck/crates
# （无输出 = 0 命中）
```

规范 §1 把自己定义为三项：异步 HITL · 动态签名权限注册表 · 沙箱权限映射。生态现状：

| 规范项 | 生态现状 |
|---|---|
| §2 动态注册表（`capability_mapping.toml` + `[meta].signer_l0_hash` + `[integrity] Ed25519`） | **0 命中，完全不存在** |
| §2.2 启动期验签链（L0 gRPC over UDS → 验签 → 失败 `0x05FA` 拒启动并锁定沙箱） | **0 命中**（无 L0 客户端） |
| §3 异步 HITL 令牌 | 只有终局拒绝（见 §5） |
| §4 逻辑 scope ↔ 沙箱 capability 映射 | 存在，但是**硬编码**（`Tuck/crates/tuck-gateway/src/matrix.rs` `[Guard, Hold, Mapping]` 优先级表） |

规范 §2 开头反对的正是「把安全映射硬编码进执行器二进制」。**诚实说明**：Tuck 是独立进程，不构成对这句话的字面违反；但「改一条映射＝改代码重编译」这个事实仍然成立，而规范要的是签名的、可热更新的注册表。⇒ 差距在**层**不在**字段**。

## 3. ② 信任锚：前半句按修正，后半句自陈已成立、缺的是「确认凭证」

- 信任锚 = CAPABILITY-13 §2.1 `[integrity]` 的 Ed25519 签名 + `signer_l0_hash`（= Creator L0 Genesis Key Hash），启动期按 §2.2 校验，失败抛 `0x05FA` 并拒绝启动。`INTENT-7-SECURE` 是 BIND-19 的**可选实现之一**（其 README 自述）。
- 后半句（「显式不绑定必须是一次可审计的降级事件，不是应用层沉默放行」）**原样接受**，并已在仓里成立：`Unbound` 现在不是默认值而是**一等状态** —— `anaphase-helix/src/pipeline/mod.rs` `BindingState::Unbound` + `refusal()` 单一文案出处、`src/run_cycle/mod.rs` 执行点四态门 + `ring_execution_refusal()` → `context.last_execution_refusal`，判据 `tests/execution_gate.rs` 3 例（ADR-0054 §1.2）。⇒ 已经是**可断言的降级事件**；缺的是「谁确认的」这一步凭证，那份凭证在规范里就是 §3 的挑战-响应令牌 ⇒ 接 §5。

## 4. ③ 密钥轮换：按字段对齐；但「不必参考 GPG」这句要改

原文 §5 字段（`:123-131`）：`previous_key_hash` · `new_public_key` · `rotation_proof`（`ed25519_signature_by_previous_private_key(new_public_key_hash)`）。责任方明写是**Anaphase**（`:127` `Anaphase` 用旧公钥验 `rotation_proof`；`:129` 原子换键后立即重验注册表并记事件）。

**措辞纠正**：原文 `:121` 自己写的是「supports **GPG-style** asymmetric key rotation」——所以不能说「不必参考 GPG 风格」，正确说法是「**原文用 GPG-style 作类比，但字段是自有形状**」⇒ 按那三个字段对齐，不要按 GPG 的 web-of-trust / 公钥服务器对齐。`previous_key_hash` 在规范里是 `sha256_of_current_signing_key`，`new_public_key` 是 raw ed25519 串，`rotation_proof` 签的是 `new_public_key_hash` —— 三者形状固定，无自由度。

⇒ 落地依赖 §2（没有注册表就没有「换的那把钥匙开什么」）。所以顺序排在 §2 之后，不是「成本最低」。

## 5. ④ HITL / K23 对照（结论：不是一物两名，是两物一名）

| | 规范 CAPABILITY-13 §3 | anaphase 现状 | Tuck 现状 |
|---|---|---|---|
| 载体 | `Pending_HITL_Token`（`§3.1:82-90`） | `GateVerdict::HitlRequired(String)`（`src/security.rs:85`） | `DecisionConfig::NeedHumanConfirm`（`Tuck/crates/tuck-core/src/policy.rs:84/94/105`） |
| 字段 | `correlation_id` · `challenge_hash`(sha256 of target command+params) · `requested_permissions[]` · `timeout_ms` · `timestamp` | **只有 `String`（reason）** | 有超时配置 `:185` "Human confirmation timeout in seconds" |
| 语义 | **非阻塞**：yield 线程 → 任务转 `WAITING` → 带签名的 `Response_HITL_Token` 回推 → 验签 → 恢复 FSM → 2PC（`§3.2:99-101`） | `permits()` = false ⇒ **终局挡路** + `blocked` 账本（`src/security.rs:15`） | 决策来源（链路 B，不在请求通路） |
| 超时 | `0x05FC CONSENSUS_TIMEOUT`（`§6`） | 无 | 有配置项，无回推路径 |

**⇒ K23 的真实形状**：`NeedHumanConfirm`（Tuck）与 `HitlRequired`（anaphase）不是同一物的两个名字，而是规范 §3 的**两端**：
- `NeedHumanConfirm` = 「**决定要问人**」（触发侧，含超时）
- `HitlRequired` = 「**问到之后怎么处置**」——但被写成了**终局拒绝**，而规范要求的是一个**可恢复的 pending 态**

⇒ 因此 K16 M3/M4 的「该不该无数据翻转」在规范下问错了问题：要拆的不是「翻不翻」，而是把 `HitlRequired` 拆成 `Pending{token}`（可恢复）+ `Reject`（终局）。这与我此前在绑定层做的三态→四态拆分同型，可复用那套判据写法。

⇒ 但**拆分依赖 §2 的验签**（token 必须能验签）。所以「先做 HITL 对照」能拿到答案，**拿不到立刻落地**——落地顺序被 §2 卡住。这是我与转达顺序建议唯一的分歧点。

## 6. 禁令（接受，且现状已符合）

「不得把证书/签名校验放进每次执行前的实时路径。」现状符合：执行点门只读 `binding` 枚举（`src/run_cycle/mod.rs` 四态门），不做任何密码学运算（`tests/execution_gate.rs` 3 例）。

将来接 §2 时：**启动期验一次并缓存结论**，仅在 key rotation 时重验（§5.3 本来就如此要求）。注意这不与禁令冲突——禁令管的是**每次执行**，§2.2 管的是**每次启动**。

## 7. 与转达建议的分歧（三处，均为机械读数所出）

1. **排序**：把 HITL 对照排第一可以，但它的**落地**被 §2 验签卡住 ⇒ 真正的第一步是 §2 注册表的**立项**（不是实现，也不是 HITL 代码）。否则「先做 HITL」会得到一份无法落地的对照。
2. **「整条链路不在栈上」**：只在 2/3 条链成立；cockpit 链在栈上，且它是唯一在栈上的。
3. **「不必参考 GPG 风格」**：原文自称 GPG-style，应改措辞而非结论（见 §4）。

## 8. 明确不做

- 不在 anaphase 新造防重放机制（总纲：能对齐就不新造）。
- 不把证书/签名校验放进执行热路径。
- 不在 K32 口径、Tuck 实体级 `kind:"redact"` 形状、ECOSYSTEM §0.1 未裁前动它们。
- 不改 `commonintents/` 下任何仓（只读语料）。

## 9. 待裁（不自裁）

| 事项 | 授权档 | 说明 |
|---|---|---|
| §2 注册表**立项**（写 ADR，不实现） | 自治 | 需要园丁同意先立项再谈实现 |
| 传输层是否搬上 CI-144 帧（tentacle gRPC / Tuck HTTP → PFP+BIND-19） | **②** | 这是传输层替换，不是加字段；成本量级与「补个 Seq」完全不同 |
| K23 拆分（`HitlRequired` → `Pending{token}` + `Reject`） | **②** | 依赖 §2 验签，且动 `GateVerdict` 公开形状 |
| 密钥轮换（§5）落地 | **②** | 依赖 §2，且需要 L0 客户端 |

## 10. 未核

- 转达建议称「第 1/2/3 次复发」中前三次的结论我此前已各自实证（ADR-0004 D5 已批准落地 · D7 焊死旁路 · 跨仓引用写仓名违 108 次）。本轮**未重新复核**这三条，沿用既往收据。
- BIND-19 / PFP / SAP 三份规范的**帧级细节**本轮只核了被引用的那几行，未做全文普查；若要把 tentacle 链搬上 CI-144，需先做这三份的逐节对照（新的工作量，未开始）。
