// ============================================================================
// ⚙️  Auto-generated Schema Module
// Description: Data types: ForgeRequest / ForgeResponse
// File: templates/forge_ide/modules/schema.yaml
// ============================================================================

use serde::{Serialize, Deserialize};

// ------------------------------------------------------------------------
// ENUMS & STRUCTS
// ------------------------------------------------------------------------

/// Canonical request definition for Forge IDE backends

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ForgeRequest {
    /// Render a manifest file
    RenderManifest,
    /// Build a crate (debug or release)
    BuildCrate,
    /// Return environment info
    GetEnv,
    /// Custom user command
    Custom
}



/// Canonical response envelope


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForgeResponse {
    /// Field: status
    pub status: ForgeStatus,
    /// Field: message
    pub message: String,
    /// Field: data
    pub data: Option<serde_json::Value>
}

impl ForgeResponse {
    /// Construct a successful response without data
    pub fn ok(
        msg: impl Into<String>
    ) -> Self {
        Self {
            status: ForgeStatus::Ok,
            message: msg.into(),
            data: None
        }
    }

    /// Construct an error response without data
    pub fn error(
        msg: impl Into<String>
    ) -> Self {
        Self {
            status: ForgeStatus::Error,
            message: msg.into(),
            data: None
        }
    }

    /// Construct a successful response with attached data
    pub fn with_data<T: Serialize>(
        msg: impl Into<String>,
        data: &T
    ) -> Self {
        Self {
            status: ForgeStatus::Ok,
            message: msg.into(),
            data: Some(serde_json::to_value(data).unwrap_or_default())
        }
    }

}


/// Result status codes for ForgeResponse

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ForgeStatus {
    /// Operation succeeded
    Ok,
    /// Operation failed
    Error,
    /// Operation skipped
    Skipped
}




