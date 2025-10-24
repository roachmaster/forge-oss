// ============================================================================
// Forge Template Codegen Expansion Engine
// File: expand.rs
// Role: Recursive expander for ValuesTemplateExpansion across arrays, objects,
//       and scalar values with deterministic structure and hasNext flags.
// ============================================================================

use serde_json::{Map, Value};
use crate::codegen::utils::{
    expand_string_value,
    insert_expansions,
    insert_has_next_flag,
};
use crate::codegen::flags::insert_type_flags;

/// Expand a single element of an array (handles strings, objects, and nested structures).
fn expand_array_element(v: &Value) -> Value {
    match v {
        Value::String(s) => Value::Object(expand_string_value("value", s)),
        Value::Object(o) => Value::Object(expand_object_fields(o)),
        _ => v.clone(),
    }
}

/// Recursively expand all key/value pairs inside an object.
/// - Adds naming variants via `ValuesTemplateExpansion`
/// - Recurses into nested objects/arrays
/// - Appends type flags for Mustache logic
pub fn expand_object_fields(obj: &Map<String, Value>) -> Map<String, Value> {
    let mut expanded = obj.clone();

    for (k, v) in obj {
        if let Some(s) = v.as_str() {
            insert_expansions(&mut expanded, k, s);
            insert_type_flags(&mut expanded, v);
        } else if v.is_array() || v.is_object() {
            expanded.insert(k.clone(), expand_value(v));
            insert_type_flags(&mut expanded, v);
        } else {
            expanded.insert(k.clone(), v.clone());
        }
    }

    insert_type_flags(&mut expanded, &Value::Object(obj.clone()));
    expanded
}

/// Recursively expand any JSON value into a deterministic Mustache context.
pub fn expand_value(value: &Value) -> Value {
    match value {
        Value::Array(arr) => {
            let out: Vec<Value> = arr.iter().map(expand_array_element).collect();
            insert_has_next_flag(out)
        }
        Value::Object(o) => Value::Object(expand_object_fields(o)),
        Value::String(s) => Value::Object(expand_string_value("value", s)),
        _ => value.clone(),
    }
}
