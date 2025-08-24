mod anything;
mod fish;

pub use anything::*;
pub use fish::*;

use crate::data::FishId;
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
