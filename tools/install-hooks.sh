#!/usr/bin/env bash
# Install the shared hooks into every repo. Idempotent.
#
# .git/hooks/ is not versioned, so hooks that matter have to live in a
# directory that is. `core.hooksPath` points each repo at that directory.
# With five repos, doing it by hand is a step that will be forgotten, so it is
# a script.
#
# Usage:  ./install-hooks.sh            install into all known repos
#         ./install-hooks.sh --check    report status, change nothing
set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOKS="$HERE/hooks"
WS="$(cd "$HERE/../.." && pwd)"
REPOS="Cellrix Tuck anaphase-helix helix-mind FlowModus helix-tentacle BIND-19 HelixECO-Glove"
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
