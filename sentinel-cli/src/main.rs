use clap::{Parser, Subcommand};
use sentinel_core::Severity;

#[derive(Debug, Parser)]
#[command(name = "sentinel", about = "AppSec Scanning Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Scan {
        #[arg(long)]
        url: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        severity: Severity,
        #[arg(long)]
        evidence: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan {
            url,
            title,
            severity,
            evidence,
        } => {
            println!("url: {}", url);
            println!("title: {}", title);
            println!("severity: {:?}", severity);
            println!("evidence: {:?}", evidence);
        }
    }
}
