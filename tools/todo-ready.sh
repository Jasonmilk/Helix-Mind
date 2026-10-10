# ★ 欠账进报告头（2026-10-10）：不进报告头就永远靠外挂记忆背 —— 而外挂是人的，不是生态的。
_report_field_debts() {
  local f="$ROOT/docs/helixECO/DEBTS.md"
  [ -f "$f" ] || { echo "▸ 欠账：—（缺 DEBTS.md）"; return 0; }
  local open_
  open_=$(grep -cE '\| (⬜|⏳|⏸)' "$f" 2>/dev/null)
  [ -n "$open_" ] || open_=0
  echo "▸ 欠账（**未销号 ${open_} 条** · 真源 docs/helixECO/DEBTS.md）："
  # 列序（2026-10-10 加 K 号列）：项 | K号/非K理由 | 挂账起 | 状态 | 解锁条件 | 归属
  grep -E '\| (⬜|⏳|⏸)' "$f" 2>/dev/null | while IFS='|' read -r _ item k since st cond owner _r; do
    printf '    %-38s [%s] 起 %-11s 归属 %s\n' \
      "$(printf %s "$item" | sed 's/^ *//;s/ *$//')" "$(printf %s "$k" | sed 's/^ *//;s/ *$//')" \
      "$(printf %s "$since" | sed 's/^ *//;s/ *$//')" "$(printf %s "$owner" | sed 's/^ *//;s/ *$//')"
  done
}

#!/usr/bin/env bash
# todo-ready.sh —— 【候选期租客】：**只读的顾问**。回答："现在能做的是什么 · 该改哪一行"。
#
# ★ 为什么是租客不是引擎（reviewer 2026-10-10，我认）：
#   · **痛点未实证**：本生态每台仪器都事故催生（sync-upstream 生于 CI 红 · redrate 生于四轮欠条 · §0 生于清单漂移）；
#     而"它要治的病"**刚被 §0 处理完、还没失败记录** ⇒ **给还没失败的制度造替代品 = 仪器厂引力回归。**
#   · ⇒ **不进 phyt-DNA 引擎层**（无入户资格：不入 UPSTREAM.sha256、不进 manifest、不参与六仓同步）。
#   · ⇒ **只读顾问**（按我们自己的授权三级守则：**零信用记录的工具从"顾问"做起**；`--done` 只打印"改哪一行怎么改"）。
#
# ★ 防腐三条（reviewer 2026-10-10 二，写在代码里而不只写在文档里）：
#   ① **入户资格靠房租（使用记录），不靠长相（设计）**：两周日志不达标 ⇒ 删，没有"挺好看留下来"这条。
#   ② **严格分层**：本脚本**只是视图**，真相永远在 `KNOWN_ISSUES` ⇒
#      **若哪天发现自己在脚本里"补数据"，就是造了第六真相 ⇒ 即刻冻结。**
#   ③ **冻结期不偷跑**：两周内不进引擎/UPSTREAM、不做写回、不扩字段。
#
# ★ 解冻条款：docs/helixECO/PROPOSAL-workitem-dag.md（自 2026-10-10 起算）；使用日志：docs/helixECO/USAGE-candidate-tools.md
set -uo pipefail
# ★ 自解仓库根（2026-10-10）：**在任意 cwd 都能用** —— 否则"报告头贴读数"这条腿会在别处静默为空。
ROOT=$(cd "$(dirname "$0")/.." && pwd)
ISSUES="${PHYT_ISSUES:-$ROOT/docs/helixECO/KNOWN_ISSUES.md}"
[ -f "$ISSUES" ] || { echo "★ 找不到登记册：$ISSUES" >&2; exit 2; }

