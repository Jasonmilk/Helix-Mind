#!/usr/bin/env bash
# convergence-screen.sh — 合拢验收**一屏**（C-066/C-069）
#
# 性质：这不是新工具，是把**已有判据的读数**打成一屏。合拢的定义就是这一屏达标：
#   一屏读数达标 = 合拢完成；队列尾部项（7 环 READY 之外的事）**不阻塞**合拢。
# 已知基线：anaphase 的 K32 两条 caliber 红与 Tuck 的 0 条 forensic 红，**写在这里**，
#   否则一屏永远带红、判不了"完成"。
#
# 为什么放在 helix-mind/tools 而不是 kernel：bootstrap 是 kernel 内建命令（队列尾部），
#   而**验收工具不能排在它要验收的事件之后**（上一轮 B2b 顺延六轮的教训）。
set -uo pipefail
W="$(cd "$(dirname "$0")/../.." && pwd)"     # workspace root
HERE="$(cd "$(dirname "$0")/.." && pwd)"     # helix-mind
K="$W/thinking/phyt-DNA的任务编排思路"
fail=0
row(){ printf '  %-30s %s\n' "$1" "$2"; }

echo "═══ 合拢一屏（$(date -u +%Y-%m-%dT%H:%MZ)）════════════════════════════"
echo
echo "① 仓状态（head / dirty）"
for r in Cellrix Tuck anaphase-helix helix-mind FlowModus helix-tentacle phyt-DNA; do
  [ -d "$W/$r/.git" ] || { row "$r" "缺目录"; fail=1; continue; }
  h=$(git -C "$W/$r" rev-parse --short HEAD 2>/dev/null)
  d=$(git -C "$W/$r" status --porcelain 2>/dev/null | wc -l | tr -d ' ')
  row "$r" "${h}·dirty=${d}"
  [ "$d" = 0 ] || fail=1
done

echo
echo "② 判据（本屏实跑，非引用）"
# Tuck：KNOWN-LIVE + K32 的两条 caliber 是**已知基线**（C-066）。判法是机械的：
# 除这两条之外还有「名单外失败」⇒ 才判红。只写"已基线"而不判，就是给它豁免权。
( cd "$W/Tuck" && python3 tools/expected-red.py >/tmp/sc_er.out 2>&1 )
others=$(grep -c '名单外失败' /tmp/sc_er.out || true)
k32=$(grep -c '名单外失败：.*ci_line_budget\|名单外失败：the_line_budget\|名单外失败：the_override' /tmp/sc_er.out || true)
row "Tuck KNOWN-LIVE" "$(grep -m1 '^KNOWN-LIVE' /tmp/sc_er.out)"
row "Tuck 名单外失败" "${others} 条（其中 K32 基线 ${k32} 条）"
[ "$others" = "$k32" ] || fail=1
if ( cd "$HERE" && python3 ../anaphase-helix/ci/check_ci144_profile.py --root . --file docs/helixECO/consensus-ledger.md >/tmp/sc_lg.out 2>&1 ); then row "台账 evidence" "$(tail -1 /tmp/sc_lg.out)"; else row "台账 evidence" "★ RED"; fail=1; fi
if ( cd "$HERE" && python3 tools/check-source-language.py --selftest >/dev/null 2>&1 ); then row "source-language selftest" "OK"; else row "source-language selftest" "★ RED"; fail=1; fi
if ( cd "$K" && python3 kernel.py --selftest >/dev/null 2>&1 ); then row "kernel selftest" "OK"; else row "kernel selftest" "★ RED"; fail=1; fi
if ( cd "$K" && python3 kernel.py --overdue --triggers /tmp/sc_trig.toml >/tmp/sc_od.out 2>&1 ); then row "触发器 lint" "$(tail -1 /tmp/sc_od.out)"; else row "触发器 lint" "★ RED（见 /tmp/sc_od.out）"; fail=1; fi

echo
echo "③ 两份**不能混**的读数"
( cd "$K" && python3 kernel.py --status 2>/dev/null | grep -E "^ready=|^blocked=" | sed 's/^/  环（仓间连通性）  /' ) || true
( cd "$K" && python3 kernel.py --rings queue-rings.json --status 2>/dev/null | grep -E "^ready=|^blocked=" | sed 's/^/  队列（任务）        /' ) || true

echo
echo "④ 已知基线（不是红，也别读成绿）"
row "anaphase K32" "2 条 caliber 口径红（基线，非本轮引入）"
row "A 组 forensic" "0 条（9→0 靠修好，见 Tuck 的 expected-red）"
echo
echo "── 屏幕判定: $([ "$fail" = 0 ] && echo '达标（可声称合拢）' || echo '★ 未达标')"
exit "$fail"
