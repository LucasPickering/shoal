//! Fish-related routes

use crate::data::{Fish, FishId, SessionId, SessionStore, Store};
use axum::{Extension, Json, extract::Path};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Create new temporary session
///
/// TODO more
#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = 200,
        description = "Create a new temporary session",
        body = LoginResponse)
    ),
    tag = "fish"
)]
pub async fn login(
    Extension(store): Extension<Store>,
) -> crate::Result<Json<LoginResponse>> {
    let response = store.create_session().await?;
    Ok(Json(response))
}

/// List fish
#[utoipa::path(
    get,
    path = "/fish",
    responses(
        (status = 200, description = "List of fish", body = Vec<Fish>)
    ),
    tag = "fish"
)]
pub async fn list_fish(store: SessionStore) -> crate::Result<Json<Vec<Fish>>> {
    store.list().await.map(Json)
}

/// Get a fish by ID
#[utoipa::path(
    get,
    path = "/fish/{id}",
    responses(
        (status = 200, description = "Fish found", body = Fish),
        (status = 404, description = "Fish not found")
    ),
    params(
        ("id" = FishId, Path, description = "Fish ID")
    ),
    tag = "fish"
)]
pub async fn get_fish_by_id(
    store: SessionStore,
    Path(id): Path<FishId>,
) -> crate::Result<Json<Fish>> {
    store.get(id).await.map(Json)
}

/// Create a new fish
///
/// Requires an active session
/// TODO document header
#[utoipa::path(
    post,
    path = "/fish",
    request_body = CreateFishRequest,
    responses(
        (status = 201, description = "Fish created successfully", body = Fish)
    ),
    tag = "fish"
)]
pub async fn create_fish(
    store: SessionStore,
    Json(body): Json<CreateFishRequest>,
) -> crate::Result<Json<Fish>> {
    store.create(body).await.map(Json)
}

/// Update an existing fish
///
/// Requires an active session
/// TODO document header
#[utoipa::path(
    patch,
    path = "/fish/{id}",
    request_body = UpdateFishRequest,
    responses(
        (status = 200, description = "Fish updated successfully", body = Fish),
        (status = 404, description = "Fish not found")
    ),
    params(
        ("id" = u32, Path, description = "Fish ID")
    ),
    tag = "fish"
)]
pub async fn update_fish(
    store: SessionStore,
    Path(id): Path<FishId>,
    Json(body): Json<UpdateFishRequest>,
) -> crate::Result<Json<Fish>> {
    store.update(id, body).await.map(Json)
}

/// Delete an existing fish
///
/// Requires an active session
/// TODO document header
#[utoipa::path(
    delete,
    path = "/fish/{id}",
    responses(
        (status = 200, description = "Fish deleted successfully", body = Fish),
        (status = 404, description = "Fish not found")
    ),
    params(
        ("id" = u32, Path, description = "Fish ID")
    ),
    tag = "fish"
)]
pub async fn delete_fish(
    store: SessionStore,
    Path(id): Path<FishId>,
) -> crate::Result<Json<Fish>> {
    store.delete(id).await.map(Json)
}

/// Request body for `POST /fish`
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFishRequest {
    pub name: String,
    pub species: String,
    pub age: u32,
    pub weight_kg: f64,
}

/// Request body for `PATCH /fish`
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFishRequest {
    pub name: Option<String>,
    pub species: Option<String>,
    pub age: Option<u32>,
    pub weight_kg: Option<f64>,
}

/// Response body for `POST /login`
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub id: SessionId,
    pub expires_at: String,
}
