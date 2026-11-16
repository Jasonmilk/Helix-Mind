# 只读普查：绝对行数阈值 与 指向不存在文件的模板句（2026-11-16）

> **只列清单，不修、不追。** 口径裁定见 `anaphase-helix/docs/decisions/ADR-0042-400-line-debt.md` S.7。

## ① 还有谁把绝对行数当门

```
扫描面：anaphase-helix/{ci,.github} · helix-mind/{tools,.github} · Cellrix/{tools,.github} · Tuck/tools · phyt-DNA/tools
模式  ：budget…300 / 300…NOCL|行|lines / over 300
结果  ：**0 处**（anaphase 的 300 已改为 --budget-mode report；其余仓无 300 门）
```

## ② README/CI 里指向不存在文件的模板句（按 仓根 / 工作区根 / 编排仓 三处解析）

| 仓 | 指错的引用（样例，最多 5 条） |
|---|---|
| anaphase-helix | `README.md→helix-mind/docs/helixECO/HANDOFF.md` · `…→helix-mind/docs/helixECO/KNOWN_ISSUES.md` |
| helix-mind | 无 |
| Cellrix | `README.md→ci/baseline.toml` · `→config.toml` · `→ci/pits.toml` · `→ECOSYSTEM.md` · `→helix-mind/docs/helixECO/KNOWN_ISSUES.md` |
| Tuck | `README.md→KNOWN_ISSUES.md` · `→helix-mind/docs/helixECO/HANDOFF.md` · `→…KNOWN_ISSUES.md` |
| phyt-DNA | `README.md→tools/todo-ready.sh` · `→hook_test.sh` · `→helix-mind/docs/helixECO/DIAGNOSIS-METHODS.md` · `→…KNOWN_ISSUES.md` · `→…HANDOFF.md` |

典型成因（不修，仅归因）：
```
· handoff 真身在 docs/helixECO/handoff/HANDOFF.md（多一层 handoff/）⇒ 多处引用少一层
· Cellrix/README 里若干路径是从 phyt-DNA 骨架抄来的模板句（ci/baseline.toml 等本仓不存在）
· KNOWN_ISSUES 三仓都指向 helix-mind 的同一份，但该路径本身写错
```

## ③ 边界

```
本清单是**读数**，不是待办：不新增判据、不修引用、不追历史。
要修时按仓逐个改，且同批跑 `docgen --check` / `check-adr-heads` 验证。
```
