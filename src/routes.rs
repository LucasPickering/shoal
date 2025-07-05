use crate::data::{FISHES, Fish, FishId};
use axum::{Json, extract::Path, http::StatusCode};
use rand::Rng;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
pub struct ErrorResponse {
    success: bool,
    message: String,
}

// Handler functions
#[utoipa::path(
    get,
    path = "/fish",
    responses(
        (status = 200, description = "List of all fish", body = Vec<Fish>)
    ),
    tag = "fish"
)]
pub async fn get_all_fish() -> Json<Vec<Fish>> {
    Json(FISHES.values().cloned().collect())
}

#[utoipa::path(
    get,
    path = "/fish/{id}",
    responses(
        (status = 200, description = "Fish found", body = ApiResponse<Fish>),
        (status = 404, description = "Fish not found", body = ErrorResponse)
    ),
    params(
        ("id" = FishId, Path, description = "Fish ID")
    ),
    tag = "fish"
)]
pub async fn get_fish_by_id(
    Path(id): Path<FishId>,
) -> Result<Json<&'static Fish>, (StatusCode, Json<ErrorResponse>)> {
    match FISHES.get(&id) {
        Some(fish) => Ok(Json(fish)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                success: false,
                message: format!("Fish with ID {} not found", id),
            }),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/fish",
    request_body = CreateFishRequest,
    responses(
        (status = 201, description = "Fish created successfully", body = ApiResponse<Fish>)
    ),
    tag = "fish"
)]
pub async fn create_fish(Json(body): Json<CreateFishRequest>) -> Json<Fish> {
    let id = FishId(rand::rng().random());
    Json(Fish {
        id,
        name: body.name,
        species: body.species,
        age: body.age,
        weight_kg: body.weight_kg,
    })
}

#[utoipa::path(
    put,
    path = "/fish/{id}",
    request_body = UpdateFishRequest,
    responses(
        (status = 200, description = "Fish updated successfully", body = ApiResponse<Fish>),
        (status = 404, description = "Fish not found", body = ErrorResponse)
    ),
    params(
        ("id" = u32, Path, description = "Fish ID")
    ),
    tag = "fish"
)]
pub async fn update_fish(
    Path(id): Path<FishId>,
    Json(body): Json<UpdateFishRequest>,
) -> Result<Json<Fish>, (StatusCode, Json<ErrorResponse>)> {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/fish/{id}",
    responses(
        (status = 200, description = "Fish deleted successfully", body = ApiResponse<Fish>),
        (status = 404, description = "Fish not found", body = ErrorResponse)
    ),
    params(
        ("id" = u32, Path, description = "Fish ID")
    ),
    tag = "fish"
)]
pub async fn delete_fish(
    Path(id): Path<FishId>,
) -> Result<Json<Fish>, (StatusCode, Json<ErrorResponse>)> {
    todo!()
}
