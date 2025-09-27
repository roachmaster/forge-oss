use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::config::AppState;

/// Simple token-checking middleware.
/// If `AppState.token` is `Some`, it verifies the `X-Forge-Token` header.
pub async fn validate_request(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(expected) = &state.token {
        let ok = req
            .headers()
            .get("X-Forge-Token")
            .and_then(|v| v.to_str().ok())
            .map(|v| v == expected)
            .unwrap_or(false);

        if !ok {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("missing or invalid X-Forge-Token"))
                .unwrap();
        }
    }
    next.run(req).await
}
