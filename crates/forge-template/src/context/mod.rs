//! Context wrapper + serde bridge; re-exports `Value`.

use std::collections::BTreeMap;

pub mod value;
pub mod condition;
mod values_template_expansion;
pub mod codegen_ctx;
pub(crate) mod helpers;
pub mod field_codegen;

pub use value::Value;

/// Wrapper around a string-keyed map used for rendering contexts.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Context(pub BTreeMap<String, Value>);

impl Context {
    /// Create an empty context.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Insert a key/value into the context.
    pub fn insert<K: Into<String>>(&mut self, key: K, val: Value) {
        self.0.insert(key.into(), val);
    }

    /// Borrow the inner map.
    pub fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.0
    }

    /// Convert any `serde::Serialize` value into a `Context`.
    /// If the root is not an object, it is wrapped as `{"value": <root>}`.
    pub fn from_serde<T: serde::Serialize>(
        t: &T,
    ) -> Result<Self, crate::errors::RenderError> {
        let v = value::from_serde(t)?;
        Ok(match v {
            Value::Object(map) => Context(map),
            other => {
                let mut m = BTreeMap::new();
                m.insert("value".to_string(), other);
                Context(m)
            }
        })
    }
}
