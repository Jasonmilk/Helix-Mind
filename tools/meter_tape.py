#!/usr/bin/env python3
"""Token meter over recorded tapes (post-hoc accounting, outside the subject).

Why this exists: the chain has no live metering inlet, but every reasoning round trip
already writes `assistant/usage` rows (prompt / completion / cached). Accounting after
the fact is therefore measurable; what is missing is pre-emptive blocking. This tool
only accounts - it never judges whether the spend was worth it (the instrument does not
read its own verdict).

Three states, never collapsed:
  measured       - usage rows and no rail marker -> summed
  rail (0)       - bypass=rail and no usage row    -> a real zero (no LLM call)
  inconsistent   - bypass=rail AND usage rows      -> a rail round cannot spend; counted, exit 4
  UNMEASURABLE   - neither rail nor usage          -> counted, never treated as 0

Exit codes: 0 = accounted; 1 = over --cap; 4 = UNMEASURABLE present (or no tapes).

Usage: meter_tape.py [--dir DIR] [--cap TOKENS] [--json] [--top N]
"""
import argparse, glob, json, os, sys


def classify(path):
    rows = []
    with open(path, encoding='utf-8', errors='replace') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except Exception:
                return ('unmeasurable', 0, {'reason': 'bad JSON line'})
    usage = [r.get('data') or {} for r in rows if r.get('type') == 'assistant/usage']
    rail_rows = [r for r in rows if r.get('type') == 'assistant/reply'
                 and (r.get('data') or {}).get('bypass') == 'rail']
    if usage and rail_rows:
        # A rail round makes no model call, so usage on the same tape is contradictory.
        return ('inconsistent', 0, {'reason': 'rail marker together with usage rows'})
    if usage:
        p = sum(int(u.get('prompt_tokens') or 0) for u in usage)
        c = sum(int(u.get('completion_tokens') or 0) for u in usage)
        k = sum(int(u.get('cached_tokens') or 0) for u in usage)
        models = sorted({str(u.get('model')) for u in usage})
        return ('measured', p + c, {'prompt': p, 'completion': c, 'cached': k,
                                    'calls': len(usage), 'models': models})
    if rail_rows:
        return ('rail', 0, {'reason': 'rail bypass: no model call, real zero'})
    return ('unmeasurable', 0, {'reason': 'no usage row and not a rail round'})


def _selftest():
    """Birth receipt: every state reachable, and one token of change must move the total."""
    import tempfile, shutil
    ok = []
    d = tempfile.mkdtemp(prefix='meter-selftest-')

    def tape(name, rows):
        os.makedirs(os.path.join(d, name), exist_ok=True)
        with open(os.path.join(d, name, 't.events.jsonl'), 'w', encoding='utf-8') as f:
            for r in rows:
                f.write(json.dumps(r, ensure_ascii=False) + '\n')
        return os.path.join(d, name)

    base = [{'type': 'turn/start', 'data': {}}, {'type': 'user/message', 'data': {'text': 'x'}}]
    t_meas = tape('measured', base + [{'type': 'assistant/usage',
                  'data': {'model': 'm', 'prompt_tokens': 100, 'completion_tokens': 20}}])
    t_rail = tape('rail', base + [{'type': 'assistant/reply', 'data': {'model': None, 'bypass': 'rail'}}])
    t_mixed = tape('inconsistent', base + [
        {'type': 'assistant/usage', 'data': {'model': 'm', 'prompt_tokens': 7, 'completion_tokens': 3}},
        {'type': 'assistant/reply', 'data': {'model': None, 'bypass': 'rail'}}])
    t_none = tape('unmeasurable', base + [{'type': 'assistant/reply', 'data': {'model': 'm', 'bypass': None}}])
    t_shift = tape('measured_shift', base + [{'type': 'assistant/usage',
                   'data': {'model': 'm', 'prompt_tokens': 101, 'completion_tokens': 20}}])

    def state(p):
        return classify(os.path.join(p, 't.events.jsonl'))

    ok.append(('measured', state(t_meas)[0] == 'measured' and state(t_meas)[1] == 120))
    ok.append(('rail zero', state(t_rail)[0] == 'rail' and state(t_rail)[1] == 0))
    ok.append(('rail+usage inconsistent', state(t_mixed)[0] == 'inconsistent'))
    ok.append(('no usage unmeasurable', state(t_none)[0] == 'unmeasurable'))
    ok.append(('differential sensitivity', state(t_meas)[1] != state(t_shift)[1]))
    shutil.rmtree(d, ignore_errors=True)
    bad = [n for n, good in ok if not good]
    for n, good in ok:
        print('  %-24s %s' % (n, 'ok' if good else 'FAIL'))
    print('selftest: %d/%d' % (len(ok) - len(bad), len(ok)))
    return 0 if not bad else 1


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--dir', default='/tmp/t2round/events')
    ap.add_argument('--cap', type=int, default=None)
    ap.add_argument('--json', action='store_true')
    ap.add_argument('--top', type=int, default=5)
    ap.add_argument('--selftest', action='store_true')
    a = ap.parse_args()
    if a.selftest:
        return _selftest()

    paths = sorted(glob.glob(os.path.join(a.dir, '*.events.jsonl')), key=os.path.getmtime)
    if not paths:
        print('UNMEASURABLE: no tape under %s' % a.dir)
        return 4

    per = []
    for p in paths:
        state, tokens, extra = classify(p)
        per.append({'period': os.path.basename(p).replace('.events.jsonl', ''),
                    'state': state, 'tokens': tokens, **extra})

    measured = [x for x in per if x['state'] == 'measured']
    rail = [x for x in per if x['state'] == 'rail']
    unmeas = [x for x in per if x['state'] == 'unmeasurable']
    incons = [x for x in per if x['state'] == 'inconsistent']
    total = sum(x['tokens'] for x in measured)

    if a.json:
        print(json.dumps({'tapes': len(per), 'measured': len(measured), 'rail': len(rail),
                          'unmeasurable': len(unmeas), 'inconsistent': len(incons), 'tokens': total,
                          'cap': a.cap, 'per_period': per}, ensure_ascii=False))
    else:
        print('盘点：tape %d 份 · measured %d · rail(真 0) %d · inconsistent %d · UNMEASURABLE %d'
              % (len(per), len(measured), len(rail), len(incons), len(unmeas)))
        print('合计 tokens = %d（prompt %d + completion %d · cached %d）'
              % (total, sum(x.get('prompt', 0) for x in measured),
                 sum(x.get('completion', 0) for x in measured),
                 sum(x.get('cached', 0) for x in measured)))
        for x in sorted(measured, key=lambda z: -z['tokens'])[:a.top]:
            print('  %-46s %6d tokens  calls=%d  %s'
                  % (x['period'][:46], x['tokens'], x.get('calls', 0), ','.join(x.get('models', []))))
        for x in incons[:a.top]:
            print('  INCONSISTENT %-34s %s' % (x['period'][:34], x.get('reason')))
        for x in unmeas[:a.top]:
            print('  UNMEASURABLE %-34s %s' % (x['period'][:34], x.get('reason')))
        if a.cap is not None:
            print('帽 = %d ⇒ %s' % (a.cap, 'RED 超帽' if total > a.cap else 'ok'))

    if a.cap is not None and total > a.cap:
        return 1
    if unmeas or incons:
        return 4
    return 0


if __name__ == '__main__':
    sys.exit(main())
