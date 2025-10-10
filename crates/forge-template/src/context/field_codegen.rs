use serde_json::{Map, Value};
use crate::context::values_template_expansion::ValuesTemplateExpansion;
use std::collections::BTreeMap;
use crate::context::helpers::json_type_to_rust;

/// Expand (key,value) and merge into a serde_json object (Pascal/snake/camel, etc.).
fn merge_expansions(obj: &mut Map<String, Value>, key: &str, value: &str) {
    let m: BTreeMap<String, String> = ValuesTemplateExpansion::expands_to_map(key, value);
    for (k, v) in m {
        obj.insert(k, Value::String(v));
    }
}

/// Build a `field` object given a `name`, its spec, and a setter prefix (header/payload).
#[inline]
pub fn build_field_object(name: &str, spec: &Value, setter_prefix: &str) -> Value {
    use serde_json::json;

    let json_ty = spec.get("type").and_then(Value::as_str).unwrap_or("string");
    let rust_ty = json_type_to_rust(json_ty);

    // base map
    let mut field = super::helpers::make_field_map(name, json_ty, rust_ty);

    // add expansions derived from name (snake/camel/etc.)
    merge_expansions(&mut field, "name", name);

    // setter method name
    let snake = field
        .get("name_snake_case")
        .and_then(Value::as_str)
        .unwrap_or(name);
    field.insert("setter_method".into(), Value::String(format!("{}_{}", setter_prefix, snake)));

    // param + insert-kind derived from json type
    let (param_ty, insert_kind) = match json_ty {
        "string"  => ("impl Into<String>", "string"),
        "boolean" => ("bool",              "boolean"),
        "integer" => ("i64",               "integer"),
        "number"  => ("f64",               "number"),
        _         => ("serde_json::Value", "raw"),
    };
    field.insert("setter_param_type".into(), Value::String(param_ty.into()));
    field.insert("insert_value_kind".into(), Value::String(insert_kind.into()));

    // boolean convenience flags for mustache branching
    field.insert("is_string".into(),  Value::Bool(json_ty == "string"));
    field.insert("is_boolean".into(), Value::Bool(json_ty == "boolean"));
    field.insert("is_integer".into(), Value::Bool(json_ty == "integer"));
    field.insert("is_number".into(),  Value::Bool(json_ty == "number"));

    Value::Object(field)
}