case "${1:-}" in
  --report-head)
    # ★ D9（reviewer 2026-10-10）：**报告头由生成器产出** —— "视图不该靠记，该靠生成"。
    #   诊断来源：读数二连断（我只贴了矩阵），说明**每一轮都靠我记得贴 → 一定漏**。
    #   它把三件拼在一起：① 读数（本脚本）② CI 矩阵（引擎工具）③ D/R 表的桩（从登记册现取）。
    echo "════════ 报告头（生成，非手写）· $(date -u +%Y-%m-%dT%H:%M:%SZ) ════════"
    # ★ D13（reviewer 2026-10-10，D2 移植）：**生成器任何一段挂 ⇒ 整体非零 + 具名哪段** ——
    #   **绝不允许"半截报告头"静默流出**（半截最危险：它看起来像完整的，只是少了点东西）。
    _fail=0
    if ! bash "$ROOT/tools/todo-ready.sh" 2>/dev/null; then
      echo "★ 段【读数】失败（--report-head 不完整）" >&2; _fail=1
    fi
    # ★★ 补丁一（reviewer 2026-10-10）：**SYNC 行也由生成器算** —— 凡是能生成的，不靠记。
    #   实测教训：我写过"六仓 SYNC"而实际只有 2 仓动过（口径轻微回退）。
    #   做法：把上一轮各仓 HEAD 存进**仓库之外的快照**（与使用日志同处，可丢弃），
    #   本轮用 `git rev-parse` 逐个比对 ⇒ 机械算出"本轮动了 X 仓，哪几个"。
    # ★ 快照进仓（reviewer 2026-10-10 三刺②）：原先放 $HOME ⇒ 跨环境漂移、不可复现
    #   ⇒ 改为**仓内 `.helix/`**（随 checkout 走；已在 .gitignore 里）
    SNAP="${HELIX_HEAD_SNAPSHOT:-$ROOT/.helix/report-head-last.txt}"
    mkdir -p "$(dirname "$SNAP")" 2>/dev/null || true
    changed=""; total=0
    for x in anaphase-helix helix-mind Cellrix FlowModus Tuck phyt-DNA; do
      [ -d "$ROOT/../$x/.git" ] || continue
      total=$((total+1))
      now=$(git -C "$ROOT/../$x" rev-parse --short=7 HEAD 2>/dev/null)
      was=$(grep "^$x " "$SNAP" 2>/dev/null | awk '{print $2}')
      [ -n "$was" ] && [ "$now" != "$was" ] && changed="$changed $x($now)"
    done
    if [ -z "$changed" ]; then
      printf '▸ 本轮动了：**0 仓**（与上一轮快照比对得出）· 共 %s 仓
' "$total"
    else
      printf '▸ 本轮动了：**%s 仓** ⇒%s\n' "$(echo $changed | wc -w | tr -d ' ')" "$changed"
      # ★ 内容收据机械化（reviewer 2026-10-10 旗二）：每笔的 commit 首行**自动带出**（git log 就有）
      #   ⇒ 不再靠"我记得写"（b29182b 有、33e00ab 又漏 = 靠记性不是机制）
      for x in $changed; do
        printf '    %-16s %s\n' "$x" "$(git -C "$ROOT/../$x" log -1 --format=%s 2>/dev/null | cut -c1-88)"
      done
    fi
    : > "$SNAP"
    for x in anaphase-helix helix-mind Cellrix FlowModus Tuck phyt-DNA; do
      [ -d "$ROOT/../$x/.git" ] || continue
      printf '%s %s\n' "$x" "$(git -C "$ROOT/../$x" rev-parse --short=7 HEAD 2>/dev/null)" >> "$SNAP"
    done
    # ★ 小件①（reviewer 2026-10-10）：**光的哈希**一行 —— 口径声明为 `git hash-object` 前 7 位
    #   （**不碰 VISION.md 内容**）；光变了，报告第一屏就能看见。
    if [ -f "$ROOT/../phyt-DNA/VISION.md" ]; then
      vh=$(cd "$ROOT/../phyt-DNA" && git hash-object VISION.md 2>/dev/null | cut -c1-7)
      printf '\n▸ 光：vision@%s（git hash-object 前 7 位 · phyt-DNA/VISION.md；内容一字不改是它的价值）\n' "${vh:-unknown}"
      # ★ 光已更新标记（2026-10-10）：与上轮快照比对 ⇒ 变了就【连打几轮】（光动了要让每滴水知道）
      vsnap="${HELIX_VISION_SNAPSHOT:-$ROOT/.helix/vision-last.txt}"
      mkdir -p "$(dirname "$vsnap")" 2>/dev/null || true
      vprev=$(cat "$vsnap" 2>/dev/null || true)
      if [ -n "$vprev" ] && [ "$vprev" != "$vh" ]; then
        printf '▸ ★ **光已更新**：vision@%s → vision@%s（本轮起连续几轮标注；判读方向可能随之改变）\n' "$vprev" "$vh"
      fi
      printf '%s' "$vh" > "$vsnap"
    fi
    # ★ 小件②：**评审倒计时**（冻结期自 2026-10-10 起算两周）—— 读数回归，不靠记
    if [ -f "$ROOT/docs/helixECO/USAGE-candidate-tools.md" ]; then
      n=$(grep -c '^| 2026-' "$ROOT/docs/helixECO/USAGE-candidate-tools.md" 2>/dev/null || echo 0)
      act=$(grep -c '有\*\*' "$ROOT/docs/helixECO/USAGE-candidate-tools.md" 2>/dev/null || echo 0)
      read -r end left <<< "$(python3 -c "
