use axum::{
    Json,
    extract::rejection::PathRejection,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    ClientFail(StatusCode, String),
    DatabaseError,
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
        }
    }
}

impl From<PathRejection> for AppError {
    fn from(_rejection: PathRejection) -> Self {
        let message = "Buku tidak ditemukan".to_string();
        AppError::ClientFail(StatusCode::NOT_FOUND, message)
    }
}
