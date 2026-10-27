# HANDOFF · 入口（**指针，不是容器**；≤40 行 —— 对齐 RNA.md §三 的 INDEX 段）

> **一句话入口**：*"按 `RNA.md` 的四段漏斗走一遍"* —— 它加载什么，由**当前任务**决定，不由本文件决定。
> **为什么不在这里堆内容**（人类 2026-10-09 的裁定：*"应该按需获取、按需加载，而不是一上来就那么上头；
> 这不优雅，也不够极致复用"*）：本文件曾有五节，其中**四节与既有源重复**（见下表）。
> **⇒ 重复的清单会漂，而且会把注意力一次打满。** 本文件只留**不可导出**的那一点。

## 状态是**导出的**，不是**存的**（物理事实优先：能量出来的不存）

```bash
# ① 现在在哪 —— 量出来，不抄在本文件里（抄了就会腐）
for r in anaphase-helix helix-mind Cellrix FlowModus Tuck; do printf '%-16s %s\n' "$r" "$(git -C $r rev-parse --short HEAD)"; done
git -C phyt-DNA rev-parse --short HEAD

# ② 等你 —— 唯一真源：../register/KNOWN_ISSUES.md 的「为什么还没修」列里标【待裁决】的行
grep -n '待裁决' helix-mind/docs/helixECO/register/KNOWN_ISSUES.md

# ③ 已有什么（别重复发明）—— ①引擎生成的闸门清单 ②【判据与决定的正文】在各仓 docs/decisions/ ③【诊断法】docs/helixECO/canon/DIAGNOSIS-METHODS.md
#    （anaphase 最新两条：ADR-0050 快照 golden 与判据诚实 · ADR-0051 被 drop 的 shutdown 发送端；索引见各仓 docs/decisions/README.md）
cd phyt-DNA && bash tools/validate.sh --index

# ④ 怎么验 —— 三条命令见 phyt-DNA/RNA.md 的招牌块（probe-all · --index · ci-local）
```

## **唯一**留在这里的（不可导出）：下一件 = 分批计划

> 人类 2026-10-09：*"一次性发现这么多问题，别急！修不怕，就怕修完这边、那边坏了！
> 需要做好计划！一步一步向确定性迈进！"*
> **⇒ 排序原则：先做【只加判据/测试】（零行为风险），再做【注释/清理】，最后才【改行为】。**
> **⇒ 铁律：每一批做完 → 跑全部自证 → 才进下一批。**（自证入口：phyt-DNA `ci-local.sh` · Cellrix `run_all.js` · Tuck `verify.sh`）

