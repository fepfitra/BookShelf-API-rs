use std::sync::Arc;

use axum::{
    Router,
    extract::{MatchedPath, Request},
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::services::book::handler::{
    create_book, delete_book, get_book_by_id, get_books, update_book,
};
use crate::{repos::book::inmemory::InMemoryBookRepo, services::book::BookState};

pub fn app() -> Router {
    let book_repo = InMemoryBookRepo::default();
    let book_router = Router::new()
        .route("/", post(create_book).get(get_books))
        .route(
            "/{id}",
            get(get_book_by_id).put(update_book).delete(delete_book),
        )
        .with_state(BookState {
            repo: Arc::new(book_repo),
        });

    let app = Router::new().nest("/books", book_router).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(()),
    );
    app
}
