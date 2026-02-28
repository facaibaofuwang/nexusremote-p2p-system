//! NexusRemote - Decentralized P2P Remote Control System

use clap::Parser;
use nexusremote::*;
use tracing::{info, Level};

/// NexusRemote CLI
#[derive(Parser, Debug)]
#[command(name = "nexusremote")]
#[command(about = "Decentralized P2P remote control system with token incentives", long_about = None)]
struct Cli {
    /// Subcommand
    #[command(subcommand)]
    command: Commands,
}

/// CLI commands
#[derive(Debug, Parser)]
enum Commands {
    /// Start the node
    Start {
        /// Enable relay mode
        #[arg(long)]
        relay: bool,
    },
    
    /// Mine initial tokens
    Mine,
    
    /// Check wallet status
    Wallet,
    
    /// Find a peer
    Find {
        /// Target device ID (hex)
        target: String,
    },
    
    /// Run network simulation
    Simulate {
        /// Number of nodes
        #[arg(long, default_value = "100")]
        nodes: usize,
        
        /// Number of lookups
        #[arg(long, default_value = "1000")]
        lookups: usize,
    },
    
    /// Test weighted routing
    TestRouting,
    
    /// Show version
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Start { relay } => {
            info!("Starting NexusRemote node...");
            if *relay {
                info!("Relay mode enabled");
            }
            info!("Node started. Press Ctrl+C to stop.");
        }
        
        Commands::Mine => {
            info!("Starting PoW mining for initial tokens...");
            let keypair = core::crypto::NodeKeypair::generate();
            let miner = wallet::mining::PowMiner::new();
            
            info!("This may take a few minutes...");
            let result = miner.mine_initial_tokens(keypair.node_id().as_bytes()).await?;
            
            if result.success {
                info!("🎉 Mining successful!");
                info!("Reward: {}", result.reward);
                info!("Time taken: {:.2}s", result.time_taken);
                info!("Attempts: {}", result.attempts);
            }
        }
        
        Commands::Wallet => {
            info!("Wallet status:");
            info!("Balance: 0 NEXUS");
            info!("Reputation: 100");
            info!("(Wallet functionality will be implemented)");
        }
        
        Commands::Find { target } => {
            info!("Looking for peer: {}", target);
            info!("(Peer discovery will be implemented)");
        }
        
        Commands::Simulate { nodes, lookups } => {
            info!("Starting network simulation...");
            info!("Nodes: {}", nodes);
            info!("Lookups: {}", lookups);
            
            let mut sim = simulator::network::NetworkSimulator::new();
            sim.create_random_nodes(*nodes);
            sim.connect_mesh(10);
            
            let results = sim.run_routing_simulation(*lookups);
            
            info!("\n=== Simulation Results ===");
            info!("Total nodes: {}", results.total_nodes);
            info!("High reputation nodes: {}", results.high_rep_nodes);
            info!("Low reputation nodes: {}", results.low_rep_nodes);
            info!("High reputation selection rate: {:.2}%", results.high_rep_selection_rate * 100.0);
            
            let expected_rate = results.high_rep_nodes as f64 / results.total_nodes as f64;
            info!("Expected random rate: {:.2}%", expected_rate * 100.0);
            
            if results.high_rep_selection_rate > expected_rate {
                let advantage = results.high_rep_selection_rate / expected_rate;
                info!("✅ High reputation nodes have {:.2}x routing advantage!", advantage);
            } else {
                info!("⚠️ No significant reputation advantage observed");
            }
        }
        
        Commands::TestRouting => {
            info!("Testing weighted routing algorithm...");
            
            let mut sim = simulator::network::NetworkSimulator::new();
            let has_advantage = sim.demonstrate_weighted_routing();
            
            if has_advantage {
                info!("✅ SUCCESS: High reputation nodes are preferred in routing!");
            } else {
                info!("⚠️ Test completed, but advantage may be small");
            }
            
            info!("Run 'nexusremote simulate' for detailed statistics");
        }
        
        Commands::Version => {
            println!("NexusRemote v3.0.0");
            println!("A decentralized P2P remote control system with token incentives");
            println!();
            println!("Project homepage: https://github.com/nexusremote/nexusremote");
        }
    }
    
    Ok(())
}
