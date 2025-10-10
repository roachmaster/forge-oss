use proc_macro::TokenStream;

mod args;
mod env;
mod templates;

/// Render code from YAML + Mustache, given two string args:
///   render_yaml!("<template.rel.or.abs>", "<yaml.rel.or.abs>")
///
/// - If an arg is absolute, we use it as-is.
/// - If an arg is relative, we prefix it with FORGE_ROOT.
#[proc_macro]
pub fn render_yaml(input: TokenStream) -> TokenStream {
    templates::expand_render_yaml(input).unwrap_or_else(|e| {
        format!("compile_error!(\"{e}\");").parse().unwrap()
    })
}
