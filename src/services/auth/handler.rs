use std::sync::LazyLock;

use axum::{Json, response::IntoResponse};
use jsonwebtoken::{Header, encode};
use serde_json::json;

use crate::{AppError, services::auth::AuthParams};

use super::{Claims, Keys};

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| Keys::new(b"test"));

#[utoipa::path(
    post,
    path = "/auth",
    request_body = AuthParams,
    responses(
        (status = 200, description = "Authentication successful"),
        (status = 401, description = "Authentication failed"),
    )
)]
pub async fn authorize(Json(params): Json<AuthParams>) -> Result<impl IntoResponse, AppError> {
    if params.client_id.is_empty() || params.client_secret.is_empty() {
        return Err(AppError::MissingCredentials);
    }
    if params.client_id != "foo" || params.client_secret != "bar" {
        return Err(AppError::WrongCredentials);
    }

    let claims = Claims {
        sub: params.client_id,
        exp: 10000000000,
    };
    let token =
        encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AppError::TokenCreation)?;

    Ok(Json(json!({ "token": token })))
}

#[utoipa::path(
    get,
    path = "/auth",
    responses(
        (status = 200, description = "Access to protected resource granted"),
        (status = 401, description = "Invalid or missing token"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn protected(claims: Claims) -> Result<impl IntoResponse, AppError> {
    Ok(Json(
        json!({ "message": format!("Hello, {}!", claims.sub) }),
    ))
}
