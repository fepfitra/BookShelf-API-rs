use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tracing_subscriber::prelude::*;

mod utils;
pub use utils::error::AppError;

mod services;
use services::book::{BookState, handler::create_book, repo::InMemoryBookRepo};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let book_repo = InMemoryBookRepo::default();
    let book_router = Router::new()
        .route("/", post(create_book))
        .with_state(BookState {
            repo: Arc::new(book_repo),
        });

    let app = Router::new()
        .route("/", get(root))
        .nest("/books", book_router);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    tracing::info!("Server running on http:{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
