#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""safe_edit.py —— 原子写 + 写后自检。★ 为什么存在（2026-10-10 实测事故）：
`io.open(p,'w')` **打开即截断**；若脚本在 open 与 write 之间抛异常（语法/类型错误），
文件被留成 **0 字节**，且 `git add -A` 会把这场灾难**提交**（本轮实测：一个文件被毁 3 笔才被发现）。
⇒ 规则：**先写临时文件 · 过自检 · 再原子改名**；自检不过就**原封不动**。
用法：from safe_edit import safe_write; safe_write(path, text, must_contain=['# 标题'], min_bytes=100)
"""
import io, os, sys, tempfile

def safe_write(path, text, must_contain=(), min_bytes=1):
    data = text.encode('utf-8')
    if len(data) < min_bytes:
        sys.exit('★ 自检失败（太短，拒绝写入）: %s = %d 字节 < %d' % (path, len(data), min_bytes))
    for anchor in must_contain:
        if anchor not in text:
            sys.exit('★ 自检失败（锚点缺失，拒绝写入）: %s 里没有 %r' % (path, anchor))
    d = os.path.dirname(os.path.abspath(path))
    fd, tmp = tempfile.mkstemp(dir=d, prefix='.safe_edit.')
    try:
        with os.fdopen(fd, 'wb') as fh:
            fh.write(data)
        os.replace(tmp, path)          # 原子：读者要么看旧、要么看新，绝不见半截
    except BaseException:
        os.path.exists(tmp) and os.unlink(tmp)
        raise
    got = os.path.getsize(path)
    if got != len(data):
        sys.exit('★ 回读不一致: %s 期望 %d 实得 %d' % (path, len(data), got))
    print('  ✅ safe_write: %s ⇒ %d 字节（自检通过）' % (path, got))
    return got
