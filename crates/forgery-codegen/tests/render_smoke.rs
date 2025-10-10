use forgery_codegen::render_from_yaml_and_template;

#[test]
fn smoke_render() {
    let yaml = "/Users/leonardorocha/forge-oss/resources/templates/design_tokens.yaml";
    let tpl  = "/Users/leonardorocha/forge-oss/resources/templates/css_from_tokens.mustache";

    let out = render_from_yaml_and_template(yaml, tpl).unwrap();
    println!("render output:\n{}", out);
}
