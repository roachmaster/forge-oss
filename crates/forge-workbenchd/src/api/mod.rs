use axum::{routing::{get, post}, Router};
use crate::AppState;

mod save;
pub use save::post_save;

mod health;
mod snapshot;
mod ws;
mod open;

/// Public router that composes the feature routes.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::healthz))
        .route("/v1/snapshot", get(snapshot::get_snapshot))
        .route("/v1/open", post(open::post_open_file))
        .route("/ws", get(ws::ws_handler))
        // use the re-exported `post_save` directly:
        .route("/v1/save", post(post_save))
        .with_state(state)
}
