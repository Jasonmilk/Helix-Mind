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
/// 这条同时记录了一个仍然存在的洞：被抑制的节点**若成为种子**会因种子豁免
/// 而存活——那属于 D3（抑制改确定性门控）的范围，不在这里修。
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
