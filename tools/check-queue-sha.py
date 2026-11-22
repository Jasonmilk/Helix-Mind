#!/usr/bin/env python3
"""Check the ledger's *effective* artefacts against their recorded sha16.

Scope: read-only. It reads one ledger file and verifies lines of the fixed form

    - 生效：<relative path> sha16=<16 hex>

Nothing else is parsed: legacy sha spellings stay snapshots and are not judged.
A section without such a line is a point-in-time snapshot, never "stale".

Exit codes: 0 = every recorded artefact matches; 1 = at least one mismatch;
            4 = no effective line found (UNMEASURABLE, never "all green").

Usage: check-queue-sha.py [--queue PATH] [--root DIR]
"""
import sys, os, re, hashlib

LINE = re.compile(r'^-\s*生效：\s*(\S+)\s+sha16=([0-9a-f]{16})\s*$')


def sha16(path):
    h = hashlib.sha256()
    with open(path, 'rb') as f:
        for chunk in iter(lambda: f.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()[:16]


def main(argv):
    queue = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'docs', 'helixECO', 'execution-queue.md')
    root = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
    i = 1
    while i < len(argv):
        if argv[i] == '--queue':
            queue = argv[i + 1]; i += 2
        elif argv[i] == '--root':
            root = argv[i + 1]; i += 2
        else:
            print('unknown argument: %s' % argv[i]); return 2

    entries, scanned = [], 0
    if not os.path.exists(queue):
        print('UNMEASURABLE: queue not found: %s' % queue)
        return 4
    with open(queue, encoding='utf-8') as f:
        for line in f:
            scanned += 1
            m = LINE.match(line.rstrip('\n'))
            if m:
                entries.append((m.group(1), m.group(2)))

    if not entries:
        # Zero hits must be reported as zero hits, never as "nothing is stale".
        print('%d hits / %d scanned  =>  UNMEASURABLE (no effective line in this ledger)' % (len(entries), scanned))
        return 4

    ok, stale, unmeas = 0, 0, 0
    for rel, want in entries:
        p = rel if os.path.isabs(rel) else os.path.join(root, rel)
        if not os.path.exists(p):
            print('UNMEASURABLE  %-52s  (file missing)' % rel); unmeas += 1; continue
        got = sha16(p)
        if got == want:
            print('consistent    %-52s  sha16=%s' % (rel, got)); ok += 1
        else:
            print('STALE         %-52s  recorded=%s actual=%s' % (rel, want, got)); stale += 1

    print('%d hits / %d scanned  =>  consistent %d · stale %d · UNMEASURABLE %d' % (len(entries), scanned, ok, stale, unmeas))
    if stale:
        return 1
    if unmeas:
        return 4
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
