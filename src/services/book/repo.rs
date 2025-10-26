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
    fn get_books(
        &self,
        _name: Option<String>,
        _reading: Option<bool>,
        _finished: Option<bool>,
    ) -> Vec<BookSummary> {
        unimplemented!()
    }
    fn get_book_by_id(&self, _id: Uuid) -> Option<Book> {
        unimplemented!()
    }
    fn delete_book(&self, _id: Uuid) -> Uuid {
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
    fn get_books(
        &self,
        name: Option<String>,
        reading: Option<bool>,
        finished: Option<bool>,
    ) -> Vec<BookSummary> {
        let map = self.map.lock().unwrap();
        let mut books: Vec<_> = map.values().collect();

        if let Some(name_filter) = &name {
            books.retain(|book| {
                book.name
                    .to_lowercase()
                    .contains(&name_filter.to_lowercase())
            });
        }
        if let Some(reading_filter) = reading {
            books.retain(|book| book.reading == reading_filter);
        }

        if let Some(finished_filter) = finished {
            books.retain(|book| book.finished == finished_filter);
        }

        books
            .into_iter()
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
    fn delete_book(&self, id: Uuid) -> Uuid {
        self.map
            .lock()
            .unwrap()
            .remove(&id)
            .map(|book| book.id)
            .unwrap_or(id)
    }
}
