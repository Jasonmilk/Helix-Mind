#!/usr/bin/env bash
# check-adr-heads.sh —— 每份 ADR 的头必须可解析（"解析不了就不是 ADR"）。
# ★ 为什么把它从 workflow 里挪出来：原来那段 python 直接嵌在 YAML 的 run 块里，
#   而 **YAML 少一个缩进就整份非法**（实测症状：workflow 有 **0 个 job**，run 的 name 显示成文件路径）。
#   ⇒ **换一种不再需要小心的组织方式**：YAML 只调用脚本，脚本自己管缩进与引号。
set -uo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
fail=0
for f in "$root"/docs/decisions/*.md; do
  [ -e "$f" ] || continue
  out=$(python3 "$root/tools/adr_head.py" "$f" 2>&1) || { echo "★ 解析器失败: $f"; fail=1; continue; }
  printf '%s' "$out" | python3 -c 'import json,sys; d=json.load(sys.stdin); sys.exit(0 if d.get("ok") else 1)' \
    || { echo "★ ADR 头不可解析: $f"; fail=1; }
done
[ "$fail" = 0 ] && echo "OK: 全部 ADR 头可解析" || echo "★ RED: 有 ADR 头不可解析"
exit $fail
