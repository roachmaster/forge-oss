//! Inline template source.

/// Simple wrapper for inline template text.
#[derive(Debug, Clone)]
pub struct InlineSource {
    pub(crate) text: String,
}

impl InlineSource {
    /// Construct a new inline template source.
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into() }
    }
}