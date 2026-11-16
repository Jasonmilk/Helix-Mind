# CI 长期红的四个根因与处置（2026-11-16）

> 结论先说：**没有一个是"测试写得差"**。四个都是"判据把单仓检出/文件格式的物理事实读错了"。

## ① Helix-Mind：workflow 文件本身无效 ⇒ 每次必红且 `jobs=0`

```
现象：最近 12 次 run 全 failure，`jobs total_count = 0`（不是测试红，是根本没跑）
真因：step 里写了 `run: echo "UNMEASURABLE: …"` —— YAML 的引号不是语法，`: ` 被当映射分隔
      ruby -ryaml ⇒ "mapping values are not allowed in this context at line 37 column 32"
处置：改 block scalar（`run: |` + 缩进），并把坑写进注释
读数：YAML OK · jobs=gate · steps=10 → 下次 run success
```

## ② Cellrix：引擎指纹失配

```
现象：step「引擎副本未漂（自签指纹）」failure
真因：改了 tools/determinism.sh（web 自测 → 具名 SKIP）却未重签
处置：维护者正门 `tools/upstream-manifest.sh --write`（4 → 7 个引擎文件）
读数：shasum -c rc=0 → run success
```

## ③ anaphase：行预算 300（三无阈值）当门

```
现象：step「① 测试」failure，本地同样是 2 条红
真因：300 无 ADR 来源、无校准、无所有者（ADR-0042 S.6），却被造成门
处置：删 `--budget`（含 default=300）与 `--budget-mode`；只报原始值 + ratchet 判增长
读数：ci_line_budget 18/18 · 全量 0 红
```

## ④ anaphase：单仓检出被当成"多仓违规"（连红四次的真凶）

```
现象：step「③ ADR 引用可解析」连续 4 次 failure
真因链：
  a. CI 只 checkout 本仓 ⇒ `Cellrix:ADR-0048 §319` 等 144 处无法解析
  b. 第一版按"兄弟仓数量 ≤1"判单仓 ⇒ repo_dirs() 注入 self alias ⇒ len=2 ⇒ 判定失效
  c. 第二版修好后仍红：CI 工作区里可能多一个临时目录 ⇒ 又被看成多仓
  d. `--selftest` 里 3 例拿 Cellrix 真 ADR 当参照物 ⇒ 单仓必然 FAIL（且 UnboundLocalError）
定论（物理事实优先）：判"仓不存在"要**看磁盘**，不看计数 —
  仓目录不存在        ⇒ 本次检出不可验 ⇒ 具名 UNMEASURABLE（不算违规）
  仓目录存在但无该 ADR ⇒ 真违规（写错仓名/号）
读数：单仓 clone（带陪衬目录）rc=0 且打印 UNMEASURABLE 144；
      mkdir 空 Cellrix ⇒ rc=1（106 处悬空节号被点名）；删掉 ⇒ rc=0
```

## 通用的四条（下次别再踩）

```
1. `jobs=0` 优先怀疑 workflow 文件无效（YAML），不是测试 —— 本地 ruby -ryaml 一秒可证
2. 单仓检出是物理事实：跨仓判据必须"看磁盘"，且不可量要具名，不能判红
3. 改了被指纹覆盖的文件，必须走正门重签（并说明分叉），否则 CI 必红
4. `|| true` / `: ` / 兄弟仓计数 —— 这三类都在"把不可量编译成合法值"（同一族）
```

## 当前 CI 状态（2026-11-16 21:00 UTC）

```
anaphase-helix  ce1744d  success
Helix-Mind      3baab56  success
Cellrix         f2cb52f  success
Tuck            c833034  success（11-03）
FlowModus       9359a8f  success（10-25）
phyt-DNA        5f017fa  success（11-03）
helix-tentacle  no runs（无 workflow 触发）
```
