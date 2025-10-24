// ============================================================================
// Forge Template Codegen Type Flags
// File: flags.rs
// Role: Adds `is_*` boolean flags to JSON maps for Mustache logic-less templates.
//       Helps templates differentiate between strings, integers, floats, booleans,
//       and raw JSON values without runtime introspection.
// ============================================================================

use serde_json::{Map, Value};

/// Insert boolean type flags based on a given `serde_json::Value`.
/// These are explicit because Mustache cannot introspect at runtime.
pub fn insert_type_flags(target: &mut Map<String, Value>, value: &Value) {
    let (is_string, is_boolean, is_integer, is_number, is_object, is_array, is_null) =
        match value {
            Value::String(_) => (true, false, false, false, false, false, false),
            Value::Bool(_)   => (false, true,  false, false, false, false, false),
            Value::Number(n) => {
                if n.is_i64() {
                    (false, false, true,  false, false, false, false)
                } else {
                    (false, false, false, true,  false, false, false)
                }
            }
            Value::Object(_) => (false, false, false, false, true,  false, false),
            Value::Array(_)  => (false, false, false, false, false, true,  false),
            Value::Null      => (false, false, false, false, false, false, true),
        };

    target.insert("is_string".into(),  Value::Bool(is_string));
    target.insert("is_boolean".into(), Value::Bool(is_boolean));
    target.insert("is_integer".into(), Value::Bool(is_integer));
    target.insert("is_number".into(),  Value::Bool(is_number));
    target.insert("is_object".into(),  Value::Bool(is_object));
    target.insert("is_array".into(),   Value::Bool(is_array));
    target.insert("is_null".into(),    Value::Bool(is_null));
}
