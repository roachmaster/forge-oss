//! Render options: HTML escaping, missing-key policy, partials placeholder,
//! and whitespace handling modes.

use std::path::PathBuf;

/// How to handle `{{missing}}` keys.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MissingKeyPolicy {
    Error,
    Empty,
    KeepTag,
}

/// Whitespace handling modes applied to the final output.
/// - Keep:      leave output as-is
/// - TrimLines: trim trailing spaces and drop lines that are all whitespace
/// - SmartIndent: TrimLines + collapse runs of blank lines to a single blank line
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WhitespaceMode {
    Keep,
    TrimLines,
    SmartIndent,
}

/// Where to resolve partials from (MVP placeholder).
// add at top
use std::collections::HashMap;

// replace the Partials enum with this:
#[derive(Debug, Clone)]
pub enum Partials {
    /// No partials.
    None,
    /// Resolve `{{> name}}` from a directory: `dir/name.mustache`.
    Dir(std::path::PathBuf),
    /// Resolve from an in-memory map: `name` → template text.
    Map(HashMap<String, String>),
    /// Prefer map; if missing, fall back to dir.
    MapThenDir {
        map: HashMap<String, String>,
        dir: std::path::PathBuf,
    },
}
impl Default for Partials {
    fn default() -> Self { Partials::None }
}

/// Options that influence rendering behavior.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub html_escape: bool,
    pub on_missing: MissingKeyPolicy,
    pub partials: Partials,
    pub whitespace: WhitespaceMode,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            html_escape: true,
            on_missing: MissingKeyPolicy::Error,
            partials: Partials::None,
            whitespace: WhitespaceMode::Keep,
        }
    }
}