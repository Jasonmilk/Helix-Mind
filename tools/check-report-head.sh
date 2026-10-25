#!/usr/bin/env bash
# check-report-head.sh —— 【报告头完整性】闸门（reviewer ③ 2026-10-11 · 焊死今天那个洞）
#
# ★ 为什么存在：报告头生成器被两处 `${var}（`（变量紧跟中文）**静默打死在 31 行**，
#   其后的【欠账 · 必出七段自证 · 矩阵】从未打印——**而 ci-local 七项全绿**（它从不断言头齐不齐）。
#   ⇒ 本闸门两档：
#     `--death-only`（CI 用）：只要【生成器跑完不被打死】+【自证行存在】。CI 只 checkout 本仓
#                              （没有五仓兄弟 ⇒ 光/预算等段天然取不到），故**只判死不死**。
#     全档（本地/ci-local 用）：另判【派生自证无 ✗】+【无"★ 缺段"行】——六仓在时这才是真判。
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"; cd "$ROOT"
mode="${1:-full}"
out=$(bash tools/todo-ready.sh --report-head 2>/tmp/.chk-head.err); rc=$?
if [ "$rc" != 0 ]; then
  # ★ 2026-10-25：**rc 不再作"判死"依据**（生成器在有段落失败时会故意 exit 1 = 诚实信号，不是死亡）。
  #   判死改为【正断言】：输出里必须有"跑完哨兵"。真死（语法错/被杀/缺解释器）时哨兵必然缺席。
  if ! printf '%s' "$out" | grep -q 'self-attest: 报告头生成器跑完'; then
    echo "★ 报告头生成器【死于半途】（无跑完哨兵 · rc=${rc}）—— 头必然残缺"; head -3 /tmp/.chk-head.err | sed 's/^/    /'; exit 1
  fi
  echo "NOTE: 生成器跑完，但 rc=${rc}（= 有段落失败；CI 无兄弟仓时属预期）—— 不判死" >&2
fi
printf '%s' "$out" | grep -q '必出七段自证' || { echo "★ 报告头里没有【必出七段自证】行 —— 生成器没跑到末尾"; exit 1; }
if [ "$mode" = "--death-only" ]; then echo "OK: 生成器跑完且自证行在（CI 只判死不死）"; exit 0; fi
miss=$(printf '%s' "$out" | grep -o '★ 缺段：.*' || true)
[ -n "$miss" ] && { echo "★ 报告头缺段：$miss"; exit 1; }
printf '%s' "$out" | grep -oE '必出七段自证.*' | grep -q '✗' && { echo "★ 派生自证含 ✗：$(printf '%s' "$out" | grep -oE '必出七段自证.*')"; exit 1; }
echo "OK: 报告头七段齐（派生自证无 ✗）"
