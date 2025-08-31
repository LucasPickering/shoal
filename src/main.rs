#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod data;
mod error;
mod routes;

pub use crate::error::{Error, Result};

use crate::{
    data::{Fish, Store},
    error::ErrorDetail,
};
use axum::{
    Extension, Router,
    response::Redirect,
    routing::{any, get, post},
};
use routes::*;
use std::env;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> crate::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initial an in-memory DB for fish
    let store = Store::new()?;

    // Build our application with routes
    let app = Router::new()
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .route("/login", post(login))
        .route("/fish", get(list_fish).post(create_fish))
        .route(
            "/fish/{id}",
            get(get_fish_by_id).patch(update_fish).delete(delete_fish),
        )
        .route("/anything", any(anything))
        .route("/anything/{*path}", any(anything))
        .layer(Extension(store))
        .layer(TraceLayer::new_for_http());

    // Run the server
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&host).await?;
    println!("Listening on http://{host}");
    axum::serve(listener, app).await?;
    Ok(())
}

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(list_fish, get_fish_by_id, create_fish, update_fish, delete_fish,),
    components(schemas(
        Fish,
        CreateFishRequest,
        UpdateFishRequest,
        ErrorDetail,
    )),
    info(title = "Shoal API", description = "TODO", version = "0.1.0")
)]
struct ApiDoc;
