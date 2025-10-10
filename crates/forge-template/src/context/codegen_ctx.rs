use crate::context::values_template_expansion::ValuesTemplateExpansion;
use crate::context::helpers::{
    as_object_map, enumerate_with_has_next, header_file_or_default, header_template_or_default,
};
// add these imports back at the top of codegen_ctx.rs
use crate::context::field_codegen::build_fields_vec;
use crate::context::helpers::{
    get_message_name_or_default, section_fields_map, section_required_array,
};
use serde_json::{json, Value};

// keep your existing merge_expansions(...) helper

/// Build the final, flattened AST-like JSON for codegen from a *conditioned schema* (JSON).
/// This is used by render_schema_template and remains header/payload agnostic.
pub fn build_codegen_context(conditioned: &Value) -> Value {
    // ---- pull sections via helpers ----
    let msg_name        = get_message_name_or_default(conditioned, "Message");
    let header_fields   = section_fields_map(conditioned, "header");
    let header_required = section_required_array(conditioned, "header");
    let payload_fields  = section_fields_map(conditioned, "payload");
    let payload_required= section_required_array(conditioned, "payload");

    // ---- top object ----
    let mut top = serde_json::Map::new();
    top.insert("message_name".into(), Value::String(msg_name.to_string()));
    merge_expansions(&mut top, "message_name", msg_name);

    // ---- header.fields[] & payload.fields[] ----
    let header_vec  = build_fields_vec(&header_fields, "header");
    let payload_vec = build_fields_vec(&payload_fields, "payload");

    // ---- assemble ----
    top.insert("header".into(),  json!({ "required": header_required,  "fields": header_vec }));
    top.insert("payload".into(), json!({ "required": payload_required, "fields": payload_vec }));

    Value::Object(top)
}
// ---------- small header helpers ----------
#[inline]
fn header_type(conditioned: &Value) -> Option<&str> {
    conditioned.get("header")
        .and_then(|h| h.get("type"))
        .and_then(|v| v.as_str())
}

#[inline]
fn header_type_or_default(conditioned: &Value, default: &str) -> String {
    header_type(conditioned).unwrap_or(default).to_string()
}

// ---------- expansions ----------
fn merge_expansions(obj: &mut serde_json::Map<String, Value>, key: &str, value: &str) {
    use std::collections::BTreeMap;
    let m: BTreeMap<String, String> = ValuesTemplateExpansion::expands_to_map(key, value);
    for (k, v) in m { obj.insert(k, Value::String(v)); }
}

// ---------- Adapter trait: header-driven, payload-agnostic core ----------
trait PayloadAdapter {
    /// Normalize the **payload** section into a codegen-friendly shape.
    fn normalize(&self, payload: &Value) -> Value;
    /// A short discriminator for debugging/context
    fn kind(&self) -> &'static str;
}

// ---------------- Lambdas adapter (legacy) ----------------
struct LambdasAdapter;

impl LambdasAdapter {
    fn normalize_primitive_lambdas(&self, payload: &Value) -> Vec<Value> {
        let arr = payload.get("primitive_lambdas").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let mut out = Vec::with_capacity(arr.len());
        for (_i, item, has_next) in enumerate_with_has_next(&arr) {
            let mut obj = as_object_map(item);
            // has_next flag
            obj.insert("has_next".into(), Value::Bool(has_next));
            // name + expansions
            if let Some(Value::String(name)) = obj.remove("name") {
                merge_expansions(&mut obj, "name", &name);
                obj.insert("name".into(), Value::String(name));
            }
            // add value_hex to bitwave.value if present
            if let Some(bitwave_obj) = obj.get_mut("bitwave").and_then(|v| v.as_object_mut()) {
                if let Some(values) = bitwave_obj.get("value").and_then(|v| v.as_array()) {
                    let value_hex: Vec<Value> = values
                        .iter()
                        .map(|elem| {
                            if let Some(n) = elem.as_u64() {
                                // 0xFFFF_FFFF_FFFF_FFFF formatting
                                let s = format!("{n:016X}");
                                let mut out = String::with_capacity(2 + 16 + 3);
                                out.push_str("0x");
                                for (i, ch) in s.chars().enumerate() {
                                    if i > 0 && i % 4 == 0 { out.push('_'); }
                                    out.push(ch);
                                }
                                Value::String(out)
                            } else if let Some(s) = elem.as_str() {
                                Value::String(s.to_string())
                            } else {
                                elem.clone()
                            }
                        })
                        .collect();
                    bitwave_obj.insert("value_hex".to_string(), Value::Array(value_hex));
                }
            }
            out.push(Value::Object(obj));
        }
        out
    }
}
impl PayloadAdapter for LambdasAdapter {
    fn normalize(&self, payload: &Value) -> Value {
        let prims = self.normalize_primitive_lambdas(payload);
        json!({
            "primitive_lambdas_length": prims.len(),
            "primitive_lambdas": prims
        })
    }
    fn kind(&self) -> &'static str { "lambdas" }
}

