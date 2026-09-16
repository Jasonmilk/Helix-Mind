//! SA-Core 扩散深度的合成图回归网（ADR-0042 D0 / D1 / D2 / D5）。
//!
//! # 为什么必须用合成图，而不是跑真实库
//!
//! 真实库当前 `edges` **0 行**：零边时 `sum_abs == 0`，根本不传播，于是
//! 「只返回种子」这个结果**有两个互相独立的原因**（没边 + 绝对闸门），
//! 在真实库上无法把闸门这一条单独隔离出来。合成图是唯一能在建边（T5）之前
//! 看见并锁住闸门行为的仪器。
//!
//! # 这份测试锁的是什么
//!
//! 它**不是**在锁现状（现状是坏的），而是在锁 **ADR-0042 修复后的语义**，
//! 使「退回绝对闸门」或「不小心把闸门调成什么也不放过」都会被立刻抓住。
//! 每一条断言都对应一个曾经的失效模式，注释里写明是哪一个。

use helix_mind_core::config::RetrievalConfig;
use helix_mind_core::graph::{CognitiveMode, Edge, Node, RelationType};
use helix_mind_core::sa_core::SaCoreParams;
use helix_mind_storage::topology::MemoryTopology;
use uuid::Uuid;

/// 被移除的绝对闸门值。保留为常量，使「退回绝对刻度」的回归可被表达。
const REMOVED_ABSOLUTE_GATE: f64 = 0.8;

fn node(id: Uuid) -> Node {
    Node {
        id,
        ..Default::default()
    }
}

fn hard_edge(source: Uuid, target: Uuid, relation_type: RelationType) -> Edge {
    Edge {
        source_id: source,
        target_id: target,
        weight: 1.0,
        relation_type,
        is_soft: false,
    }
}

/// `ids[0] → ids[1] → … → ids[len-1]`，全部单位权硬边。
fn chain(len: usize, topo: &mut MemoryTopology) -> Vec<Uuid> {
    let ids: Vec<Uuid> = (0..len).map(|_| Uuid::new_v4()).collect();
    for &id in &ids {
        topo.add_node(&node(id));
    }
    for pair in ids.windows(2) {
        let e = hard_edge(pair[0], pair[1], RelationType::Causal);
        topo.add_edge(pair[0], pair[1], &e).unwrap();
    }
    ids
}

/// 返回集合在链上的最大到达下标（None = 一个都没到）。
fn depth(ids: &[Uuid], chain: &[Uuid]) -> Option<usize> {
    chain
        .iter()
        .enumerate()
        .filter(|(_, id)| ids.contains(id))
        .map(|(i, _)| i)
        .max()
}

fn skilled(heliotropism: f64) -> SaCoreParams {
    SaCoreParams::for_mode(CognitiveMode::Skilled, heliotropism, &RetrievalConfig::default())
}

// ── D0：闸门必须相对化 ──────────────────────────────────────────────

/// 修复前：绝对闸门 0.8 高于第一跳上界 α=0.5，非种子节点在第 1 轮就被清零，
/// 扩散退化成「只返回种子」。修复后必须真的多跳。
#[test]
fn relative_gate_admits_multi_hop_diffusion() {
    let mut topo = MemoryTopology::new();
    let c = chain(8, &mut topo);
    let params = skilled(0.0);

    let (ids, _, _, _) = topo.skilled_traverse(&c[..1], &params, 8, 0, 100);

    let reached = depth(&ids, &c).expect("seed itself must be returned");
    assert!(
        reached >= 2,
        "expected real diffusion (depth >= 2), got depth {reached}; \
         this is the exact symptom of an absolute gate above alpha"
    );
}

