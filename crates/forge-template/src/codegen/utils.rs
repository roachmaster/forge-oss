// ============================================================================
// Forge Template Codegen Utilities
// File: utils.rs
// Role: Shared helpers for expansion routines — keeps `expand.rs` clean by
//       encapsulating string/value expansion, hasNext flag insertion, and
//       ValueTemplateExpansion integration logic.
// ============================================================================

use serde_json::{Map, Value};
use std::collections::BTreeMap;
use crate::context::values_template_expansion::ValuesTemplateExpansion;

// ----------------------------------------------------------------------------
// Expansion Utilities
// ----------------------------------------------------------------------------

/// Expand a string key/value pair into a JSON object with all case variants.
/// Example: ("value", "model") → { "value": "model", "value_snake_case": "model", ... }
pub fn expand_string_value(key: &str, value: &str) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(key.into(), Value::String(value.to_string()));
    insert_expansions(&mut map, key, value);
    map
}

/// Generate all case variants (snake, Pascal, camel, etc.) and insert into the map.
pub fn insert_expansions(map: &mut Map<String, Value>, key: &str, value: &str) {
    let expansions: BTreeMap<String, String> = ValuesTemplateExpansion::expands_to_map(key, value);
    for (k, v) in expansions {
        map.insert(k, Value::String(v));
    }
}

// ----------------------------------------------------------------------------
// Array Utilities
// ----------------------------------------------------------------------------

/// Insert a `hasNext` boolean flag for all array elements.
/// - Used in Mustache templates to detect the last element.
/// - Returns `Value::Array` with modified elements.
pub fn insert_has_next_flag(arr: Vec<Value>) -> Value {
    let len = arr.len();
    let mut out = Vec::with_capacity(len);

    for (i, mut v) in arr.into_iter().enumerate() {
        if let Some(obj) = v.as_object_mut() {
            obj.insert("hasNext".into(), Value::Bool(i < len - 1));
            out.push(Value::Object(obj.clone()));
        } else {
            out.push(v);
        }
    }

    Value::Array(out)
}
