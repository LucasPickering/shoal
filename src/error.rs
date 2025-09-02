use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::io;
use thiserror::Error;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

/// Any error that can occur within the service
#[derive(Debug, Error)]
pub enum Error {
    /// I/O error transmitting on the network
    #[error(transparent)]
    Io(#[from] io::Error),

    /// User requested a resource that doesn't exist
    #[error("Not found")]
    NotFound,

    /// User submitted a session ID that's either invalid or no longer in the
    /// DB
    #[error("Session `{}` not found", String::from_utf8_lossy(.session_id))]
    SessionNotFound {
        /// Requested session ID. Session ID is specified by header value which
        /// may not be UTF-8, so this stores the bytes and converts lossily at
        /// display time
        session_id: Vec<u8>,
    },

    /// Error accessing the DB
    #[error(transparent)]
    Sqlite(rusqlite::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, detail) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::SessionNotFound { .. } => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            Self::Io(_) | Self::Sqlite(_) => {
                error!("Internal server error: {self}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_owned(),
                )
            }
        };
        (status_code, Json(ErrorDetail { detail })).into_response()
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

/// Body for error responses
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    detail: String,
}
