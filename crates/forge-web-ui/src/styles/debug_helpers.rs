pub const CSS: &str = r#"
/* ---------------- Debug helpers (toggle in Elements panel) ---------------- */
/* Add class 'debug-frames' on <body> to visualize layout boxes */
body.debug-frames #body { outline:2px dashed #88c; }
body.debug-frames #sidebar { outline:2px dashed #c88; }
body.debug-frames [data-role='editor'] { outline:2px dashed #8c8; }
body.debug-frames #editor-host { outline:2px solid #46a; }
"#;
