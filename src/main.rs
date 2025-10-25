use axum::{
    Router,
    extract::{MatchedPath, Request},
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::prelude::*;

mod utils;
pub use utils::error::AppError;

mod services;
use services::book::{
    BookState,
    handler::{create_book, get_book_by_id, get_books, update_book},
    repo::InMemoryBookRepo,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let book_repo = InMemoryBookRepo::default();
    let book_router = Router::new()
        .route("/", post(create_book).get(get_books))
        .route("/{id}", get(get_book_by_id).put(update_book))
        .with_state(BookState {
            repo: Arc::new(book_repo),
        });

    let app = Router::new()
        .route("/", get(root))
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    tracing::info!("Server running on http:{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
