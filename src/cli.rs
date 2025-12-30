use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "BackupTool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Sincronizza due cartelle
    Sync {
        #[arg(short, long)]
        src: String,
        #[arg(short, long)]
        dst: String,
    },
}
