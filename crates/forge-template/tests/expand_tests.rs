// ============================================================================
// Forge Integration Test
// File: tests/expand_tests.rs
// Crate: forge-template
// Description: Verifies recursive expansion, type flags, and hasNext behavior.
// ============================================================================

use forge_template::context::codegen_ctx::build_yaml_codegen_context;
use serde_json::json;

#[test]
fn expand_generates_case_variants_and_flags() {
    let input = json!({
        "header": { "kind": "ui_component", "name": "button" },
        "payload": {
            "mods": ["model", "style"],
            "exports": [
                { "mod": "model", "name": "Button" },
                { "mod": "style", "name": "ButtonStyle" }
            ]
        }
    });

    let ctx = build_yaml_codegen_context(&input);
    let payload = ctx.get("payload").unwrap();

    // --- verify expansions for header.name
    assert_eq!(payload["name_snake_case"], "button");
    assert_eq!(payload["name_PascalCase"], "Button");

    // --- verify mods array expansions
    let mods = payload["mods"].as_array().expect("mods array");
    assert_eq!(mods.len(), 2);
    assert_eq!(mods[0]["value_snake_case"], "model");
    assert_eq!(mods[0]["hasNext"], true);
    assert_eq!(mods[1]["hasNext"], false);

    // --- verify exports array expansions
    let exports = payload["exports"].as_array().expect("exports array");
    assert_eq!(exports.len(), 2);
    assert_eq!(exports[0]["mod_snake_case"], "model");
    assert_eq!(exports[1]["name_PascalCase"], "ButtonStyle");

    println!("✅ expand_generates_case_variants_and_flags executed successfully.");
}
