use utoipa::OpenApi;

pub mod book;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::services::book::handler::create_book,
        crate::services::book::handler::get_books,
        crate::services::book::handler::get_book_by_id,
        crate::services::book::handler::update_book,
        crate::services::book::handler::delete_book
    ),
    components(schemas(
        crate::services::book::handler::BookParams,
        crate::services::book::handler::BooksQuery
    )),
    tags()
)]
pub struct ApiDoc;
