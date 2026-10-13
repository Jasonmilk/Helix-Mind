#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""safe_edit.py —— 原子写 + 写后自检。★ 为什么存在（2026-10-10 实测事故）：
`io.open(p,'w')` **打开即截断**；若脚本在 open 与 write 之间抛异常（语法/类型错误），
文件被留成 **0 字节**，且 `git add -A` 会把这场灾难**提交**（本轮实测：一个文件被毁 3 笔才被发现）。
⇒ 规则：**先写临时文件 · 过自检 · 再原子改名**；自检不过就**原封不动**。
用法：from safe_edit import safe_write; safe_write(path, text, must_contain=['# 标题'], min_bytes=100)
"""
import stat  # ★ 模式保留：丢执行位是【哑失败】（只用 bash 跑时永远看不见）
import io, os, sys, tempfile

def safe_write(path, text, must_contain=(), min_bytes=1, allow_existing=False):
    # ★ 9（2026-10-13）：**改严** —— 两次真实翻车（替换静默没落地；`must_contain` 命中**旧行**从而"证明"了没发生的事）
    #   ① 新内容与旧内容**逐字节相同** ⇒ 拒绝（"替换未落地"必须红，不许静默成功）
    #   ② `must_contain` 的锚点在**旧内容里就有** ⇒ 拒绝（它证明不了新内容落地；确有正当场合请显式 `allow_existing=True`）
    #   更好的是用本模块已有的 `patch(old,new,must_contain_new,expect_old_count)` —— 它对旧文计数也算。
    old_text = None
    try:
        old_text = io.open(path, encoding='utf-8').read()
    except OSError:
        pass
    data = text.encode('utf-8')
    if len(data) < min_bytes:
        sys.exit('★ 自检失败（太短，拒绝写入）: %s = %d 字节 < %d' % (path, len(data), min_bytes))
    if old_text is not None and old_text == text:
        sys.exit('★ 自检失败（**替换未落地**：新内容与旧内容逐字节相同）: %s（改了半天没改到东西 ⇒ 必红）' % path)
    for anchor in must_contain:
        if anchor not in text:
            sys.exit('★ 自检失败（锚点缺失，拒绝写入）: %s 里没有 %r' % (path, anchor))
        if old_text is not None and anchor in old_text and not allow_existing:
            sys.exit('★ 自检失败（**断言无效**：校验串 %r 在**旧内容里就有** ⇒ 它不能证明新内容落地）: %s'
                     '（若确为正当场合，显式 allow_existing=True）' % (anchor, path))
    d = os.path.dirname(os.path.abspath(path))
    fd, tmp = tempfile.mkstemp(dir=d, prefix='.safe_edit.')
    try:
        with os.fdopen(fd, 'wb') as fh:
            fh.write(data)
        _mode = None
        try:
            _mode = stat.S_IMODE(os.stat(path).st_mode)   # 原文件模式（os.replace 后 tmp 是默认 umask ⇒ 丢执行位）
        except OSError:
            pass
        os.replace(tmp, path)          # 原子：读者要么看旧、要么看新，绝不见半截
        if _mode is not None:
            os.chmod(path, _mode)
    except BaseException:
        os.path.exists(tmp) and os.unlink(tmp)
        raise
    got = os.path.getsize(path)
    if got != len(data):
        sys.exit('★ 回读不一致: %s 期望 %d 实得 %d' % (path, len(data), got))
    print('  ✅ safe_write: %s ⇒ %d 字节（自检通过）' % (path, got))
    return got


def patch(path, old, new, min_bytes=1, must_contain_new=(  ), expect_old_count=1):
    """★ assert-then-replace（reviewer ⑤ · 2026-10-12）：**先断言锚点存在，再替换**。

    为什么存在（实测根因，不是"记得用 grep -n"）：补丁脚本里 `s.replace(old, new, 1)`
    在 `old` **不匹配**时是**静默 no-op** —— "没写进去"被当成"写了"；而 safe_write 的自检
    **确实报警了**，但调用方没把它当**阻断**。⇒ 本函数把两件事焊成一件：
      ① `old` 必须存在（可指定出现次数）⇒ 否则 **SystemExit**（整链中断）
      ② 替换后 `new` 必须在、`old` 必须不在 ⇒ 否则 **SystemExit**
      ③ 之后走 `safe_write`（原子写 + 回读 + 锚点自检）
    """
    import io as _io
    s = _io.open(path, encoding='utf-8').read()
    n = s.count(old)
    if n != expect_old_count:
        raise SystemExit('★ patch 中断：%s 里 `%s` 出现 %d 次（期望 %d）—— **锚点不匹配，拒绝静默 no-op**'
                         % (path, old[:60], n, expect_old_count))
    s2 = s.replace(old, new, expect_old_count)
    if new not in s2:
        raise SystemExit('★ patch 中断：替换后新串未出现（%s）' % path)
    if old in s2:
        raise SystemExit('★ patch 中断：替换后旧串仍在（%s）' % path)
    got = safe_write(path, s2, must_contain=tuple(must_contain_new) or (new[:24],), min_bytes=min_bytes)
    print('  ✅ patch: %s ⇒ %d 字节（旧串已消失 · 新串已在位）' % (path, got))
    return got
