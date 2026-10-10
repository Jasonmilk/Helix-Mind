# ✅ "经历会话卡没有正常展示" —— 修复并**在真浏览器里验证**（走真实入口：点卡）

> 2026-10-09 ｜ 人类长期反馈的症状 ｜ `Cellrix@e0615b4` ｜ 未改 ADR 正文（只在测试注释与代码注释里记录修订）

## 一、症状与根因

**症状**（人类）：*"经历会话卡没有正常展示"*。

**根因**：面板的**展示读**取的是**血缘路径**（`loadWindow` 的默认），不是这一段。
- `session_list.js:469` `Cx.loadWindow(periodId)` —— 点卡后把**整条链**载进对话视图
- （同族的另一处 `prove_track.js:288` 已在 P3 中修为 `scope='period'`）

**实测证据**：`run-f911e602dcba3236-p006ac889…` 该段自身 16 个事件，
而默认窗口载入 **24 段 / 405 个事件**，最新一轮在末尾、**"Turn 1" 是五天前的一句**。

## 二、修复（B 方案，三处）

1. `web/assets/session_list.js` —— `loadWindow(periodId, 'period')`
2. `web/tests/all_views_test.js` —— 期望来源由「链」改为「已载入的窗口」，注释同步
3. **ADR-0018 T4 不必改**：T4 的任务是 *"经历侧栏改为消费装配层 ✅ bd252ef"*，
   **它没有说"点一下要显示整条链"** —— 那句话写在**测试的注释**里（`all_views_test:237`）。
   侧栏**仍然**消费 `loadWindow`，只是 scope 不同 ⇒ **与 T4 不冲突。**
   ADR-0021 的血缘路径对**上下文加载**仍然正确 —— 那是服务端的读，本次未动。

**判据同步不弱化**：`all_views_test` 的期望**仍由本进程独立重算**
（"the app's own count would agree with itself — the shape of every false green"这条纪律保留），
只改了"**哪个窗口**"；且 `:321` 本就承认 `expect.periods` 可以是 1。

## 三、验证（两层）

**回归（四条全绿）**：
```
all_views_test        exit=0  0 FAIL     session_list_test     exit=0  36 passed
chain_window_test     exit=0  10 passed  prove_track_rows_test exit=0  20 passed
```

**★ 真浏览器 · 真实入口（点侧栏里那张卡）**：
```
cardClicked: true
periodWindowEvents: 11        ← 这一段自己的事件
turnsInChat: 1
distinctPeriodsInChat: 1      ← 修前是 24 段 / 405 事件
```

⇒ **症状消失**：点一张卡 ⇒ 视图里只有那一段。

## 四、★ 这件事证明了 phyt-DNA **有效**（人类问的"这证明 phyt-DNA 有效吗"）

**过程**：我改 `session_list.js` ⇒ `all_views_test` **立刻变红** ⇒ 我才发现"对面站着一条成文契约"。
**⇒ 在我提交之前就拦住了。** v1 无约束时，这条改动会**直接落地**，人类几天后才会发现
"会话卡还是不对"。

**⇒ 闸门第一次真的替我们省下了钱。** 而且**红得有用**：它不但拦住了，还让我**找到了正确的措辞**
（"冲突对象是一条测试注释里的旧契约，不是 T4 本身"）—— 若没有这条红，我会**继续猜**。

**⇒ 这正是人类说的**：*"错误是走路撞出来的，我们只管保持逻辑清晰，选择正确的路走，遇到问题、
分析问题、解决问题就好"*。**判据不是束缚，是把"撞"的代价从"几天后才发现"降到"提交前"。**
