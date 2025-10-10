//! Helpers registry (stub). Define a registration API; implement later.

use std::collections::HashMap;

/// A helper is a callable that may inspect key/values and return a string.
/// Signature is a placeholder; refine once you design helper invocation.
pub type HelperFn = fn(args: &str) -> String;

#[derive(Default, Debug, Clone)]
pub struct Helpers {
    map: HashMap<String, HelperFn>,
}

impl Helpers {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn register<S: Into<String>>(&mut self, name: S, f: HelperFn) {
        self.map.insert(name.into(), f);
    }
    pub fn get(&self, name: &str) -> Option<&HelperFn> { self.map.get(name) }
}