#!/usr/bin/env python3
"""Check that non-exempt ecosystem repos keep source code and comments in English.

Why this exists
---------------
"Source code must be English" was stated by the owner but had no check.
A rule without a check is a claim without an anchor (the K10 family:
a convention exists, is violated N times, and nothing turns red).

Owner decision (2026-10-30):
  - phyt-DNA  : EXEMPT (Chinese allowed in source)
  - every other ecosystem repo: source code, comments, and commit messages
    must be English.
  - commits already pushed before the rule date are NOT retroactive.

Rules
-----
  R1  Exemption is decided PER FILE from the repo name in the path, not from
      the root the caller happened to pass.
  R2  Any CJK codepoint in a scanned source file is a hit, unless the line
      carries an explicit `i18n-ok:` marker.
  R3  Scanning zero files is NOT clean -- it means we read nothing (exit 3).
  R4  Output is English, because this script's own failure text is what a
      later grep has to match.

Exit codes: 0 clean / 1 hits / 3 read-nothing
"""
import io
import os
import re
import sys

EXEMPT_REPOS = {"phyt-DNA", "phyt-dna"}
# D2 (ruled 2026-10-31): commonintents is READ-ONLY SPEC, never modified by us.
# A red on a repo we are forbidden to fix is not a violation, it is a statement of
# fact -> not attributable -> it drowns the reds that ARE ours. Excluded, not exempt.
EXCLUDED_REPOS = {"commonintents"}
SOURCE_EXT = {".rs", ".py", ".go", ".ts", ".js", ".java", ".c", ".h", ".sh"}
CJK = re.compile(r"[\u3000-\u303f\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff00-\uffef]")
MARKER = "i18n-ok:"


def repo_of(path):
    parts = os.path.normpath(path).split(os.sep)
    for p in parts:
        if p in EXEMPT_REPOS:
            return p
    return None


def component_in(path, names):
    for part in os.path.normpath(path).split(os.sep):
        if part in names:
            return part
    return None


def is_exempt(path):
    return component_in(path, EXEMPT_REPOS) is not None


def is_excluded(path):
    return component_in(path, EXCLUDED_REPOS) is not None


def scan(root):
    """scanned counts EVERY source file seen; exempted is a separate counter."""
    hits, scanned, exempted, excluded = [], 0, 0, 0
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        for fn in filenames:
            if os.path.splitext(fn)[1] not in SOURCE_EXT:
                continue
            full = os.path.join(dirpath, fn)
            scanned += 1
            if is_exempt(full):
                exempted += 1
                continue
            if is_excluded(full):
                excluded += 1
                continue
            try:
                lines = io.open(full, encoding="utf-8", errors="replace").read().splitlines()
            except OSError:
                continue
            for n, line in enumerate(lines, 1):
                if CJK.search(line) and MARKER not in line:
                    hits.append("%s:%d %s" % (full, n, line.strip()[:80]))
    return scanned, exempted, excluded, hits


def bucket_of(path, root):
    """Top component RELATIVE to the scan root. Named bucket_of, not repo_of:
    this file already has a repo_of() and re-defining it silently broke the
    exemption rule (self-caught: 3/7 selftest cases flipped red)."""
    rel = os.path.relpath(path.split(":")[0], root)
    head = rel.split(os.sep)
    return head[0] if len(head) > 1 else "."


def tally(hits, root):
    """Every hit lands in exactly one bucket -> the parts MUST sum to the total.
    An implicit remainder bucket is how '10212 != 13623' happens: list them all."""
    out = {}
    for h in hits:
        r = bucket_of(h, root)
        out[r] = out.get(r, 0) + 1
    return sorted(out.items(), key=lambda kv: (-kv[1], kv[0]))


def run(root, quiet=False):
    scanned, exempted, excluded, hits = scan(root)
    if scanned == 0:
        print("CHECKER DEAD: scanned 0 files -> not clean, read nothing")
        return 3
    if not quiet:
        print("scanned %d source file(s), %d exempt, %d excluded(read-only spec), "
              "%d checked" % (scanned, exempted, excluded, scanned - exempted - excluded))
    if hits:
        for h in hits:
            print("RED: " + h)
        if not quiet:
            parts = tally(hits, root)
            print("RED unit: lines (one line = one hit)")
            for repo, n in parts:
                print("  %-24s %d" % (repo, n))
            print("  %-24s %d  (sum of the parts above, by construction)"
                  % ("TOTAL", sum(n for _, n in parts)))
        return 1
    if not quiet:
        print("OK: no unmarked CJK in non-exempt source")
    return 0


def selftest():
    import shutil
    import tempfile

    cases = [
        ("clean-en", {"anaphase-helix/src/a.rs": "fn main() {}\n"}, 0),
        ("zh-comment", {"anaphase-helix/src/a.rs": "// \u4e3a\u4ec0\u4e48\nfn main() {}\n"}, 1),
        ("exempt-zh", {"phyt-DNA/src/a.rs": "// \u4e2d\u6587\u5141\u8bb8\n"}, 0),
        ("marker-ok", {"anaphase-helix/src/a.rs": "// i18n-ok: user-facing \u4e2d\u6587\n"}, 0),
        ("empty", {"docs/readme.md": "hi\n"}, 3),
        ("nested-exempt", {"phyt-DNA/sub/src/a.rs": "// \u4e2d\u6587\n"}, 0),
        # all-exempt tree must still report scanned>0 (the exemption read-nothing bug)
        ("all-exempt-not-dead", {"phyt-DNA/src/a.rs": "// \u4e2d\u6587\n", "phyt-DNA/b.rs": "// x\n"}, 0),
    ]

    fails = 0
    cases.append(("excluded-repo", {"commonintents/foo/src/a.rs": "// \u4e2d\u6587\n"}, 0))
    for name, files, want in cases:
        d = tempfile.mkdtemp()
        try:
            for rel, body in files.items():
                p = os.path.join(d, rel)
                os.makedirs(os.path.dirname(p), exist_ok=True)
                io.open(p, "w", encoding="utf-8").write(body)
            got = run(d, quiet=True)
        finally:
            shutil.rmtree(d)
        ok = got == want
        if not ok:
            fails += 1
        print("%s %-20s want=%d got=%d" % ("PASS" if ok else "FAIL", name, want, got))
    print("selftest: %d/%d" % (len(cases) - fails, len(cases)))
    return 0 if fails == 0 else 1


def main():
    if "--selftest" in sys.argv:
        return selftest()
    root = sys.argv[1] if len(sys.argv) > 1 else ".."
    return run(root)


if __name__ == "__main__":
    sys.exit(main())
