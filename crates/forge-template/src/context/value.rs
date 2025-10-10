//! Value model + serde bridge for template contexts.

use std::collections::BTreeMap;
use serde::Serialize;

/// Minimal JSON-like value type we control (stable, no serde_json leakage).
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

/// A common top-level map for contexts (e.g., {"name": "leo"}).
pub type ContextMap = BTreeMap<String, Value>;

impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Self {
        use serde_json::Value as J;
        match v {
            J::Null => Value::Null,
            J::Bool(b) => Value::Bool(b),
            J::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
            J::String(s) => Value::String(s),
            J::Array(a) => Value::Array(a.into_iter().map(Value::from).collect()),
            J::Object(o) => {
                let mut map = BTreeMap::new();
                for (k, vv) in o {
                    map.insert(k, Value::from(vv));
                }
                Value::Object(map)
            }
        }
    }
}

/// Convert any `Serialize` data into our `Value`.
pub fn from_serde<T: Serialize>(t: &T) -> Result<Value, crate::errors::RenderError> {
    serde_json::to_value(t)
        .map(Value::from)
        .map_err(|e| crate::errors::RenderError::Serde(e.to_string()))
}

/// Convenience: convert a serializable map/struct into a top-level `ContextMap`.
/// If the serialized root is not an object, it will be wrapped as {"value": <root>}.
pub fn object_from_serde<T: Serialize>(
    t: &T,
) -> Result<ContextMap, crate::errors::RenderError> {
    let v = from_serde(t)?;
    match v {
        Value::Object(m) => Ok(m),
        other => {
            let mut m = BTreeMap::new();
            m.insert("value".to_string(), other);
            Ok(m)
        }
    }
}