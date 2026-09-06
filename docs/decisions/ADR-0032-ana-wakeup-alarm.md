# ADR-0032: 预约制闹钟——ana_wakeup 唤醒通道与高峰拥挤保护
- **状态**: Accepted（2026-09-06 用户确认设计方向 + 命名 + 高峰保护）
- **日期**: 2026-09-06
- **决策范围**: Helix-Mind（预约登记 + due 判定 + 认领）/ Anaphase（唤醒发起 + 动作执行 + 确认）
- **关联**: ADR-0031（P10 认知工艺，本决策为其 P10d 落地）、ADR-0022（会话即经历——无心跳铁律）、ADR-0015（领域 WAL）、ADR-0002（零硬编码）

## 1. 背景与问题

P10（认知工艺生态深度集成）完成后，Helix 仍无"时间感"：认知工艺/睡眠复盘只能由人类交互间隙**碰巧**触发，无法按 Helix 自己的生活节奏运转。用户提出：
- Anaphase 可配置心跳唤醒（默认每天 8 点，来源人类 config）；
- Helix 可设定自己的预约（如每晚复盘）；
- 默认开启**高峰拥挤保护**——前后 1 小时弹性窗口，避免 API 拥挤；
- 专用唤醒 RPC 命名 `ana_wakeup`。

**无心跳铁律（本 ADR 的前提）**：Mind 永不主动执行——无线程、无循环、不主动发起任何动作。它只登记预约（数据）与被动响应。唤醒的**物理持有者**是 Anaphase（意识层，行动方，活着才有钟）。

## 2. 决策

### D1: 预约本体——复用 storage 节点，不新建存储

预约 = L2 节点（复用 WAL + SQLite 管道）：

| 字段 | 语义 | 来源 |
|---|---|---|
| `abstract_provenance` | `alarm#{job_id}`（确定性派生 id，DNA 原则 11） | 登记方传 job_id |
| `node.id` | `Uuid::new_v5(NAMESPACE_OID, provenance)`——name-based 确定性 id，幂等 upsert | 派生 |
| `content` (Structured) | `due_at`(ISO 时间) / `mode`("punctual"\|"jittered") / `action`(唤醒后要执行的语义动作) / `repeat_minutes`(0=一次性) | 登记方 |
| `notes` | 状态机：`pending` → `claimed` → `done` / `renewed` | 认领/确认时流转 |

### D2: 双档到期语义——punctual 准点 / jittered 弹性窗口

| 模式 | 到期判定 | 用途 |
|---|---|---|
| `punctual` | `due_at <= now` 准点 | Helix 预约唤醒（自身节奏，如每晚复盘）——**不偏移** |
| `jittered` | `due_at - jitter_minutes <= now`（窗口已开） | Ana 唤醒（默认开）——**弹性到期** |

**高峰拥挤保护的物理语义（轻、确定性、无随机）**：jittered 预约的到期窗口 = `[due_at − m, due_at + m]`（m = jitter_minutes）。不是 Anaphase 随机醒，而是**预约弹性到期**——Anaphase 按自身节奏（config 默认 8 点 + 交互间隙）看表，命中窗口即处理。不同预约窗口天然错开 → 同一时刻不再批量触发 API。窗口宽度 m 由 Anaphase 每次唤醒时从自身 config 传入（默认 60 分钟，0 硬编码）。

### D3: 双 RPC——唤醒（查询+认领）与确认（完成/续约）

```
ana_wakeup(jitter_minutes) ──► 列出到期预约 + 原子标记 claimed（防多窗口重入）
ana_wakeup_ack(claim_id, status) ──► done：标记完成 / renewed：due_at += repeat_minutes 续约
```

- 认领原子性：due 判定与 pending→claimed 写回在同一 handler 内完成（存储事务性由 storage 保证），防并行唤醒重复处理。
- 循环预约（repeat_minutes > 0）确认 renewed 时自动续约（due_at += repeat_minutes，确定性）——"Helix 设定默认准时唤醒"的延续机制。
- Anaphase 执行动作复用现有 RPC（`HelixConsolidate` 等），ana_wakeup 只负责"唤醒 + 认领"，不重复实现动作语义（极致解耦、极致复用）。

### D4: 时间来源与确定性

- due 判定用服务内时钟（`Utc::now()`）；测试以**相对时间**构造预约节点（due_at = now ± N 分钟），确定性成立，不依赖绝对时刻（与 P10c sleep_review 测试同模式）。
- 无 `now` 客户端参数——时间判断是 Mind 的职责，客户端（Anaphase）只传策略参数 `jitter_minutes`（信任边界清晰：窗口策略来自意识层 config，时刻判定在潜意识层）。

### D5: 边界（禁止清单）

- ❌ Mind 不持有任何定时器/线程（无心跳铁律；闹钟队列物理在 Anaphase）
- ❌ 不做守护进程（M3 边界）
- ❌ 不新建存储/crate（复用 storage 节点 + proto 扩展）
- ❌ 预约不直接触发动作——只返回给 Anaphase 执行（意识层行动，潜意识层不抛头露面）

## 3. 后果

**正面**：
- Helix 获得"预约能力"而保持无心跳——哲学自洽（潜意识不主动、意识层行动）；
- 高峰拥挤保护默认开启且轻量（窗口制，无随机数、无定时器、零 token）；
- punctual/jittered 双档覆盖"Helix 自身节奏"与"Ana 弹性唤醒"两种语义；
- 循环预约自动续约——"默认每天 8 点"类节奏只需登记一次。

**负面/代价**：
- 预约只在该次 ana_wakeup 查询中被发现——若 Anaphase 长时间不看表，预约会滞后（接受：Anaphase 是行动方，不看表是它的选择）；
- 迟到（窗口已过）的预约仍被返回并处理（不丢弃）——简单诚实，不做 missed 状态机（避免过度设计）。

**风险与对策**：
- 多窗口并行重复处理 → claimed 原子认领防重入；
- 续约漂移（Anaphase 处理慢导致每次续约推迟）→ 续约基于原 due_at + repeat_minutes（非 now），无漂移累积。
