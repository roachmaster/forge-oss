use crate::args::one_string_arg;
use crate::env::repo_root;
use crate::templates::{render_for_msg_type, render_for_yaml_payload};
use proc_macro::TokenStream;

// ---------- small inner expander (JSON schema → builder) ----------
pub(crate) fn expand_builder_schema_render(input: TokenStream) -> Result<TokenStream, String> {
    let design_pattern = "builder";
    let msg_type = one_string_arg(input)?;
    let repo_root = repo_root()?;
    render_for_msg_type(&repo_root, &msg_type, design_pattern)
}

// ---------- small inner expander (YAML payload → template decided by YAML) ----------
pub(crate) fn expand_yaml_schema_render(input: TokenStream) -> Result<TokenStream, String> {
    // The macro takes a single string arg: the YAML payload name
    // e.g. render_yaml!("lambdas")
    let payload_name = one_string_arg(input)?;
    let repo_root = repo_root()?;

    render_for_yaml_payload(&repo_root, &payload_name)
}