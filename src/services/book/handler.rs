use axum::extract::{Path, Query};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::AppError;
use crate::repos::book::Book;

use super::BookState;

#[derive(Deserialize, ToSchema)]
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
    #[allow(dead_code)]
    pub inserted_at: DateTime<Utc>,
    #[serde(default)]
    #[allow(dead_code)]
    pub updated_at: DateTime<Utc>,
}
#[derive(Deserialize, ToSchema)]
pub struct BooksQuery {
    name: Option<String>,
    reading: Option<String>,
    finished: Option<String>,
}

#[utoipa::path(
    post,
    path = "/books",
    request_body = BookParams,
    responses(
        (status = 201, description = "Buku berhasil ditambahkan"),
        (status = 400, description = "Gagal menambahkan buku"),
    )
)]
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
    let id = state.repo.save_book(&book).await;

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

#[utoipa::path(
    get,
    path = "/books",
    params(
        ("name" = Option<String>, Query, description = "Filter books by name containing the given string"),
        ("reading" = Option<String>, Query, description = "Filter books by reading status (1 for reading, 0 for not reading)"),
        ("finished" = Option<String>, Query, description = "Filter books by finished status (1 for finished, 0 for not finished)"),
    ),
    responses(
        (status = 200, description = "List of books retrieved successfully"),
    )
)]
pub async fn get_books(
    State(state): State<BookState>,
    Query(query): Query<BooksQuery>,
) -> impl IntoResponse {
    let reading = query
        .reading
        .and_then(|s| s.parse::<u8>().ok())
        .map(|n| n == 1);
    let finished = query
        .finished
        .and_then(|s| s.parse::<u8>().ok())
        .map(|n| n == 1);
    let name = query.name;

    let books = state.repo.get_books(name, reading, finished).await;

    let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
    let body = Json(json!({
        "status": "success",
        "data": {
            "books": books
        }
    }));

    (StatusCode::OK, headers, body)
}

#[utoipa::path(
    get,
    path = "/books/{id}",
    responses(
        (status = 200, description = "Buku ditemukan"),
        (status = 404, description = "Buku tidak ditemukan"),
    ),
    params(
        ("id" = String, Path, description = "ID of the book to retrieve"),
    )
)]
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
    if let Some(book) = state.repo.get_book_by_id(book_id).await {
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

#[utoipa::path(
    put,
    path = "/books/{id}",
    request_body = BookParams,
    responses(
        (status = 200, description = "Buku berhasil diperbarui"),
        (status = 400, description = "Gagal memperbarui buku"),
        (status = 404, description = "Buku tidak ditemukan"),
    ),
    params(
        ("id" = String, Path, description = "ID of the book to update"),
    )
)]
pub async fn update_book(
    State(state): State<BookState>,
    Path(id): Path<String>,
    Json(params): Json<BookParams>,
) -> Result<impl IntoResponse, AppError> {
    let book_id = match Uuid::parse_str(&id) {
        Ok(data) => data,
        Err(_) => {
            let message = "Gagal memperbarui buku. Id tidak ditemukan".to_string();
            return Err(AppError::ClientFail(StatusCode::NOT_FOUND, message));
        }
    };

    if params.name.is_empty() {
        let message = "Gagal memperbarui buku. Mohon isi nama buku".to_string();
        return Err(AppError::ClientFail(StatusCode::BAD_REQUEST, message));
    }

    if params.read_page > params.page_count {
        let message =
            "Gagal memperbarui buku. readPage tidak boleh lebih besar dari pageCount".to_string();
        return Err(AppError::ClientFail(StatusCode::BAD_REQUEST, message));
    }

    if let Some(mut book) = state.repo.get_book_by_id(book_id).await {
        book.name = params.name;
        book.year = if params.year == 0 {
            book.year
        } else {
            params.year
        };
        book.author = if params.author.is_empty() {
            book.author
        } else {
            params.author
        };
        book.summary = if params.summary.is_empty() {
            book.summary
        } else {
            params.summary
        };
        book.publisher = if params.publisher.is_empty() {
            book.publisher
        } else {
            params.publisher
        };
        book.page_count = if params.page_count == 0 {
            book.page_count
        } else {
            params.page_count
        };
        book.read_page = if params.read_page == 0 {
            book.read_page
        } else {
            params.read_page
        };
        book.reading = if params.reading {
            params.reading
        } else {
            book.reading
        };
        book.finished = if params.reading {
            params.read_page == params.page_count
        } else {
            false
        };
        book.updated_at = Utc::now();

        state.repo.save_book(&book).await;

        let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
        let body = Json(json!({
            "status": "success",
            "message": "Buku berhasil diperbarui"
        }));

        Ok((StatusCode::OK, headers, body))
    } else {
        let message = "Gagal memperbarui buku. Id tidak ditemukan".to_string();
        Err(AppError::ClientFail(StatusCode::NOT_FOUND, message))
    }
}

#[utoipa::path(
    delete,
    path = "/books/{id}",
    responses(
        (status = 200, description = "Buku berhasil dihapus"),
        (status = 404, description = "Buku gagal dihapus. Id tidak ditemukan"),
    ),
    params(
        ("id" = String, Path, description = "ID of the book to delete"),
    )
)]
pub async fn delete_book(
    State(state): State<BookState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let book_id = match Uuid::parse_str(&id) {
        Ok(data) => data,
        Err(_) => {
            let message = "Buku gagal dihapus. Id tidak ditemukan".to_string();
            return Err(AppError::ClientFail(StatusCode::NOT_FOUND, message));
        }
    };

    let deleted_id = state.repo.delete_book(book_id).await;

    let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];
    let body = Json(json!({
        "status": "success",
        "message": "Buku berhasil dihapus",
        "data": {
            "bookId": deleted_id
        }
    }));

    Ok((StatusCode::OK, headers, body))
}
