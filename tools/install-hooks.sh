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
