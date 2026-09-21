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
REPOS="Cellrix Tuck anaphase-helix helix-mind FlowModus helix-tentacle BIND-19 HelixECO-Glove Helix-MCP-Learner lodestone-md lodestone-spec lumtract phyt-DNA commonintents/.github"

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
commonintents/BIND-19 | pending-human | excluded 2026-09-21; not endorsed. Approving would add this repo to REPOS and the hooks would then install there
commonintents/CAPABILITY-13 | pending-human | excluded 2026-09-21; not endorsed. Approving would add this repo to REPOS and the hooks would then install there
commonintents/INTENT-7 | pending-human | excluded 2026-09-21; not endorsed. Approving would add this repo to REPOS and the hooks would then install there
commonintents/INTENT-7-SECURE | pending-human | excluded 2026-09-21; not endorsed. Approving would add this repo to REPOS and the hooks would then install there
commonintents/PFP-xCF14 | pending-human | excluded 2026-09-21; not endorsed. Approving would add this repo to REPOS and the hooks would then install there
commonintents/SAP-xCF14 | pending-human | excluded 2026-09-21; not endorsed. Approving would add this repo to REPOS and the hooks would then install there
"
MODE="${1:-install}"

if [ ! -d "$HOOKS" ]; then
  echo "no hooks directory at $HOOKS" >&2
  exit 1
fi
chmod +x "$HOOKS"/* 2>/dev/null

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
