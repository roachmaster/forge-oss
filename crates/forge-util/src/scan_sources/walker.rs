use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::model::{CrateMap, DirNode};

fn is_excluded(name: &str, excludes: &[String]) -> bool {
    excludes.iter().any(|e| e == name)
}

fn push_sorted(vec: &mut Vec<String>, name: String) {
    vec.push(name);
    vec.sort();
}

fn read_dir_sorted(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    for e in fs::read_dir(path).with_context(|| format!("read_dir {}", path.display()))? {
        let e = e?;
        entries.push(e.path());
    }
    entries.sort();
    Ok(entries)
}

fn build_dir_node(dir: &Path, excludes: &[String], verbose: bool) -> Result<DirNode> {
    if verbose {
        eprintln!("  • visiting dir {}", dir.display());
    }

    let mut node = DirNode::default();

    for entry in read_dir_sorted(dir)? {
        let name = entry.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if name.is_empty() || is_excluded(&name, excludes) {
            if verbose {
                eprintln!("    ⤷ skip {}", entry.display());
            }
            continue;
        }

        if entry.is_dir() {
            let child = build_dir_node(&entry, excludes, verbose)?;
            if !child.files.is_empty() || !child.dirs.is_empty() {
                node.dirs.insert(name, child);
            }
        } else if entry.is_file() {
            if entry.extension().and_then(|s| s.to_str()).unwrap_or("") == "rs" {
                push_sorted(&mut node.files, name);
            }
        }
    }

    if verbose {
        eprintln!(
            "    ⤷ done {} (files: {}, subdirs: {})",
            dir.display(),
            node.files.len(),
            node.dirs.len()
        );
    }

    Ok(node)
}

pub fn scan_workspace(crates_root: &Path, excludes: &[String], verbose: bool) -> Result<CrateMap> {
    let mut map = BTreeMap::new();

    let entries = match fs::read_dir(crates_root) {
        Ok(rd) => rd,
        Err(_) => {
            // If workspace layout is repo root (no `crates/`), fall back to parent
            // but keep behavior stable: only consider immediate children that are crates (have Cargo.toml)
            if verbose {
                eprintln!("⚠️  `crates/` not found at {}, falling back to {}", 
                    crates_root.display(),
                    crates_root.parent().map(|p| p.display().to_string()).unwrap_or_default()
                );
            }
            return Ok(map); // or choose to walk parent; current CLI sets root correctly so this shouldn't happen
        }
    };

    let mut crate_dirs: Vec<PathBuf> = Vec::new();
    for e in entries {
        let p = e?.path();
        if p.is_dir() && p.join("Cargo.toml").exists() {
            crate_dirs.push(p);
        }
    }
    crate_dirs.sort();

    for cd in crate_dirs {
        let crate_name = cd.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if crate_name.is_empty() { continue; }

        if verbose {
            eprintln!("🔹 crate `{}` at {}", crate_name, cd.display());
        }

        // Only scan `src` within each crate
        let src = cd.join("src");
        let node = if src.exists() && src.is_dir() {
            build_dir_node(&src, excludes, verbose)?
        } else {
            DirNode::default()
        };

        map.insert(crate_name, node);
    }

    Ok(map)
}
