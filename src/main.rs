mod app;
mod repos;
mod services;
mod utils;

use app::app;
use services::ApiDoc;
use tracing_subscriber::prelude::*;
pub use utils::error::AppError;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

    let app = app()
        .await
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    tracing::info!("Server running on http:{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
