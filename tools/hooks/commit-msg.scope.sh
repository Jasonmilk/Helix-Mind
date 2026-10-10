#!/usr/bin/env bash
# Table-driven assertion over `commit-msg`'s *claim* about its own scope.
#
# WHY THIS EXISTS.
#
# `commit-msg` guards three path prefixes. Its comment used to say they were
# "two Cellrix trees plus the ADRs, and nothing else" — a repository-scoped
# claim. The mechanism is not repository-scoped: `git diff --cached --numstat`
# prints paths relative to the repo being committed, so `docs/decisions/`
# matches in every repo that has one. Declared scope and actual scope differed,
# and **nothing would have noticed** (K-104c; same shape as K-104a's
# hand-maintained repo list and K-104b's category wording).
#
# This script is that missing observer. It asserts four things:
#
#   1. the `GUARDED=` line is byte-for-byte what HEAD has — i.e. the gate's
#      behaviour did not change while its declaration was corrected;
#   2. the `@scope:` line's prefix set equals the GUARDED alternation — i.e. the
#      *declaration* is machine-checked, not prose;
#   3. the comment states the scope is repo-agnostic (the specific thing that
#      was unstated before);
#   4. every row of `guarded_paths.tsv` agrees with the regex: claimed-covered
#      paths match, claimed-uncovered paths do not.
#
# WHAT IT IS NOT. It is not a gate and it does not run on commit: it is a
# checker you (or CI) run. Exit codes follow `commonintents/.github/tools/`
# `check_proto_sync.py`: 0 = all claims hold; 1 = a claim is false (that is
# red); 2 = the checker itself is broken (never report a confident wrong answer).

set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOK="$HERE/commit-msg"
TABLE="$HERE/guarded_paths.tsv"
REPO="$(cd "$HERE/../.." && pwd)"

fail=0
note() { printf '%s\n' "$*"; }

[ -f "$HOOK" ] || { note "CHECKER ERROR: $HOOK 不存在"; exit 2; }
[ -f "$TABLE" ] || { note "CHECKER ERROR: $TABLE 不存在"; exit 2; }

guarded_line="$(grep -E '^GUARDED=' "$HOOK" | head -1)"
if [ -z "$guarded_line" ]; then
  note "CHECKER ERROR: $HOOK 里没有 GUARDED= 赋值 —— 解析器与机制脱节，拒绝给结论"
  exit 2
fi
guarded="${guarded_line#GUARDED=}"
guarded="${guarded#\'}"
guarded="${guarded%\'}"

# ---- 1. behaviour: the regex must be byte-identical to HEAD's --------------
head_hook="$(git -C "$REPO" show HEAD:tools/hooks/commit-msg 2>/dev/null)" || {
  note "CHECKER ERROR: 读不到 HEAD 的 tools/hooks/commit-msg（${REPO}）"; exit 2; }
head_line="$(printf '%s\n' "$head_hook" | grep -E '^GUARDED=' | head -1)"
if [ "$guarded_line" = "$head_line" ]; then
  note "OK   1 GUARDED 与 HEAD 逐字节相同（闸门行为未变）：$guarded_line"
else
  note "FAIL 1 GUARDED 与 HEAD 不同 —— 这不是「修正声明」，这是**改闸门**"
  note "        HEAD: $head_line"
  note "        现在: $guarded_line"
  fail=1
fi

# ---- 2. declaration == mechanism ------------------------------------------
declared="$(grep -E '^# @scope:' "$HOOK" | head -1 | sed 's/.*prefixes:[[:space:]]*//')"
if [ -z "$declared" ]; then
  note "FAIL 2 注释里没有机器可读的 \`# @scope:\` 行 —— 覆盖范围只是散文，没法被检查"
  fail=1
else
  alt="${guarded#^*(}"
  alt="${alt%)}"
  alt_set="$(printf '%s\n' "$alt" | tr '|' '\n' | sort | tr '\n' ' ')"
  decl_set="$(printf '%s\n' "$declared" | tr ' ' '\n' | grep -v '^$' | sort | tr '\n' ' ')"
  if [ "$alt_set" = "$decl_set" ]; then
    note "OK   2 @scope 与 GUARDED 的前缀集相同：$decl_set"
  else
    note "FAIL 2 声明的范围与机制的范围不一致（规则：不一致处必须显式登记）"
    note "        @scope:  $decl_set"
    note "        GUARDED: $alt_set"
    fail=1
  fi
fi

# ---- 3. the repo-agnostic fact must be stated -----------------------------
if grep -q 'repo-agnostic' "$HOOK"; then
  note "OK   3 注释显式声明了「前缀与仓库无关」（K-104c 更正过的那个事实）"
else
  note "FAIL 3 注释没有说明前缀与仓库无关 —— 这正是 K-104c 里被漏掉的声明"
  fail=1
fi

# ---- 4. replay the claim table -------------------------------------------
rows=0
while IFS=$'\t' read -r claim repo path basis; do
  case "$claim" in
    ''|'#'*) continue ;;
  esac
  if [ "$claim" = "claim" ]; then continue; fi   # 表头行
  case "$claim" in
    match|nomatch) ;;
    *) note "CHECKER ERROR: $TABLE 里 claim=$claim 不认识（只认 match/nomatch）"; exit 2 ;;
  esac
  if [ -z "${path:-}" ]; then
    note "CHECKER ERROR: $TABLE 有一行没有 path：$claim $repo"; exit 2
  fi
  actual=nomatch
  printf '%s\n' "$path" | grep -Eq "$guarded" && actual=match
  rows=$((rows + 1))
  if [ "$actual" = "$claim" ]; then
    printf '     %-7s %-20s %s\n' "$actual" "$repo" "$path"
  else
    printf 'FAIL %-7s (claimed %s) %-20s %s\n' "$actual" "$claim" "$repo" "$path"
    note "       依据：$basis"
    fail=1
  fi
done < "$TABLE"

if [ "$rows" -eq 0 ]; then
  note "CHECKER ERROR: $TABLE 里一行都没有 —— 空表会让这个检查器「全部通过」"
  exit 2
fi
note "OK   4 判据表的 $rows 行全部与正则一致"

if [ "$fail" -ne 0 ]; then
  note "FAIL —— 上面有对不上的行。修**声明**，不要为了让表变绿而改正则。"
  exit 1
fi
note "OK   commit-msg 的声明与机制一致，且闸门行为与 HEAD 逐字节相同"
exit 0
