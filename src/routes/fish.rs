//! Fish-related routes

use crate::data::{Fish, FishId, SessionId, SessionStore, Store};
use axum::{Extension, Json, extract::Path};
use serde::{Deserialize, Serialize};

/// Create new temporary session
pub async fn login(
    Extension(store): Extension<Store>,
) -> crate::Result<Json<LoginResponse>> {
    let response = store.create_session().await?;
    Ok(Json(response))
}

/// List fish
pub async fn list_fish(store: SessionStore) -> crate::Result<Json<Vec<Fish>>> {
    store.list().await.map(Json)
}

/// Get a fish by ID
pub async fn get_fish_by_id(
    store: SessionStore,
    Path(id): Path<FishId>,
) -> crate::Result<Json<Fish>> {
    store.get(id).await.map(Json)
}

/// Create a new fish
pub async fn create_fish(
    store: SessionStore,
    Json(body): Json<CreateFishRequest>,
) -> crate::Result<Json<Fish>> {
    store.create(body).await.map(Json)
}

/// Update an existing fish
pub async fn update_fish(
    store: SessionStore,
    Path(id): Path<FishId>,
    Json(body): Json<UpdateFishRequest>,
) -> crate::Result<Json<Fish>> {
    store.update(id, body).await.map(Json)
}

/// Delete an existing fish
pub async fn delete_fish(
    store: SessionStore,
    Path(id): Path<FishId>,
) -> crate::Result<Json<Fish>> {
    store.delete(id).await.map(Json)
}

/// Request body for `POST /fish`
#[derive(Debug, Deserialize)]
pub struct CreateFishRequest {
    pub name: String,
    pub species: String,
    pub age: u32,
    pub weight_kg: f64,
}

/// Request body for `PATCH /fish`
#[derive(Debug, Deserialize)]
pub struct UpdateFishRequest {
    pub name: Option<String>,
    pub species: Option<String>,
    pub age: Option<u32>,
    pub weight_kg: Option<f64>,
}

/// Response body for `POST /login`
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: SessionId,
    pub expires_at: String,
}