/// 把「为什么绝对闸门必然致命」写成可执行的算术，而不是注释里的一句话：
/// 单位权单边下第一跳激活**恰好等于 α**，而 α < 0.8 恒成立（D1 保证 α ≤ 0.95
/// 且 skilled 基准 0.5）。若有人把闸门改回绝对刻度，这条会立刻红。
#[test]
fn first_hop_activation_equals_alpha_and_cannot_clear_the_removed_absolute_gate() {
    let mut topo = MemoryTopology::new();
    let c = chain(3, &mut topo);
    let params = skilled(0.0);

    let (_, activations, _, _) = topo.skilled_traverse(&c[..1], &params, 1, 0, 100);
    let first_hop = activations
        .iter()
        .find(|(id, _)| *id == c[1])
        .map(|(_, a)| *a)
        .expect("first hop must be activated");

    assert!(
        (first_hop - params.alpha).abs() < 1e-9,
        "first-hop activation {first_hop} should equal alpha {}",
        params.alpha
    );
    assert!(
        first_hop < REMOVED_ABSOLUTE_GATE,
        "the removed absolute gate {REMOVED_ABSOLUTE_GATE} sat above the first-hop \
         ceiling {first_hop}: nothing non-seed could ever survive it"
    );
}

/// 尺度不变性：同样的 τ 必须在**不同种子数**下给出**同样深度**。
///
/// 这条是「锚峰值而不是锚总质量」的直接理由：总质量随种子数线性增长，
/// 若锚总质量，4 条链的查询会比 1 条链的查询探索得更浅（τ 的含义随查询规模漂移）。
#[test]
fn gate_depth_is_invariant_in_seed_count() {
    let params = skilled(0.0);

    let mut single = MemoryTopology::new();
    let one = chain(8, &mut single);
    let (ids_one, _, _, _) = single.skilled_traverse(&one[..1], &params, 8, 0, 100);
    let depth_one = depth(&ids_one, &one).unwrap();

    // 4 条互不相连的同样链，各以自己的链首为种子。
    let mut multi = MemoryTopology::new();
    let mut heads = Vec::new();
    let mut all = Vec::new();
    for _ in 0..4 {
        let c = chain(8, &mut multi);
        heads.push(c[0]);
        all.push(c);
    }
    let (ids_multi, _, _, _) = multi.skilled_traverse(&heads, &params, 8, 0, 100);
    let depth_multi = all
        .iter()
        .map(|c| depth(&ids_multi, c).unwrap())
        .max()
        .unwrap();

    assert_eq!(
        depth_one, depth_multi,
        "the same tau must mean the same depth regardless of seed count \
         (one seed reached {depth_one}, four seeds reached {depth_multi})"
    );
}

/// D5 召回底线：**种子永不被闸门熄灭**，哪怕闸门被设成荒谬的值。
/// 这是 `GROWTH.md` 记为「种子保底」的那条不变量；它必须继续成立。
#[test]
fn seeds_survive_even_an_absurd_gate() {
    let mut topo = MemoryTopology::new();
    let c = chain(4, &mut topo);
    let absurd = SaCoreParams {
        gate_relative_tau: 10.0,
        ..skilled(0.0)
    };

    let (ids, _, _, _) = topo.skilled_traverse(&c[..1], &absurd, 4, 0, 100);
    assert!(ids.contains(&c[0]), "the seed must always be returned");
}

/// 孤立种子（无任何边）也必须被返回：这正是「种子保底」注释所记的失效模式
/// （孤立节点在 `(1−α)·1.0 = 0.5` 处若低于阈值，召回会静默死掉）。
#[test]
fn isolated_seed_is_returned_even_with_no_edges_at_all() {
    let mut topo = MemoryTopology::new();
    let id = Uuid::new_v4();
    topo.add_node(&node(id));

    let (ids, _, _, _) = topo.skilled_traverse(&[id], &skilled(0.0), 4, 0, 100);
    assert_eq!(ids, vec![id]);
}

// ── D2：迭代预算真的成为预算 ────────────────────────────────────────

/// 修复前 `max_hops` 被闸门架空：调 3 与调 12 结果完全相同。
/// 修复后它是**真正生效**的约束，所以更大的预算必须到达更深。
#[test]
fn iteration_budget_now_controls_depth() {
    let mut topo = MemoryTopology::new();
    let c = chain(10, &mut topo);
    let params = skilled(0.0);

    let (shallow, _, _, _) = topo.skilled_traverse(&c[..1], &params, 2, 0, 100);
    let (deep, _, _, _) = topo.skilled_traverse(&c[..1], &params, 8, 0, 100);

    let d_shallow = depth(&shallow, &c).unwrap();
    let d_deep = depth(&deep, &c).unwrap();
    assert!(
        d_deep > d_shallow,
        "budget must bind: 2 iterations reached {d_shallow}, 8 reached {d_deep}"
    );
}

