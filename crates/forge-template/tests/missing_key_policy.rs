use forge_template::context::Context;
use forge_template::engine::render_with;
use forge_template::options::{MissingKeyPolicy, RenderOptions};
use forge_template::source::{inline::InlineSource, TemplateSource};

#[test]
fn missing_key_empty_policy_renders_empty() {
    let ctx = Context::new(); // no keys
    let mut opts = RenderOptions::default();
    opts.on_missing = MissingKeyPolicy::Empty;
    let src = TemplateSource::from(InlineSource::new("Hi {{who}}!"));

    let out = render_with(src, &ctx, &opts).unwrap();
    assert_eq!(out, "Hi !");
}

#[test]
fn missing_key_error_policy_raises_error() {
    let ctx = Context::new(); // no keys
    let mut opts = RenderOptions::default();
    opts.on_missing = MissingKeyPolicy::Error;
    let src = TemplateSource::from(InlineSource::new("Hi {{who}}!"));

    let err = render_with(src, &ctx, &opts).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("missing key"), "was: {msg}");
}

#[test]
fn missing_key_keep_tag_policy_leaves_tag() {
    let ctx = Context::new(); // no keys
    let mut opts = RenderOptions::default();
    opts.on_missing = MissingKeyPolicy::KeepTag;
    let src = TemplateSource::from(InlineSource::new("Hi {{who}}! {{{who}}}"));

    let out = render_with(src, &ctx, &opts).unwrap();
    assert_eq!(out, "Hi {{who}}! {{{who}}}");
}
