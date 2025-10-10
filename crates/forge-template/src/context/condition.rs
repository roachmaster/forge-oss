// crates/forge-template/src/context/condition.rs
//! Generic conditioning: only stabilize the shapes of `header` and `payload`.
//! - If `header` is an object, pass it through; otherwise use `{}`
//! - If `payload` is an object, pass it through; otherwise use `{}`
//! No other keys are interpreted or synthesized here.

use serde_json::{Map, Value};

/// Return `v` as an owned object map if it's an object; else an empty `{}`.
#[inline]
fn as_object_map(v: &Value) -> Map<String, Value> {
    v.as_object().cloned().unwrap_or_default()
}

/// Generic conditioner used for both JSON schema inputs and YAML inputs.
/// Only references `header` and `payload`.
#[inline]
fn condition_generic(raw: &Value) -> Value {
    let header = as_object_map(raw.get("header").unwrap_or(&Value::Null));
    let payload = as_object_map(raw.get("payload").unwrap_or(&Value::Null));

    let mut out = Map::new();
    out.insert("header".to_string(), Value::Object(header));
    out.insert("payload".to_string(), Value::Object(payload));
    Value::Object(out)
}

/// JSON path: generic—only return `{ header: {...}, payload: {...} }`.
pub fn condition_schema(raw: &Value) -> Value {
    condition_generic(raw)
}

/// YAML path: generic—only return `{ header: {...}, payload: {...} }`.
pub fn condition_yaml(raw: &Value) -> Value {
    condition_generic(raw)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn generic_condition_missing_sections() {
        let raw = json!({"other": 123});
        let out = condition_schema(&raw);
        assert!(out.get("header").unwrap().is_object());
        assert!(out.get("payload").unwrap().is_object());
        assert_eq!(out["header"].as_object().unwrap().len(), 0);
        assert_eq!(out["payload"].as_object().unwrap().len(), 0);
    }

    #[test]
    fn generic_condition_passes_through_objects() {
        let raw = json!({
            "header": { "template": "x.mustache", "type": "dto", "custom": 42 },
            "payload": { "structs": [ {"name":"A","fields":{"x":"u32"}} ], "extra": true }
        });
        let out = condition_yaml(&raw);
        assert_eq!(out["header"]["template"], "x.mustache");
        assert_eq!(out["header"]["type"], "dto");
        assert_eq!(out["header"]["custom"], 42);
        assert!(out["payload"]["structs"].is_array());
        assert_eq!(out["payload"]["extra"], true);
    }

    #[test]
    fn non_object_sections_become_empty_objects() {
        let raw = json!({
            "header": "not-an-object",
            "payload": 123
        });
        let out = condition_yaml(&raw);
        assert!(out["header"].is_object());
        assert!(out["payload"].is_object());
        assert_eq!(out["header"].as_object().unwrap().len(), 0);
        assert_eq!(out["payload"].as_object().unwrap().len(), 0);
    }
}
