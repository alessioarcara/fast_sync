use clap::Parser;
use std::fs;

use anyhow::{Context, Ok, Result};
use mimalloc::MiMalloc;

mod args;
mod backup;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<()> {
    let args = args::Args::parse();

    let source = fs::canonicalize(&args.source).context("Source invalida")?;
    fs::create_dir_all(&args.dest)?;
    let dest = fs::canonicalize(&args.dest).context("Dest invalida")?;

    println!("🚀 Fast Sync avviato...");
    println!("   📂 Da: {:?}", source);
    println!("   📂 A:  {:?}", dest);
    backup::sync_recursive(&source, &dest)?;
    //backup::prune_files(&source, &dest)?;
    println!("✅ Finito.");
    Ok(())
}
