use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    ClientFail(StatusCode, String),
    DatabaseError,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
        match self {
            AppError::ClientFail(status, message) => (
                status,
                headers,
                Json(json!({"status": "fail", "message": message})),
            )
                .into_response(),
            AppError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                Json(json!({"status": "error", "message": "Database error"})),
            )
                .into_response(),
            AppError::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                headers,
                Json(json!({"status": "fail", "message": "Wrong credentials"})),
            )
                .into_response(),
            AppError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                headers,
                Json(json!({"status": "fail", "message": "Missing credentials"})),
            )
                .into_response(),
            AppError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                Json(json!({"status": "error", "message": "Token creation error"})),
            )
                .into_response(),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                headers,
                Json(json!({"status": "fail", "message": "Invalid token"})),
            )
                .into_response(),
        }
    }
}
