//! Pre-render YAML stage (MVP).
//! Steps:
//! 1) read YAML bytes
//! 2) YAML -> JSON (pretty, logged)
//! 3) JSON expansion (derive naming variants for: message type, every struct name, every field key)
//! 4) JSON -> Mustache data mappings (extract strings, add stage marker)
//! 5) return final map for Mustache

use std::collections::BTreeMap;

/// Final map type for Mustache (MVP: string→string).
pub type MustacheMap = BTreeMap<String, String>;

/// Run the pre-render YAML stage.
/// - `values_yaml_path`: path to values.yaml
/// Returns the final map used by the renderer.
pub fn run(values_yaml_path: &str) -> Result<MustacheMap, Box<dyn std::error::Error>> {
    println!("[pre_render_yaml_stage] start path='{values_yaml_path}'");

    // 1) Read YAML bytes
    let yaml_bytes = read_yaml_bytes(values_yaml_path)?;

    // 2) Convert YAML -> JSON (pretty)
    let json = yaml_to_json(&yaml_bytes)?;

    // 3) Expansion over JSON (derive templating-friendly keys for EVERYTHING)
    let json_expanded = expansion_json(&json)?;

    // 4) Convert JSON -> Mustache data mappings
    let data = json_to_mustache_map(&json_expanded)?;

    // 5) Return final map
    println!("[pre_render_yaml_stage] done; keys={}", data.len());
    Ok(data)
}

// ---------------------- stages ----------------------

fn read_yaml_bytes(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("[pre_render_yaml_stage] read_yaml_bytes('{path}')");
    Ok(std::fs::read(path)?)
}

fn yaml_to_json(yaml: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    println!("[pre_render_yaml_stage] yaml_to_json()");
    let value: serde_yaml::Value = serde_yaml::from_slice(yaml)?;
    let json_pretty = serde_json::to_string_pretty(&value)?;
    println!("[pre_render_yaml_stage] JSON (pretty):\n{}\n--- end JSON ---", json_pretty);
    Ok(json_pretty)
}

/// Expand JSON by adding naming variants derived from a canonical snake_case baseline.
/// We expand:
/// - header.type  → message_type_*
/// - payload.structs[].name → struct_name_* (attached under that struct)
/// - payload.structs[].fields keys → <struct>_<field>_* (attached under fields_expanded)
fn expansion_json(json_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[pre_render_yaml_stage] expansion_json()");

    // Parse the input JSON
    let mut v: serde_json::Value = serde_json::from_str(json_input)?;

    // ---------- header.type → message_type_* ----------
    let message_type: Option<String> = v
        .get("header")
        .and_then(|h| h.get("type"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    if let Some(mt) = message_type.as_deref() {
        let pairs = crate::pipeline::expansion::expand_pair("message_type", mt);
        if let Some(root) = v.as_object_mut() {
            let header_entry = root.entry("header").or_insert(serde_json::json!({}));
            if let Some(header_obj) = header_entry.as_object_mut() {
                for (k, val) in pairs {
                    header_obj.insert(k, serde_json::Value::String(val));
                }
            }
        }
    }

    // ---------- payload.structs[].name + fields keys ----------
    // Collect indices to avoid borrow issues if we need nested mutations.
    // We mutate each struct object independently.
    if let Some(payload) = v.get_mut("payload") {
        if let Some(structs) = payload.get_mut("structs") {
            if let Some(arr) = structs.as_array_mut() {
                for st in arr.iter_mut() {
                    // Work on a single struct object
                    let (struct_name, struct_snake) = {
                        let name_opt = st.get("name").and_then(|n| n.as_str()).unwrap_or_default();
                        // Get snake casing using our expansion engine as a baseline builder.
                        let tmp = crate::pipeline::expansion::expand_pair("tmp", name_opt);
                        let snake = tmp.get("tmp_snake_case").cloned().unwrap_or_else(|| name_opt.to_string());
                        (name_opt.to_string(), snake)
                    };

                    // Insert struct_name_* variants onto the struct itself.
                    if let Some(st_obj) = st.as_object_mut() {
                        let struct_pairs = crate::pipeline::expansion::expand_pair("struct_name", &struct_name);
                        for (k, val) in struct_pairs {
                            st_obj.insert(k, serde_json::Value::String(val));
                        }

                        // Now expand each field key. We create/replace a "fields_expanded" object.
                        if let Some(fields_obj) = st_obj.get_mut("fields").and_then(|f| f.as_object_mut()) {
                            // Snapshot keys to avoid double borrow.
                            let field_keys: Vec<String> = fields_obj.keys().cloned().collect();
                            let mut expanded_map = serde_json::Map::new();

                            for fk in field_keys {
                                let base = format!("{}_{}", struct_snake, fk);
                                let pairs = crate::pipeline::expansion::expand_pair(&base, &fk);
                                for (k, val) in pairs {
                                    expanded_map.insert(k, serde_json::Value::String(val));
                                }
                            }

                            st_obj.insert("fields_expanded".to_string(), serde_json::Value::Object(expanded_map));
                        }
                    }
                }
            }
        }
    }

    let pretty = serde_json::to_string_pretty(&v)?;
    println!("[pre_render_yaml_stage] JSON+expansion (pretty):\n{}\n--- end JSON ---", pretty);
    Ok(serde_json::to_string(&v)?)
}

/// Very light “mapping” step for now:
/// - Pull all string fields from `header` into the mustache map
/// - Add a `_stage` marker so tests can assert the pipeline ran
fn json_to_mustache_map(json: &str) -> Result<MustacheMap, Box<dyn std::error::Error>> {
    println!("[pre_render_yaml_stage] json_to_mustache_map()");
    let v: serde_json::Value = serde_json::from_str(json)?;
    let mut map = MustacheMap::new();

    if let Some(header) = v.get("header").and_then(|h| h.as_object()) {
        for (k, val) in header {
            if let Some(s) = val.as_str() {
                map.insert(k.clone(), s.to_string());
            }
        }
    }

    // Ceremonial marker
    map.insert("_stage".into(), "pre_render_yaml_stage".into());
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_pre_render_yaml_stage() {
        // Base path comes from FORGE_ROOT (must be set when running tests)
        let forge_root = std::env::var("FORGE_ROOT")
            .expect("FORGE_ROOT must be set (e.g. /Users/leonardorocha/forge-oss)");
        let path = format!("{}/resources/templates/design_tokens.yaml", forge_root);

        assert!(
            std::path::Path::new(&path).exists(),
            "missing {}",
            path
        );

        let out = run(&path).expect("pipeline run ok");

        // pipeline marker
        assert_eq!(
            out.get("_stage").map(String::as_str),
            Some("pre_render_yaml_stage")
        );

        // If header.type exists, expansion should have produced message_type_* keys
        if out.contains_key("message_type_snake_case") {
            assert!(out.get("message_type_PascalCase").is_some());
            assert!(out.get("message_type_camelCase").is_some());
            assert!(out.get("message_type_SCREAMING_SNAKE_CASE").is_some());
            assert!(out.get("message_type_kebab_case").is_some());
        }
    }
}
