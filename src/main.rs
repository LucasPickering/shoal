#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod data;
mod error;
mod routes;

pub use crate::error::{Error, Result};

use crate::data::Store;
use axum::{
    Extension, Router,
    body::Body,
    http::header::CONTENT_TYPE,
    response::{Html, Redirect, Response},
    routing::{any, get, post},
};
use routes::*;
use std::{env, time::Duration};
use tower_http::trace::TraceLayer;
use tracing::info;

// Include docs so we can ship as a single binary
const DOCS_HTML: &[u8] = include_bytes!("../static/docs.html");
const OPENAPI_YML: &[u8] = include_bytes!("../static/openapi.yml");

#[tokio::main]
async fn main() -> crate::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initial an in-memory DB for fish
    let store = Store::new()?;

    // Build our application with routes
    let app = Router::new()
        // API docs
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs", get(|| async { Html(DOCS_HTML) }))
        .route(
            "/openapi.yml",
            get(|| async {
                Response::builder()
                    .header(CONTENT_TYPE, "application/yaml")
                    .body(Body::from(OPENAPI_YML))
                    .unwrap()
            }),
        )
        // Routes
        .route("/login", post(login))
        .route("/fish", get(list_fish).post(create_fish))
        .route(
            "/fish/{id}",
            get(get_fish_by_id).patch(update_fish).delete(delete_fish),
        )
        .route("/anything", any(anything))
        .route("/anything/{*path}", any(anything))
        .fallback(|| async { Error::NotFound })
        .layer(Extension(store.clone()))
        .layer(TraceLayer::new_for_http());

    // Start a background task to reap expired sessions
    tokio::spawn(reap_sessions(store));

    // Run the server
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&host).await?;
    println!("Listening on http://{host}");
    axum::serve(listener, app).await?;
    Ok(())
}

/// Background task to reap expired sessions
async fn reap_sessions(store: Store) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        match store.reap_sessions().await {
            Ok(sessions) => info!(?sessions, "Deleted expired sessions"),
            Err(error) => tracing::error!(
                error = &error as &dyn std::error::Error,
                "Error reaping sessions"
            ),
        }
    }
}
