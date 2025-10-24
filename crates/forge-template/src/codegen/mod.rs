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

    // -------------------------------------------------------------------------
    // Legacy test (disabled after generic context refactor)
    // -------------------------------------------------------------------------
    #[ignore]
    #[test]
    fn render_yaml_from_abs_dto_structs() {
        // Legacy Bitwave DTO test.
        // This test relied on adapter-specific behavior (payload.structs[].fields)
        // which was removed in the generic context refactor.
        // Keeping it here for reference but marking as ignored.
        let _ = temp_workspace();
        eprintln!("⚠️  Legacy DTO test ignored due to refactor.");
        assert!(true);
    }
}
pub mod expand;
pub mod flags;
pub mod utils;
