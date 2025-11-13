// ============================================================================
// ⚙️  Auto-generated Router Module
// Description : ForgeRouter — entrypoint for dispatching requests via ForgeProviderRegistry
// Source      : templates/forge_ide/generated/router.yaml
// Template    : templates/forge_ide/router.mustache
// ============================================================================

use crate::provider::ForgeProviderRegistry;
use crate::schema::{ForgeRequest, ForgeResponse};
use std::sync::Arc;

// ------------------------------------------------------------------------
// STRUCT & IMPLEMENTATION
// ------------------------------------------------------------------------
/// Central router — handles incoming requests via the provider registry
#[derive(Default)]
pub struct ForgeRouter {
    pub registry: Arc<ForgeProviderRegistry>,
}

impl ForgeRouter {
    // ----------------------------------------------------------------
    // METHOD: new
    // ----------------------------------------------------------------

    pub fn new(registry: Arc<ForgeProviderRegistry>) -> Self {
        Self { registry: registry }
    }

    // ----------------------------------------------------------------
    // METHOD: handle
    // ----------------------------------------------------------------
    pub async fn handle(&self, request: &ForgeRequest) -> ForgeResponse {
        self.registry.dispatch(request).await
    }

    // ----------------------------------------------------------------
    // METHOD: info
    // ----------------------------------------------------------------

    pub fn info(&self) -> Vec<&'static str> {
        self.registry.list_registered()
    }
}
