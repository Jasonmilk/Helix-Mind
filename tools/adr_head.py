#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Parse an ADR header. ONE parser for this fact, committed, shared by callers.

Why this file exists: the ADR index was rebuilt by hand (F7) because no parser
had been committed. That left the index a second, hand-maintained copy of a
fact the files already carry — the same "one fact, several derivations" shape
that Cellrix:ADR-0018 exists to remove.

Two callers, one source:
  * the pre-commit gate, which rejects a commit whose ADR does not parse
  * any future index rebuild

Header shape it accepts (checked against every committed ADR):

    # ADR-0019: 事件流坐标与通道身份        <- hash required in new files
    - **状态**: **Proposed**（…）            <- status, optional
    - **日期**: 2026-09-15                   <- date, optional

Tolerant by design, in the direction that matters: a missing status or date is
None, not an error. An unparseable FIRST LINE is an error — that is the case
that catches a file replaced by something that is not an ADR.

Usage:
    python3 adr_head.py <file>...     # one JSON object per file; exit 1 on any bad
    from adr_head import parse        # or import it
"""
import io
import json
import re
import sys

# `# ADR-NNNN: title`. The hash is required (new files) but the separator may be
# an ASCII colon or a full-width one — both appear in the existing corpus.
HEAD = re.compile(r"^#\s*ADR-(\d{4})\s*[:：]\s*(\S.*?)\s*$")

# `- **状态**: value` / `- **日期**: value`. Field names are Chinese by the
# repo's convention for ADR bodies; the parser is the only place that knows it.
FIELD = re.compile(r"^-\s*\*\*(状态|日期)\*\*\s*[:：]\s*(.+?)\s*$")

FIELDS = {"状态": "status", "日期": "date"}


def parse(path):
    """Return the header dict, or None if the first line is not an ADR header."""
    with io.open(path, encoding="utf-8") as fh:
        lines = fh.read().split("\n")
    if not lines:
        return None
    m = HEAD.match(lines[0])
    if not m:
        return None
    out = {"number": m.group(1), "title": m.group(2), "status": None, "date": None}
    # the header block is short; stop at the first blank line after it
    for line in lines[1:40]:
        if line.strip() == "" and out["status"]:
            break
        f = FIELD.match(line)
        if f:
            out[FIELDS[f.group(1)]] = f.group(2)
    return out


def main(argv):
    bad = 0
    for path in argv[1:]:
        h = parse(path)
        if h is None:
            bad += 1
            first = ""
            try:
                with io.open(path, encoding="utf-8") as fh:
                    first = fh.readline().rstrip("\n")[:70]
            except OSError as e:
                first = "(unreadable: %s)" % e
            print(json.dumps({"file": path, "ok": False, "firstLine": first},
                             ensure_ascii=False))
        else:
            h["ok"] = True
            h["file"] = path
            print(json.dumps(h, ensure_ascii=False))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
