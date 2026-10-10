#!/usr/bin/env bash
# todo-ready.sh —— 【候选期租客】：只读、只打印，回答一个问题：**现在能做的是什么**。
#
# ★ 为什么它是"租客"不是"引擎"（reviewer 2026-10-10 的裁决，我认）：
#   · **痛点未实证**：这套生态每台仪器都是事故催生（sync-upstream 生于 CI 红 · redrate 生于四轮欠条 ·
#     §0 生于清单漂移）；而"它要治的病"**刚被 §0 处理完、还没有失败记录** ⇒
#     **给还没失败的制度造替代品 = 仪器厂引力回归。**
#   · ⇒ **不进 `phyt-DNA/tools/`**（无入户资格：不入 UPSTREAM.sha256、不进 manifest、不参与六仓同步）。
#   · ⇒ **只读**（不写账本、不改登记册）—— 按我们自己的**授权三级守则**：**零信用记录的工具从"顾问"做起**；
#     攒够信用再拿写权限。**工具也要过三级守则。**
#   · ⇒ **解冻条款**见 docs/helixECO/PROPOSAL-workitem-dag.md（两周使用日志达标 ⇒ 评审引擎化；不达标 ⇒ 删脚本、留字段）。
#
# ★ 首跑记录（2026-10-10，本工具的"首用回本"）：**第一版用 BSD sed 解析中文表格 ⇒ 打印垃圾**
#   （正是 reviewer 第二刀点名的坑）⇒ 已改 python 解析（编码安全）。**这条负面证据支持"暂不引擎化"。**
set -uo pipefail
ISSUES="${PHYT_ISSUES:-docs/helixECO/KNOWN_ISSUES.md}"
[ -f "$ISSUES" ] || { echo "★ 找不到登记册：$ISSUES（可用 PHYT_ISSUES=… 指定）" >&2; exit 2; }

python3 - "$ISSUES" <<'PY'
import io, re, sys
s = io.open(sys.argv[1], encoding='utf-8').read()
ready, waiting = [], []
for line in s.split('\n'):
    m = re.match(r'^\| \*\*(K[0-9]+)\*\* \|', line)
    if not m: continue
    kid = m.group(1)
    f = re.search(r'〔deps=(.*?) · prio=(.*?) · class=(.*?)(?: · blocks=(.*?))?〕', line)
    if not f: continue
    deps, prio, cls, blocks = f.group(1), f.group(2), f.group(3), (f.group(4) or '—')
    if '升级' in cls: waiting.append((kid, deps))
    elif '自治' in cls: ready.append((kid, prio, blocks, deps))
print('▸ 现在能做（class=自治）—— 真源：%s\n' % sys.argv[1])
for kid, prio, blocks, deps in sorted(ready, key=lambda r: (r[1] != '**high**', r[0])):
    print('  %-5s prio=%-9s 挡着 → %-22s deps=%s' % (kid, prio.replace('*',''), blocks, deps))
print('\n▸ 等人类一句话（class=升级）—— **不要等它们**，先做上面那些')
for kid, deps in waiting:
    note = deps if deps != '—' else '无依赖，只等人'
    print('  %-5s (%s)' % (kid, note))
print('\n▸ 注意事项（按类别，来自 DIAGNOSIS-METHODS.md）')
print('  · 改判据/加测试 ⇒ 绿必须带 run 收据（red-lifecycle ①）')
print('  · 排查 flaky   ⇒ 先问『这个仪器死掉时输出长什么样』（silent-family）')
print('  · 写闸门       ⇒ 不是所有纪律都该成为闸门（hard:true vs hard:false 要分型）')
PY
LOG="${HELIX_TODO_LOG:-$HOME/.helix/todo-ready.log}"     # ★ 使用日志写在【仓库之外】（解冻条款的证据源）
mkdir -p "$(dirname "$LOG")" 2>/dev/null && printf '%s run\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$LOG" 2>/dev/null || true
