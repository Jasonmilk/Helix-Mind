#!/usr/bin/env bash
# ② acceptance probe: "inject a large change ⇒ judged by ratio, not by a path prefix".
#
# THE CLAIM UNDER TEST (and why it needs a probe at all). `commit-msg` judges
# largeness by path prefix, so the *same* wholesale replacement is gated in
# `web/src/` and ignored in `src/`. That is a known, deliberate shape — but it
# means "the gate measures size" is false as stated: it measures size *in three
# places*. Nothing in the repo contradicted the shape until `diff-ratio.sh`
# gave the honest wider form a name.
#
# WHAT THIS PROBE ASSERTS (all on throwaway repos under mktemp):
#
#   A. ratio-derived judgement exists and fires on a wholesale replacement of an
#      UNGUARDED path (`src/legacy.rs`, 400 lines rewritten) ⇒ REPLACED.
#      **This is the ② criterion**, and it is RED as soon as `diff-ratio.sh` is
#      absent — before this change there was no ratio judgement anywhere.
#   B. that same staged change is still PASSED by `commit-msg` ⇒ adopting the
#      ratio bound did not change what gets blocked.
#   C. the pre-existing behaviour is retained: a wholesale replacement of a
#      GUARDED path (`web/src/legacy.js`) is still REJECTED, and a small change
#      is still passed, and `[large]` still overrides.
#   D. every one of those decisions is identical between the working-tree hook
#      and HEAD's hook (byte-comparison of exit code + output), so "behaviour
#      must not change" is measured, not asserted.
#   E. the declaration check (`commit-msg.scope.sh`) passes.
#
#   I5. the instrument's OWN declared scope is covered. `diff-ratio.sh` declares
#      four verdicts and one rule about the fourth:
#
#        REPLACED      deleted/before >= RATIO_MAX and churn >= MIN_LINES
#        NEW           absent at HEAD and churn >= MIN_LINES — "the two must not
#                      be conflated: absence of a before-image is not evidence of
#                      one"
#        LOCAL         everything else
#        UNMEASURABLE  binary files — "**never silently 0**"
#
#      Before this change A and C3 exercised only REPLACED and LOCAL. NEW and
#      UNMEASURABLE — the two the comment goes out of its way to name, i.e. the
#      two where a silent regression is most plausible — were **claimed and
#      unobserved**: exactly the shape I5 is about (declared scope > actual
#      coverage). Steps A2/A3 observe them. They constrain the *instrument*,
#      which `commit-msg` does not call, so no gate decision moves.
#
# Exit: 0 = all green; 1 = a criterion is red; 2 = the probe itself is broken.
set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RATIO="$HERE/diff-ratio.sh"
HOOK="$HERE/commit-msg"
SCOPE="$HERE/commit-msg.scope.sh"
REPO="$(cd "$HERE/../.." && pwd)"

fail=0
red() { printf 'RED   %s\n' "$*"; fail=1; }
ok() { printf 'OK    %s\n' "$*"; }
bad() { printf 'FAIL  %s\n' "$*"; fail=1; }

for f in "$HOOK" "$SCOPE"; do
  [ -f "$f" ] || { echo "CHECKER ERROR: missing $f" >&2; exit 2; }
done

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

git -C "$TMP" init -q || { echo "CHECKER ERROR: git init failed" >&2; exit 2; }
git -C "$TMP" config user.email probe@example.invalid
git -C "$TMP" config user.name probe
mkdir -p "$TMP/src" "$TMP/web/src"

# A 400-line file is the "before" image for both fixtures.
seq 1 400 | sed 's/^/old-line-/' > "$TMP/src/legacy.rs"
seq 1 400 | sed 's/^/old-line-/' > "$TMP/web/src/legacy.js"
git -C "$TMP" add -A >/dev/null
git -C "$TMP" commit -qm baseline

# HEAD's hook, for the behaviour-equivalence comparison.
git -C "$REPO" show HEAD:tools/hooks/commit-msg > "$TMP/commit-msg.head" 2>/dev/null || {
  echo "CHECKER ERROR: cannot read HEAD's commit-msg from ${REPO}" >&2; exit 2; }

# Put the fixture repo back to its committed state (index + worktree).
# NOTE: this is a throwaway repo under mktemp; the probe never touches the
# grader's working tree, and never uses `git reset --hard` anywhere.
restore_fixture() { git -C "$TMP" restore --source=HEAD --staged --worktree -- . ; }

# Stage a wholesale rewrite of $1 (all 400 lines replaced).
wholesale() {
  seq 1 400 | sed 's/^/new-line-/' > "$TMP/$1"
  git -C "$TMP" add -- "$1"
}

# Run a hook in the fixture repo. Prints the exit code; output lands in $TMP/out.
run_hook() { # $1 = hook path, $2 = message file
  : > "$TMP/out"
  ( cd "$TMP" && bash "$1" "$2" ) >>"$TMP/out" 2>&1
  echo $?
}