/// 收敛判据存在性的可观测后果：预算给到远超所需时，结果**不再变化**——
/// 说明定答案的是不动点，而不是截断。修复前唯一的停止规则是硬截断。
#[test]
fn result_stops_changing_once_converged() {
    let mut topo = MemoryTopology::new();
    let c = chain(10, &mut topo);
    let params = skilled(0.0);

    let (ids_a, act_a, _, _) = topo.skilled_traverse(&c[..1], &params, 40, 0, 100);
    let (ids_b, act_b, _, _) = topo.skilled_traverse(&c[..1], &params, 120, 0, 100);

    assert_eq!(ids_a, ids_b, "the retrieved set must be a fixed point");
    let map = |v: &[(Uuid, f64)]| v.iter().cloned().collect::<std::collections::HashMap<_, _>>();
    let (ma, mb) = (map(&act_a), map(&act_b));
    for (id, a) in &ma {
        let b = mb.get(id).copied().unwrap_or(0.0);
        assert!(
            (a - b).abs() < 1e-9,
            "activation for {id} drifted between budgets: {a} vs {b}"
        );
    }
}

// ── D1：α 的来源，以及它不破坏抑制语义 ──────────────────────────────

/// 向阳度提高 ⇒ 扩散更广。这是 README 承诺的语义，修复后必须真的可观测。
///
/// 断言必须是**严格**不等：初版写成 `>=`，变异测试（把 `heliotropism` 换成常量
/// `0.0`）证明它**空转**——两边相等时 `>=` 照样通过。严格不等才真正锁住
/// 「α 由 heliotropism 派生」这条因果。
#[test]
fn optimistic_heliotropism_reaches_strictly_farther_than_defensive() {
    let mut topo = MemoryTopology::new();
    let c = chain(10, &mut topo);

    let defensive = skilled(-1.0);
    let optimistic = skilled(1.0);
    assert!(
        optimistic.alpha > defensive.alpha,
        "the two heliotropism extremes must yield different alpha"
    );

    let (def_ids, _, _, _) = topo.skilled_traverse(&c[..1], &defensive, 10, 0, 100);
    let (opt_ids, _, _, _) = topo.skilled_traverse(&c[..1], &optimistic, 10, 0, 100);

    let d_def = depth(&def_ids, &c).unwrap();
    let d_opt = depth(&opt_ids, &c).unwrap();
    assert!(
        d_opt > d_def,
        "optimistic (alpha={}) must reach strictly farther than defensive (alpha={}); \
         got depth {d_opt} vs {d_def}",
        optimistic.alpha,
        defensive.alpha
    );
}

/// CORRECTS 的非种子目标必须仍被压到 0（抑制语义未被 D0 破坏）。
#[test]
fn corrects_still_suppresses_a_non_seed_target() {
    let mut topo = MemoryTopology::new();
    let s = Uuid::new_v4();
    let stale = Uuid::new_v4();
    topo.add_node(&node(s));
    topo.add_node(&node(stale));
    let e = hard_edge(s, stale, RelationType::Corrects);
    topo.add_edge(s, stale, &e).unwrap();

    let (ids, activations, _, _) = topo.skilled_traverse(&[s], &skilled(0.0), 4, 0, 100);

    assert!(ids.contains(&s));
    assert!(
        !ids.contains(&stale),
        "a CORRECTS target must not be returned as a live hit"
    );
    assert!(
        activations.iter().all(|(_, a)| *a > 0.0),
        "no negative activation may leak into the white-box vector"
    );
}

// ── D3：抑制是确定性门控，不是会被扇出稀释的软权重 ──────────────────

