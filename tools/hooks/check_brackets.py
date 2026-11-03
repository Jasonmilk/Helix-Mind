#!/usr/bin/env python3
"""commit 信息里方括号必须配对 —— 「特殊字符被 shell 吃掉」已发生四次。

能判的交给机器：配对失衡 ⇒ 有文本被截断/吃掉。该不该写那句（语义）留给人。
Exit: 0 配对 / 1 失衡 / 3 读不到文件（不可量，也按失败处理：判不了就别放行）。
"""
import io
import sys

if len(sys.argv) < 2:
    sys.exit(3)
try:
    body = io.open(sys.argv[1], encoding="utf-8", errors="replace").read()
except OSError:
    sys.exit(3)
bal = 0
for ch in body:
    if ch == "[":
        bal += 1
    elif ch == "]":
        bal -= 1
    if bal < 0:
        break
sys.exit(0 if bal == 0 else 1)
