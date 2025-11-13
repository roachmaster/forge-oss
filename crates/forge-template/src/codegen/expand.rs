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

// -----------------------------------------------------------------------------
// Expansion Policy — selective filtering for which keys may be expanded
// -----------------------------------------------------------------------------

/// Returns whether a given key should *not* be expanded at all.
/// This includes semantic/documentation fields like desc/docs/comments.
pub fn is_non_expanding_key(key: &str) -> bool {
    matches!(
        key,
        "desc"
            | "description"
            | "docs"
            | "comment"
            | "note"
            | "explain"
            | "details"
            | "summary"
    )
}

/// Determines whether a key/value pair is safe to expand into naming variants.
/// Context-aware: skips code fragments, Rust syntax, and doc fields.
pub fn is_expandable_key(key: &str, value: &Value) -> bool {
    // Never expand documentation-like fields
    if is_non_expanding_key(key) {
        return false;
    }

    // If it's a string, inspect its content
    if let Some(s) = value.as_str() {
        let looks_like_code = s.contains("::")
            || s.contains("()")
            || s.contains("<")
            || s.contains(">")
            || s.contains("=>")
            || s.contains(";")
            || s.contains("{")
            || s.contains("}")
            || s.contains("let ")
            || s.contains("self.")
            || s.contains("::new");

        // Skip expansions for code fragments or type syntax
        if looks_like_code {
            return false;
        }
    }

    // Expand only semantic identifier-style fields
    matches!(
        key,
        "name"
            | "type"
            | "module"
            | "pattern"
            | "variant"
            | "variable"
            | "call"
            | "path"
            | "value" // only if value passes above check
    )
}

// -----------------------------------------------------------------------------
// Core Expansion Logic
// -----------------------------------------------------------------------------

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
/// - Respects non-expanding keys
/// - Recurses into nested objects/arrays
/// - Appends type flags for Mustache logic
pub fn expand_object_fields(obj: &Map<String, Value>) -> Map<String, Value> {
    let mut expanded = obj.clone();

    for (k, v) in obj {
        if is_non_expanding_key(k) {
            // Preserve literal doc fields
            expanded.insert(k.clone(), v.clone());
            continue;
        }

        if let Some(s) = v.as_str() {
            // Only expand if allowed for this key/value pair
            if is_expandable_key(k, v) {
                insert_expansions(&mut expanded, k, s);
            }
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