import datetime
d=datetime.date(2026,10,10)+datetime.timedelta(days=14)
print(d.isoformat(), (d-datetime.date.today()).days)" 2>/dev/null)"
      printf '▸ 候选工具评审：截止 %s（还剩 %s 天）· 使用日志 %s 条 · 行动变化=有 %s 条\n' "${end:-?}" "${left:-?}" "$n" "$act"
    fi

    _report_field_debts
    # ★★ 必出七段【自证】(D15 补丁 · 2026-10-10)：为什么需要它 ——
    #   实测翻案：评审说"光/倒计时/USAGE 掉了"，而它们**一直在生成器里**；
    #   掉的是我的**呈现**（每次都 `head -14` 截断后才贴）⇒ **观察者的取景框造出了假缺口**。
    #   解法：**生成器自报齐不齐** ⇒ 任何残片贴上时**自暴不全**，无需人去比对字段清单。
    printf '▸ 必出七段自证：光=%s 倒计时=%s USAGE=%s 能做/等人/不许动=%s 本轮动了=%s 矩阵=%s 欠账=%s\n' \
      "$([ -f "$ROOT/../phyt-DNA/VISION.md" ] && echo ✅ || echo —)" \
      "$([ -f "$ROOT/docs/helixECO/USAGE-candidate-tools.md" ] && echo ✅ || echo —)" \
      "$([ -f "$ROOT/docs/helixECO/USAGE-candidate-tools.md" ] && echo ✅ || echo —)" \
      "$([ -f "$ROOT/docs/helixECO/KNOWN_ISSUES.md" ] && echo ✅ || echo —)" \
      "$([ -n "$SNAP" ] && echo ✅ || echo —)" \
      "$([ -x "$ROOT/../phyt-DNA/tools/ci-matrix.sh" ] && echo ✅ || echo —)" \
      "$([ -f "$ROOT/docs/helixECO/DEBTS.md" ] && echo ✅ || echo —)"
      "$([ -f "$ROOT/../phyt-DNA/VISION.md" ] && echo ✅ || echo ★缺)" \
      "$([ -f "$ROOT/docs/helixECO/USAGE-candidate-tools.md" ] && echo ✅ || echo ★缺)" \
      "$([ -f "$ROOT/docs/helixECO/USAGE-candidate-tools.md" ] && echo ✅ || echo ★缺)" \
      "$([ -f "$ROOT/docs/helixECO/KNOWN_ISSUES.md" ] && echo ✅ || echo ★缺)" \
      "$([ -f "$SNAP" ] && echo ✅ || echo ★缺)" \
      "$([ -x "$ROOT/../phyt-DNA/tools/ci-matrix.sh" ] && echo ✅ || echo ★缺)"
    if [ -x "$ROOT/../phyt-DNA/tools/ci-matrix.sh" ]; then
      echo
      if ! bash "$ROOT/../phyt-DNA/tools/ci-matrix.sh"; then
        echo "★ 段【CI 矩阵】失败（--report-head 不完整）" >&2; _fail=1
      fi
    else
      echo "★ 段【CI 矩阵】缺失：找不到 phyt-DNA/tools/ci-matrix.sh" >&2; _fail=1
    fi
    echo; echo "▸ 收件箱（R/D）与未修（K）—— 逐条引用；状态取自登记册"
    python3 -c "
import io,re,sys
s=io.open(sys.argv[1],encoding='utf-8').read()
for sec,pat in (('R（审查跟进）',r'^\| \*\*(R[0-9]+)\*\*'),('D（D 系列）',r'^\| \*\*(D[0-9]+)\*\*'),('K（未修）',r'^\| \*\*(K[0-9]+)\*\* \|')):
    rows=[l for l in s.split(chr(10)) if re.match(pat,l)]
    print('  §0/%s：%d 条' % (sec,len(rows)))
    for r in rows[:16]:
        c=[x.strip() for x in r.split('|')]
        kid=re.sub(r'[*\[\]]','',c[1])[:9]; st=(c[2][:26] if len(c)>2 else '')
        print('    %-10s %s' % (kid,st))
" "$ISSUES"
    if [ "$_fail" != 0 ]; then
      echo "★ --report-head 有段落失败 ⇒ 拒绝输出为完整报告头（退出非零）" >&2
      exit 1
    fi
    exit 0;;
  --done) python3 - "$ISSUES" "${2:?用法: --done <ID>}" <<'PY'
import io, re, sys
s=io.open(sys.argv[1],encoding='utf-8').read(); kid=sys.argv[2]
for n,line in enumerate(s.split('\n'),1):
    if re.match(r'^\| \*\*%s\*\* \|' % re.escape(kid), line):
        print('▸ 完成 %s ⇒ **顾问模式（只打印"改哪一行怎么改"，不代写）**\n' % kid)
        print('  ① 文件: %s   行: %d' % (sys.argv[1], n))
        print('  ② 把它从 §1【未修】迁到 §2【已修】**并带收据**：')
        print('     · 修于 <commit>（本仓或对应仓）')
        print('     · 判据 = <哪条测试/哪条命令>，以及那次运行的读数')
        print('  ③ 若它还有字段行（deps/prio/class/blocks）⇒ 一并迁走，别留半行')
        print('\n  ⇒ 人手抄这三步；攒够信用后本工具才申请写权限（授权三级守则）。')
        break
