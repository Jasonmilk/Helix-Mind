# tools — 跨仓共用的小工具

## `adr_head.py` —— ADR 头部的**唯一**解析器

提取 `# ADR-NNNN: 标题` 与 `- **状态**: …` / `- **日期**: …`。

**为什么单独入库**：ADR 索引曾被**手工重建**（F7），因为没有解析器可复用 ——
于是索引成了同一事实的第二份手工副本，正是 `Cellrix:ADR-0018` 要治的病。
现在闸门与将来的索引重建**共用这一份**。

```bash
python3 adr_head.py <file>...     # 每个文件一行 JSON；任一不可解析则退出码 1
```

## `hooks/pre-commit` —— 两道闸门

1. **ADR 首行必须可解析** —— 2026-09-15 有 ADR **三次**被整份替换成聊天回复正文，
   每次首行都露了馅。
2. **单次 diff 超过 50 行直接拒绝** —— 第三次事故的真实失效点是
   **作者看到了 `+62/-258` 却没有停**，因为**没有任何机制强制停**。
   确认是大改动时：`SKIP_LARGE_DIFF=1 git commit …`

两道闸门都 **fail-open**：解析器或 `python3` 缺失时**警告并放行**，
不阻断提交。一个拒绝一切的闸门 = 覆盖率 100%、断言为零的测试。

## `install-hooks.sh` —— 把 `core.hooksPath` 指向 `hooks/`

`.git/hooks/` 不受版本控制，所以要装到受控目录；五个仓逐一手工设置是会被遗忘的步骤，故用脚本。

```bash
./install-hooks.sh            # 装到所有已知仓
./install-hooks.sh --check    # 只看状态
```

---

## ⚠️ 状态：**尚未完成双向验证，暂勿 install**

已验（单向）：

| 闸门 | 证据 |
|---|---|
| gate1 true positive | 回复正文 ⇒ 拒绝，退出码 1 ✓ |
| gate1 true negative | 合法 ADR ⇒ 通过，退出码 0 ✓（正确解出 number/title/status/date） |

**未验（缺）：**

- gate1 尚未用**真实被污染的文件**测过（此前用的是手写字符串）
- **gate2 从未成功拦截过** —— 一次 80 行 diff 的提交**直接通过了**
- gate2 未用「提交前后 `git rev-list --count` 对比」确认它真的拦住了

**⇒ 在双向验证完成前，不要跑 `install-hooks.sh`。**
