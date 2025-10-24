use proc_macro::TokenStream;
use crate::args::two_string_args;
use crate::env::forge_root;

/// Macro expander for `render_yaml!(<template>, <yaml>)`.
/// Resolves absolute paths via FORGE_ROOT and uses `forge_template::codegen::render_yaml_from_abs`.
pub(crate) fn expand_render_yaml(input: TokenStream) -> Result<TokenStream, String> {
    let (template_in, yaml_in) = two_string_args(input)?;

    let root = forge_root()?;
    let template_abs = absolutize(&root, &template_in);
    let yaml_abs = absolutize(&root, &yaml_in);

    eprintln!("[forge-proc] render_yaml:");
    eprintln!("  template_abs = {}", template_abs);
    eprintln!("  yaml_abs     = {}", yaml_abs);

    // Call into forge-template directly
    match forge_template::codegen::render_yaml_from_abs(&yaml_abs, &template_abs) {
        Ok(rendered) => {
            // Return the rendered code as a TokenStream
            rendered.parse::<TokenStream>().map_err(|e| {
                format!("forge-proc: failed to parse rendered template into tokens: {e}")
            })
        }
        Err(e) => Err(format!("forge-proc: render_yaml failed: {e:?}")),
    }
}

#[inline]
fn absolutize(root: &str, path_in: &str) -> String {
    use std::path::Path;
    let p = Path::new(path_in);
    if p.is_absolute() {
        path_in.to_string()
    } else {
        let rel = path_in.trim_start_matches('/');
        format!("{}/{}", root.trim_end_matches('/'), rel)
    }
}
