use std::path::{Path, PathBuf};
// forge-template/src/context/helpers.rs
use serde_json::{json, Map, Value};

// ============================================================================
// message / header / payload accessors
// ============================================================================

/// Get `"message_name"` with a default.
#[inline]
pub fn get_message_name_or<'a>(raw: &'a Value, default: &'a str) -> &'a str {
    raw.get("message_name").and_then(Value::as_str).unwrap_or(default)
}
/// Join a path segment to repo root
pub fn join_repo<P: AsRef<Path>>(repo_root: P, rel: &str) -> PathBuf {
    repo_root.as_ref().join(rel)
}
/// Back-compat alias: same as `get_message_name_or`.
#[inline]
pub fn get_message_name_or_default<'a>(raw: &'a Value, default: &'a str) -> &'a str {
    get_message_name_or(raw, default)
}

/// Internal: fetch a subobject like conditioned["header"] or ["payload"].
#[inline]
fn get_section_obj<'a>(conditioned: &'a Value, section: &str) -> Option<&'a Map<String, Value>> {
    conditioned.get(section)?.as_object()
}

/// Generic: get a string from a section (e.g. "header") with default.
#[inline]
pub fn section_str_or<'a>(
    conditioned: &'a Value,
    section: &str,
    key: &str,
    default: &'a str,
) -> &'a str {
    get_section_obj(conditioned, section)
        .and_then(|sec| sec.get(key))
        .and_then(Value::as_str)
        .unwrap_or(default)
}

/// Convenience: header.file with default.
#[inline]
pub fn header_file_or_default<'a>(conditioned: &'a Value, default: &'a str) -> &'a str {
    section_str_or(conditioned, "header", "file", default)
}

/// Convenience: header.template with default.
#[inline]
pub fn header_template_or_default<'a>(conditioned: &'a Value, default: &'a str) -> &'a str {
    section_str_or(conditioned, "header", "template", default)
}

/// Convenience: header.type with default. (Used to choose payload adapter; e.g. "dto", "lambdas")
#[inline]
pub fn header_type_or_default<'a>(conditioned: &'a Value, default: &'a str) -> &'a str {
    section_str_or(conditioned, "header", "type", default)
}

// Optional generic helper if you want a consistent pattern for any header key.
#[inline]
pub fn header_str_or_default<'a>(conditioned: &'a Value, key: &str, default: &'a str) -> &'a str {
    section_str_or(conditioned, "header", key, default)
}

/// Generic: get an **array** (cloned) from a section (e.g. "payload") key, or empty.
#[inline]
pub fn section_array_or(conditioned: &Value, section: &str, key: &str) -> Vec<Value> {
    get_section_obj(conditioned, section)
        .and_then(|sec| sec.get(key))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

/// Convenience: payload.primitive_lambdas as Vec<Value> (or empty).
#[inline]
pub fn primitive_lambdas_array(conditioned: &Value) -> Vec<Value> {
    section_array_or(conditioned, "payload", "primitive_lambdas")
}

/// Generic: conditioned["<section>"]["fields"] -> Map (or empty).
#[inline]
pub fn section_fields_map(conditioned: &Value, section: &str) -> Map<String, Value> {
    get_section_obj(conditioned, section)
        .and_then(|sec| sec.get("fields").and_then(Value::as_object))
        .cloned()
        .unwrap_or_default()
}

/// Generic: conditioned["<section>"]["required"] -> Vec<Value> (or empty).
#[inline]
pub fn section_required_array(conditioned: &Value, section: &str) -> Vec<Value> {
    get_section_obj(conditioned, section)
        .and_then(|sec| sec.get("required").and_then(Value::as_array))
        .cloned()
        .unwrap_or_default()
}

// ============================================================================
// small JSON conveniences (frequently used patterns)
// ============================================================================

/// Fetch a field from a JSON object safely, falling back to `default` if not found.
#[inline]
pub fn get_or(obj: &Value, key: &str, default: Value) -> Value {
    obj.get(key).cloned().unwrap_or(default)
}

/// Fetch a string field from a JSON object safely, falling back to `default` if not found.
#[inline]
pub fn get_str_or<'a>(obj: &'a Value, key: &str, default: &'a str) -> &'a str {
    obj.get(key).and_then(Value::as_str).unwrap_or(default)
}

/// Fetch a bool field from a JSON object, falling back to a computed default if missing.
#[inline]
pub fn get_bool_or<F>(obj: &Value, key: &str, fallback: F) -> bool
where
    F: FnOnce() -> bool,
{
    obj.get(key).and_then(Value::as_bool).unwrap_or_else(fallback)
}

/// Get the "payload" object safely, falling back to `{}` if missing or wrong type.
#[inline]
pub fn get_payload_obj(raw: &Value) -> Map<String, Value> {
    raw.get("payload")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default()
}

/// Check if a JSON value is an array of length `len`.
#[inline]
pub fn array_len_is(value: &Value, len: usize) -> bool {
    value.as_array().map(|a| a.len() == len).unwrap_or(false)
}

/// Normalize a `value` field into an array, wrapping scalars or strings.
/// Always returns a JSON array for consistency.
#[inline]
pub fn normalize_value_array(bw: &Value) -> Value {
    match bw.get("value") {
        Some(Value::Array(a))  => Value::Array(a.clone()),
        Some(Value::Number(n)) => json!([n]),
        Some(Value::String(s)) => json!([s]),
        Some(other)            => json!([other]),
        None                   => json!([]),
    }
}

// ============================================================================
// tiny, reusable codegen helpers
// ============================================================================

/// Map JSON schema "type" -> Rust type (for serde_json Value construction in setters).
#[inline]
pub fn json_type_to_rust(t: &str) -> &'static str {
    match t {
        "string"  => "String",
        "boolean" => "bool",
        "integer" => "i64",
        "number"  => "f64",
        _ => "serde_json::Value",
    }
}

/// Iterate a slice and yield `(index, &item, has_next)`.
#[inline]
pub fn enumerate_with_has_next<T>(
    slice: &[T],
) -> impl Iterator<Item=(usize, &T, bool)> {
    (0..slice.len()).map(move |i| (i, &slice[i], i + 1 < slice.len()))
}

/// Convert a `serde_json::Value` into an owned object map ({} if not an object).
#[inline]
pub fn as_object_map(v: &Value) -> Map<String, Value> {
    v.as_object().cloned().unwrap_or_default()
}

/// Insert a `bool` into a JSON object map under `key`.
#[inline]
pub fn insert_bool_kv(target: &mut Map<String, Value>, key: &str, val: bool) {
    target.insert(key.to_string(), Value::Bool(val));
}

/// Make a `{ name, json_type, rust_type }` field map.
#[inline]
pub fn make_field_map(name: &str, json_ty: &str, rust_ty: &str) -> Map<String, Value> {
    let mut m = Map::new();
    m.insert("name".into(),      Value::String(name.to_string()));
    m.insert("json_type".into(), Value::String(json_ty.to_string()));
    m.insert("rust_type".into(), Value::String(rust_ty.to_string()));
    m
}