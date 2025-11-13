// ============================================================================
// ⚙️  Auto-generated Command Module
// Description: Defines the ForgeIdeCommand trait and dispatch enums
// File: templates/forge_ide/generated/command.yaml
// ============================================================================

use crate::schema::{ForgeRequest, ForgeResponse};
use async_trait::async_trait;

// ------------------------------------------------------------------------
// TRAIT & ENUM DEFINITIONS
// ------------------------------------------------------------------------

/// Common interface for all Forge IDE commands

#[async_trait]
pub trait ForgeIdeCommand: Send + Sync {
    /// Executes the command given an incoming ForgeRequest.
    async fn execute(&self, request: &ForgeRequest) -> ForgeResponse;
    /// Optional: name of the command for logging/debugging.
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Helper enum for generic dispatch (optional in later steps)

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForgeCommandKind {
    Build,
    Render,
    Env,
    Custom,
}
