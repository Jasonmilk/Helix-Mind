# 第三个根因：**默认续接**（人类真实使用反馈，已结构性定位）

> 2026-10-09 ｜ 来源：人类**第一次真实使用面板**。优先级高于 model 腿与闸门。

## 一、链（逐跳，全部有出处）

| 跳 | 位置 | 事实 |
|---|---|---|
| 1 | `Cellrix:web/assets/panel_tree.js` `mountSidebar` | **打开面板时默认选中最新一段**：`if (!opts.selected && periods.length && periods[0].period_id) { var want = periods[0].period_id; opts = {...opts, selected: want}; }` |
| 2 | `panel_tree.js` `selectOne` → `Cx.setNav` | 该选中写进 `nav`（`chat.js` 注释自述：*"written by the sidebar"*） |
| 3 | `Cellrix:web/assets/chat.js` `sendChat` | `body: JSON.stringify({ message: text, job_id: job })`，`job` 取自 `nav.meta.job_id` |
| 4 | `anaphase:src/main.rs` `/v1/chat` | `job_id` ⇒ `resolve_one` ⇒ `context.resume_period = Some(…)` |
| 5 | `anaphase:src/run_cycle/reasoning.rs:158` | `let leaf = self.context.resume_period.as_deref()?;` ⇒ 于是**注入骨架** |

**注入侧是对的**（`resume_period` 为 `None` 时不注入任何东西）；
**"强制续接"来自第 1 跳** —— 你没选任何会话，**面板替你选了**。

## 二、缺口（人类的判断，我同意）

这一整场（骨架 12 行 / gist / 驳回 / 撤销 / 流水账）做的全是**注入侧** ——
回答"**续接时给 AI 看什么**"。

**而"要不要续接"这条路，从来没人做过。**
我们有 `!settle` / `!reject` / `!revoke`，**唯独没有"从空开始"** —— 而它恰恰是日常最常用的动作。

> 与既有纪律直接冲突：**"是否继承上一段"是语义选择，不是架构开关 —— 不该由默认值替你决定。**

## 三、判据（人类给的，不需要先知道根因就能测）

> **不选中任何会话 → 开一轮新对话 → 注入中不得出现任何历史轮次。**
> **变异**：把默认续接去掉 ⇒ 应**仍**不注入（防止它是靠别处补上的）。

**可直接执行**（我下一轮的第一件事）：

```
① 起栈（up --restart，面板现在在 SSOT :50050）
② 造 2 段经历（有历史）
③ 打开面板、**不点任何卡**、直接发一句全新的话
④ 读该轮的 prompt：不得含任何历史轮次（无骨架段、无既有 period_id）
⑤ 变异：把 panel_tree.js 的默认 selected 去掉 ⇒ 重跑 ⇒ 仍不得注入
```

## 四、我**没做**的事（明说）

- **未测**该判据（预算用尽）。
- **未修**默认选中（那是产品决策：默认选中本身可能是有意的"继续上一次"；缺的是**"新开一段"这个显式动作**）。
- `up --restart` 是人类正确用法；我此前用裸二进制只为受控实验。**`up` 的 `WEB_PORT_DEFAULT` 已从 8080 修到 50050**（`5748f67`）。

## 五、人类对 model 腿的更正（已收到）

> **别用 `last_model()` 换 `reasoning_mode`** —— 两个都是"配置的"；
> 判据要的是"**真正服务的**"。

⇒ `chat_model_test.js` 要的是**上游响应里报的那个模型**（`ReasoningEntry.model` 应取
`reason.last_meta().model` —— 即**实测**值），而不是把两个配置值互相对调。
**记下，接 model 腿时按这条做。**

---

## 六、判据**实测：红**（人类真实使用的数据，2026-10-09 10:22）

人类以 `up --restart` 起栈、**打开面板、没点任何卡**、发了一句全新的话。真实 store（`Jasonmilk/.helix/events`，464 period）：

```
run-49e589d3be4353cc-p006ac84ff7000000   '你好! 今天广州天气如何?'   ← 全新会话的第一句
   resume_from = run-f911e602dcba3236-p006ac1fa2100004b              ← 一个 2026-10-04 的 period（5 天前）
   context/inject: chars=800  injected_chars=672  nodes=20           ← 20 个记忆节点被注入

run-9e901b965a772d51-p006ac8500200001   '你是谁?'
   resume_from = 同一个 10-04 period
   context/inject: injected_chars=845  nodes=20
```

**⇒ 判据原话成立并变红**：
> 不选中任何会话 → 开一轮新对话 → 注入中**出现了**历史轮次（`resume_from` 指向 5 天前的 period，
> 并注入 20 个记忆节点 / 672 字符）。**两句话续接的是同一个旧 period —— 即面板默认选中的那一个。**

**对照**：同一次真实数据里，`有父 129 / 464` —— 也就是说**约 28% 的轮次带父**，
而其中相当一部分**不是人类主动续接**，是默认选中替他做的。

**变异（未测）**：把 `panel_tree.js` 的默认 `selected` 去掉 ⇒ 应**仍**不注入。

## 七、这解释了为什么"前两个修好了，痛感还在"

| 根因 | 状态 |
|---|---|
| 重号（号牌按说的话算） | ✅ B18 修了（`period_id` 唯一） |
| 截断（每轮只看 400 字） | ✅ 收敛修了（骨架 + gist） |
| **默认续接（你以为聊新的，系统接回旧的）** | ❌ **今天才浮出来，且实测为红** |

前两个修好之后，**第三个成了唯一还活着的那个** —— 而它的表现恰好就是"重复和混乱"。
