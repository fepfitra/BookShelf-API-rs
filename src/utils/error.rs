use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Client error: {1}")]
    ClientFail(StatusCode, String),
    #[error("Database error")]
    DatabaseError,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
        let (status, status_type, message) = match self {
            AppError::ClientFail(status, message) => (status, "fail", message),
            AppError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "error", self.to_string())
            }
            AppError::WrongCredentials => (StatusCode::UNAUTHORIZED, "fail", self.to_string()),
            AppError::MissingCredentials => (StatusCode::UNAUTHORIZED, "fail", self.to_string()),
            AppError::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "error", self.to_string())
            }
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "fail", self.to_string()),
        };

        (
            status,
            headers,
            Json(json!({"status": status_type, "message": message})),
        )
            .into_response()
    }
}
