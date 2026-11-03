#!/usr/bin/env python3
"""J1/J2/J3' for governance/terms.toml (TERM-SPEC v1).

J1 重抄 diff   : every gene name in phyt-DNA/DNA.md has an entry, and no entry invents one.
                 gene = a bold heading WITH a non-empty body. That rule is mechanical; it is
                 what tells 10 genes from the 11th heading ("此伤疤已结痂", empty body).
J2 一词一义    : no zh appears twice, no en maps to two ids  (ISO 704: one entry per meaning).
J3' 双面非空   : zh_def and en_def both non-empty.
                 两面是同一概念的两个投影 ⇒ 这里只判"都在",不判"是否互译"(机器判不了,软记录).

Exit: 0 clean / 1 violation / 3 read nothing.
"""
import io, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TERMS = os.path.join(ROOT, "governance", "terms.toml")
DNA = os.path.join(ROOT, "..", "phyt-DNA", "DNA.md")


def genes():
    text = io.open(DNA, encoding="utf-8").read()
    blocks = {}
    for m in re.finditer(r"^\*\*(.+?)\*\*\s*\n(.*?)(?=^\*\*|\Z)", text, re.M | re.S):
        blocks[m.group(1)] = m.group(2).strip()
    return {k for k, v in blocks.items() if v and not re.search(r"[，。；]", k)}


def terms():
    out, cur = [], None
    for line in io.open(TERMS, encoding="utf-8"):
        line = line.strip()
        if line == "[[term]]":
            cur = {}
            out.append(cur)
        elif cur is not None and "=" in line and not line.startswith("#"):
            k, _, v = line.partition("=")
            cur[k.strip()] = v.strip().strip('"')
    return out


PLACEHOLDER = ("todo", "同上", "任何", "相关", "需要时", "tbd", "xxx")


def admission(t):
    """准入层：歧义合并**从软问题变成可判状态**（2026-11-03）。

    实测反例（为什么仅靠'双面非空'不够）：`Unavailable` = 「没有应答者或应答者失败」
    两面都非空、en→id 唯一，却把两个处置相反的概念合并在一个名字里 ⇒ 必须红。
    """
    bad = []
    tid = t.get("id", "?")
    for face in ("zh_def", "en_def"):
        v = (t.get(face) or "").strip()
        if not v:
            bad.append("H2 双面之一为空: %s/%s" % (tid, face))
        elif any(w in v.lower() for w in PLACEHOLDER):
            bad.append("H2 定义面是占位/宽词: %s/%s = %r" % (tid, face, v[:40]))
    atoms = t.get("concept_atoms", "1")
    try:
        atoms_n = int(atoms)
    except ValueError:
        bad.append("H2 concept_atoms 不是整数: %s = %r" % (tid, atoms))
        atoms_n = 1
    split_into = t.get("split_into", "[]")
    has_split = bool(split_into and split_into not in ("[]", ""))
    if atoms_n > 1 and not has_split:
        bad.append("H2 polysemy_not_split: %s 有 %d 个概念原子但 split_into 为空" % (tid, atoms_n))
    amb = t.get("ambiguity_status", "")
    if amb not in ("clear", "unresolved"):
        bad.append("H3 ambiguity_status 非法: %s = %r（只许 clear|unresolved）" % (tid, amb))
    if amb == "unresolved" and t.get("status") == "admitted":
        bad.append("H3 admitted 却是 unresolved: %s —— 未决只能 draft" % tid)
    if not (t.get("alignment_note") or "").strip():
        bad.append("H3 缺 alignment_note（两面互译/消歧的审读记录）: %s" % tid)
    return bad


def main():
    g, t = genes(), terms()
    if not g or not t:
        print("CHECKER DEAD: genes=%d terms=%d -> read nothing" % (len(g), len(t)))
        return 3
    bad = []
    by_zh = {x.get("zh") for x in t}
    for name in sorted(g - by_zh):
        bad.append("J1: DNA 里的基因名没有 entry: %s" % name)
    for name in sorted(by_zh - g):
        bad.append("J1: entry 发明了 DNA 里没有的名: %s" % name)
    seen_zh, seen_en = {}, {}
    for x in t:
        if x.get("zh") in seen_zh:
            bad.append("J2: 显示名重复: %s" % x.get("zh"))
        seen_zh[x.get("zh")] = x.get("id")
        if x.get("en") in seen_en and seen_en[x["en"]] != x.get("id"):
            bad.append("J2: en 映射到两个 id: %s" % x.get("en"))
        seen_en[x.get("en")] = x.get("id")
        if not x.get("zh_def") or not x.get("en_def"):
            bad.append("J3': 双面之一为空: %s" % x.get("id"))
    for x in t:
        bad.extend(admission(x))
    print("terms %d · DNA genes %d · admitted_unresolved %d · draft_unresolved %d" % (
        len(t), len(g),
        sum(1 for x in t if x.get("ambiguity_status") == "unresolved" and x.get("status") == "admitted"),
        sum(1 for x in t if x.get("ambiguity_status") == "unresolved" and x.get("status") != "admitted"),
    ))
    for b in bad:
        print("★ " + b)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
