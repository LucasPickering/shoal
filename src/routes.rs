use crate::data::{Fish, FishId, Store};
use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Handler functions
#[utoipa::path(
    get,
    path = "/fish",
    responses(
        (status = 200, description = "List of all fish", body = Vec<Fish>)
    ),
    tag = "fish"
)]
pub async fn list_fish() -> Json<Vec<&'static Fish>> {
    Json(Store::all().collect())
}

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
    Path(id): Path<FishId>,
) -> Result<Json<&'static Fish>, (StatusCode, Json<ErrorDetail>)> {
    Store::get_or_404(id).map(Json)
}

#[utoipa::path(
    post,
    path = "/fish",
    request_body = CreateFishRequest,
    responses(
        (status = 201, description = "Fish created successfully", body = Fish)
    ),
    tag = "fish"
)]
pub async fn create_fish(Json(body): Json<CreateFishRequest>) -> Json<Fish> {
    Json(Fish {
        id: Store::next_id(),
        name: body.name,
        species: body.species,
        age: body.age,
        weight_kg: body.weight_kg,
    })
}

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
    Path(id): Path<FishId>,
    Json(body): Json<UpdateFishRequest>,
) -> Result<Json<Fish>, (StatusCode, Json<ErrorDetail>)> {
    let fish = Store::get_or_404(id)?;
    Ok(Json(Fish {
        id,
        name: body.name.unwrap_or_else(|| fish.name.clone()),
        species: body.species.unwrap_or_else(|| fish.species.clone()),
        age: body.age.unwrap_or(fish.age),
        weight_kg: body.weight_kg.unwrap_or(fish.weight_kg),
    }))
}

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
    Path(id): Path<FishId>,
) -> Result<Json<&'static Fish>, (StatusCode, Json<ErrorDetail>)> {
    // Return the "deleted" fish
    Store::get_or_404(id).map(Json)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateFishRequest {
    name: String,
    species: String,
    age: u32,
    weight_kg: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFishRequest {
    name: Option<String>,
    species: Option<String>,
    age: Option<u32>,
    weight_kg: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorDetail {
    message: String,
}

impl ErrorDetail {
    /// Get a 404 response for an unknown fish ID
    pub fn not_found(id: FishId) -> (StatusCode, Json<Self>) {
        (
            StatusCode::NOT_FOUND,
            Json(Self {
                message: format!("No fish with ID {id}"),
            }),
        )
    }
}
