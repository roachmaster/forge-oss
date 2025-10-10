use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

use forge_template::context::{value::Value, Context};
use forge_template::engine::render_with;
use forge_template::options::{Partials, RenderOptions};
use forge_template::source::{file::FileSource, TemplateSource};

#[test]
fn partials_from_map_take_precedence_over_dir() {
    // temp dir with a Dir partial
    let td = tempdir().unwrap();
    let dir = td.path();
    fs::write(dir.join("_greeting.mustache"), "Hello Dir").unwrap();
    fs::write(dir.join("main.mustache"), "{{> _greeting}}, {{who}}").unwrap();

    // Map override with same partial name
    let mut map = HashMap::new();
    map.insert("_greeting".to_string(), "Hello Map".to_string());

    // Options: MapThenDir so map wins
    let mut opts = RenderOptions::default();
    opts.partials = Partials::MapThenDir { map, dir: dir.into() };

    // Context
    let mut ctx = Context::new();
    ctx.insert("who", Value::String("Leo".into()));

    let src = TemplateSource::from(FileSource::new(dir.join("main.mustache")));
    let out = render_with(src, &ctx, &opts).unwrap();
    assert_eq!(out, "Hello Map, Leo");
}