//! Filters registry (stub). Define a registration API; implement later.

use std::collections::HashMap;

/// A filter takes an input string and returns a transformed string.
pub type FilterFn = fn(&str) -> String;

#[derive(Default, Debug, Clone)]
pub struct Filters {
    map: HashMap<String, FilterFn>,
}

impl Filters {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn register<S: Into<String>>(&mut self, name: S, f: FilterFn) {
        self.map.insert(name.into(), f);
    }
    pub fn get(&self, name: &str) -> Option<&FilterFn> { self.map.get(name) }
}