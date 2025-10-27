use utoipa::OpenApi;

pub mod auth;
pub mod book;

use crate::services;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::services::book::handler::create_book,
        crate::services::book::handler::get_books,
        crate::services::book::handler::get_book_by_id,
        crate::services::book::handler::update_book,
        crate::services::book::handler::delete_book,
        crate::services::auth::handler::authorize,
        crate::services::auth::handler::protected
    ),
    components(schemas(
        crate::services::book::handler::BookParams,
        crate::services::book::handler::BooksQuery,
        crate::services::auth::AuthParams
    )),
    tags()
)]
pub struct ApiDoc;
