use clap::{Parser, Subcommand};
use helix_mind_core::config::Config;
use helix_mind_core::tracing::init_tracing;
use helix_mind_storage::StorageEngine;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "helix-mind")]
#[command(version = "0.1.0")]
#[command(about = "Helix-Mind: Digital lifeform memory hub")]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Helix-Mind daemon
    Run,
    /// View system status
    View {
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long)]
        phase: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    init_tracing(cli.verbose);

    // Load configuration
    std::env::set_var("HELIX_MIND_CONFIG", cli.config.to_string_lossy().as_ref());
    let config = Config::load()?;
    // P0 debt fix: create all declared parent directories so first run
    // (with an absent `./data`) does not fail with ENOENT.
    config.ensure_dirs()?;
    tracing::info!("Configuration loaded. Core hash: {}", config.compute_core_hash());

    // Initialize storage (returns Arc)
    let storage = StorageEngine::new(&config.storage).await?;

    // Load gene lock
    let gene_lock_content =
        tokio::fs::read_to_string(&config.gene_lock.file_path).await?;
    let gene_lock = helix_mind_core::graph::L0GeneLock::from_markdown(&gene_lock_content)?;
    tracing::info!(
        "Gene lock loaded: {} (hash: {})",
        gene_lock.lineage_name,
        gene_lock.l0_hash
    );

    match cli.command {
        Commands::Run => {
            // ── Build engine instances ──────────────────────────

            // Retrieval engine (Phase 2: five-stage impasse escalation)
            let retrieval = Arc::new(helix_mind_retrieval::RetrievalEngine::new(
                config.retrieval.clone(),
                storage.clone(),
            ));

            // Metabolism engine (Phase 3: event-driven, decay + symbolic solver)
            let metabolism = Arc::new(helix_mind_metabolism::MetabolismEngine::new(
                config.metabolism.clone(),
                storage.clone(),
            ));

            // Federation engine (Phase 4: sandbox review + DAG sharing)
            let federation = Arc::new(helix_mind_federation::FederationEngine::new(
                config.federation.clone(),
                storage.clone(),
            ));

            // Reincarnation engine (Phase 5: sunset, emergency dusk, epoch, rebirth)
            let reincarnation = Arc::new(helix_mind_reincarnation::ReincarnationEngine::new(
                config.lifecycle.clone(),
                storage.clone(),
            ));

            // ── Assemble service ─────────────────────────────────
            let service = helix_mind_api::HelixMindServiceImpl::new(
                config.clone(),
                storage.clone(),
                retrieval,
                metabolism,
                federation,
                reincarnation,
            );

            // 传输模式由 ApiConfig.transport 决定（TCP / UDS，ADR-0019 P3b）
            helix_mind_api::serve(&config.api, service).await?;
        }
        Commands::View { format: _, phase: _ } => {
            let stats = storage.get_stats().await?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
    }

    Ok(())
}
