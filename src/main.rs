#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod data;
mod routes;

use crate::data::Fish;
use axum::{Router, response::Redirect, routing::get};
use routes::*;
use std::{env, io};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Build our application with routes
    let app = Router::new()
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .route("/fish", get(list_fish).post(create_fish))
        .route(
            "/fish/{id}",
            get(get_fish_by_id).patch(update_fish).delete(delete_fish),
        );

    // Run the server
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&host).await?;
    println!("Listening on http://{host}");
    axum::serve(listener, app).await
}

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(list_fish, get_fish_by_id, create_fish, update_fish, delete_fish,),
    components(schemas(
        Fish,
        CreateFishRequest,
        UpdateFishRequest,
        ErrorDetail
    )),
    info(title = "Shoal API", description = "TODO", version = "0.1.0")
)]
struct ApiDoc;
