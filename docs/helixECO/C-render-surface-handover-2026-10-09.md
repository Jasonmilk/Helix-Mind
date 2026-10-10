# C 渲染面 —— 交接记录（起点已钉死，未动手）

> 2026-10-09 ｜ 数据面已交付（`anaphase:adb3df9`）；**渲染面本文件只记录起点，未改一行 Cellrix**。

## 一、先说结论：改哪个文件

**活的 DAG 侧栏是 `Cellrix:web/assets/panel_tree.js`，不是 `session_list.js`。**

依据（实测）：

```js
// web/assets/session_list.js:84-94
if (window.CxPanelTree && window.CxPanelTree.mountSidebar) {
  window.CxPanelTree.mountSidebar(periods, {
    fetchRows: ...,
    onRows: function (box, rows) { window.CxPanelTree.renderRows(box, rows); },
```

⇒ `session_list.js:196-240` 那段行 HTML 构造（`'<div class="nm">' + … + preview + reply + flat`）
**只是回退路径**。往那里加投影 = **改了不生效**。

## 二、为什么这条要单独写下来

**这是第 7 条指标陷阱（测错对象）在实现层的同一形状**：
「断言落在错误的层」的兄弟是「**改动落在错误的文件**」。两者都产出一个
**看起来做了、实际没接上**的结果 —— 而且都比"没做"更难发现，因为代码 diff 是存在的。

⇒ 规矩：**动手前先确认真实调用链**（这里是 `mountSidebar` → `renderRows`），
而不是先找"看起来该改的那段代码"。

## 三、好消息：寻址约定**已经存在**，不必新造

`panel_tree.js` 本来就用同一套可寻址约定（人类 2026-10-09 的追加要求：**碳硅同构**）：

| 现有约定 | 位置 |
|---|---|
| `role="tree"` | `panel_tree.js:131` |
| `data-path` | `:148` |
| `data-mode-kind` | `:153` |
| `data-count` / `data-error` | `:165-177` |
| 行身份 | `session_list.js:346` `data-job` = `period_id` |

⇒ 按**同一约定**加三块即可（**不引入新机制**）：

| 要画的 | 建议的寻址 |
|---|---|
| 收敛状态 | `data-role="converge-status"` + `data-status`（`None` ⇒ `derived`） |
| 沉淀区 gist | `data-role="gist"` |
| 被驳回支线（**整本流水账**） | `<ul data-role="rejection-ledger" data-current="rejected|revoked">`，每行 `<li data-action data-when data-who>` |

`data-current` 由 `rejected`（派生自日志最后一行）决定；**整本 `rejection_log` 必须逐行画出** ——
反悔不是擦除，**面板不得替用户把历史擦了**。

## 四、验证路径（不许跳过）

`web/tests/all_views_test.js` 已有 jsdom 夹具（同族：`walk_state_test.js` 开头
`const REQUIRES = 'jsdom'`）。渲染断言应加在那里：
**逐元素断言可寻址** —— 人类看到几块，AI 就能指到几块
（**不允许"人类可见但 AI 不可寻址"**）。

⇒ 断言必须落在**渲染输出**上（不是落在"我写的那个函数"上）—— 第 7 条陷阱。

## 五、本轮边界（人类 2026-10-09）

不引入视觉体系 · **不动 `base.html` 外壳** · 不引入容器与构建体系 ·
沉淀区如实渲染（**不许为了好看假装完整**）。
