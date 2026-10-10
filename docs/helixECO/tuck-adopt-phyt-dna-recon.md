# Tuck 接入 phyt-DNA —— **散步（只读侦察）结论**

> 人类 2026-10-09：*"tuck 好像还是干净的，没有 phyt-DNA，可以考虑接入？！"*
> **⇒ 本文件只做一件事：量清"接入"的规模与形状。**（先查后写）

## 一、Tuck 现状（实测）

| 项 | 现状 |
|---|---|
| ADR | **6 份**，在 `docs/decisions/`，头部是**散文**（`- **状态**: Accepted（…）`） |
| **闸门三要素** | `hard: true` **0** · `applies-to` **0** · `check: \|` **0** ⇒ **零闸门**（phyt-DNA 格式） |
| `tools/` | 有（`verify.sh` —— 一条命令自证，本次会话加的） |
| `fixtures/` · `ledger/` · 根 `decisions/` | **都没有**（ADR 在 `docs/decisions/`） |
| `docs/spec/` | 有（5 项） |

## 二、★ 但"没有闸门能力"是**错的** —— 能力早就在

```
crates/tuck-gateway/src/access.rs（7057 字节）
  "One table, two effects"（Allow/Deny 同表 + `effect` 字段）
  pub struct AccessTable · pub fn allowed(&self) -> bool { self.observe_only || self.effect == Effect::Allow }
  ⇒ ★★ `observe_only` 【存在】（K21 那轮我据 ADR-0005 的 ⬜ 推过反结论 —— **此处更正**）
tests: tuck-gateway 单独 51 + 8 + 22 passed（含 `observe_only_records_without_enforcing`）
docs/PLAN.md:100 的验收数字：单测 22 + capability 6 + notify 3 · 端到端 8（含"观察模式不拦/观察模式仍上报"）·
  **变异五组均被捕获 ⇒ 非空转** · 全量 --all-features 406 passed
```

**⇒ 结论：Tuck 缺的**不是安全能力**，是**统一记账与统一闸门格式**。**

## 三、★ 顺手撞到的一处"同一事实两个状态"（与 phyt-DNA 的 F9 判例同族）

```
docs/decisions/ADR-0005 的 T1–T8 表：【全部 ⬜（未做）】
docs/PLAN.md:96            ："H-1..H-8 准入闸门全部交付" ✅，且 :100 给了详细验收数字
实测：access.rs 存在 · observe_only 存在 · 测试通过 ⇒ **PLAN 是对的，ADR 的表没更新**
```

**⇒ 可推广的判例（F9 那次方向相反，但同一条通则）：**

| 判例 | 两侧 | 以谁为准 |
|---|---|---|
| **F9**（FlowModus） | PLAN ⏳ vs GROWTH ✅（**有验收数字**） | **GROWTH** |
| **本处**（Tuck） | PLAN ✅（**有验收数字**） vs ADR T1–T8 ⬜ | **PLAN** |
| **通则** | —— | **同一事实两个状态时，以带验收数字/证据的那一侧为准** |

**⚠️ 而 ADR-0005 的 T1–T8 表**该更新**（它是权威卷 ⇒ 属"改权威卷" ⇒ 需人类授权）。**

## 四、所以"接入 phyt-DNA"的实际形状（三档，由小到大）

| 档 | 内容 | 规模 | 需授权？ |
|---|---|---|---|
| **① 只接"账"** | 加 `ledger/`（append-only）+ 把 `verify.sh` 的结果写账 ⇒ **先有"谁在什么时候验过"** | **小** | 否（加法） |
| **② 接"闸门格式"** | 把 Tuck **已有的**判据（access 表 · observe_only · 变异五组）**写成 phyt-DNA 格式的闸门**（`hard`/`applies-to`/`check`/`redtest` + `fixtures/<id>/{inject,counter}.sh`）⇒ 那时 `validate.sh --probe-all` 才**有东西可跑** | **中**（要动 `docs/decisions/` ⇒ **改权威卷**） | **是** |
| **③ 全量接入** | 连 `check-baseline` / `docs/INDEX` / `RNA.md` 一套 | 大 | 是 |

**⇒ 建议顺序：① 先做（零风险、立刻有账）→ 得人类授权后再 ②。**

## 五、本条**未做**（明说）

- **未改 Tuck 任何文件**（散步而已）
- **未接入 phyt-DNA**（要等"哪一档"的决定；②③ 都动权威卷）
- **未更新 ADR-0005 的 T1–T8 表**（那是改权威卷，需授权）