#[inline]
pub fn build_fields_vec(fields: &Map<String, Value>, setter_prefix: &str) -> Vec<Value> {
    fields
        .iter()
        .map(|(name, spec)| build_field_object(name, spec, setter_prefix))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn mk_spec(t: &str) -> Value {
        json!({ "type": t })
    }

    #[test]
    fn build_field_object_integer_payload() {
        let spec   = mk_spec("integer");
        let fieldV = build_field_object("user_id", &spec, "payload");

        // must be an object
        let field = fieldV.as_object().expect("field should be an object");

        // base keys populated
        assert_eq!(field.get("name").and_then(Value::as_str), Some("user_id"));
        assert_eq!(field.get("json_type").and_then(Value::as_str), Some("integer"));
        assert_eq!(field.get("rust_type").and_then(Value::as_str), Some("i64"));

        // expansions present (from ValuesTemplateExpansion)
        assert!(field.get("name_snake_case").and_then(Value::as_str).is_some());
        assert!(field.get("name_PascalCase").and_then(Value::as_str).is_some());
        assert!(field.get("name_camelCase").and_then(Value::as_str).is_some());

        // setter method incorporates prefix + snake_case
        let setter = field.get("setter_method").and_then(Value::as_str).unwrap();
        assert!(setter.starts_with("payload_"));

        // param + insert kind
        assert_eq!(field.get("setter_param_type").and_then(Value::as_str), Some("i64"));
        assert_eq!(field.get("insert_value_kind").and_then(Value::as_str), Some("integer"));

        // boolean flags match type
        assert_eq!(field.get("is_integer").and_then(Value::as_bool), Some(true));
        assert_eq!(field.get("is_string").and_then(Value::as_bool),  Some(false));
        assert_eq!(field.get("is_boolean").and_then(Value::as_bool), Some(false));
        assert_eq!(field.get("is_number").and_then(Value::as_bool),  Some(false));
    }

    #[test]
    fn build_field_object_string_header() {
        let spec   = mk_spec("string");
        let fieldV = build_field_object("version", &spec, "header");
        let field  = fieldV.as_object().unwrap();

        assert_eq!(field.get("name").and_then(Value::as_str), Some("version"));
        assert_eq!(field.get("json_type").and_then(Value::as_str), Some("string"));
        assert_eq!(field.get("rust_type").and_then(Value::as_str), Some("String"));

        // setter for header prefix
        let setter = field.get("setter_method").and_then(Value::as_str).unwrap();
        assert!(setter.starts_with("header_"));

        // param kind for string
        assert_eq!(field.get("setter_param_type").and_then(Value::as_str), Some("impl Into<String>"));
        assert_eq!(field.get("insert_value_kind").and_then(Value::as_str), Some("string"));

        // flags
        assert_eq!(field.get("is_string").and_then(Value::as_bool),  Some(true));
        assert_eq!(field.get("is_boolean").and_then(Value::as_bool), Some(false));
        assert_eq!(field.get("is_integer").and_then(Value::as_bool), Some(false));
        assert_eq!(field.get("is_number").and_then(Value::as_bool),  Some(false));
    }

    #[test]
    fn build_fields_vec_multiple() {
        // preserve insertion order of serde_json::Map
        let mut fields = Map::new();
        fields.insert("amount".into(),  mk_spec("number"));
        fields.insert("ok".into(),      mk_spec("boolean"));

        let vec = build_fields_vec(&fields, "payload");
        assert_eq!(vec.len(), 2);

        // 0 -> amount
        let f0 = vec[0].as_object().unwrap();
        assert_eq!(f0.get("name").and_then(Value::as_str), Some("amount"));
        assert_eq!(f0.get("json_type").and_then(Value::as_str), Some("number"));
        assert_eq!(f0.get("rust_type").and_then(Value::as_str), Some("f64"));
        assert_eq!(f0.get("insert_value_kind").and_then(Value::as_str), Some("number"));
        assert_eq!(f0.get("is_number").and_then(Value::as_bool), Some(true));

        // 1 -> ok
        let f1 = vec[1].as_object().unwrap();
        assert_eq!(f1.get("name").and_then(Value::as_str), Some("ok"));
        assert_eq!(f1.get("json_type").and_then(Value::as_str), Some("boolean"));
        assert_eq!(f1.get("rust_type").and_then(Value::as_str), Some("bool"));
        assert_eq!(f1.get("insert_value_kind").and_then(Value::as_str), Some("boolean"));
        assert_eq!(f1.get("is_boolean").and_then(Value::as_bool), Some(true));

        // both should have a setter method beginning with "payload_"
        assert!(f0.get("setter_method").and_then(Value::as_str).unwrap().starts_with("payload_"));
        assert!(f1.get("setter_method").and_then(Value::as_str).unwrap().starts_with("payload_"));
    }

    #[test]
    fn default_type_when_missing() {
        // if "type" is missing, default should behave as "string"
        let spec = json!({});
        let fieldV = build_field_object("note", &spec, "payload");
        let field  = fieldV.as_object().unwrap();

        assert_eq!(field.get("json_type").and_then(Value::as_str), Some("string"));
        assert_eq!(field.get("rust_type").and_then(Value::as_str), Some("String"));
        assert_eq!(field.get("is_string").and_then(Value::as_bool), Some(true));
        assert_eq!(field.get("insert_value_kind").and_then(Value::as_str), Some("string"));
    }
}