//! Simple error abstraction.

#[derive(Debug)]
pub enum RenderError {
    Io(String),
    Serde(String),
    Template(String),
}

impl From<std::io::Error> for RenderError {
    fn from(e: std::io::Error) -> Self { RenderError::Io(e.to_string()) }
}

impl From<serde_json::Error> for RenderError {
    fn from(e: serde_json::Error) -> Self { RenderError::Serde(e.to_string()) }
}
