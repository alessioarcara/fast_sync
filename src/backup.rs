use anyhow::{Ok, Result};
use jwalk::WalkDir;
use rayon::prelude::*;
use std::{
    fs::{self},
    path::Path,
    time::SystemTime,
};

pub fn sync_recursive(src: &Path, dst: &Path) -> Result<()> {
    WalkDir::new(src)
        .follow_links(false)
        .skip_hidden(false)
        .sort(false)
        .parallelism(jwalk::Parallelism::RayonNewPool(0)) // Usa pool thread ottimale
        .into_iter()
        .par_bridge()
        .for_each(|entry| {
            if let Err(e) = process_copy_entry(entry, &src, &dst) {
                eprintln!("⚠️  Errore file: {}", e);
            }
        });

    Ok(())
}

pub fn compress_recursive(src: &Path, dst: &Path) -> Result<()> {
    Ok(())
}

fn process_copy_entry(
    entry: std::result::Result<jwalk::DirEntry<((), ())>, jwalk::Error>,
    src: &Path,
    dst: &Path,
) -> Result<()> {
    let entry = entry?;
    if !entry.file_type().is_file() {
        return Ok(());
    }

    let src_path = entry.path();
    let relative_path = src_path.strip_prefix(src)?;
    let dest_path = dst.join(relative_path);

    let src_meta = entry.metadata()?;

    if should_copy(&src_meta, &dest_path) {
        let _ = copy(&src_path, &dest_path);
    }

    Ok(())
}

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
