// ============================================================================
// ⚙️  Auto-generated Provider Module
// Description : Forge provider registry — manages and dispatches ForgeIdeCommand instances
// Source      : templates/forge_ide/generated/provider.yaml
// Template    : templates/forge_ide/provider.mustache
// ============================================================================

use async_trait::async_trait;
use crate::schema::{ ForgeRequest, ForgeResponse };
use crate::command::{ ForgeIdeCommand, ForgeCommandKind };
use std::collections::{ HashMap };
use std::sync::{ Arc };

// ------------------------------------------------------------------------
// STRUCT & IMPLEMENTATION
// ------------------------------------------------------------------------
/// Central registry for all Forge IDE command providers
#[derive(Debug, Default)]
pub struct ForgeProviderRegistry {
    pub providers: HashMap<ForgeCommandKind, Arc<dyn ForgeIdeCommand>>,
}

impl ForgeProviderRegistry {
    // ----------------------------------------------------------------
    // METHOD: new
    // ----------------------------------------------------------------
    
    pub fn new() -> Self {

        Self { providers: HashMap::new(), }



    }

    // ----------------------------------------------------------------
    // METHOD: register
    // ----------------------------------------------------------------
    
    pub fn register(self: &mut self, kind: ForgeCommandKind, command: Arc<dyn ForgeIdeCommand>) -> () {


        self.providers.insert(kind, command);


    }

    // ----------------------------------------------------------------
    // METHOD: dispatch
    // ----------------------------------------------------------------
    pub async fn dispatch(self: &self, request: &ForgeRequest) -> ForgeResponse {
    



        match request {
            RenderManifest => self.dispatch_render(request).await,
            BuildCrate => self.dispatch_build(request).await,
            GetEnv => self.dispatch_env(request).await,
            Custom => self.dispatch_custom(request).await,
        }
    }

    // ----------------------------------------------------------------
    // METHOD: list_registered
    // ----------------------------------------------------------------
    
    pub fn list_registered(self: &self) -> Vec<&'static str> {



        self.providers
        .values()
        .map(|c| c.name())
        .collect::<Vec<_>>()

    }

}

// ------------------------------------------------------------------------
// DISPATCH HANDLERS (auto-generated stubs)
// ------------------------------------------------------------------------
impl ForgeProviderRegistry {
                    
    pub async fn dispatch_render(
        &self,
        _request: &ForgeRequest
    ) -> ForgeResponse {
        todo!("Handle Render request variant");
    }


    pub async fn dispatch_build(
        &self,
        _request: &ForgeRequest
    ) -> ForgeResponse {
        todo!("Handle Build request variant");
    }


    pub async fn dispatch_env(
        &self,
        _request: &ForgeRequest
    ) -> ForgeResponse {
        todo!("Handle Env request variant");
    }


    pub async fn dispatch_custom(
        &self,
        _request: &ForgeRequest
    ) -> ForgeResponse {
        todo!("Handle Custom request variant");
    }

            
}

