#!/usr/bin/env bash
# todo-ready.sh —— 【候选期租客】：**只读的顾问**。回答："现在能做的是什么 · 该改哪一行"。
#
# ★ 为什么是租客不是引擎（reviewer 2026-10-10，我认）：
#   · **痛点未实证**：本生态每台仪器都事故催生（sync-upstream 生于 CI 红 · redrate 生于四轮欠条 · §0 生于清单漂移）；
#     而"它要治的病"**刚被 §0 处理完、还没失败记录** ⇒ **给还没失败的制度造替代品 = 仪器厂引力回归。**
#   · ⇒ **不进 phyt-DNA 引擎层**（无入户资格：不入 UPSTREAM.sha256、不进 manifest、不参与六仓同步）。
#   · ⇒ **只读顾问**（按我们自己的授权三级守则：**零信用记录的工具从"顾问"做起**；`--done` 只打印"改哪一行怎么改"）。
#
# ★ 防腐三条（reviewer 2026-10-10 二，写在代码里而不只写在文档里）：
#   ① **入户资格靠房租（使用记录），不靠长相（设计）**：两周日志不达标 ⇒ 删，没有"挺好看留下来"这条。
#   ② **严格分层**：本脚本**只是视图**，真相永远在 `KNOWN_ISSUES` ⇒
#      **若哪天发现自己在脚本里"补数据"，就是造了第六真相 ⇒ 即刻冻结。**
#   ③ **冻结期不偷跑**：两周内不进引擎/UPSTREAM、不做写回、不扩字段。
#
# ★ 解冻条款：docs/helixECO/PROPOSAL-workitem-dag.md（自 2026-10-10 起算）；使用日志：docs/helixECO/USAGE-candidate-tools.md
set -uo pipefail
# ★ 自解仓库根（2026-10-10）：**在任意 cwd 都能用** —— 否则"报告头贴读数"这条腿会在别处静默为空。
ROOT=$(cd "$(dirname "$0")/.." && pwd)
ISSUES="${PHYT_ISSUES:-$ROOT/docs/helixECO/KNOWN_ISSUES.md}"
[ -f "$ISSUES" ] || { echo "★ 找不到登记册：$ISSUES" >&2; exit 2; }

case "${1:-}" in
  --done) python3 - "$ISSUES" "${2:?用法: --done <ID>}" <<'PY'
import io, re, sys
s=io.open(sys.argv[1],encoding='utf-8').read(); kid=sys.argv[2]
for n,line in enumerate(s.split('\n'),1):
    if re.match(r'^\| \*\*%s\*\* \|' % re.escape(kid), line):
        print('▸ 完成 %s ⇒ **顾问模式（只打印"改哪一行怎么改"，不代写）**\n' % kid)
        print('  ① 文件: %s   行: %d' % (sys.argv[1], n))
        print('  ② 把它从 §1【未修】迁到 §2【已修】**并带收据**：')
        print('     · 修于 <commit>（本仓或对应仓）')
        print('     · 判据 = <哪条测试/哪条命令>，以及那次运行的读数')
        print('  ③ 若它还有字段行（deps/prio/class/blocks）⇒ 一并迁走，别留半行')
        print('\n  ⇒ 人手抄这三步；攒够信用后本工具才申请写权限（授权三级守则）。')
        break
else:
    print('★ 找不到声明行：%s' % kid); sys.exit(1)
PY
    ;;
  *)
python3 - "$ISSUES" <<'PY'
# -*- coding: utf-8 -*-
import io, re, sys
s = io.open(sys.argv[1], encoding='utf-8').read()
items = []
for line in s.split('\n'):
    m = re.match(r'^\| \*\*(K[0-9]+)\*\* \|', line)
    if not m: continue
    f = re.search(r'〔deps=(.*?) · prio=(.*?) · class=(.*?)(?: · blocks=(.*?))?〕', line)
    if not f: continue
    items.append((m.group(1), f.group(1), f.group(2), f.group(3), (f.group(4) or '—')))

# ★ prio 推导（reviewer 一①：决策已写死，这里是它的落地）：
#   **由 blocks 推导**，人不再手填 ⇒ "排队人数决定重要性"。
#   高价值目标（DSH 对话界面 / 证轨）= 关键路径 ⇒ 挡着它的 = high；挡着其它 = medium；不挡任何 = low。
def derived(blocks):
    if blocks == '—' or not blocks.strip(): return 'low'
    if 'DSH' in blocks or '证轨' in blocks: return 'high'
    return 'medium'

ready, waiting = [], []
# ★ 三分桶（2026-10-10 修）：**"人类冻结"不许被列成"能做"** —— 那是指南针指反。
ready, waiting, frozen = [], [], []
for kid, deps, decl, cls, blocks in items:
    if '禁行' in cls: frozen.append((kid,'禁行')); continue
    if '冻结' in cls: frozen.append((kid,'人类冻结')); continue
    if '升级' in cls: waiting.append((kid, deps, cls)); continue
    ready.append((kid, derived(blocks), decl, blocks, deps))
ready.sort(key=lambda r: ({'high':0,'medium':1,'low':2}[r[1]], r[0]))

print('▸ 现在能做（class=自治）—— prio **由 blocks 推导**（人不再手填）· 真源：%s\n' % sys.argv[1])
for kid, dp, decl, blocks, deps in ready:
    warn = ''
    if decl not in ('—','**high**','high') and decl.replace('*','') != dp:
        warn = '  ⚠️ 人手填 prio=%s 与推导 %s 矛盾 ⇒ **矛盾即信息，要解释不要覆盖**' % (decl.replace('*',''), dp)
    print('  %-5s 推导=%-6s 挡着 → %-22s deps=%s%s' % (kid, dp, blocks, deps, warn))
print('\n▸ 等人类一句话（class=升级）—— **不要等它们**，先做上面那些')
for kid, deps, cls in waiting:
    print('  %-5s （%s）' % (kid, deps if deps != '—' else '无依赖，只等人'))
print('\n▸ ⛔ 不许动（人类冻结 / 禁行）—— **不在你的清单里**')
for kid, why in frozen: print('  %-5s （%s）' % (kid, why))
print('\n▸ 注意事项（按类别，来自 DIAGNOSIS-METHODS.md）')
print('  · 改判据/加测试 ⇒ 绿必须带 run 收据（red-lifecycle ①）')
print('  · 排查 flaky   ⇒ 先问『这个仪器死掉时输出长什么样』（silent-family）')
print('  · 写闸门       ⇒ 不是所有纪律都该成为闸门（hard:true vs hard:false 要分型）')
print('\n▸ 顾问模式：完成某条 ⇒ `bash tools/todo-ready.sh --done <ID>` **只打印"改哪一行怎么改"**（不代写）')
PY
    ;;
esac
LOG="${HELIX_TODO_LOG:-$HOME/.helix/todo-ready.log}"
mkdir -p "$(dirname "$LOG")" 2>/dev/null && printf '%s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "${1:---ready}" >> "$LOG" 2>/dev/null || true
