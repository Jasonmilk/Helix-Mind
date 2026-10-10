#!/usr/bin/env bash
# msg.sh —— 写提交/合并信息到文件，避免 shell 吃字（Trap 37 家族的机械解：不改小心，改组织）。
# ★ 为什么存在：实测四次——反引号被当命令替换（`2cc82f4` ⇒ command not found）· 引号内套引号 ·
#   中文旁的变量未加花括号。**把文本放进文件，shell 就无从下手**。
# 用法：bash tools/msg.sh /tmp/msg.txt <<'MSG'  ...任意内容（含反引号/引号/$）...  MSG
#       git commit -F /tmp/msg.txt     |     git merge --no-ff -F /tmp/msg.txt <branch>
set -euo pipefail
out="${1:?用法: msg.sh <输出文件>  （内容走 stdin，用引号 heredoc）}"
cat > "$out"
printf '  ✅ 信息已落 %s（%d 字节）· 用 `-F %s` 交给 git\n' "$out" "$(wc -c < "$out" | tr -d ' ')" "$out"
