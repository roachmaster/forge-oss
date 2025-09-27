mod api;
mod services;
mod config;
mod middleware;

use crate::config::{AppState, Args};
use crate::middleware::validate_request;
use axum::{Router, middleware::from_fn_with_state};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let root = args
        .root
        .or_else(|| std::env::var_os("FORGE_ROOT").map(std::path::PathBuf::from))
        .unwrap_or(std::env::current_dir()?);

    let state = AppState { root, token: args.token };

    let app: Router = api::router(state.clone())
        .layer(from_fn_with_state(state.clone(), validate_request))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(args.bind).await?;
    println!("forge-workbenchd listening on http://{}/", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

