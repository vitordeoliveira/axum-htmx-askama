use axum::{http::StatusCode, response::IntoResponse};
use sqlx::migrate::MigrateError;

pub type Result<T> = core::result::Result<T, Error>;

// #[derive(Serialize, Clone, Debug, strum_macros::AsRefStr)]
#[derive(Debug, Clone)]
pub enum Error {
    InternalServer,
    Database { reason: String },
    TodoNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response
        StatusCode::INTERNAL_SERVER_ERROR.into_response()

        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
        // Insert the Error into the response.
        // response.extensions_mut().insert(self);
    }
}

impl From<MigrateError> for Error {
    fn from(value: MigrateError) -> Self {
        Error::Database {
            reason: value.to_string(),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        // debug(&value);
        Error::Database {
            reason: value.to_string(),
        }
    }
}