/// 这是 ADR-0042 §2 P0-1 的可执行复现，也是 D3 立论的根据。
///
/// 旧实现把 `Corrects` 当 `-1.0` 送进传播矩阵，再被**行归一化**稀释。
/// **稀释的分母是「纠正者」的出度**（`sum_abs` 对源点求和），所以要注意方向：
/// 扇出必须加在 `current`（纠正者）身上。设 `current` 有 `S` 条正边和 1 条
/// Corrects，归一化后抑制只剩 `-1/(S+1)`；`S = 9` 时是 `-0.1`。
///
/// 而它为什么**只有对种子才致命**：非种子节点的负值会被闸门（`val < theta`）
/// 归零，所以负值本身活不下来；真正漏网的是**同时是种子的陈旧节点** ——
/// 种子豁免让它绕过闸门，于是 `α·(-0.1) + (1−α)·1 = 0.45 > 0` 活了下来。
/// 这正是第一轮审查说的「越 hub 越压不住」唯一真正可达的形态。
///
/// D3 之后抑制根本不进矩阵，扇出**无法**影响它。
#[test]
fn suppression_is_immune_to_corrector_fanout_dilution() {
    let cfg = RetrievalConfig::default();

    // `stale` 始终是种子（查询命中它），`current` 的出度随 fanout 变化。
    let stale_survives = |fanout: usize| -> bool {
        let mut topo = MemoryTopology::new();
        let current = Uuid::new_v4();
        let stale = Uuid::new_v4();
        let mut stale_node = node(stale);
        // 关键：陈旧标记来自 `Node::corrected_by`（既有字段，非新字段）。
        stale_node.corrected_by = Some(current);
        topo.add_node(&node(current));
        topo.add_node(&stale_node);
        // 纠正者 → 陈旧节点：旧实现里这条边就是被稀释的抑制来源。
        let correcting = hard_edge(current, stale, RelationType::Corrects);
        topo.add_edge(current, stale, &correcting).unwrap();
        // 纠节者的其它正边：它们抬高归一化分母，从而稀释抑制。
        for _ in 0..fanout {
            let other = Uuid::new_v4();
            topo.add_node(&node(other));
            let e = hard_edge(current, other, RelationType::Causal);
            topo.add_edge(current, other, &e).unwrap();
        }

        let params = SaCoreParams::for_mode(CognitiveMode::Skilled, 0.0, &cfg);
        let (ids, _, _, _) = topo.skilled_traverse(&[current, stale], &params, 4, 0, 100);
        ids.contains(&stale)
    };

    assert!(
        !stale_survives(0),
        "a superseded seed must not survive when the corrector has no fan-out"
    );
    assert!(
        !stale_survives(9),
        "a superseded seed must not survive when the corrector IS a hub — under the \
         old arithmetic weight the inhibition was diluted to -0.1 and it DID survive"
    );
}

/// 种子豁免不得给过时知识开后门（第一轮审查指出的那个洞）。
///
/// 旧实现：`:365` 的 `a_0[j] == 0.0` 让**种子**绕过闸门 ⇒ 被纠正的节点只要
/// 查询命中它就会作为「活跃知识」返回。D3 让抑制门控**优先于**种子豁免。
#[test]
fn supersession_overrides_the_seed_exemption() {
    let mut topo = MemoryTopology::new();
    let stale = Uuid::new_v4();
    let mut stale_node = node(stale);
    stale_node.corrected_by = Some(Uuid::new_v4());
    topo.add_node(&stale_node);

    let (ids, _, _, _) = topo.skilled_traverse(&[stale], &skilled(0.0), 4, 0, 100);

    assert!(
        ids.is_empty(),
        "a query hit that has been superseded must not be returned as live, \
         but got {ids:?}"
    );
}

/// 记录上述行为的一个真实代价：若查询**只**命中过时知识，活跃集合会变空。
/// 这是 D3 有意的取义（宁可不答，不可把过时知识当真知），但它必须在测试里
/// 显式存在，而不是等生产上被发现。
///
/// 逃生口是协议层的 `include_recessive` 一类的显式开关（尚未实现）——
/// 历史仍然完整保留在存储里供审计。
#[test]
fn superseded_only_query_yields_an_empty_live_set_by_design() {
    let mut topo = MemoryTopology::new();
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    let mut na = node(a);
    na.corrected_by = Some(b);
    let mut nb = node(b);
    nb.corrected_by = Some(a);
    topo.add_node(&na);
    topo.add_node(&nb);

    let (ids, _, _, _) = topo.skilled_traverse(&[a, b], &skilled(0.0), 4, 0, 100);
    assert!(ids.is_empty(), "expected an empty live set, got {ids:?}");
}

