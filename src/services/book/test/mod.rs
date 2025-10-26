use axum::{
    Router,
    body::Body,
    http::{Method, Request, header},
};

use serde_json::{Value, json};
use tower::ServiceExt;

pub mod del;
pub mod get;
pub mod post;
pub mod put;

#[allow(dead_code)]
pub fn new_book_dummy() -> Value {
    json!({
        "name": "Buku A",
        "year": 2010,
        "author": "John Doe",
        "summary": "Lorem ipsum dolor sit amet",
        "publisher": "Dicoding Indonesia",
        "pageCount": 100,
        "readPage": 25,
        "reading": false
    })
}

#[allow(dead_code)]
pub fn update_book_dummy() -> Value {
    json!({
        "name": "Buku Revisi",
        "year": 2011,
        "author": "Jane Doe",
        "summary": "Lorem Dolor sit Ametttt",
        "publisher": "Dicoding",
        "pageCount": 200,
        "readPage": 26,
        "reading": false
    })
}

#[allow(dead_code)]
async fn get_ready_service(app: &mut Router) -> &mut Router {
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .expect("Service should be ready")
}

#[allow(dead_code)]
fn build_create_book_request(payload: Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri("/books")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap()
}

#[allow(dead_code)]
fn build_get_books_request() -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri("/books")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap()
}

#[allow(dead_code)]
fn build_get_book_by_id_request(id: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(format!("/books/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap()
}

#[allow(dead_code)]
fn build_update_book_request(id: &str, payload: Value) -> Request<Body> {
    Request::builder()
        .method(Method::PUT)
        .uri(format!("/books/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap()
}

#[allow(dead_code)]
fn build_delete_book_request(id: &str) -> Request<Body> {
    Request::builder()
        .method(Method::DELETE)
        .uri(format!("/books/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap()
}
