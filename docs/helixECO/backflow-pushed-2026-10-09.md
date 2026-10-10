# ★ 回流已推上 phyt-DNA v2 —— 复利落地

> 2026-10-09 ｜ 人类："直接推 v2 就好！本地备份可以删了！README.zh-CN.md 暂不更新！"
> ｜ **已完成并推送** ｜ `github.com/Jasonmilk/phyt-DNA` 分支 `v2`

## 一、推了什么

```
8538876  →  da2465b   （v2 分支）
[T7] feat(backflow): 采用者 Cellrix 首次真实使用 phyt-DNA v2 的七条回流
check-baseline.sh: PASS（13 个文件基线已刷新）
```

| 回流条 | 落到哪 |
|---|---|
| **P2** 路径声明式 | `tools/validate.sh` —— **沿用本仓既有惯例**：新增 `PHYT_DECISIONS` / `PHYT_FIXTURES`（`PHYT_LEDGER` 本来就有） |
| **P3** 零闸门具名 BLOCK | 同文件（空集合上的"全部通过"是伪证） |
| **P5 / 假-gate 修** | `gates()` 只认 `ADR-*.md`（文档里的 YAML 示例含 `hard: true` 时 README 会被当成闸门） |
| **P11** 测电仪 | `tools/validate.sh`（probe 写账本 `kind:probe` · `--probe-all` · 反例夹具 `counter.sh`）+ **`docs/MULTIMETER.md`（新）** |
| P11 文档同步 | `VISION.md` · `docs/PROTECTION.md` · `README.md` · `ledger/README.md` · `fixtures/README.md` |

**未动**：`--cull`（法官 T7）· `^hard: true$` 锚定匹配 —— **远端比我新的部分一律保留。**

## 二、★ 第一次 `--probe-all` 的结果：**本仓自己不合规，而此前没人知道**

```
6 个闸门全部 RED（都活着）✅
但【每个】都报告：无反例夹具 ⇒ 都只证了"该红的红"
并且三条闸门报出本仓自身的不合规：
  PLAN.md 177 行 > 150（它自己的上限）
  layer1 三层合计 326 行 > 280（DNA+RNA+SPEC）
  装饰性检查残留（教训未落地到工件）: examples/phyt.yml
```

**⇒ 这些一直存在**。此前是**逐个 `--probe`**，**没有一次跑全部** ⇒ **漂移没有现形。**
**⇒ 这正是 `--probe-all`（P11）的第一份价值：它让一组闸门的整体状态可见。**

## 三、★ 移植过程中发现的更重要的事：**我差点推错**

```
远端 v2：方法论文件在【根目录】· tools/ 有 baseline.sha256 · fixtures/ 6 个闸门 · ledger/ 有 appeals · README 中英双版
本地旧快照：方法论文件在 template/ 下 · fixtures/ 1 个 · 无基线
```

**⇒ 本地 `phyt-DNA/` 是**旧世代**快照。若不先克隆比对就直接推，会把旧布局**覆盖到真仓**。**
**⇒ 教训（可入册）："提交推送 X"之前，先确认 X **是一个仓**、以及**本地与远端是不是同一世代**。**

**⇒ 处置（已做）**：删旧快照 ⇒ **用真克隆替换**（`Jasonmilk/phyt-DNA/`，分支 `v2` @ `da2465b`）
⇒ **以后改的就是真仓，不会再犯同一个错。**

## 四、仍未做（明说）

- **`README.zh-CN.md` 未更新**（人类指示暂不更新）⇒ 中英两版**目前不同步**（英文有第二根轴一节，中文没有）
- **本仓自身的三点不合规未修**（`PLAN.md` 超长 · `layer1` 超长 · `examples/phyt.yml` 残留）——
  **那是 phyt-DNA 自己的编辑决定，我不擅动**
- **6 个闸门都缺反例夹具**（`counter.sh`）⇒ 要不要补，是 phyt-DNA 的选择
- `fixtures/` 里没有 `counter.sh` 的闸门会被**具名**跳过（P11 机制），**不会静默算通过** ✅
