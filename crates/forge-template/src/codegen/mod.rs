mod macros;
mod render;

use std::path::Path;

use crate::codegen::render::{compile_template, render_with_ctx};
use crate::context::{codegen_ctx, condition};
use crate::read_and_parse;
use anyhow::Result;
use serde_json::Value;

// ============================================================================
// Public API (absolute paths only)
// ============================================================================

/// Render from absolute YAML + absolute Mustache template paths.
/// Pipeline: YAML → condition → build codegen ctx → compile template → render.
pub fn render_yaml_from_abs<P1: AsRef<Path>, P2: AsRef<Path>>(
    yaml_abs: P1,
    template_abs: P2,
) -> Result<String> {
    // 1) parse YAML
    let raw_yaml: Value = read_and_parse!(yaml, yaml_abs.as_ref())?;

    // 2) condition to normalized shape (header/payload)
    let conditioned = condition::condition_yaml(&raw_yaml);

    // 3) build codegen context via header-driven adapter
    let ctx = codegen_ctx::build_yaml_codegen_context(&conditioned);

    // 4) compile template and render
    let tpl = compile_template(template_abs.as_ref())?;
    let rendered = render_with_ctx(&ctx, &tpl)?;
    Ok(rendered)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// create a unique temp dir under the system temp directory
    fn temp_workspace() -> PathBuf {
        let mut dir = std::env::temp_dir();
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        dir.push(format!("forge_template_abs_test_{}", stamp));
        fs::create_dir_all(&dir).expect("make temp dir");
        dir
    }

    #[test]
    fn render_yaml_from_abs_dto_structs() {
        // workspace
        let root = temp_workspace();

        // --- write YAML (DTO) ---
        let yaml_path = root.join("harmonic_chamber.yaml");
        let yaml = r#"---
header:
  file: harmonic_chamber.yaml
  template: rust_structs_codegen.mustache
  type: dto
payload:
  structs:
    - name: Foo128
      fields:
        a_field: BitWave128
        b_field: u32
"#;
        fs::write(&yaml_path, yaml).unwrap();

        // --- write template ---
        // Very small mustache using the DTO adapter payload
        let tpl_path = root.join("rust_structs_codegen.mustache");
        let tpl = r#"
{{#payload.structs}}
pub struct {{name}} {
{{#fields}}    pub {{name}}: {{type}},{{^is_last}}
{{/is_last}}
{{/fields}}
}
{{/payload.structs}}
"#;
        fs::write(&tpl_path, tpl).unwrap();

        // --- render using absolute paths ---
        let out = render_yaml_from_abs(&yaml_path, &tpl_path)
            .expect("render YAML DTO");

        // --- assertions ---
        assert!(out.contains("pub struct Foo128"), "struct name missing");
        // order is sorted by field name in adapter; a_field then b_field
        assert!(out.contains("pub a_field: BitWave128"), "field a_field missing");
        assert!(out.contains("pub b_field: u32"), "field b_field missing");
    }
}
