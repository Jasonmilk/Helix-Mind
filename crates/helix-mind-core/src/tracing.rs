use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing(verbose: u8) {
    let filter = match verbose {
        0 => EnvFilter::new("helix_mind=error"),
        1 => EnvFilter::new("helix_mind=info"),
        _ => EnvFilter::new("helix_mind=debug"),
    };

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();
}
