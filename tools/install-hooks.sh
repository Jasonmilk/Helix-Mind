#!/usr/bin/env bash
# Install the shared hooks into every repo. Idempotent.
#
# .git/hooks/ is not versioned, so hooks that matter have to live in a
# directory that is. `core.hooksPath` points each repo at that directory.
#
# The list below is hand-maintained, and it fell behind: it said "five repos",
# then held eight, while the workspace held thirteen plus one nested repo. Six
# of them — Helix-MCP-Learner, lodestone-md, lodestone-spec, lumtract,
# phyt-DNA, commonintents/.github — had no hooks at all, so neither the ADR
# first-line gate nor the `[large]` marker gate ran there. They appeared to be
# covered because this script exists; nothing said which repos it covered.
#
# K-104. The lesson is not "add the missing names" — it is that **a
# hand-maintained list inside the script that was written to stop
# hand-maintenance being forgotten is the same defect one level up.** A repo
# with no `docs/decisions/` is a no-op for the ADR gate, so over-listing is
# cheap and under-listing is silent; list every repo in the workspace, and
# `--check` is the thing that makes an omission visible.
#
# Usage:  ./install-hooks.sh            install into all known repos
#         ./install-hooks.sh --check    report status, change nothing
set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOKS="$HERE/hooks"
WS="$(cd "$HERE/../.." && pwd)"
REPOS="Cellrix Tuck anaphase-helix helix-mind FlowModus helix-tentacle
       HelixECO-Glove Helix-MCP-Learner lodestone-md lodestone-spec lumtract phyt-DNA
       commonintents/.github
       commonintents/BIND-19 commonintents/CAPABILITY-13 commonintents/INTENT-7
       commonintents/INTENT-7-SECURE commonintents/PFP-xCF14 commonintents/SAP-xCF14"

# `BIND-19` used to appear here as a TOP-LEVEL repo, and `commonintents/BIND-19`
# also appeared. They are two independent clones of the same origin
# (CommonIntents/BIND-19): same HEAD, same branch — and the nested one was a strict
# superset (local `main`, one more tag, two more remote refs). The top-level copy
# was the only CommonIntents repo outside `commonintents/`, so the name was drift
# rather than a second subject. The top-level clone was removed (2026-09-22) after
# checking it held nothing unique: no stash, no extra worktree, no untracked files,
# no local-only commits, and its tags/remotes were subsets. One family, one name.

# ONE declaration, not a first line plus an append. The first attempt wrote
# `REPOS="… existing …"` and then `REPOS="$REPOS commonintents/…"` with the
# rationale in comments between them. That works when a shell runs it and reads as
# completeness to a human — and the I5 checker went RED on it, because
# `shell_declaration` reads the FIRST `REPOS="` to its closing quote and never sees
# an append. The 6 repos were hooked and simultaneously undeclared: exactly the
# declared-vs-actual gap this list exists to close. One literal list, split across
# lines for width only.
#
# The six protocol repos are separate from commonintents/.github on purpose. An
# earlier version held them in EXCLUDE as `pending-human`, reasoning that hooking
# them would extend the protocol family to new ground. That was the wrong way
# round, and the human corrected it (2026-09-22): commonintents/.github carries the
# FAMILY protocol, and the projects under it are the protocol BODIES — material
# parked only in `.github` is material nobody sees. They are ordinary repos and must
# be committable as such. Named individually rather than by glob so that `--check`
# and the I5 assertion can still name a repo that has gone missing.

