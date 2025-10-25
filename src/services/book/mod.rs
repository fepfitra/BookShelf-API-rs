use self::repo::BookRepo;
use std::sync::Arc;

pub mod handler;
pub mod repo;

#[derive(Clone)]
pub struct BookState {
    pub repo: Arc<dyn BookRepo>,
}
