use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    pub finished: bool,
    pub updated_at: DateTime<Utc>,
    pub inserted_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct BookSummary {
    pub id: Uuid,
    pub name: String,
    pub publisher: String,
}

pub trait BookRepo: Send + Sync {
    fn save_book(&self, _book: &Book) -> Uuid {
        unimplemented!()
    }
    fn get_books(&self) -> Vec<BookSummary> {
        unimplemented!()
    }
    fn get_book_by_id(&self, _id: Uuid) -> Option<Book> {
        unimplemented!()
    }
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
    fn get_book_by_id(&self, id: Uuid) -> Option<Book> {
        self.map.lock().unwrap().get(&id).cloned()
    }
}