# The other half of the declaration: what is deliberately NOT hooked, and why.
# Read by `anaphase-helix/tests/security_gate.rs`, which goes red for any git repo
# that is in neither list, for an EXCLUDE name that is not a repo, for a row with no
# reason, and for a row whose status is not in the checker's known set — so this
# cannot decay into a list that follows reality instead of constraining it, and a
# NEW repo still trips the gate.
#
# Pipe-separated: <repo> | <status> | <reason>.
#   status is a STATE, not a reason. Two of them are known:
#     pending-human — waiting on a human ruling; NOT endorsed by the author. The
#                     reason must still say what would change if it were approved.
#     decided       — a human ruled to keep it out; the reason must say on what basis.
#   Keeping them apart matters: if every row says "pending", the list is a queue with
#   no exit, and a queue that never drains is a permanent exemption.
#
# No calendar `due` here on purpose. A date would make the assertion go red for the
# passage of time rather than for a false claim, which is the "ritual" failure the
# project already rejects; a date that gets pushed forward each cycle IS a permanent
# exemption wearing a deadline. The bound lives where it can be enforced — a row in
# the ruling queue with an owner and a due (commonintents/.github/HANDOFF-0920.md).
#
# Declaration, not a switch: EXCLUDE is read by nothing but that assertion, so
# adding a repo here does NOT stop the loop below from hooking it. To change the
# policy, move the name into REPOS (which the loop consumes) and drop the row.
EXCLUDE="
# 之前是空的，且那是声明而非疏忽：当时工作区里每个 git 仓都在 REPOS。该声明保留 —— I5 断言
# 读它、并拒绝没有它的脚本：声明过的空表可复审，缺席的不可。
#
# thinking/phyt-DNA的任务编排思路 | decided | 驱动层（环判据编排内核），不是生态仓
#   2026-10-27 实测触发 anaphase-helix tests/security_gate.rs 的 I5：它 `git init` 后成为
#   工作区里的第 20 个 git 仓，既不在 REPOS 也不在 EXCLUDE ⇒ 具名红（K-104a：声明覆盖 ≠ 实际覆盖）。
#   为什么不放进 REPOS：它跑的是跨仓「环」编排（内核 + 环定义 + 账本），提交语义与生态仓不同，
#   生态的 ADR 首行门 / [large] 标记门对它没有对应物；硬挂上去只会产生「为过判据而填的表」。
#   它的方向由人裁决，不由钩子裁决。
# ★ 陷阱（当场踩到）：本声明体内【不得出现 ASCII 双引号字符】—— 解析器在第一个双引号处截断，
#   于是 Excluded 读成 []（「读到空」 与 「缺席」 分不开），I5 照样红。写理由用「」。
thinking/phyt-DNA的任务编排思路 | decided | 驱动层仓，非生态仓；职责/监视关系/为何在生态仓外：helixECO/ECOSYSTEM.md §0.2（那边是唯一真相，本行只是 I5 的声明入口）
"
MODE="${1:-install}"

if [ ! -d "$HOOKS" ]; then
  echo "no hooks directory at $HOOKS" >&2
  exit 1
fi
# Only the hooks themselves are executable, and only by suffix. `chmod +x ./*`
# also caught the DATA beside them: `guarded_paths.tsv` came back as a mode change
# (100644 -> 100755) on every install, so a data file was being published as a
# program by the very script that exists to keep this directory honest. A mode is
# part of what a file claims to be; `git diff --summary` showing a permission flip
# is a real diff, not noise.
for h in commit-msg pre-commit pre-push post-commit prepare-commit-msg; do
  [ -f "$HOOKS/$h" ] && chmod +x "$HOOKS/$h" 2>/dev/null
done
for h in "$HOOKS"/*.sh; do [ -f "$h" ] && chmod +x "$h" 2>/dev/null; done

for r in $REPOS; do
  d="$WS/$r"
  [ -d "$d/.git" ] || continue
  cur="$(git -C "$d" config --get core.hooksPath || true)"
  if [ "$MODE" = "--check" ]; then
    if [ "$cur" = "$HOOKS" ]; then printf "  %-18s ok\n" "$r"
    else printf "  %-18s NOT installed (%s)\n" "$r" "${cur:-unset}"; fi
    continue
  fi
  git -C "$d" config core.hooksPath "$HOOKS"
  printf "  %-18s -> %s\n" "$r" "$HOOKS"
done

if [ "$MODE" != "--check" ]; then
  echo ""
  echo "hooks installed. Verify with: $0 --check"
fi
