
---

## 六、被委派产物**已补齐**（`Cellrix@83a5d22`）—— 归档于 §二 的"缺失"之上

```
docs/vision/architecture-v12.html        1026 行 / 91KB   ← 架构研讨图 v12
docs/vision/dag-v5.0-milestones.md       1175 行 / 75KB   ← 其标题自述 "DAG 版 v5.0（目标导向）"
docs/vision/README.md                    只作指针与来源说明（**不复制不重述** —— A5：不允许第二份清单）
```

**⇒ 落点说明**：两份产物此前在**工作区** `Cellrix update/`（未入库）。
**⇒ 因此"内容挤在一起"的真因被证实并解除了**：不是排版，是**委派无处可去**。
**⇒ 未改 `VISION.md` 一字**（权威文本不被改写是 SSOT 的前提）。

**未落**：同目录另有四个验收器（`cellrix_dag_verify.py` · `cellrix_logic_verify.py` ·
`cellrix_mutants.py` · `cellrix_mutate_artifacts.py`），对应 VISION 的 **"判据真跑"** ——
应在 `tools/` 且**实际跑通**后入库（**未跑通的不入库**）。

---

## 七、★ phyt-DNA 对齐（人类已 clone）—— 三个缺口**恰好是 Cellrix 长期手工做、反复出错的三件事**

```
模板要求                  Cellrix 实有
tools/validate.sh         ✗ 缺   ┐
tools/spec-lint.sh        ✗ 缺   │ 四个验证器脚本全缺
tools/claim-check.sh      ✗ 缺   │
tools/check-baseline.sh   ✗ 缺   ┘
decisions/                ✅ docs/decisions（24 份 ADR）
ledger/                   ✗ 缺   ← 证据账本（README.md + hits-2026.jsonl）
fixtures/                 ✗ 缺   ← 变异夹具（ADR-…/inject.sh）
```

| 缺的 | 我们一直在手工做的 | 本会话的代价 |
|---|---|---|
| **`fixtures/`（`inject.sh`）** | **每轮手工造变异**（改文件 / touch / kill 端口） | **已两次付代价**：一次把用户的栈带走、一次自己污染了对照（407=407 那回） |
| **`ledger/`（证据账本）** | 手写 `proven/red/held` 计数、手写 `CRITERIA-INVENTORY` | harness 自己打印 **"self-declared — 结构上是声明，不是证据"** |
| **`tools/validate.sh`** | 人肉跑 `node web/tests/run_all.js` | **Cellrix 无 CI**（`.github/workflows` 不存在） |

**⇒ 这不是"缺三个文件"，是"三件本该机械化的纪律还在靠人守"。**
**⇒ 而且它们与 Cellrix 自己的判据文化同源**：`fixtures` = "一条判据必须能变红"的**机械保证**；
`ledger` = "报告要能被归因"的**账本**；`validate` = "判据真跑"的**开关**。

**⇒ 迁移建议（按人类哲学：极致复用 · 单一职责 · 按需）**：
1. **先用模板原样落地三个机制**（`tools/*.sh` · `ledger/` · `fixtures/`），**跑通再加内容** ——
   不重写、不"改进"，先让它们**在 Cellrix 真的跑起来**（判据能变红才算数）
2. `fixtures` 的第一个夹具就用**我们本会话真的做过的变异**（如 `asset_parity` 的 mtime 做旧）——
   **有真实反例可挂**
3. `ledger` 与 harness 对接：把 `run_all.js` 的输出**写进 `hits-*.jsonl`**，
   替代那张**手写的** `CRITERIA-INVENTORY.md`（手写表 = 声明；生成 = 证据）
4. `validate.sh` 作为**唯一入口**（人跑与 CI 同一条命令 ⇒ 0 硬编码的第二个真相）

**⚠️ 未做**：上述四项**都未做**（预算用尽）。**且我明确不"先塞文件再补跑"** ——
未跑通的工具入库，就是本会话反复避免的"改了没验证"。
