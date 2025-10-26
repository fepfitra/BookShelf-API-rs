use std::sync::Arc;

use crate::repos::book::BookRepo;

pub mod handler;
pub mod test;

#[derive(Clone)]
pub struct BookState {
    pub repo: Arc<dyn BookRepo>,
}
