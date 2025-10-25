use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::BookState;
use super::repo::Book;
use crate::AppError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookParams {
    #[serde(default)]
    pub name: String,
    pub year: i32,
    pub author: String,
    pub summary: String,
    pub publisher: String,
    pub page_count: i32,
    pub read_page: i32,
    pub reading: bool,
}

pub async fn create_book(
    State(state): State<BookState>,
    Json(params): Json<BookParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.name.is_empty() {
        let message = "Gagal menambahkan buku. Mohon isi nama buku".to_string();
        return Err(AppError::ClientFail(StatusCode::BAD_REQUEST, message));
    }

    let book = Book {
        id: Uuid::new_v4(),
        name: params.name,
        year: params.year,
        publisher: params.publisher,
        author: params.author,
        summary: params.summary,
        page_count: params.page_count,
        read_page: params.read_page,
        reading: params.reading,
    };
    let id = state.repo.save_book(&book);

    let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];

    let body = Json(json!({
        "status": "success",
        "message": "Buku berhasil ditambahkan",
        "data": {
            "bookId": id
        }
    }));

    Ok((StatusCode::CREATED, headers, body))
}
