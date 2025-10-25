use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    ClientFail(StatusCode, String),
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
        }
    }
}