/// 抑制是**确定性**的：多次运行、以及改变预算，结果都一致。
/// 软权重的老实现会随迭代次数变化（每轮衰减一次），门控不会。
#[test]
fn suppression_is_deterministic_across_budgets() {
    let mut topo = MemoryTopology::new();
    let current = Uuid::new_v4();
    let stale = Uuid::new_v4();
    let mut stale_node = node(stale);
    stale_node.corrected_by = Some(current);
    topo.add_node(&node(current));
    topo.add_node(&stale_node);
    let e = hard_edge(current, stale, RelationType::Corrects);
    topo.add_edge(current, stale, &e).unwrap();

    let run = |budget: usize| {
        let (ids, _, _, _) = topo.skilled_traverse(&[current], &skilled(0.0), budget, 0, 100);
        ids.contains(&stale)
    };
    for budget in [1, 2, 4, 32] {
        assert!(!run(budget), "budget {budget} let a superseded node through");
    }
}

/// imagination 的 `temperature` 现在调制**相对** τ：温度越高越接纳弱激活。
#[test]
fn imagination_temperature_relaxes_the_relative_gate() {
    let mut topo = MemoryTopology::new();
    let c = chain(10, &mut topo);
    let cfg = RetrievalConfig::default();
    let params = SaCoreParams::for_mode(CognitiveMode::Imagination, 0.0, &cfg);

    let (cold, _, _, _) = topo.imagination_traverse(&c[..1], 0.0, &params, 10, 0, 100);
    let (hot, _, _, _) = topo.imagination_traverse(&c[..1], 1.0, &params, 10, 0, 100);

    assert!(
        depth(&hot, &c) >= depth(&cold, &c),
        "temperature=1.0 (tau→0, no pruning) must reach at least as far as temperature=0.0"
    );
}

// ── D8：环会回响，但不会失控 ────────────────────────────────────────

/// 软边成环（spec 允许，`SIMILAR_TO` 是其代表）时能量在环上往复。
///
/// 人类补充了这条要求：「有环是有风险的，必须有次树衰减，否则可能会死循环」。
/// 本测试把兜住它的三层逐一钉住：
///
/// 1. **不可能死循环**：迭代是 `for _ in 0..max_iterations` 的**有界**循环，
///    并在收敛时提前 `break`。结构上不存在无界循环——这是最强的一层。
/// 2. **不可能膨胀**：行归一化给出 `‖W‖₁ = 1`，叠加项 `(1−α)·a_0` 使
///    `ρ(αW) ≤ α ≤ 0.95 < 1`，总质量上界恒为「种子数 k」。环上往复只会
///    **震荡后收敛**，不会放大。
/// 3. **数值实证**：α=0.7 的二环收敛到 `(0.588, 0.412)`，和恒为 1.0。
#[test]
fn soft_edge_cycle_reverberates_but_never_runs_away() {
    let cfg = RetrievalConfig::default();
    let params = SaCoreParams::for_mode(CognitiveMode::Anchor, 0.0, &cfg);

    let mut topo = MemoryTopology::new();
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    topo.add_node(&node(a));
    topo.add_node(&node(b));
    // 双向软边 = 一个环。spec 对 SIMILAR_TO 明确「可成环」。
    for (s, t) in [(a, b), (b, a)] {
        let e = Edge {
            source_id: s,
            target_id: t,
            weight: 0.9,
            relation_type: RelationType::SimilarTo,
            is_soft: true,
        };
        topo.add_edge(s, t, &e).unwrap();
    }

    let mass = |acts: &[(Uuid, f64)]| acts.iter().map(|(_, v)| v.abs()).sum::<f64>();

    // 预算从 1 拉到 64：总质量必须始终被种子数（1）兜住。
    for budget in [1usize, 2, 4, 16, 64] {
        let (_, acts, _, _) = topo.anchor_traverse(&[a], &params, budget, 0, 100);
        let m = mass(&acts);
        assert!(
            m <= 1.0 + 1e-9,
            "a soft-edge cycle inflated total mass to {m} at budget {budget}; \
             seed count is 1, so the cycle is running away"
        );
        assert!(
            acts.iter().all(|(_, v)| v.abs() <= 1.0 + 1e-9),
            "an individual node exceeded the seed mass at budget {budget}"
        );
    }

    // 收敛：预算远超所需时结果不再变化 ⇒ 回响消退，而非持续泵送。
    let (_, wide, _, _) = topo.anchor_traverse(&[a], &params, 64, 0, 100);
    let (_, wider, _, _) = topo.anchor_traverse(&[a], &params, 400, 0, 100);
    let map = |v: &[(Uuid, f64)]| v.iter().cloned().collect::<std::collections::HashMap<_, _>>();
    let (mw, mwr) = (map(&wide), map(&wider));
    for (id, v) in &mw {
        let w = mwr.get(id).copied().unwrap_or(0.0);
        assert!(
            (v - w).abs() < 1e-9,
            "a cycle kept changing at larger budgets ({id}: {v} vs {w}) — it is not settling"
        );
    }
}

