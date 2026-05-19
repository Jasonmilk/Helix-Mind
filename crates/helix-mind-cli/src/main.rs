use clap::Parser;
use helix_mind_core::config::Config;
use helix_mind_core::tracing::init_tracing;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Run the Helix-Mind daemon
    Run {
        /// Verbose logging
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    /// View system status
    View {
        /// What to view: lifecycle, storage, metabolism, federation
        #[arg(short, long)]
        phase: String,
        /// Output format: json or human
        #[arg(short, long, default_value = "human")]
        format: String,
    },
    /// Trigger manual digest
    Digest,
    /// Trigger manual crystallization
    Crystallize,
    /// Trigger manual hibernate
    Hibernate,
    /// Trigger manual reincarnation
    Reincarnate {
        /// Confirmation token
        #[arg(short, long)]
        confirm: String,
    },
    /// Share DAG to federation
    Share {
        /// Target Helix ID
        #[arg(short, long)]
        target: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load config
    let config = Config::load()?;

    match cli {
        Cli::Run { verbose } => {
            init_tracing(verbose);
            tracing::info!("Starting Helix-Mind daemon");

            // Initialize storage
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            // Initialize retrieval
            let retrieval = Arc::new(helix_mind_retrieval::RetrievalEngine::new(config.retrieval.clone(), storage.clone()));
            // Initialize metabolism
            let metabolism = Arc::new(helix_mind_metabolism::MetabolismEngine::new(config.metabolism.clone(), storage.clone()));
            metabolism.start().await?;
            // Initialize federation
            let federation = Arc::new(helix_mind_federation::FederationEngine::new(config.federation.clone(), storage.clone()));
            federation.start().await?;
            // Initialize reincarnation
            let reincarnation = Arc::new(helix_mind_reincarnation::ReincarnationEngine::new(config.lifecycle.clone(), storage.clone()));
            reincarnation.start().await?;
            // Initialize API server
            let service = helix_mind_api::HelixMindServiceImpl::new(
                config.clone(),
                storage.clone(),
                retrieval.clone(),
                metabolism.clone(),
                federation.clone(),
                reincarnation.clone(),
            );
            service.start(&config.api.listen_addr).await?;
        }
        Cli::View { phase, format } => {
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            let reincarnation = Arc::new(helix_mind_reincarnation::ReincarnationEngine::new(config.lifecycle.clone(), storage.clone()));

            match phase.as_str() {
                "lifecycle" => {
                    let phase = reincarnation.get_phase().await?;
                    let remaining = reincarnation.get_countdown_remaining().await?;
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                            "phase": phase,
                            "countdown_remaining": remaining,
                        }))?);
                    } else {
                        println!("Lifecycle phase: {}", phase);
                        if let Some(r) = remaining {
                            println!("Countdown remaining: {} minutes", r);
                        }
                    }
                }
                "storage" => {
                    let stats = storage.get_stats().await?;
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&stats)?);
                    } else {
                        println!("Total nodes: {}", stats.total_nodes);
                        println!("Total interactions: {}", stats.total_interactions);
                    }
                }
                _ => {
                    eprintln!("Unknown phase: {}", phase);
                    std::process::exit(1);
                }
            }
        }
        Cli::Digest => {
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            let metabolism = helix_mind_metabolism::MetabolismEngine::new(config.metabolism, storage);
            metabolism.trigger_digest().await?;
            println!("Digest completed");
        }
        Cli::Crystallize => {
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            let metabolism = helix_mind_metabolism::MetabolismEngine::new(config.metabolism, storage);
            metabolism.trigger_crystallize().await?;
            println!("Crystallization completed");
        }
        Cli::Hibernate => {
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            let metabolism = helix_mind_metabolism::MetabolismEngine::new(config.metabolism, storage);
            metabolism.trigger_hibernate().await?;
            println!("Hibernate completed");
        }
        Cli::Reincarnate { confirm } => {
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            let reincarnation = helix_mind_reincarnation::ReincarnationEngine::new(config.lifecycle, storage);
            let new_generation = reincarnation.trigger_reincarnation(&confirm).await?;
            println!("Reincarnation completed, new generation: {}", new_generation);
        }
        Cli::Share { target } => {
            let storage = Arc::new(helix_mind_storage::StorageEngine::new(&config.storage).await?);
            let federation = helix_mind_federation::FederationEngine::new(config.federation, storage);
            let cid = federation.share_dag(target).await?;
            println!("DAG shared, CID: {}", cid);
        }
    }

    Ok(())
}
