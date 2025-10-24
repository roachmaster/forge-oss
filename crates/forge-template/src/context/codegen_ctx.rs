// ============================================================================
// Forge Template — Codegen Context Orchestrator
// File: context/codegen_ctx.rs
// Role: Bridges conditioned YAML to the recursive expansion engine under
//       `codegen::expand`, providing the outer header/payload structure.
// ============================================================================

use serde_json::{json, Map, Value};
use crate::context::helpers::{header_file_or_default, header_template_or_default};
use crate::codegen::expand::expand_value;

// -----------------------------------------------------------------------------
// Public Context Builders
// -----------------------------------------------------------------------------

/// Domain-agnostic build: merges header + payload and expands naming variants recursively.
/// This function now delegates to `codegen::expand::expand_value` for all deep logic.
pub fn build_codegen_context(conditioned: &Value) -> Value {
    let header = conditioned.get("header").cloned().unwrap_or_else(|| json!({}));
    let payload = conditioned.get("payload").cloned().unwrap_or_else(|| json!({}));

    // Merge header + payload
    let mut merged: Map<String, Value> = header.as_object().cloned().unwrap_or_default();
    if let Some(pobj) = payload.as_object() {
        for (k, v) in pobj {
            merged.insert(k.clone(), v.clone());
        }
    }

    expand_value(&Value::Object(merged))
}

/// Standard entrypoint used by `render_yaml_from_abs`.
/// Builds `{ header, payload }` where payload is fully expanded through the new engine.
pub fn build_yaml_codegen_context(conditioned: &Value) -> Value {
    let file = header_file_or_default(conditioned, "unknown");
    let template = header_template_or_default(conditioned, "default");
    let ytype = conditioned
        .get("header")
        .and_then(|h| h.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let header = conditioned.get("header").cloned().unwrap_or_else(|| json!({}));
    let payload = conditioned.get("payload").cloned().unwrap_or_else(|| json!({}));

    // Merge
    let mut merged: Map<String, Value> = header.as_object().cloned().unwrap_or_default();
    if let Some(pobj) = payload.as_object() {
        for (k, v) in pobj {
            merged.insert(k.clone(), v.clone());
        }
    }

    let expanded = expand_value(&Value::Object(merged));

    json!({
        "header": {
            "file": file,
            "template": template,
            "type": ytype
        },
        "payload": expanded
    })
}
