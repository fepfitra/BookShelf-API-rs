use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

// Import from our sibling 'repo' module and parent 'book' module
use super::BookState;
use super::repo::Book;

#[derive(Deserialize)]
pub struct BookParams {
    pub name: String,
    pub year: i32,
    pub author: String,
    pub summary: String,
    pub publisher: String,
    pub pageCount: i32,
    pub readPage: i32,
    pub reading: bool,
}

pub async fn create_book(
    State(state): State<BookState>,
    Json(params): Json<BookParams>,
) -> impl IntoResponse {
    let book = Book {
        id: Uuid::new_v4(),
        name: params.name,
        year: params.year,
        publisher: params.publisher,
        author: params.author,
        summary: params.summary,
        pageCount: params.pageCount,
        readPage: params.readPage,
        reading: params.reading,
    };
    let id = state.repo.save_book(&book);

    let headers = [(header::CONTENT_TYPE, "application/json; charset=utf-8")];

    (StatusCode::CREATED, headers, Json(json!({"id": id})))
}