/// `decay_factor` 的作用面：它**不衰减幅值**，只在同一节点的多条出边之间
/// **重新分配**份额。
///
/// 为什么：归一化是 `w / Σ|w|`，分子分母**同比缩放**，所以任何一个源的出边
/// 份额恒和为 1，与 `decay_factor` 无关。于是「靠软边衰减来防止环失控」这个
/// 直觉**在本实现下不成立**——真正兜住失控的是 `α < 1` 与有界迭代（见上一条）。
///
/// 本测试用「只有一条软出边」的图把这一点暴露到极致：没有兄弟边可分配，
/// 于是 `decay_factor` 从 1.0 改到 0.5 **完全不改变结果**。
#[test]
fn soft_edge_decay_only_redistributes_it_never_attenuates() {
    let build = |decay: f64| {
        let mut cfg = RetrievalConfig::default();
        cfg.soft_edge_decay_factor = decay;
        SaCoreParams::for_mode(CognitiveMode::Anchor, 0.0, &cfg)
    };

    let mut topo = MemoryTopology::new();
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    topo.add_node(&node(a));
    topo.add_node(&node(b));
    let e = Edge {
        source_id: a,
        target_id: b,
        weight: 0.9,
        relation_type: RelationType::SimilarTo,
        is_soft: true,
    };
    topo.add_edge(a, b, &e).unwrap();

    let (ids_full, act_full, _, _) = topo.anchor_traverse(&[a], &build(1.0), 8, 0, 100);
    let (ids_half, act_half, _, _) = topo.anchor_traverse(&[a], &build(0.5), 8, 0, 100);

    assert_eq!(ids_full, ids_half, "the reachable set must be identical");
    let map = |v: &[(Uuid, f64)]| v.iter().cloned().collect::<std::collections::HashMap<_, _>>();
    let (mf, mh) = (map(&act_full), map(&act_half));
    for (id, v) in &mf {
        let h = mh.get(id).copied().unwrap_or(0.0);
        assert!(
            (v - h).abs() < 1e-12,
            "decay_factor changed an activation magnitude ({id}: {v} vs {h}); \
             under row normalisation it can only redistribute among siblings"
        );
    }
}

/// Skilled 把软边衰减设为 0 ⇒ **软边在主力检索模式下完全惰性**。
/// 这解释了「为什么创造力不会自然出现」：Stage 1 恒为 Skilled，
/// 而联想回路只存在于软边上。创造力被**设计性地**放在 Anchor/Imagination。
#[test]
fn skilled_mode_disables_soft_edges_entirely() {
    let cfg = RetrievalConfig::default();
    let params = SaCoreParams::for_mode(CognitiveMode::Skilled, 0.0, &cfg);
    assert_eq!(params.decay_factor, 0.0);

    let mut topo = MemoryTopology::new();
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    topo.add_node(&node(a));
    topo.add_node(&node(b));
    let e = Edge {
        source_id: a,
        target_id: b,
        weight: 0.9,
        relation_type: RelationType::SimilarTo,
        is_soft: true,
    };
    topo.add_edge(a, b, &e).unwrap();

    let (ids, _, _, _) = topo.skilled_traverse(&[a], &params, 8, 0, 100);
    assert_eq!(ids, vec![a], "an association edge must be inert in Skilled mode");
}

