# 可测性普查：从脚本位置推导 root 的判据（2026-11-05）

> 起因：`check-adr-heads.sh` 的「编号与文件名不一致」因 `sed \s` 在 BSD 方言下恒不匹配，
> **本地从未生效过**——而这只有在给它装上 `--root` 能喂夹具之后才被发现。
> 推论：**不可注入 ⇒ 不可测 ⇒ 连"它自己是死的"都看不见。**

## 全仓命中（12 个脚本推导 root）

| 脚本 | 可注入性 | 备注 |
|---|---|---|
| `helix-mind/tools/check-adr-heads.sh` | **--root（本轮新增）** | 形1/5/6 已验证 |
| `helix-mind/tools/branch-map.sh` | none | 被 CI 直接调用 |
| `helix-mind/tools/check-report-head.sh` | none | 被 CI 直接调用 |
| `helix-mind/tools/todo-ready.sh` | env | — |
| `helix-mind/tools/convergence-screen.sh` | n/a | 顶层入口，root 由自身位置定是**刻意**的 |
| `phyt-DNA/tools/check-pool.sh` | positional + env | 已有注入点 |
| `phyt-DNA/tools/ci-matrix.sh` | env | — |
| `phyt-DNA/tools/deprecate.sh` | --root | 已有 |
| `phyt-DNA/tools/meter.sh` | env | — |
| `phyt-DNA/tools/regex-census.sh` | none | 已知 bug A（`/tmp` 未复现） |
| `phyt-DNA/tools/frozen-body.sh` | none | — |
| `phyt-DNA/tools/deprecated_skip_test.sh` | env | — |

## 结论（不写成计划，只写读数）

```
12 命中：3 个已有 --root（含本轮 1 个）· 3 个 positional/env · 1 个刻意不可注入（顶层入口）
        ⇒ **5 个既无参数也无 env**：branch-map / check-report-head / regex-census / frozen-body（+1 待核）
```

这 5 个的"改坏必红"目前**只能靠本仓恰好坏掉来验证**——与 `check-adr-heads` 修好前同形。
其中 `branch-map.sh` 与 `check-report-head.sh` 被 CI 直接调用（失败会红在 CI），
`regex-census.sh` 已登记 bug A（口径待复现）。

## 边界

本普查只报"有没有注入点"，**不报**"没注入点就一定漏"——顶层的 `convergence-screen.sh`
由自身位置定 root 是刻意设计（它是入口，不是被判对象）。判据是：**被判对象能否被指向别处**。