| 批 | 内容 | 类型 | 风险 | 判据（能红） |
|---|---|---|---|---|
| **1** | **K22 的"危险动作形状"判据**（`rm -rf` / force push / **权威卷路径** ⇒ pre 时刻拦，因为实测它对危险是 fail-open） | 只加判据 | **零行为风险** | 喂 `rm -rf` ⇒ 必须 exit 2 |
| **1** | **G6-②**：`examples/fixtures/inject.sh` 加一次行使 · **G6-③**：`appeals` 账加守卫 | 只加判据/测试 | 零 | 回归判据各一条 |
| | ✅ **G6-② 已完成（2026-10-27）**：`tools/ci-local.sh` 新增 **step 5b**（真跑示范 + 判行数**越过** ADR 自己写的阈值，不写死 150）｜收据：正样本 `OK (真跑一次：151 行注入 ⇒ 1 → 152 行 > 阈值 150)` · 注入①`seq 1 151→100` ⇒ `★ RED …未越过阈值 150` · 注入②示范改回写死路径 ⇒ `★ RED 示范没用「${F}」` · 注入③文件不存在 ⇒ `★ RED 示范文件不存在` ｜步数 11 → **12** |
| | ✅ **G6-③ 本就存在（2026-10-27 核实）**：`phyt-DNA/decisions/ADR-20261009-appeals-must-be-parseable.md`（`hard: true` · `probe: yes` · `--index` = `yes \| yes`）｜单门 probe 三形实测全绿（基线綠 / 注入后 `RED — 检出: 申诉账不可解析…第 1 行不是合法 JSON` / 反例守住）⇒ 无需新建 |
| **1** | **K18**：ci144 `vendored` 一致性判据（不作重构） | 只加判据 | 零 | 改一处字段 ⇒ 必红 |
| | ✅ **K18 已完成（2026-10-27）**：新判据 `anaphase-helix/tests/ci144_vendored_consistency.rs`（逐行状态机抽 `pub struct`/`pub enum` 形状，逐个与 `Cellrix/protocol/src` 对齐；**顺序敏感**——MessagePack 里 struct 按声明序编数组）｜四格收据：一致 ⇒ `CONSISTENT: 14 个类型逐字段一致` rc=0 · 上游多一字段 ⇒ rc=101 `CapabilityManifest：成员不同 … 只在上游: ["color_tag"]` · 上游互挨两个字段次序 ⇒ rc=101 `成员相同但【顺序】不同 … 顺序也是契约` · 上游不可得 ⇒ `NEEDS-INPUT` + pass，`CELLRIX_REQUIRED=1` ⇒ 具名红 |
| **2** | **K20 低成本半**：`pipeline::run()` 加一行指向活路径的注释 · **K21**：Tuck 存量 clippy 清理 | 注释/清理 | 低 | 注释不等于判据 ⇒ **K21 要真删才红** |
| | ✅ **K20 低成本半 本就存在（2026-10-27 核实）**：`anaphase-helix/src/pipeline/mod.rs:376-388` 的注释块（2026-10-09 写）已指向活路径，且**只写符号名不写行号**（防自己先腐）｜锚点实测仍有效：`fn execute_structured` 1 · `fn arm_reflection` 1 · `HelixState::Reflection` 13 · `self.arm_reflection()` 2（`grep -rn … src/ tests/` 计数）⇒ 无需新写，只补收据。
| | ✅ **K21 已完成（2026-10-27，`Tuck:71feaec`）**：`cargo clippy --all-targets --all-features` 诊断站点 **27 → 0**（rc=0），16 个文件全是等价变换；**真删**而不是 allow 掉 | 收据：改前 27 站点 / 改后 0 ｜ 回归 `cargo test --all-features --no-fail-fast` ⇒ passed=431 failed=0 ignored=8（13 suite），**与 git stash 取的基线逐数相同** ｜ `bash tools/verify.sh` ⇒ rc=0「全绿（clippy 存量警告 0 条）」｜唯一非纯风格的一条：`file_store::save()` 的 MutexGuard 原本跨 `.await`（已收进块作用域）
| **3** | **K16 给「做」装门**（M1 Tuck 端点 → M2 装配观察态 → M3 翻转 → M4 判据） | **改行为** | **高** | 见 `../observations/2026-10-09/D8-PLAN.md`（M2/M3 必须分开；反证=停 Tuck 必须具名拒绝） |
| **3** | **K17 并发**：`build_agent` 可复用部分按需缓存 · 无界→有界+背压 · 并发上限具名 | **改行为** | 中 | 并发 N ⇒ 见"上限生效"的**具名**证据 |
| **4** | **K19**：`verdict` / `GateVerdict` 改名 | 重构 | 中 | 纯改名 ⇒ 全量自证必须仍绿 |

**❌ 别做的事**：不要为了"顺手"把批 1 与批 3 合并 —— **那正是"修完这边、那边坏了"的来源**。
**❌ 不新建状态文件**：②③④ 都现量（见上）；**加闸门/判据前先跑 `bash tools/validate.sh --index`**。

**更新规则**：只有本节的"下一件/批次"需要人写；其余三节每次现量（量出来的不会腐）。