else:
    print('★ 找不到声明行：%s' % kid); sys.exit(1)
PY
    ;;
  *)
python3 - "$ISSUES" <<'PY'
# -*- coding: utf-8 -*-
# ★ 静默家族第 ⑥ 形态候选（reviewer 2026-10-10 二）：**空读数无法区分"全做完了"与"没找到数据"**。
#   ⇒ 两条规矩：**数据源缺失 ⇒ 非零退出 + stderr 报路径**（此处由上面 `[ -f ]` 已保证）；
#     **真空（表里没有可解析项）⇒ 正常退出，但显式打印"(无就绪项)"**，绝不静默留白。
import io, re, sys
try:
    s = io.open(sys.argv[1], encoding='utf-8').read()
except OSError as e:
    print('★ 读不到登记册: %s' % e, file=sys.stderr); sys.exit(2)
items = []
for line in s.split('\n'):
    m = re.match(r'^\| \*\*(K[0-9]+)\*\* \|', line)
    if not m: continue
    f = re.search(r'〔deps=(.*?) · prio=(.*?) · class=(.*?)(?: · blocks=(.*?))?〕', line)
    if not f: continue
    items.append((m.group(1), f.group(1), f.group(2), f.group(3), (f.group(4) or '—')))

# ★ prio 推导（reviewer 一①：决策已写死，这里是它的落地）：
#   **由 blocks 推导**，人不再手填 ⇒ "排队人数决定重要性"。
#   高价值目标（DSH 对话界面 / 证轨）= 关键路径 ⇒ 挡着它的 = high；挡着其它 = medium；不挡任何 = low。
def derived(blocks):
    if blocks == '—' or not blocks.strip(): return 'low'
    if 'DSH' in blocks or '证轨' in blocks: return 'high'
    return 'medium'

ready, waiting = [], []
# ★ 三分桶（2026-10-10 修）：**"人类冻结"不许被列成"能做"** —— 那是指南针指反。
ready, waiting, frozen = [], [], []
for kid, deps, decl, cls, blocks in items:
    if '禁行' in cls: frozen.append((kid,'禁行')); continue
    if '冻结' in cls: frozen.append((kid,'人类冻结')); continue
    if '升级' in cls: waiting.append((kid, deps, cls)); continue
    ready.append((kid, derived(blocks), decl, blocks, deps))
ready.sort(key=lambda r: ({'high':0,'medium':1,'low':2}[r[1]], r[0]))

print('▸ 现在能做（class=自治）—— prio **由 blocks 推导**（人不再手填）· 真源：%s\n' % sys.argv[1])
for kid, dp, decl, blocks, deps in ready:
    warn = ''
    if decl not in ('—','**high**','high') and decl.replace('*','') != dp:
        warn = '  ⚠️ 人手填 prio=%s 与推导 %s 矛盾 ⇒ **矛盾即信息，要解释不要覆盖**' % (decl.replace('*',''), dp)
    print('  %-5s 推导=%-6s 挡着 → %-22s deps=%s%s' % (kid, dp, blocks, deps, warn))
print('\n▸ 等人类一句话（class=升级）—— **不要等它们**，先做上面那些')
for kid, deps, cls in waiting:
    print('  %-5s （%s）' % (kid, deps if deps != '—' else '无依赖，只等人'))
if not (ready or waiting or frozen):
    print('\n（无就绪项）—— 登记册里没有可解析的声明行 ⇒ **这不是"全做完了"，是"没找到数据"**')
print('\n▸ ⛔ 不许动（人类冻结 / 禁行）—— **不在你的清单里**')
for kid, why in frozen: print('  %-5s （%s）' % (kid, why))
print('\n▸ 注意事项（按类别，来自 DIAGNOSIS-METHODS.md）')
print('  · 改判据/加测试 ⇒ 绿必须带 run 收据（red-lifecycle ①）')
print('  · 排查 flaky   ⇒ 先问『这个仪器死掉时输出长什么样』（silent-family）')
print('  · 写闸门       ⇒ 不是所有纪律都该成为闸门（hard:true vs hard:false 要分型）')
print('\n▸ 顾问模式：完成某条 ⇒ `bash tools/todo-ready.sh --done <ID>` **只打印"改哪一行怎么改"**（不代写）')
PY
    ;;
esac
LOG="${HELIX_TODO_LOG:-$HOME/.helix/todo-ready.log}"
mkdir -p "$(dirname "$LOG")" 2>/dev/null && printf '%s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "${1:---ready}" >> "$LOG" 2>/dev/null || true
