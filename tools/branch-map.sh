#!/usr/bin/env bash
# branch-map.sh —— 六仓分支地图【生成器】（人类 2026-10-10：「确保文档不会落后于代码」）。
#
# ★ 为什么是生成器而不是手写文档：分支/HEAD 是**每天都变的事实**；手写必然落后。
#   生成器读 git ⇒ 落后变成**结构上不可能**；`--check` 让 CI 把"生成物过期"变成红。
# 用法：bash tools/branch-map.sh           # 打印（重定向写进文档）
#       bash tools/branch-map.sh --check   # 与文档里 <!-- BEGIN/END GENERATED --> 之间比对，过期则非零
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"; ROOT="$ROOT/.."
# ★ 12 仓（2026-10-10 更正）：我一直误说"六仓"，实际工作区有 12 个 git 仓
REPOS="anaphase-helix helix-mind Cellrix FlowModus Tuck phyt-DNA helix-tentacle Helix-MCP-Learner HelixECO-Glove lodestone-md lodestone-spec lumtract"

gen_footer() {
  echo
  echo "### 12 仓全景中的【另 5 仓】（在 \`main\`、无 \`rs\`）"
  echo "| 仓 | 判定 |"
  echo "|---|---|"
  for x in Helix-MCP-Learner HelixECO-Glove lodestone-md lodestone-spec lumtract; do
    [ -d "$ROOT/$x/.git" ] || continue
    printf "| %s | 无 \`rs\` 主干 ⇒ **不参与本轮收敛**（另族；若将来纳入，须先立主干） |\n" "$x"
  done
  echo
  echo "> ★ 第 7 员勘测（2026-10-10）：**helix-tentacle** 默认分支 = \`main\`，但**有 \`rs\` 主干**（线性，最后一笔 2026-09-21）"
  echo "> 且**没有** \`feature/convergence-M0M7\` ⇒ **不是本轮收敛批的成员**；故人侧"默认分支切 rs"要点的仓数**不等于 12**（勘测后定）。"
}
gen() {
  echo "| 仓 | 活跃分支 | 主干(rs) | feature 领先/落后 rs | merge-base | 已并入 rs? | main 与 rs 的关系 | 工作树 |"
  echo "|---|---|---|---|---|---|---|---|"
  for x in $REPOS; do
    [ -d "$ROOT/$x/.git" ] || continue
    g(){ git -C "$ROOT/$x" "$@" 2>/dev/null; }
    cur=$(g rev-parse --abbrev-ref HEAD)
    def=$(g symbolic-ref refs/remotes/origin/HEAD | sed 's|refs/remotes/origin/||')
    have_rs=$(g rev-parse --verify rs >/dev/null 2>&1 && echo rs || (g rev-parse --verify origin/rs >/dev/null 2>&1 && echo origin/rs))
    if [ "$x" = "phyt-DNA" ]; then echo "| **$x** | \`$cur\` | — | — | — | — | — | $( [ -z "$(g status --porcelain)" ] && echo '干净' || echo 'dirty' ) |"; continue; fi
    rsr="${have_rs:-—}"
    if [ "$rsr" != "—" ]; then
      a=$(g log --oneline "$rsr..HEAD" | wc -l | tr -d ' ')
      z=$(g log --oneline "HEAD..$rsr" | wc -l | tr -d ' ')
      mb=$(g merge-base "$rsr" HEAD | cut -c1-7)
      if g merge-base --is-ancestor HEAD "$rsr"; then merged="✅ 是"; else merged="否"; fi
      # main 关系
      mr="—"
      if g rev-parse --verify origin/main >/dev/null 2>&1; then
        if [ -z "$(g merge-base origin/main "$rsr")" ]; then mr="**无关历史**（骨架）"
        else mr="落后 rs $(g log --oneline "origin/main..$rsr" | wc -l | tr -d ' ') 笔 · 领先 $(g log --oneline "$rsr..origin/main" | wc -l | tr -d ' ') 笔"; fi
      fi
    else a=—; z=—; mb=—; merged=—; mr=—; fi
    printf "| **%s** | \`%s\` | \`%s\` | %s / %s | \`%s\` | %s | %s | %s |\n" "$x" "$cur" "$rsr" "$a" "$z" "$mb" "$merged" "$mr" "$( [ -z "$(g status --porcelain)" ] && echo '干净' || echo 'dirty' )"
  done
  gen_footer
  echo
  echo "_生成于 \`bash tools/branch-map.sh\` · 判据：\`--check\`（生成物过期 ⇒ CI 红）_"
}

if [ "${1:-}" = "--check" ]; then
  # ★ 2026-10-10 实测教训：CI 只 checkout【本仓】，**没有兄弟仓** ⇒ 生成器读不到其余五仓
  #   ⇒ 生成表与本仓文档必然不同 ⇒ 若照判就是"过期"假红（我那一步实测 exit 1）。
  #   **规矩：取不到数不得映射成任何态** ⇒ 无兄弟仓时**具名跳过**（exit 0），本地（有兄弟仓）才是真判官。
  sib=0; for x in $REPOS; do [ -d "$ROOT/$x/.git" ] && sib=$((sib+1)); done
  if [ "$sib" -lt 2 ]; then
    echo "  ⚠ 跳过：本环境只看到 ${sib} 个仓（CI 只 checkout 本仓）⇒ 无法判读分支地图是否过期"
    echo "    （真判官 = 本地 12 仓工作区：bash tools/branch-map.sh --check）"
    exit 0
  fi
  doc="docs/helixECO/BRANCH-MAP.md"
  [ -f "$doc" ] || { echo "★ 缺 $doc"; exit 2; }
  python3 - "$doc" <<'PY' || exit 1
import io,re,subprocess,sys
doc=sys.argv[1]; s=io.open(doc,encoding='utf-8').read()
m=re.search(r'<!-- BEGIN GENERATED -->\n(.*?)\n<!-- END GENERATED -->', s, re.S)
if not m: sys.exit('★ 文档里没有 BEGIN/END GENERATED 标记（无法判过期）')
cur=m.group(1).strip()
new=subprocess.run(['bash','tools/branch-map.sh'],capture_output=True,text=True).stdout.strip()
if cur!=new:
    print('★ 生成物【过期】：文档里的表 ≠ 现在跑生成器的输出'); sys.exit(1)
print('OK: 生成物与 git 现状一致（未过期）')
PY
  exit $?
fi
gen

# ★ 12 仓全景里【另 5 仓】的名单（reviewer 2026-10-10：只报 7 个等于画了一半）