// ---------------- DTO/Structs adapter ----------------
struct DtoAdapter;

impl DtoAdapter {
    fn build_structs_vec(&self, payload: &Value) -> Vec<Value> {
        let structs = payload.get("structs").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let mut out = Vec::with_capacity(structs.len());

        for (i, st) in structs.iter().enumerate() {
            let mut sobj = as_object_map(st);
            let name = sobj.get("name").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            if name.is_empty() { continue; }

            let mut sctx = serde_json::Map::new();
            sctx.insert("name".into(), Value::String(name.clone()));
            merge_expansions(&mut sctx, "name", &name);

            // fields map -> deterministic vec
            let fields_obj = sobj.get("fields").and_then(|v| v.as_object()).cloned().unwrap_or_default();
            let mut kvs: Vec<(String,String)> = fields_obj.into_iter()
                .filter_map(|(k,v)| {
                    let fname = k.trim().to_string();
                    if fname.is_empty() { return None; }
                    let ftyp = match v {
                        Value::String(s) => s,
                        other => other.to_string(),
                    };
                    Some((fname, ftyp))
                })
                .collect();
            kvs.sort_by(|a,b| a.0.cmp(&b.0));

            let mut fvec = Vec::with_capacity(kvs.len());
            for (idx, (fname, ftyp)) in kvs.into_iter().enumerate() {
                let mut fobj = serde_json::Map::new();
                fobj.insert("name".into(), Value::String(fname.clone()));
                merge_expansions(&mut fobj, "name", &fname);
                fobj.insert("type".into(), Value::String(ftyp.clone()));
                merge_expansions(&mut fobj, "type", &ftyp);
                fobj.insert("index".into(), Value::Number(idx.into()));
                // is_last set after we know length
                fvec.push(Value::Object(fobj));
            }
            let flen = fvec.len();
            for (idx, f) in fvec.iter_mut().enumerate() {
                if let Some(m) = f.as_object_mut() {
                    m.insert("is_last".into(), Value::Bool(idx + 1 == flen));
                }
            }

            sctx.insert("fields".into(), Value::Array(fvec));
            sctx.insert("index".into(), Value::Number(i.into()));
            sctx.insert("has_next".into(), Value::Bool(i + 1 != structs.len()));

            out.push(Value::Object(sctx));
        }

        out
    }
}
impl PayloadAdapter for DtoAdapter {
    fn normalize(&self, payload: &Value) -> Value {
        let structs_vec = self.build_structs_vec(payload);
        json!({
            "structs_length": structs_vec.len(),
            "structs": structs_vec
        })
    }
    fn kind(&self) -> &'static str { "dto" }
}

// ---------------- Adapter selection (header-driven) ----------------
fn select_adapter(conditioned: &Value) -> Box<dyn PayloadAdapter> {
    match header_type(conditioned) {
        Some("dto") => Box::new(DtoAdapter),
        // If you later add `type: lambdas`, route explicitly; else fallback by template/name.
        Some("lambdas") => Box::new(LambdasAdapter),
        _ => {
            // Heuristic fallback: prefer DTO when payload.structs exists; else lambdas
            let has_structs = conditioned.get("payload")
                .and_then(|p| p.get("structs"))
                .and_then(|v| v.as_array())
                .map(|a| !a.is_empty())
                .unwrap_or(false);
            if has_structs { Box::new(DtoAdapter) } else { Box::new(LambdasAdapter) }
        }
    }
}

// ---------------- YAML codegen context (now header/payload only) ----------------
pub fn build_yaml_codegen_context(conditioned: &Value) -> Value {
    let file     = header_file_or_default(conditioned, "unknown");
    let template = header_template_or_default(conditioned, "default");
    let ytype    = header_type_or_default(conditioned, ""); // standardized metadata

    let payload = conditioned.get("payload").cloned().unwrap_or_else(|| json!({}));
    let adapter = select_adapter(conditioned);
    let norm    = adapter.normalize(&payload);

    json!({
        "header": { "file": file, "template": template, "type": ytype, "payload_kind": adapter.kind() },
        "payload": norm
    })
}