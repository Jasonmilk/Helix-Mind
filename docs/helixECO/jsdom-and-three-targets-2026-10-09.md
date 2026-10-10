# jsdom 恢复 + 三目标结论（2026-10-09）

## 目标 1 ✅ 装 jsdom（test-only）

装成 **`web/tests/package.json`（`private: true`）+ 本地 `node_modules`**：

- **不进产品依赖**：仓库根与 `web/` 下**无** `package.json`、**无** `node_modules`；
  面板仍是 Rust + `include_str!`，没有构建体系。
- `.gitignore:72` 已有 `node_modules/` ⇒ 不会误提交。

**效果（`node web/tests/run_all.js` 实跑）**：

| | 装前 | 装后 |
|---|---|---|
| 环境行 | `jsdom=no` | **`jsdom=yes`** |
| HELD | **38** | **12** |
| proven | 44 | **65** |

⇒ **+21 套现在真的跑并证明。**

## ★ 目标 1 的真正回报：沉默掩盖着真红

装好 jsdom 后 **出现 3 条红** —— 它们此前被记为 env-missing（**体面的不跑**）：

| 套件 | 性质 | 证据 |
|---|---|---|
| `agent_loop_probe_test.js` | **真红（既存）** | `PT.retryBranch` / `PT.runLoop` / `PT.replay` **不是函数** —— 入口被声明了但从未实现（或已被移除）。这正是该套件存在的目的。 |
| `p64_env_class_test.js` | **本轮安装方式造成的红** | 它用 `NODE_PATH` 模拟"能力缺失"，断言那些套件应被 **HELD BY NAME**；而**本地 `node_modules` 不经 `NODE_PATH` 解析** ⇒ 模拟失效（`0 suite(s) held`）。**红的是模拟，不是产品。** |
| `prove_track_rows_test.js` | **未查清** | 本轮未取到其失败行。 |

> **这就是"38 套一直看不见"的真实代价**：它不是 38 份无用的文档，
> 而是 **38 个哨兵里有 3 个在值守时倒下，而没人听见**。
> 目标 2 若在此刻无脑接 CI，会立刻得到一个**永久红灯** —— 正是人类自己说的
> 「永久红灯 = 没有灯」。**必须先处理这 3 条，再接闸门。**

另有 `4 unregistered`（`fork_entry_test.js` / `session_addressable_test.js` …），
跑器把它们标为 **UNREGISTERED/BLOCKING** 并具名 —— 这条设计是对的。

## 目标 4 ✅ 结案：那条真红是**我造成的**

```
FAIL panel_tree.js: no non-English COMMENT lines  [294,295,296,297,298,299,315,316]
```

**恰好是我上一轮加进 `panel_tree.js` 的 8 行中文注释**（`Cellrix:29781d6`）。
该仓规则是**注释用英文**（UI 文案留在字符串里，允许）。已英文化 ⇒
`code_language_test.js` **exit 0**，`panel_tree.js` 回到 0 条非英文行（backlog 261 → 253）。

> 顺带证实：那条测试的**真实退出码是 1**（先前我经 `| tail` 看到的 `exit=0` 是 `tail` 的）
> —— 一次**自己差点误读**。跑器报的"1 red"是**真的**。

## 目标 2 / 3 —— **未做**（明说）

- **目标 2（最小 CI / pre-commit）**：闸门规则已定（真红阻断 / 环境缺失不阻断但具名 /
  未注册具名提示），但**现在接会把 3 条既存红变成永久红灯**。**先处理那 3 条，再接。**
- **目标 3（端口被占要出声）**：未做。实测撞到的是
  `cellrix-web` 默认 **8080 与 llama-swap 相撞**（`Cellrix:ADR-0046` 记过），
  我用 `--port 18932` 绕过 —— **它当时没有出声**。

## 人类要求推广的一条纪律（已并入 GROWTH 第 10/11 条的语境）

> **未声明依赖不会自动变绿。**
> 跑器把"用了 jsdom 却没声明 `REQUIRES`"的套件**照样按依赖计数**并打 `NOTE`（P64）
> ⇒ **声明缺失不会伪装成通过**；缺能力时记为 **HELD（具名）**，不是 PASS。
> ⇒ 另一仓若照抄这套账本，必须先照抄这一条。
