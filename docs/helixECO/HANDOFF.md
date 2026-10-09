# HANDOFF · 入口（**指针，不是容器**；≤40 行 —— 对齐 RNA.md §三 的 INDEX 段）

> **一句话入口**：*"按 `RNA.md` 的四段漏斗走一遍"* —— 它加载什么，由**当前任务**决定，不由本文件决定。
> **为什么不在这里堆内容**（人类 2026-10-09 的裁定：*"应该按需获取、按需加载，而不是一上来就那么上头；
> 这不优雅，也不够极致复用"*）：本文件曾有五节，其中**四节与既有源重复**（见下表）。
> **⇒ 重复的清单会漂，而且会把注意力一次打满。** 本文件只留**不可导出**的那一点。

## 状态是**导出的**，不是**存的**（物理事实优先：能量出来的不存）

```bash
# ① 现在在哪 —— 量出来，不抄在本文件里（抄了就会腐）
for r in anaphase-helix helix-mind Cellrix FlowModus Tuck; do printf '%-16s %s\n' "$r" "$(git -C $r rev-parse --short HEAD)"; done
git -C phyt-DNA rev-parse --short HEAD

# ② 等你 —— 唯一真源：KNOWN_ISSUES.md 的「为什么还没修」列里标【待裁决】的行
grep -n '待裁决' helix-mind/docs/helixECO/KNOWN_ISSUES.md

# ③ 已有什么（别重复发明）—— 引擎生成的闸门清单
cd phyt-DNA && bash tools/validate.sh --index

# ④ 怎么验 —— 三条命令见 phyt-DNA/RNA.md 的招牌块（probe-all · --index · ci-local）
```

## **唯一**留在这里的（不可导出）：下一件

| | |
|---|---|
| **下一件（agent 建议，人可推翻）** | **K16 —— 给「做」装门**（想/说/做）。前置于它：**先读 `D8-PLAN.md` 的三个坑**（跨三处同时改 / 装门=改行为 / 门是 fail-closed ⇒ "Tuck 挂了 = Helix 不思考"） |
| **它的可执行起点** | `D8-PLAN.md` 的 **M0 已完成**（基线 `gate=none` 已确认）⇒ 下一步是 **M1**（Tuck 侧 `security/gate`，默认观察态；**需人类批准**） |
| **别做的事** | 不要新建状态文件 · 不要把 ②③④ 抄进来 · 加闸门前先跑 `--index` |

**更新规则**：只有「下一件」需要人/agent 写；其余三节**每次现量**（量出来的不会腐）。
