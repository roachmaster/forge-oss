//! Small JSON utilities (optional).

use serde_json::{Map, Value};

/// Merge two JSON objects shallowly.
pub fn merge_objects(mut a: Map<String, Value>, b: Map<String, Value>) -> Map<String, Value> {
    for (k, v) in b {
        a.insert(k, v);
    }
    a
}
