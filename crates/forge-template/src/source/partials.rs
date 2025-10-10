//! Partials resolution helpers (MVP: directory-based only).

use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::{RenderError, Result};
use crate::options::Partials;

pub fn resolve_partial(cfg: &Partials, name: &str) -> Result<Option<String>> {
    match cfg {
        Partials::None => Ok(None),

        Partials::Dir(dir) => Ok(read_from_dir(dir, name)?),

        Partials::Map(map) => Ok(map.get(name).cloned()),

        Partials::MapThenDir { map, dir } => {
            if let Some(s) = map.get(name) {
                return Ok(Some(s.clone()));
            }
            Ok(read_from_dir(dir, name)?)
        }
    }
}

fn read_from_dir(dir: &Path, name: &str) -> Result<Option<String>> {
    let path = partial_path(dir, name);
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|e| RenderError::Io {
        path: path.clone(),
        source: e,
    })?;
    Ok(Some(text))
}

fn partial_path(dir: &Path, name: &str) -> PathBuf {
    let mut p = PathBuf::from(dir);
    // Support nested partial names like "shared/field_setter"
    p.push(Path::new(name));
    if p.extension().is_none() {
        p.set_extension("mustache");
    }
    p
}