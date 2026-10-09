# 两处交代（人类要求）：`chat_model_test` 的红怎么没的 + `asset_parity_test` 比的是什么

## 第 0 件：**它不是被静音 —— 它的红取决于"我怎么调用总闸"**

`chat_model_test.js` **没有声明 `REQUIRES`**，而它从 `argv[2]` 或 `process.env.CELLRIX_PANEL` 取面板地址。

| 调用方式 | 它拿到的地址 | 结局 |
|---|---|---|
| `CELLRIX_PANEL=… node web/tests/run_all.js` | 有（环境继承给子进程） | **跑 ⇒ 红（5 failed / exit 1）** |
| `node web/tests/run_all.js`（不传） | 空 ⇒ `NEEDS-INPUT` | **记成 HELD ⇒ 不在红名单** |

⇒ **同一条缺陷、同一份代码，因为"调用方式"产生了两份互相矛盾的账。**
**没有静音、没有登记进 deferrals、没有移出 roster** —— 我查过三种可能，是第四种：**红不红由调用方式决定**。

**修**：补 `const REQUIRES = 'panel-http';`。现在**两种调用都给红**（实测），账稳定。

**修后全量**：`2 red, 79 proven, 6 held, **0 unregistered**`
—— 红从 1 回到 2，因为那条红**不再随调用方式隐身**。**这是诚实的方向。**

## 第 1 件：`asset_parity_test` 比的是 **mtime**（是，确认）

```js
const newest = { file:'', mtime:0 };            // 遍历 assets，取 mtimeMs 最大者
const binary = BINARIES.map(b => ({…, mtime: fs.statSync(b).mtimeMs })).sort(...)[0];
ok(!isStale(binary.mtime, newest.mtime), 'binary … vs newest asset …');
```

**它的理由是充分的**（注释 28-32）：页面由**占位符替换**组装（`boot.json` 是装配规格、`base.html` 是模板），
资产**不该**逐字出现在页面里 ⇒ 逐字节比会"**红得对、红错理由**"。

**但人类的结论成立**：**mtime 是签出的属性**（我上一轮自己的话）⇒
fresh clone 上所有文件 mtime = 签出时刻 ⇒ `binary vs newest asset` **随缘**。

**正确判据（设计已定，本轮未实现）**：
不变式是"**二进制里内嵌的页面 == 现在装配出来的页面**"，而这**可以不用 mtime**：

```
① 用既有 assembler（web/tests/assemble_page.js）**现在**装配一次 ⇒ 得到页面字节及其哈希
② 从二进制取它内嵌/服务的同一页面字节（HTTP 层，或 strings 提取）⇒ 哈希
③ 两个哈希必须相等
```

⇒ 正是人类早先引用的那招（"v1.96 HTTP 层逐字节对比 244858 B"）——
**不进浏览器、不看时间戳，只看内容。**
**这条同时落在"接闸门前必须在 fresh clone 验一次"的射程内**：它很可能就是那次验证会抓到的东西。

## 第 2 件：`code_language_test` 设为提交前固定一步

本会话**第二次**同类（英文注释里写中文词）⇒ **靠记忆改不掉，靠流程**。该测试很快。

## 第 3 件（未做）：接闸门 + fresh clone

门法不变。`agent_loop_probe_test` 保持**会红** —— 它可能就是闸门上唯一/二条红之一，**那是设计如此**。

## 第十九条（入册）

> **判据会把"曾经的决定"冻成真理。**
> 实例：5+1 条断言写死了"默认选中 = 最新一段"，而它们此前**从没跑过**。
> 产品决策一改，阻力出现在**测试**里，不在代码里。
> 规矩：改一个断言时，必须说明**旧断言为什么不再成立**；改完必须**变异**证明新的仍会红。
