use std::{fs, path::Path, time::SystemTime};

use anyhow::{Context, Ok, Result};
use clap::Parser;
use jwalk::WalkDir;
use mimalloc::MiMalloc;
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
struct Args {
    /// Directory sorgente
    #[arg(short, long)]
    source: String,

    /// Directory destinazione
    #[arg(short, long)]
    dest: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source_root = fs::canonicalize(&args.source)
        .with_context(|| format!("❌ Sorgente inaccessibile o inesistente: {:?}", args.source))?;

    fs::create_dir_all(&args.dest).with_context(|| {
        format!(
            "❌ Impossibile creare cartella destinazione: {:?}",
            args.dest
        )
    })?;

    let dest_root = fs::canonicalize(&args.dest)
        .with_context(|| format!("❌ Destinazione invalida: {:?}", args.dest))?;

    println!("🚀 Fast Sync avviato...");
    println!("   📂 Da: {:?}", source_root);
    println!("   📂 A:  {:?}", dest_root);

    WalkDir::new(&source_root)
        .follow_links(false)
        .skip_hidden(false)
        .sort(false)
        .parallelism(jwalk::Parallelism::RayonNewPool(0)) // Usa pool thread ottimale
        .into_iter()
        .par_bridge()
        .for_each(|entry| {
            if let Err(e) = process_entry(entry, &source_root, &dest_root) {
                eprintln!("⚠️  Errore file: {}", e);
            }
        });

    println!("✅ Finito.");
    Ok(())
}

fn process_entry(
    entry: std::result::Result<jwalk::DirEntry<((), ())>, jwalk::Error>,
    source_root: &Path,
    dest_root: &Path,
) -> Result<()> {
    let entry = entry?;
    if !entry.file_type().is_file() {
        return Ok(());
    }

    let src_path = entry.path();
    let relative_path = src_path.strip_prefix(source_root)?;
    let dest_path = dest_root.join(relative_path);

    let src_meta = entry.metadata()?;

    if should_copy(&src_meta, &dest_path) {
        let _ = copy(&src_path, &dest_path);
    }

    Ok(())
}

#[inline(always)]
fn should_copy(src_meta: &fs::Metadata, dst: &Path) -> bool {
    match fs::metadata(dst) {
        std::result::Result::Ok(dst_meta) => {
            // Check fast: Size, if different -> copy
            if src_meta.len() != dst_meta.len() {
                return true;
            }
            // Check slow: Date, if source is newer -> copy
            let src_time = src_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let dst_time = dst_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            src_time > dst_time
        }
        Err(_) => true, // If error -> overwrite
    }
}

// Optimistic copy -> first try to copy, if fails due to missing directory, create it and retry
fn copy(src: &Path, dst: &Path) -> Result<()> {
    if let Err(e) = fs::copy(src, dst) {
        if e.kind() == std::io::ErrorKind::NotFound {
            if let Some(parent) = dst.parent() {
                let _ = fs::create_dir_all(parent);
                let _ = fs::copy(src, dst);
            }
        }
    }
    Ok(())
}
