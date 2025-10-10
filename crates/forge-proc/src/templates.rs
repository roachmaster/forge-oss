use proc_macro::TokenStream;

use crate::args::two_string_args;
use crate::env::forge_root;

/// Macro expander for `render_yaml!(<template>, <yaml>)`.
/// - Accepts two string literals (template path, yaml path), each absolute or relative.
/// - If relative, they are joined to FORGE_ROOT.
/// - Then (normally) we would call into the codegen renderer.
///   For now, that call is commented out per instructions.
pub(crate) fn expand_render_yaml(input: TokenStream) -> Result<TokenStream, String> {
    let (template_in, yaml_in) = two_string_args(input)?;

    let root = forge_root()?;
    let template_abs = absolutize(&root, &template_in);
    let yaml_abs     = absolutize(&root, &yaml_in);

    // For visibility during macro expansion debugging:
    eprintln!("[forge-proc] template_abs={template_abs}");
    eprintln!("[forge-proc] yaml_abs={yaml_abs}");

    // --- future integration ---
    // use forgery_codegen::render::render_from_yaml_and_template;
    // let rendered = render_from_yaml_and_template(&yaml_abs, &template_abs)
    //     .map_err(|e| format!("render failed: {e}"))?;
    // return rendered.parse()
    //     .map_err(|e| format!("rendered output did not parse as Rust: {e}"));

    // Temporary stub: return an empty token stream so callers still compile.
    Ok("".parse().unwrap())
}

#[inline]
fn absolutize(root: &str, path_in: &str) -> String {
    use std::path::Path;
    let p = Path::new(path_in);
    if p.is_absolute() {
        path_in.to_string()
    } else {
        // normalize: avoid double slashes if caller passed a leading '/'
        let rel = path_in.trim_start_matches('/');
        format!("{}/{}", root.trim_end_matches('/'), rel)
    }
}
