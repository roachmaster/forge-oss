use std::fs;
use std::path::Path;
use anyhow::Context;
use serde_json::Value;

#[inline]
pub fn compile_template(path: &Path) -> anyhow::Result<mustache::Template> {
    let tpl_txt =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(mustache::compile_str(&tpl_txt).context("compile mustache template")?)
}

#[inline]
pub fn render_with_ctx(ctx: &Value, tpl: &mustache::Template) -> anyhow::Result<String> {
    let data = crate::utils::to_data(ctx);
    let mut out = Vec::new();
    tpl.render_data(&mut out, &data)
        .context("render mustache template")?;
    Ok(String::from_utf8(out).expect("rendered must be valid UTF-8"))
}