// ── D5：内存侧幂等（SQL 已幂等，petgraph 不会）────────────────────────

/// `add_edge` 必须在**内存**里也幂等。
///
/// `edges` 表以 `(source_id, target_id, relation_type)` 为主键且走
/// `ON CONFLICT DO UPDATE`，所以 SQL 会去重；而 `MemoryTopology::add_edge` 原先
/// 直接 `petgraph::add_edge`，会加一条**平行边**。两处不一致的后果是
/// **事实来源（SQL）与图（内存）静默分叉**，而 `sa_core_diffusion` 读的正是内存图；
/// 平行边还会让同一条关系在 `sum_abs` 与 `a_next` 里各计两次，静默改变权重。
#[test]
fn repeated_add_edge_updates_instead_of_duplicating() {
    let mut topo = MemoryTopology::new();
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    topo.add_node(&node(a));
    topo.add_node(&node(b));

    let mut e = hard_edge(a, b, RelationType::Temporal);
    e.weight = 0.3;
    topo.add_edge(a, b, &e).unwrap();
    assert_eq!(topo.graph.edge_count(), 1, "first add must create exactly one edge");

    // Same triple, new weight: must UPDATE, not append.
    e.weight = 0.9;
    topo.add_edge(a, b, &e).unwrap();
    assert_eq!(
        topo.graph.edge_count(),
        1,
        "a repeated (source, target, relation) must update in place; petgraph's \
         add_edge would have created a parallel edge and diverged from the SQL row"
    );
    let w = topo.graph.edges(topo.id_to_index[&a]).next().unwrap().weight().weight;
    assert!((w - 0.9).abs() < 1e-12, "the weight must be updated to 0.9, got {w}");

    // Same endpoints, DIFFERENT relation: a genuinely different edge (matches the
    // SQL primary key, which includes relation_type).
    let other = hard_edge(a, b, RelationType::Refines);
    topo.add_edge(a, b, &other).unwrap();
    assert_eq!(
        topo.graph.edge_count(),
        2,
        "a different relation between the same endpoints is a distinct edge"
    );
}

/// 平行边会**静默改变扩散权重**——但只在源**还有别的出边**时才如此。
///
/// 这一点必须写清楚，因为「重复边一定改变权重」是**错的**：若源只有这一个目标，
/// 平行边让 `sum_abs` 与 `a_next` **同比**放大，归一化后**完全抵消**（份额仍是 1）。
/// 只有存在兄弟边时，重复边才会**抢走兄弟的份额**。
///
/// ⇒ 本测试的图**必须**有兄弟边（A→B 重复 + A→C）。初版只建了 A→B，于是
/// 在「退回无条件 add_edge」的变异下**照样通过**（空转）；加了 A→C 才咬得住。
#[test]
fn repeated_add_edge_does_not_change_diffusion() {
    let build = |repeat: bool| {
        let mut topo = MemoryTopology::new();
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let c = Uuid::new_v4();
        topo.add_node(&node(a));
        topo.add_node(&node(b));
        topo.add_node(&node(c));
        let e = hard_edge(a, b, RelationType::Temporal);
        topo.add_edge(a, b, &e).unwrap();
        if repeat {
            topo.add_edge(a, b, &e).unwrap();
        }
        // 兄弟边：它是「重复边抢份额」这个后果的**唯一**见证者。
        let sib = hard_edge(a, c, RelationType::Causal);
        topo.add_edge(a, c, &sib).unwrap();
        let (_, acts, _, _) = topo.skilled_traverse(&[a], &skilled(0.0), 4, 0, 100);
        // Compare the activation MULTISET, not the ids: each `build()` mints fresh
        // random UUIDs, so including them would make the two runs uncomparable and
        // the assertion vacuous. Sorted descending, the values are positionally
        // meaningful regardless of which uuid landed where.
        let mut v: Vec<i64> = acts.iter().map(|(_, x)| (x * 1e12).round() as i64).collect();
        v.sort_unstable_by(|x, y| y.cmp(x));
        v
    };
    assert_eq!(
        build(false),
        build(true),
        "re-adding the same edge must be a no-op for diffusion, not a reweighting"
    );
}
