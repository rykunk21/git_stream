use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

mod commands;
mod git;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Initialize Git Stream for a repository
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Start monitoring git repository
    Watch {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Export repository data
    Export {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, default_value = "jsonl")]
        format: String,
    },
    /// Query repository data
    Query {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        author: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            println!("Initializing Git Stream in {:?}", path);
            commands::init_repository(&path)?;
        }
        Commands::Watch { path } => {
            println!("Starting repository watch on {:?}", path);
            commands::watch_repository(&path)?;
        }
        Commands::Export { path, format } => {
            println!(
                "Exporting repository data from {:?} in {} format",
                path, format
            );
            commands::export_repository(&path, &format)?;
        }
        Commands::Query {
            path,
            since,
            author,
        } => {
            println!("Querying repository {:?}", path);
            commands::query_repository(&path, since.as_deref(), author.as_deref())?;
        }
    }

    Ok(())
}
