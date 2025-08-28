use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::io;
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

pub type Result<T> = std::result::Result<T, Error>;

/// Any error that can occur within the service
/// TODO thiserror
#[derive(Debug, Error)]
pub enum Error {
    /// TODO
    #[error(transparent)]
    Io(#[from] io::Error),

    /// TODO
    #[error("Not found")]
    NotFound,

    /// TODO
    #[error(transparent)]
    Sqlite(rusqlite::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // TODO pack errors into JSON
        // TODO log errors
        let (status_code, detail) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Self::Io(_) | Self::Sqlite(_) => {
                error!("Internal server error: {self}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        (
            status_code,
            Json(ErrorDetail {
                detail: detail.into(),
            }),
        )
            .into_response()
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            // Map empty query error to 404
            rusqlite::Error::QueryReturnedNoRows => Self::NotFound,
            _ => Self::Sqlite(error),
        }
    }
}

/// TODO
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDetail {
    detail: String,
}
