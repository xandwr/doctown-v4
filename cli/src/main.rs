mod types;
mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "localdoc")]
#[command(about = "CLI tool for inspecting .docpack files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show quick info about a docpack
    Info {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,
    },

    /// Show detailed statistics
    Stats {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,
    },

    /// List nodes in the graph
    List {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,

        /// Filter by node type (function, type, module, file, cluster)
        #[arg(short, long)]
        kind: Option<String>,

        /// Only show public nodes
        #[arg(short, long)]
        public: bool,

        /// Limit number of results
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },

    /// Inspect a specific node by ID
    Inspect {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,

        /// Node ID to inspect
        #[arg(value_name = "NODE_ID")]
        node_id: String,
    },

    /// Search for nodes by name
    Search {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,

        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Case-sensitive search
        #[arg(short, long)]
        case_sensitive: bool,
    },

    /// Extract files from the docpack
    Extract {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },

    /// Show documentation for a node
    Docs {
        /// Path to .docpack file
        #[arg(value_name = "FILE")]
        docpack: PathBuf,

        /// Node ID to show documentation for
        #[arg(value_name = "NODE_ID")]
        node_id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { docpack } => {
            commands::info::run(docpack)?;
        }
        Commands::Stats { docpack } => {
            commands::stats::run(docpack)?;
        }
        Commands::List { docpack, kind, public, limit } => {
            commands::list::run(docpack, kind, public, limit)?;
        }
        Commands::Inspect { docpack, node_id } => {
            commands::inspect::run(docpack, node_id)?;
        }
        Commands::Search { docpack, query, case_sensitive } => {
            commands::search::run(docpack, query, case_sensitive)?;
        }
        Commands::Extract { docpack, output } => {
            commands::extract::run(docpack, output)?;
        }
        Commands::Docs { docpack, node_id } => {
            commands::docs::run(docpack, node_id)?;
        }
    }

    Ok(())
}
