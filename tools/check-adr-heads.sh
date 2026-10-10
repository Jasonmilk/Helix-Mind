#!/usr/bin/env bash
# check-adr-heads.sh —— 每份 ADR 的头必须可解析 + 【集合级】三条（单文件判据的盲区）。
#
# ★ 为什么有集合级三条（2026-10-10，由豆包 L2 首枪的审计暴露）：
#   原判据只查"第一行能否解析" ⇒ 它**照绿通过了两个坏结果**：
#     ① 20 个编号**撞号**（每个文件被加了第二个 `# ADR-00NN`）② 新头标题是**文件名 slug**而非文件自己的标题。
#   ⇒ 单文件检查**看不见**"集合里撞号"与"同一文件双头"，故必须在这里判。
set -uo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
fail=0
_internal_error() { echo "★ 检查内部错误（不是仓库的问题，是判据的问题）: $*" >&2; exit 3; }
trap '_internal_error "$BASH_COMMAND"' ERR
for f in "$root"/docs/decisions/*.md; do
  [ -e "$f" ] || continue
  out=$(python3 "$root/tools/adr_head.py" "$f" 2>&1) || { echo "★ 解析器失败: $(basename "$f")"; fail=1; continue; }
  printf '%s' "$out" | python3 -c 'import json,sys; d=json.load(sys.stdin); sys.exit(0 if d.get("ok") else 1)' \
    || { echo "★ ADR 头不可解析（头必须在第 0 行）: $(basename "$f")"; fail=1; }
done
echo "--- 集合级检查（单文件判据的盲区）---"
dup=$(grep -h '^# ADR-' "$root"/docs/decisions/*.md 2>/dev/null | sed -E 's/^(# ADR-[0-9]{4}).*/\1/' | sort | uniq -d || true)
if [ -n "$dup" ]; then echo "★ 编号撞号（同一编号出现多次）:"; printf '%s\n' "$dup" | sed 's/^/    /'; fail=1; fi
for f in "$root"/docs/decisions/*.md; do
  [ -e "$f" ] || continue
  n=$(grep -c '^# ADR-' "$f" || true)
  if [ "${n:-0}" -gt 1 ]; then echo "★ 双头: $(basename "$f")（${n} 个 # ADR- 行）"; fail=1; fi
done
for f in "$root"/docs/decisions/*.md; do
  [ -e "$f" ] || continue
  b=$(basename "$f")
  fn=$(printf '%s' "$b" | grep -oE '^[0-9]{4}' || true)
  if [ -z "$fn" ]; then fn=$(printf '%s' "$b" | grep -oE 'ADR-[0-9]{4}' | grep -oE '[0-9]{4}' || true); fi
  hn=$(sed -nE 's/^#\s*ADR-([0-9]{4})\s*[:：].*/\1/p' "$f" | head -1 || true)
  if [ -n "$fn" ] && [ -n "$hn" ] && [ "$fn" != "$hn" ]; then
    echo "★ 编号与文件名不一致: ${b}（文件名 ${fn} vs 头 ${hn}）"; fail=1
  fi
done
if [ "$fail" = 0 ]; then echo "OK: 全部 ADR 头可解析 · 编号唯一 · 无双头 · 与文件名一致"; else echo "★ RED: 见上"; fi
exit $fail
