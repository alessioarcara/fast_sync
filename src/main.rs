use clap::Parser;
use std::fs;

use anyhow::{Context, Ok, Result};
use mimalloc::MiMalloc;

mod backup;
mod cli;

use cli::Commands;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match &cli.command {
        Commands::Sync { src, dst } => {
            let src = fs::canonicalize(&src).context("Source invalida")?;
            fs::create_dir_all(&dst)?;
            let dst = fs::canonicalize(&dst).context("Dest invalida")?;

            println!("🔄 Syncing da {:?} a {:?}", src, dst);
            backup::sync_recursive(&src, &dst)?;
            println!("✅ Finito.");
        }
    };

    Ok(())
}
