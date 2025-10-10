use proc_macro::{TokenStream, TokenTree};

/// Parse exactly two string literal arguments separated by a comma:
///   render_yaml!("/abs/template.mustache", "/abs/values.yaml")
/// or relative paths:
///   render_yaml!("resources/templates/thing.mustache", "resources/values/thing.yaml")
pub(crate) fn two_string_args(input: TokenStream) -> Result<(String, String), String> {
    let mut literals: Vec<String> = Vec::new();

    for tt in input.into_iter() {
        match tt {
            TokenTree::Literal(l) => literals.push(l.to_string()),
            TokenTree::Punct(p) if p.as_char() == ',' => {},
            other => {
                return Err(format!(
                    "expected two string literal arguments separated by a comma, got `{other}`"
                ))
            }
        }
    }

    match literals.len() {
        2 => Ok((
            validate_and_extract(&literals[0])?,
            validate_and_extract(&literals[1])?,
        )),
        0 => Err("missing arguments; usage: render_yaml!(\"<template>\", \"<yaml>\")".into()),
        1 => Err("only one argument found; expected two string literals".into()),
        _ => Err("too many arguments; expected exactly two string literals".into()),
    }
}

// ------------------ small helpers ------------------

#[inline]
fn validate_and_extract(lit_raw: &str) -> Result<String, String> {
    ensure_is_quoted_string(lit_raw)?;
    let val = strip_quotes_and_trim(lit_raw);
    ensure_not_empty(&val)?;
    Ok(val)
}

fn ensure_is_quoted_string(lit_text: &str) -> Result<(), String> {
    if lit_text.starts_with('"') && lit_text.ends_with('"') {
        Ok(())
    } else {
        Err("argument must be a quoted string literal".into())
    }
}

fn strip_quotes_and_trim(lit_text: &str) -> String {
    lit_text.trim_matches('"').trim().to_string()
}

fn ensure_not_empty(s: &str) -> Result<(), String> {
    if s.is_empty() { Err("argument cannot be empty".into()) } else { Ok(()) }
}
