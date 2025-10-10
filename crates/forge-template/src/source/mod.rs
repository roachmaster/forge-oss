//! Template sources abstraction + re-exports.

pub mod inline;
pub mod file;
pub(crate) mod partials;

pub use inline::InlineSource;
pub use file::FileSource;

#[derive(Debug, Clone)]
pub enum TemplateSource {
    Inline { text: String },
    File { path: std::path::PathBuf },
}

impl From<InlineSource> for TemplateSource {
    fn from(src: InlineSource) -> Self {
        TemplateSource::Inline { text: src.text }
    }
}

impl From<FileSource> for TemplateSource {
    fn from(src: FileSource) -> Self {
        TemplateSource::File { path: src.path }
    }
}