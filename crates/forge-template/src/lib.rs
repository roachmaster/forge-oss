//! forge-template
//!
//! Core template rendering library for Forge projects.
//! Provides YAML → Mustache → rendered text pipeline,
//! plus an optional CLI (`forge-template`) and future service mode.

pub mod codegen;
pub mod context;
pub mod utils;
pub mod errors;
pub mod helpers;

// -----------------------------------------------------------------------------
// Optional higher layers (CLI + future HTTP service)
// -----------------------------------------------------------------------------
pub mod cli;

// -----------------------------------------------------------------------------
// Public re-exports (primary API surface)
// -----------------------------------------------------------------------------
pub use codegen::render_yaml_from_abs;
pub use errors::*;
pub use helpers::*;
