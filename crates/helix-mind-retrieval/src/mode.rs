use helix_mind_core::graph::CognitiveMode;

#[derive(Debug, Clone)]
pub struct ModeConfig {
    pub max_hops: usize,
    pub beam_width: usize,
    pub weight_threshold: f64,
    pub soft_edge_decay: f64,
    pub soft_edge_min_weight: f64,
    pub dead_end_penalty: f64,
    pub tentative_edge_weight: f64,
}

impl ModeConfig {
    pub fn for_mode(mode: CognitiveMode, base: &super::RetrievalConfig) -> Self {
        match mode {
            CognitiveMode::Skilled => Self {
                max_hops: 2,
                beam_width: 2,
                weight_threshold: 0.9,
                soft_edge_decay: 0.0, // ignore soft edges
                soft_edge_min_weight: 0.0,
                dead_end_penalty: 0.9,
                tentative_edge_weight: 0.0,
            },
            CognitiveMode::Anchor => Self {
                max_hops: base.max_hops,
                beam_width: base.beam_width,
                weight_threshold: base.weight_threshold,
                soft_edge_decay: base.soft_edge_decay_factor,
                soft_edge_min_weight: base.soft_edge_min_weight,
                dead_end_penalty: base.dead_end_penalty_factor,
                tentative_edge_weight: base.tentative_edge_weight,
            },
            CognitiveMode::Imagination => Self {
                max_hops: 5,
                beam_width: 5,
                weight_threshold: 0.3,
                soft_edge_decay: 0.5,
                soft_edge_min_weight: 0.05,
                dead_end_penalty: 0.5,
                tentative_edge_weight: 0.5,
            },
        }
    }
}
