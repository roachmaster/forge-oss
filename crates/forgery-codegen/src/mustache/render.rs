use mustache::{self, Data, Template};
use std::fs;

/// Compile a mustache template from an absolute path.
pub fn compile_template(path: &str) -> Result<Template, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read template {path}: {e}"))?;
    mustache::compile_str(&content)
        .map_err(|e| format!("failed to compile template {path}: {e}"))
}

/// Render a compiled template using a serde_json::Value context.
pub fn render_template_with_json(tpl: &Template, ctx: &serde_json::Value) -> Result<String, String> {
    let data = to_data(ctx);
    let mut buf = Vec::new();
    tpl.render_data(&mut buf, &data)
        .map_err(|e| format!("failed to render mustache template: {e}"))?;
    String::from_utf8(buf).map_err(|e| format!("render produced invalid UTF-8: {e}"))
}

/// Minimal serde_json::Value -> mustache::Data bridge (mustache 0.9 has no native JSON).
fn to_data(v: &serde_json::Value) -> Data {
    match v {
        serde_json::Value::Null => Data::String(String::new()),
        serde_json::Value::Bool(b) => Data::Bool(*b),
        // mustache::Data has no numeric variant; render numbers as strings
        serde_json::Value::Number(n) => Data::String(n.to_string()),
        serde_json::Value::String(s) => Data::String(s.clone()),
        serde_json::Value::Array(a) => {
            let items = a.iter().map(to_data).collect::<Vec<Data>>();
            Data::Vec(items)
        }
        serde_json::Value::Object(m) => {
            let mut map = std::collections::HashMap::with_capacity(m.len());
            for (k, vv) in m {
                map.insert(k.clone(), to_data(vv));
            }
            Data::Map(map)
        }
    }
}
