use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
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
    #[serde(default)]
    #[allow(dead_code)]
    pub finished: bool,
    #[serde(default)]
    pub inserted_at: DateTime<Utc>,
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
}

pub async fn create_book(
    State(state): State<BookState>,
    Json(params): Json<BookParams>,
) -> Result<impl IntoResponse, AppError> {
    if params.name.is_empty() {
        let message = "Gagal menambahkan buku. Mohon isi nama buku".to_string();
        return Err(AppError::ClientFail(StatusCode::BAD_REQUEST, message));
    }

    if params.read_page > params.page_count {
        let message =
            "Gagal menambahkan buku. readPage tidak boleh lebih besar dari pageCount".to_string();
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
        updated_at: Utc::now(),
        inserted_at: Utc::now(),
        finished: if params.reading {
            params.read_page == params.page_count
        } else {
            false
        },
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

pub async fn get_books(State(state): State<BookState>) -> impl IntoResponse {
    let books = state.repo.get_books();

    let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
    let body = Json(json!({
        "status": "success",
        "data": {
            "books": books
        }
    }));

    (StatusCode::OK, headers, body)
}

pub async fn get_book_by_id(
    State(state): State<BookState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let book_id = match Uuid::parse_str(&id) {
        Ok(data) => data,
        Err(_) => {
            let message = "Buku tidak ditemukan".to_string();
            return Err(AppError::ClientFail(StatusCode::NOT_FOUND, message));
        }
    };
    if let Some(book) = state.repo.get_book_by_id(book_id) {
        let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
        let body = Json(json!({
            "status": "success",
            "data": {
                "book": book
            }
        }));

        Ok((StatusCode::OK, headers, body))
    } else {
        let message = "Buku tidak ditemukan".to_string();
        Err(AppError::ClientFail(StatusCode::NOT_FOUND, message))
    }
}
