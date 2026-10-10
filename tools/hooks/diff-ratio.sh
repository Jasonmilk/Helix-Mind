#!/usr/bin/env bash
# Ratio-bound judgement for a staged diff — **an instrument, not a gate**.
#
# WHY IT EXISTS, AND WHY IT IS NOT WIRED INTO `commit-msg`.
#
# `commit-msg` decides "is this diff large?" by **path prefix**: three prefixes
# are guarded, everything else is free. That is a deliberate choice and its own
# comment defends it — the gate exists for one observed failure shape ("one of
# those replaced wholesale" in those three scenes), and a longer prefix list
# would measure size instead of that shape.
#
# The same comment rules that **if it should be wider, the honest form is a
# ratio bound on any path** — and that this is a decision, not a cleanup. This
# file is that bound, kept as an instrument so the decision stays undecided:
#
#   * it reports a verdict for **every** path, regardless of prefix;
#   * it changes nothing about what a commit is allowed to do — `commit-msg`
#     does not call it (`--strict` exists for a future policy hook, and is
#     deliberately not the default);
#   * `diff-ratio.acceptance.sh` proves both halves: that the verdict is
#     ratio-derived, and that `commit-msg`'s blocking decision is unchanged.
#
# WHAT IT MEASURES. `deleted / before`, where `before` is the line count of the
# file at HEAD. A wholesale replacement deletes ~100% of what was there, so this
# ratio is a direct proxy for the shape the gate cares about — and unlike a line
# count it does not fire on ordinary growth.
#
#   REPLACED      deleted/before >= RATIO_MAX and churn >= MIN_LINES
#   NEW           file absent at HEAD and churn >= MIN_LINES (nothing was
#                 replaced, so it is *not* REPLACED — the two must not be
#                 conflated: absence of a before-image is not evidence of one)
#   LOCAL         everything else
#   UNMEASURABLE  binary files (numstat prints `-`) — **never silently 0**
#
# Usage:  diff-ratio.sh [--strict]
# Env:    RATIO_MAX (default 0.8)   MIN_LINES (default 20)
# Exit:   0 = reported; 1 = --strict and something is REPLACED; 2 = cannot judge
set -u

# ★ 合并豁免（2026-10-10 实测）：`git merge` 的暂存 diff 是【对第一父】的 ⇒ 合并批必然"很大"，
#   但那是【笔数的和】，不是"某份文件被重写" ⇒ 用"大改动"判据拦合并是判据用错了地方。
#   （实测：anaphase `feature → rs` 被拦 853 行 ⇒ 要人工加 [large] 才过 —— 语义问题推给了人。）
if [ -f "$(git rev-parse --git-dir)/MERGE_HEAD" ]; then
  echo "  [diff-ratio] 合并提交（MERGE_HEAD 在）⇒ 跳过（本判据判的是编辑，不是合并）"
  exit 0
fi

RATIO_MAX="${RATIO_MAX:-0.8}"
MIN_LINES="${MIN_LINES:-20}"
STRICT=0
for arg in "$@"; do
  case "$arg" in
    --strict) STRICT=1 ;;
    -h|--help) sed -n '2,30p' "$0"; exit 0 ;;
    *) echo "usage: $0 [--strict]" >&2; exit 2 ;;
  esac
done

numstat="$(git diff --cached --numstat 2>/dev/null)" || {
  echo "CANNOT JUDGE: not a git repository (or git failed)" >&2; exit 2; }
if [ -z "$numstat" ]; then
  echo "no staged changes"
  exit 0
fi

replaced=0; new=0; local=0; unmeasurable=0; churn_total=0

while IFS=$'\t' read -r added deleted path; do
  [ -n "${path:-}" ] || continue
  if [ "$added" = "-" ] || [ "$deleted" = "-" ]; then
    printf '%-12s %s\n' UNMEASURABLE "$path"
    unmeasurable=$((unmeasurable + 1))
    continue
  fi
  churn=$((added + deleted))
  churn_total=$((churn_total + churn))
  if git cat-file -e "HEAD:$path" 2>/dev/null; then
    before="$(git show "HEAD:$path" | wc -l | tr -d ' ')"
  else
    before=0
  fi
  if [ "$before" -eq 0 ]; then
    ratio="n/a"
    if [ "$churn" -ge "$MIN_LINES" ]; then
      verdict=NEW; new=$((new + 1))
    else
      verdict=LOCAL; local=$((local + 1))
    fi
  else
    ratio="$(awk -v d="$deleted" -v b="$before" 'BEGIN{printf "%.2f", d/b}')"
    if awk -v r="$ratio" -v m="$RATIO_MAX" 'BEGIN{exit !(r >= m)}' && \
       [ "$churn" -ge "$MIN_LINES" ]; then
      verdict=REPLACED; replaced=$((replaced + 1))
    else
      verdict=LOCAL; local=$((local + 1))
    fi
  fi
  printf '%-12s deleted/before=%-5s churn=%-5s before=%-5s %s\n' \
    "$verdict" "$ratio" "$churn" "$before" "$path"
done <<< "$numstat"

echo "---"
echo "REPLACED=$replaced NEW=$new LOCAL=$local UNMEASURABLE=$unmeasurable churn_total=$churn_total"
echo "(thresholds: RATIO_MAX=$RATIO_MAX MIN_LINES=$MIN_LINES; path prefix plays no part)"

if [ "$STRICT" -eq 1 ] && [ "$replaced" -gt 0 ]; then
  echo "REJECT (--strict): $replaced path(s) look wholesale-replaced"
  exit 1
fi
exit 0
