use clap::{Parser, Subcommand};
mod build;
mod config;
mod error;
mod remote;
mod utils;

#[derive(Parser)]
#[command(name = "flutter-cross-builder")]
#[command(about = "Cross-platform Flutter build tool for Windows", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build for iOS platform
    Ios {
        #[arg(short, long)]
        config: Option<String>,
        
        #[arg(short, long)]
        release: bool,
    },
    /// Build for Windows platform
    Windows {
        #[arg(short, long)]
        config: Option<String>,
        
        #[arg(short, long)]
        release: bool,
    },
    /// Configure remote build server
    Configure {
        #[arg(short, long)]
        server: String,
        
        #[arg(short, long)]
        api_key: String,
    },
}

fn main() -> Result<(), error::BuildError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ios { config, release } => {
            build::ios::build(config, release)?;
        }
        Commands::Windows { config, release } => {
            build::windows::build(config, release)?;
        }
        Commands::Configure { server, api_key } => {
            config::save_config(&server, &api_key)?;
        }
    }
    Ok(())
}