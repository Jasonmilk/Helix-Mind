
---

## 六、★ 已落地（`phyt-DNA@` v2 本轮推送）：G2 第四圈可机械检查

**新增三件：**

```
ledger/README.md                          → kind: replay（gate_id = 被重放的那条结晶 id）
decisions/ADR-20261009-lesson-must-be-replayed.md  → 闸门（窗口 N 来自 ADR 字段 replay-window:）
fixtures/ADR-20261009-lesson-must-be-replayed/{inject.sh,counter.sh}
```

**实测**（`--probe-all`）：
```
未被重放的结晶: ADR-20261009-PLAN-must-exist（最近 10 条 kind:replay 里都没有它们的 gate_id）
反例守住 — 内容未变时不红 ✅
7 个闸门 全部 RED + 全部反例守住 ⇒ 7/7 alive
```

### ★ 夹具设计上的一个要点（值得回流）

**本仓此刻 `kind:replay` 为零 ⇒ 该闸门**本来就红**。**
**⇒ 若夹具只是"让世界保持违规"，它就**什么都没证明**（缺陷没越过阈值 —— 这正是 `fixtures/README.md` 的铁律 1）。**
**⇒ 所以 `inject.sh` 先给**每条**结晶写一行 `replay`、**唯独漏掉一条** ⇒ 红**只可能因为它**。**
**⇒ 实测红时具名 `ADR-20261009-PLAN-must-exist` —— 正是被漏的那条 ⇒ 因果被隔离。**

**⇒ 通则（建议入册）**：**当闸门的对象是"机制缺席"（而不是"文件被改坏"）时，
夹具必须**先满足所有其他条件、只留一个缺陷**，否则罚不到点子上。**

### ★ 而我在写它的时候立刻踩中了自己的第 32 条

`echo "未被重放的结晶:$miss（…）"` ⇒ **`（` 被 bash 吞进变量名** ⇒ 输出乱码、`$miss` 为空。
**⇒ 已改 `${miss}`。**（第 32 条在**同一个会话里**就复现了一次 —— 它确实是真的。）
