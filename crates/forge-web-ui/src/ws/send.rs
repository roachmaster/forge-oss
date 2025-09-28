use crate::forge_view_model::ClientIntent;
use crate::globals::with_ws;

/// Send an intent over WS (no-op if not connected).
pub fn send_intent(intent: ClientIntent) {
    if let Ok(payload) = serde_json::to_string(&intent) {
        with_ws(|ws| { let _ = ws.send_with_str(&payload); });
    }
}
