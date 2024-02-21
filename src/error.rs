use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

// #[derive(Serialize, Clone, Debug, strum_macros::AsRefStr)]
#[derive(Debug, Clone)]
pub enum Error {
    InternalServerError,
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
