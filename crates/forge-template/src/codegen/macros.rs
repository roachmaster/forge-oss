// forge-template/src/codegen/macros.rs

/// Read a file and parse it into `serde_json::Value`.
/// Fully-qualified paths are used so callers don't need to import anything.
#[macro_export]
macro_rules! read_and_parse {
    (yaml, $path:expr) => {{
        let __path = $path;
        let __text = ::std::fs::read_to_string(__path).map_err(|e| {
            ::anyhow::anyhow!(
                "failed to read YAML {}: {e}",
                ::std::path::Path::new(__path).display()
            )
        })?;
        let __val = ::serde_yaml::from_str::<::serde_json::Value>(&__text).map_err(|e| {
            ::anyhow::anyhow!(
                "failed to parse YAML {}: {e}",
                ::std::path::Path::new(__path).display()
            )
        })?;
        Ok::<::serde_json::Value, ::anyhow::Error>(__val)
    }};
    (json, $path:expr) => {{
        let __path = $path;
        let __text = ::std::fs::read_to_string(__path).map_err(|e| {
            ::anyhow::anyhow!(
                "failed to read JSON {}: {e}",
                ::std::path::Path::new(__path).display()
            )
        })?;
        let __val = ::serde_json::from_str::<::serde_json::Value>(&__text).map_err(|e| {
            ::anyhow::anyhow!(
                "failed to parse JSON {}: {e}",
                ::std::path::Path::new(__path).display()
            )
        })?;
        Ok::<::serde_json::Value, ::anyhow::Error>(__val)
    }};
}
