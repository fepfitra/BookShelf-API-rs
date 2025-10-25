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

#[derive(Serialize)]
pub struct BookSummary {
    pub id: Uuid,
    pub name: String,
    pub publisher: String,
}

pub trait BookRepo: Send + Sync {
    fn save_book(&self, book: &Book) -> Uuid;
    fn get_books(&self) -> Vec<BookSummary>;
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
    fn get_books(&self) -> Vec<BookSummary> {
        self.map
            .lock()
            .unwrap()
            .values()
            .map(|book| BookSummary {
                id: book.id,
                name: book.name.clone(),
                publisher: book.publisher.clone(),
            })
            .collect()
    }
}
