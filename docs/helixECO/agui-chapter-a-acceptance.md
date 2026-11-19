# 章 A 可复跑验收包（AG-UI 自有出口证轨）

> **这是备料，不是签字。** 三下由 gardener 自己按；本文件不代签、不写「已确认」。
> **它判不出什么**：三下里没有官方 schema 校验 ⇒ **判不出 8/8、不升格「符合官方协议」**（那是章 B 的量具）。
> 口径：签字唯一作数账 = `execution-queue.md` 的「L2 读数齐备未签」（2026-11-20 裁）；
> `governance/signoff.toml`（signed=true · 2026-11-03）标为**历史快照、不覆盖 queue 口径**，文件未改。

## 0｜准备一个 `<ROUND>`（不冻结、不用夹具）

`<ROUND>` = **当轮真跑产物目录**，至少两个文件：

```
<ROUND>/ring.json                                   # curl -s localhost:50061/v1/agent/events > ring.json
<ROUND>/events/<job_id>-<period_id>.events.jsonl    # 当轮 tape（/tmp 下最新那份）
```

取当轮 tape 与两个标识（两个都要，不要只记 job_id）：

```bash
curl -s -X POST localhost:50061/v1/chat -H 'content-type: application/json' \
  -d '{"message":"Please call the calc tool with expression 3*3 and report the result."}'
ls -t /tmp/t2round/events/*.events.jsonl | head -1     # ← 这一份就是 <ROUND>/events 的来源
python3 -c "import json,sys;d=json.loads(open(sys.argv[1]).readline());print(d['job_id'],d['period_id'])" <该 tape>
```

## 第一下｜重放带身份：同一 period 两次 diff=0，且不同 period sha 必不同

```bash
cd "${REPO_ROOT:-$HOME/Developer/Jasonmilk}/Cellrix"
R=<ROUND>; P=<period_id>
node tools/l2_casefile.js "$R/ring.json" "$R/events" ../anaphase-helix/src/run_cycle/mod.rs --expect "$P" > "$R/a.json"
node tools/l2_casefile.js "$R/ring.json" "$R/events" ../anaphase-helix/src/run_cycle/mod.rs --expect "$P" > "$R/b.json"
diff "$R/a.json" "$R/b.json" && echo "diff=0"
shasum -a 256 "$R/a.json" "$R/b.json"
```

- 期望：两条 sha 相同、`diff` 无输出。
- **口径**：`diff=0` 只证**同一份 tape 的确定性**；身份由 `--expect <period_id>` 钉住（钉不到 ⇒ rc=4，不猜）。
  要证"带身份"，还须**另一个 period 的 sha 与它不同**（见样张）。

## 第二下｜页面留白：缺口如实标，不假装完整

```bash
cd "${REPO_ROOT:-$HOME/Developer/Jasonmilk}/Cellrix"
node tools/d3_compare.js "$R/a.json" > "$R/d3.json"
node tools/l2_render.js "$R/a.json" "$R/d3.json" "$R/p.html"
grep -o 'UNMEASURABLE' "$R/p.html" | wc -l        # 缺什么就留几个空
grep -c '缺口如实标注' "$R/p.html"                 # 应有 1 处声明
```

- 期望：页面生成成功；`UNMEASURABLE` 数量 = **当轮真实缺项数**（不是 0 才叫好）；缺口声明在。
- **口径**：留白不读成"失败"，也不读成"完整"；`0` 就写 0。

## 第三下｜地面变化：退役面必须真的是 0

```bash
cd "${REPO_ROOT:-$HOME/Developer/Jasonmilk}/Cellrix"
git ls-files web | wc -l     # 期望 0
test -d web; echo $?         # 期望 1（目录不存在）
bash tools/accept_all.sh; echo "rc=$?"     # 默认跑**冻结夹具**：只证重放确定性，不是真跑验收
```

- 若要跑当轮形态：`ROUND_DIR=<ROUND> bash tools/accept_all.sh`（此时输入是当轮，不是夹具）。
- **口径**：`accept_all` 默认 rc=0 只说明夹具上的确定性与判据绿；**不当作真跑验收**。

## 附｜章 A 不覆盖的一项（供 gardener 参照，不算第三下）

```bash
cd "${REPO_ROOT:-$HOME/Developer/Jasonmilk}/Cellrix"
node tools/agui_conformance.js "$R/agui.jsonl"    # agui.jsonl = anaphase-helix/target/debug/agui <tape>
```

读数（2026-11-20 样张）：**8/8**（tool 轮）/ **5/5**（rail 轮）——
量具 = `agui_conformance.js` · 命令如上 · 输入 period 见样张 · 官方副本 sha256_16=**4b5c93226838a0e7**。
**这是章 B 的口径**，写在这里只为"同一份 tape 的另一路读数"，**不并入章 A 签字**。

## 样张读数（2026-11-20 · 供对照形态，不是通过证明）

```
<ROUND> = /tmp/chapterA（tape: run-93f71d7d5c9b397a-p006aff38c3000008.events.jsonl）
第一下：diff rc=0 · 0 字节 · sha=36c766bccd70201b（同 period 两次一致）
       另一个 period（run-6ce28faed7aa8e1f-p006aff379a000006）sha=4678ec0fbb2fd626（与上面不同 ⇒ 带身份成立）
第二下：页面生成 · 缺口标注 2 处 · UNMEASURABLE 0 处 · D3 行 6
第三下：git ls-files web=0 · test -d web ⇒ 1
附    ：agui_conformance 8/8（官方副本 sha 4b5c93226838a0e7）
```

## 边界

```
不代签 · 不写「已确认」· 不归档 · 不改 center · 不删队列 3 行 · 不动 DEATH-CERT · 页面留白保留
三下的判定权在 gardener；本包只提供可复跑命令 + 期望形态 + 口径限定
```
