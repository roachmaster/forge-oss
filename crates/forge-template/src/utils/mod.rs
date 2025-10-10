// src/context/serde_to_mustache.rs
use mustache::Data;
use std::collections::HashMap;

/// Convert any `serde_json::Value` into `mustache::Data`.
/// - Numbers are rendered as strings (mustache 0.9 doesn't have a numeric variant)
/// - Objects become `Data::Map(HashMap<..>)`
/// - Arrays become `Data::Vec(Vec<Data>)`
pub fn to_data(v: &serde_json::Value) -> Data {
    match v {
        serde_json::Value::Null => Data::String(String::new()),
        serde_json::Value::Bool(b) => Data::Bool(*b),
        serde_json::Value::Number(n) => Data::String(n.to_string()),
        serde_json::Value::String(s) => Data::String(s.clone()),
        serde_json::Value::Array(arr) => {
            let items: Vec<Data> = arr.iter().map(to_data).collect();
            Data::Vec(items)
        }
        serde_json::Value::Object(map) => {
            let mut m: HashMap<String, Data> = HashMap::with_capacity(map.len());
            for (k, v) in map {
                m.insert(k.clone(), to_data(v));
            }
            Data::Map(m)
        }
    }
}