printf 'probe: wholesale rewrite\n' > "$TMP/msg.plain"
printf '[large] probe: wholesale rewrite, acknowledged\n' > "$TMP/msg.large"
printf 'probe: small change\n' > "$TMP/msg.small"

# ---------------------------------------------------------------- A. the ratio
printf '== A. ratio-derived judgement on an UNGUARDED path ==\n'
if [ ! -f "$RATIO" ]; then
  red "② no ratio judgement present at $RATIO — a large change is judged by path prefix only"
else
  wholesale src/legacy.rs
  ratio_out="$(cd "$TMP" && bash "$RATIO" 2>&1)" || {
    echo "$ratio_out"
    echo "CHECKER ERROR: diff-ratio.sh exited non-zero" >&2; exit 2; }
  printf '%s\n' "$ratio_out" | sed 's/^/      /'
  if printf '%s\n' "$ratio_out" | grep -q '^REPLACED.*src/legacy.rs'; then
    ok "② wholesale rewrite of the UNGUARDED src/legacy.rs is judged REPLACED by ratio"
  else
    red "② ratio judgement did not call the wholesale rewrite REPLACED"
  fi
fi

# ------------------------------------- A2. the declared NEW/LOCAL distinction
#
# The judgment under test: a file that was ABSENT at HEAD is `NEW`, not
# `REPLACED` and not `LOCAL`, and "new" is not the same verdict as "local". The
# comment's own warning is the criterion — absence of a before-image is not
# evidence of one, so the two must not be conflated.
#
# A2/A3 build their **own** scratch repo: they must observe the instrument
# without disturbing the stage `$TMP` carries into B. (An earlier draft reset the
# shared fixture here, which silently emptied B's input — order-dependence
# introduced by the very step meant to strengthen the probe.) A non-zero
# instrument exit is the probe being broken, not a red criterion: it is reported
# as such, matching A's handling above.
scratch_ratio() { # $1 = shell snippet (cwd = scratch repo). Prints the report.
  local d out rc
  d="$(mktemp -d)"
  git -C "$d" init -q >/dev/null 2>&1
  git -C "$d" config user.email probe@example.invalid
  git -C "$d" config user.name probe
  mkdir -p "$d/src"
  out="$(cd "$d" && eval "$1" && git add -A >/dev/null 2>&1 && bash "$RATIO" 2>&1)"
  rc=$?
  rm -rf "$d"
  if [ "$rc" -ne 0 ]; then
    printf 'CHECKER ERROR: diff-ratio.sh exited %s in the A2/A3 scratch repo:\n%s\n' "$rc" "$out" >&2
    exit 2
  fi
  printf '%s\n' "$out"
}

printf '== A2. NEW is a distinct verdict, not LOCAL and not REPLACED (I5) ==\n'
if [ -f "$RATIO" ]; then
  r="$(scratch_ratio 'seq 1 400 | sed "s/^/brand-new-line-/" > src/large_new.rs
                      seq 1 5 | sed "s/^/tiny-new-/" > src/small_new.rs')"
  if printf '%s\n' "$r" | grep -q '^NEW[[:space:]]*deleted/before=n/a.*src/large_new.rs'; then
    ok "I5 large file absent at HEAD ⇒ NEW (not REPLACED, not LOCAL)"
  else
    red "I5 a new file is no longer reported NEW — got: $(printf '%s\n' "$r" | grep large_new.rs)"
  fi
  if printf '%s\n' "$r" | grep -q '^LOCAL[[:space:]]*deleted/before=n/a.*src/small_new.rs'; then
    ok "I5 small file absent at HEAD ⇒ LOCAL (NEW is churn-gated; the two are not conflated)"
  else
    red "I5 a below-threshold new file is no longer reported LOCAL — got: $(printf '%s\n' "$r" | grep small_new.rs)"
  fi
fi

# ------------------------------------------ A3. UNMEASURABLE is never "0"
#
# The rule under test, verbatim from the comment: binary files ⇒ UNMEASURABLE,
# "**never silently 0**". A checker can only hold that if a binary is actually
# staged and observed. Note the summary line prints all five counters; an
# anchored match would never see `UNMEASURABLE=1` in it.
printf '== A3. a binary file is UNMEASURABLE, never silently dropped (I5) ==\n'
if [ -f "$RATIO" ]; then
  r="$(scratch_ratio 'printf "bin\0\0\0\0\0\0\0\0binary payload\0\0\1\2\3" > src/blob.bin
                      seq 1 5 | sed "s/^/text-/" > src/text_control.rs')"
  printf '%s\n' "$r" | grep -q '^UNMEASURABLE.*src/blob.bin' \
    && ok "I5 binary staged ⇒ reported UNMEASURABLE (its churn is not invented)" \
    || red "I5 binary staged but not reported UNMEASURABLE — got: $(printf '%s\n' "$r" | grep blob.bin)"
  printf '%s\n' "$r" | grep -q 'UNMEASURABLE=1' \
    && ok "I5 the summary counts it as UNMEASURABLE=1 (not folded into LOCAL=0)" \
    || red "I5 the summary does not count the binary as UNMEASURABLE=1: $(printf '%s\n' "$r" | grep '^UNMEASURABLE=')"
  printf '%s\n' "$r" | grep -q '^LOCAL.*src/text_control.rs' \
    && ok "I5 control: a small text file in the same stage is still LOCAL" \
    || red "I5 control failed — small text file not LOCAL: $(printf '%s\n' "$r" | grep text_control.rs)"
