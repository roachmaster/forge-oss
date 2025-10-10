use forge_template::context::{value::Value, Context};
use forge_template::engine::{render_with, Engine};
use forge_template::options::RenderOptions;
use forge_template::source::{inline::InlineSource, TemplateSource};

#[test]
fn renders_inline_template_with_basic_context() {
    // context: { "name": "Leo", "x": 2, "ok": true }
    let mut ctx = Context::new();
    ctx.insert("name", Value::String("Leo".into()));
    ctx.insert("x", Value::Number(2.0));
    ctx.insert("ok", Value::Bool(true));

    let src = TemplateSource::from(InlineSource::new("Hello {{name}}! x={{x}} ok={{ok}}"));

    // Use stateless function
    let out = render_with(src.clone(), &ctx, &RenderOptions::default()).unwrap();
    assert_eq!(out, "Hello Leo! x=2 ok=true");

    // And via Engine instance
    let eng = Engine::new(RenderOptions::default());
    let out2 = eng.render(src, &ctx).unwrap();
    assert_eq!(out2, "Hello Leo! x=2 ok=true");
}