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
    tracing::info!("Configuration loaded. Core hash: {}", config.compute_core_hash());

    // Initialize storage (returns Arc<StorageEngine>)
    let storage = StorageEngine::new(&config.storage).await?;

    // Load gene lock
    let gene_lock_content = tokio::fs::read_to_string(&config.gene_lock.file_path).await?;
    let gene_lock = helix_mind_core::graph::L0GeneLock::from_markdown(&gene_lock_content)?;
    tracing::info!("Gene lock loaded: {} (hash: {})", gene_lock.lineage_name, gene_lock.l0_hash);

    match cli.command {
        Commands::Run => {
            // Build engine instances (all take Arc<StorageEngine> directly)
            let retrieval = Arc::new(helix_mind_retrieval::RetrievalEngine::new(
                config.retrieval.clone(),
                storage.clone(),
            ));
            let lifecycle = Arc::new(helix_mind_reincarnation::ReincarnationEngine::new(
                config.lifecycle.clone(),
                storage.clone(),
            ));
            let metabolism = Arc::new(helix_mind_metabolism::MetabolismEngine::new(
                config.metabolism.clone(),
                storage.clone(),
            ));
            let federation = Arc::new(helix_mind_federation::FederationEngine::new(
                config.federation.clone(),
                storage.clone(),
            ));

            let service = helix_mind_api::HelixMindServiceImpl::new(
                config.clone(),
                storage.clone(),
                retrieval,
                metabolism,
                federation,
                lifecycle,
            );

            let addr = config.api.listen_addr.parse()?;
            tracing::info!("Starting gRPC server on {}", addr);
            helix_mind_api::serve(addr, service).await?;
        }
        Commands::View { format: _, phase: _ } => {
            // Print basic stats
            let stats = storage.get_stats().await?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
    }

    Ok(())
}