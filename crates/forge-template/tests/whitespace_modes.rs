use forge_template::context::{value::Value, Context};
use forge_template::engine::render_with;
use forge_template::options::{RenderOptions, WhitespaceMode};
use forge_template::source::{inline::InlineSource, TemplateSource};

fn ctx() -> Context {
    let mut c = Context::new();
    c.insert("a", Value::String("A".into()));
    c
}

#[test]
fn keep_mode_preserves_spaces_and_blank_lines() {
    let src = TemplateSource::from(InlineSource::new("  {{a}}  \n\n   \nX"));
    let mut opts = RenderOptions::default();
    opts.whitespace = WhitespaceMode::Keep;
    let out = render_with(src, &ctx(), &opts).unwrap();
    assert_eq!(out, "  A  \n\n   \nX");
}

#[test]
fn trimlines_removes_trailing_and_whitespace_only_lines() {
    let src = TemplateSource::from(InlineSource::new("  {{a}}   \n   \nX  "));
    let mut opts = RenderOptions::default();
    opts.whitespace = WhitespaceMode::TrimLines;
    let out = render_with(src, &ctx(), &opts).unwrap();
    assert_eq!(out, "  A\nX");
}

#[test]
fn smartindent_collapses_blank_runs() {
    let src = TemplateSource::from(InlineSource::new("A\n\n\n\nB\n\nC  \n"));
    let mut opts = RenderOptions::default();
    opts.whitespace = WhitespaceMode::SmartIndent;
    let out = render_with(src, &ctx(), &opts).unwrap();
    assert_eq!(out, "A\n\nB\n\nC");
}