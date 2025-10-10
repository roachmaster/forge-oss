//! YAML loader helpers.

use std::fs;
use std::path::Path;
use serde_json::Value as JsonValue;

/// Load a YAML file into a serde_json::Value.
pub fn load_yaml_as_json(path: &str) -> Result<JsonValue, String> {
    println!("[io::yaml] loading {}", path);
    let abs = Path::new(path);
    let bytes = fs::read(abs)
        .map_err(|e| format!("failed to read {}: {e}", abs.display()))?;
    let yaml: serde_yaml::Value = serde_yaml::from_slice(&bytes)
        .map_err(|e| format!("failed to parse YAML {}: {e}", abs.display()))?;
    serde_json::to_value(yaml)
        .map_err(|e| format!("failed to convert YAML to JSON {}: {e}", abs.display()))
}
