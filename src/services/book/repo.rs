use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub year: i32,
    pub author: String,
    pub summary: String,
    pub publisher: String,
    pub page_count: i32,
    pub read_page: i32,
    pub reading: bool,
}

pub trait BookRepo: Send + Sync {
    fn save_book(&self, book: &Book) -> Uuid;
}

#[derive(Default, Clone)]
pub struct InMemoryBookRepo {
    map: Arc<Mutex<HashMap<Uuid, Book>>>,
}

impl BookRepo for InMemoryBookRepo {
    fn save_book(&self, book: &Book) -> Uuid {
        self.map.lock().unwrap().insert(book.id, book.clone());
        book.id
    }
}
