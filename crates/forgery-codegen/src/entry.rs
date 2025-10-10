//! Public entrypoint for rendering from YAML + Mustache.

use crate::io::yaml::load_yaml_as_json;
use crate::mustache::{compile_template, render_template_with_json};

/// Public entrypoint: read YAML + template from absolute paths and render.
pub fn render_from_yaml_and_template(
    yaml_abs: &str,
    template_abs: &str,
) -> Result<String, String> {
    let ctx_json = load_yaml_as_json(yaml_abs)?;
    let tpl = compile_template(template_abs)?;
    render_template_with_json(&tpl, &ctx_json)
}
