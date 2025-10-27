use std::sync::Arc;

use axum::{
    Router,
    extract::{MatchedPath, Request},
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{
    repos::book::inmemory::InMemoryBookRepo,
    services::{
        auth::handler::{authorize, protected},
        book::BookState,
    },
};
use crate::{
    repos::book::sqlite::SqliteBookRepo,
    services::book::handler::{create_book, delete_book, get_book_by_id, get_books, update_book},
};

pub async fn app() -> Router {
    let _inmemory_book_repo = InMemoryBookRepo::default();
    let sqlite_book_repo = SqliteBookRepo::new("sqlite::memory:".to_string()).await;
    let book_router = Router::new()
        .route("/", post(create_book).get(get_books))
        .route(
            "/{id}",
            get(get_book_by_id).put(update_book).delete(delete_book),
        )
        .with_state(BookState {
            repo: Arc::new(sqlite_book_repo),
        });
    let auth_router = Router::new().route("/", post(authorize).get(protected));

    let app = Router::new()
        .nest("/auth", auth_router)
        .nest("/books", book_router)
        .layer(
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