fi

# ------------------------------------------------- B. behaviour did not change
#
# A left the shared stage holding the wholesale rewrite of the UNGUARDED
# `src/legacy.rs`. B observes exactly that, so it must NOT reset the fixture
# first: the rewrite is uncommitted, and `restore_fixture` would restore the
# file to HEAD — i.e. delete the very change B exists to judge.
printf '== B. commit-msg still ignores that change (behaviour unchanged) ==\n'
if [ -f "$RATIO" ]; then
  code_now="$(run_hook "$HOOK" "$TMP/msg.plain")"
  code_head="$(run_hook "$TMP/commit-msg.head" "$TMP/msg.plain")"
  cp "$TMP/out" "$TMP/out.now"
  if [ "$code_now" = "0" ]; then
    ok "commit-msg passes the unguarded wholesale rewrite (exit 0) — prefix rule intact"
  else
    bad "commit-msg now blocks an unguarded path (exit $code_now) — behaviour CHANGED"
  fi
  if [ "$code_now" = "$code_head" ]; then
    ok "same decision as HEAD's hook on this fixture (both exit $code_now)"
  else
    bad "decision differs from HEAD: now=$code_now head=$code_head"
  fi
fi

# ------------------------------------------------- C. pre-existing behaviour
printf '== C. pre-existing blocking behaviour retained ==\n'
restore_fixture
wholesale web/src/legacy.js
code_now="$(run_hook "$HOOK" "$TMP/msg.plain")"; cp "$TMP/out" "$TMP/c.now"
code_head="$(run_hook "$TMP/commit-msg.head" "$TMP/msg.plain")"; cp "$TMP/out" "$TMP/c.head"
if [ "$code_now" = "1" ]; then
  ok "guarded wholesale rewrite still REJECTED (exit 1)"
else
  bad "guarded wholesale rewrite is no longer rejected (exit $code_now)"
fi
grep -q 'REJECT: guarded diff is' "$TMP/c.now" \
  && ok "rejection message unchanged: $(head -1 "$TMP/c.now")" \
  || bad "rejection message changed: $(head -1 "$TMP/c.now")"
cmp -s "$TMP/c.now" "$TMP/c.head" \
  && ok "stdout byte-identical to HEAD's hook" \
  || bad "stdout differs from HEAD's hook"

printf '== C2. the [large] marker still overrides ==\n'
restore_fixture
wholesale web/src/legacy.js
code_now="$(run_hook "$HOOK" "$TMP/msg.large")"
if [ "$code_now" = "0" ]; then
  ok "[large] marker on a guarded wholesale rewrite still passes (exit 0)"
else
  bad "[large] marker no longer overrides (exit $code_now)"
fi

printf '== C3. small changes still pass ==\n'
restore_fixture
seq 1 5 | sed 's/^/tiny-/' > "$TMP/src/small.rs"
git -C "$TMP" add -- src/small.rs
code_now="$(run_hook "$HOOK" "$TMP/msg.small")"
if [ "$code_now" = "0" ]; then
  ok "small unguarded change passes (exit 0)"
else
  bad "small unguarded change blocked (exit $code_now)"
fi
if [ -f "$RATIO" ]; then
  r="$(cd "$TMP" && bash "$RATIO" 2>&1)"
  printf '%s\n' "$r" | grep -q '^LOCAL.*src/small.rs' \
    && ok "ratio judgement calls the small change LOCAL (not REPLACED)" \
    || bad "ratio judgement misfiled the small change: $(printf '%s\n' "$r" | grep small.rs)"
fi

# ------------------------------------------------- D. declaration check
printf '== D. declared scope vs mechanism ==\n'
if bash "$SCOPE" >"$TMP/scope.out" 2>&1; then
  ok "commit-msg.scope.sh passes"
else
  sed 's/^/      /' "$TMP/scope.out"
  bad "commit-msg.scope.sh failed"
fi

echo "---"
if [ "$fail" -ne 0 ]; then
  echo "② RED — see the FAIL/RED lines above"
  exit 1
fi
echo "② GREEN — ratio judgement present and ratio-derived, commit-msg behaviour unchanged, declaration consistent"
exit 0
