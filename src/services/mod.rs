use utoipa::OpenApi;

pub mod auth;
pub mod book;

#[derive(OpenApi)]
#[openapi(
    paths(
        book::handler::create_book,
        book::handler::get_books,
        book::handler::get_book_by_id,
        book::handler::update_book,
        book::handler::delete_book,
        auth::handler::authorize,
        auth::handler::protected
    ),
    components(schemas(book::handler::BookParams, book::handler::BooksQuery, auth::AuthParams))
)]
pub struct ApiDoc;
