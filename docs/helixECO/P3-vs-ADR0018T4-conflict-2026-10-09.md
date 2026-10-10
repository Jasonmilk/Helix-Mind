# 🔴 P3 与 ADR-0018 T4 冲突 —— 并更正我先前一处错误分类

> 2026-10-09 ｜ 人类：*"UI 只要看得见、可验证，美化后置；我要 Deepseek harness 那样的导航逻辑"*
> ｜ **本条是我自己撞上的：我的 P3 改动与一条成文 ADR 冲突** ｜ 已撤回一处、留一处待裁决

## 一、我做了什么、发现了什么

人类说"经历会话卡一直没正常展示"。我据此排查，发现 **`session_list.js:469` 调用
`Cx.loadWindow(periodId)`（无 scope ⇒ 血缘路径）**，于是改成 `scope='period'`。

**改后 `all_views_test` 出现 1 条 FAIL：**

```
FAIL  every period of the chain has a turn header  [1 vs 2]
```

**而该判据的上下文写着：**

```
all_views_test:237   * conversation shows the WHOLE chain, not one period.
all_views_test:321   * …the EXPECTED total for the window that was loaded
```

⇒ 它引用的是 **ADR-0018 T4（`ONE read, two projections`）** —— **"对话视图显示整条链，不是一段"**
是一条**成文的设计决定**。

**⇒ 我撤回 `session_list.js` 的改动**（一个成文 ADR 在对面，而我没有预算验证下游：
轨迹读同一个窗口、`expect.rows` 等）。

## 二、★ 但撤回后那条红**没有消失** —— 于是暴露了更早的一件事

```
撤回后：all_views_test 仍 exit=1，仍 1 条 FAIL [1 vs 2]
        session_list_test exit=0 · chain_window_test exit=0
```

**⇒ 它不是 `session_list.js` 造成的。**
**⇒ `[1 vs 2]`（headers=1，期望=2）说明正在跑的面板**已经只显示 1 段**——
那是我的 **P3 改动**（`prove_track.js:288` 改用 `scope='period'`）造成的，且已在运行中的进程里。**

**⇒ 结论：`all_views_test` 的红**自 P3 起就存在**。而它就是我先前分类为
「🟢 顺序/环境（单跑 exit=0）」的那一条。**
**⇒ 我那次"单跑 exit=0"的观察不可靠**（当时的进程/资产状态与现在不同）——
**我把一条真实后果误判成了环境噪声。**

## 三、因此当前真实状态（更正后）

| 项目 | 更正后的状态 |
|---|---|
| **P3（证轨表用"这一段"）** | ✅ 浏览器端已验证（24 段/405 行 → 1 段/15 行）**但** ⇒ ⚠️ **与 ADR-0018 T4 冲突** |
| `all_views_test` 的红 | 🔴 **由 P3 造成**（我先前误分类为环境噪声） |
| `session_list.js` | ✅ **已撤回**（无 scope ⇒ 血缘路径，与 ADR-0018 T4 一致） |

## 四、这是一个**决定**，不是清理（我不替你决定）

两条路，各有代价：

| 路 | 代价 |
|---|---|
| **A 保留 ADR-0018 T4**（对话/轨迹显示整条链） | **撤回 P3** ⇒ 面板又"混着"⇒ 人类症状回来 |
| **B 采纳"展示=这一段"** | **需要修订 ADR-0018 T4**（它是成文的），并更新 `all_views_test:237/357` 的期望来源（从"链"改为"已载入的窗口"，**注意 `:321` 已承认 `expect.periods` 可以是 1** ⇒ 改动很小） |
| **C 二者并存**（轨迹=链，对话=这一段） | 需要明确**哪个视图是哪个**，并各自有判据 —— 但这会**制造两个真相**，与 A5 冲突 |

**⇒ 我的建议：B**，理由：人类症状（"Turn 1 是几天前的旧句"）**是用户可见的错误**，
而 ADR-0018 T4 的措辞是**对窗口的描述**（"the conversation shows the whole chain"），
在**这个**问题（点一张卡看一段经历）上，它描述的是**旧行为**而非意图。
**但这是修 ADR，属人类裁决范围，我不擅自做。**

## 五、本轮**未改任何已提交的东西**（除撤回自己未提交的改动）

- `session_list.js` 改动：**已撤回**（未提交过）
- 工作区：`dirty=0`
- **未修订 ADR-0018 T4**、**未改 `all_views_test` 的期望来源** —— 等人类裁决 A/B/C
