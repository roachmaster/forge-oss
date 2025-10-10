use forge_template::cache;
use forge_template::context::{value::Value, Context};
use forge_template::engine::render_with;
use forge_template::options::RenderOptions;
use forge_template::source::{inline::InlineSource, TemplateSource};

#[test]
fn same_inputs_hit_parse_cache() {
    cache::reset();

    let mut ctx = Context::new();
    ctx.insert("x", Value::Number(42.0));

    let src = TemplateSource::from(InlineSource::new("Value {{x}}"));
    let opts = RenderOptions::default();

    // First render → miss
    let out1 = render_with(src.clone(), &ctx, &opts).unwrap();
    assert_eq!(out1, "Value 42");

    // Second render → hit
    let out2 = render_with(src, &ctx, &opts).unwrap();
    assert_eq!(out2, "Value 42");

    assert_eq!(cache::misses(), 1, "first should miss");
    assert_eq!(cache::hits(), 1, "second should hit");
}