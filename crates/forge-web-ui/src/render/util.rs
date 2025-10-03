// crates/forge-web-ui/src/render/util.rs

/// Infer Monaco language id from a file path.
pub fn language_from_path(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs"   => "rust",
        "ts"   => "typescript",
        "tsx"  => "typescript",
        "js"   => "javascript",
        "jsx"  => "javascript",
        "toml" => "toml",
        "json" => "json",
        "md"   => "markdown",
        "html" => "html",
        "css"  => "css",
        _      => "plaintext",
    }
    .to_string()
}
