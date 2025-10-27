use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

pub mod inmemory;

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

#[async_trait]
pub trait BookRepo: Send + Sync {
    async fn save_book(&self, _book: &Book) -> Uuid {
        unimplemented!()
    }
    async fn get_books(
        &self,
        _name: Option<String>,
        _reading: Option<bool>,
        _finished: Option<bool>,
    ) -> Vec<BookSummary> {
        unimplemented!()
    }
    async fn get_book_by_id(&self, _id: Uuid) -> Option<Book> {
        unimplemented!()
    }
    async fn delete_book(&self, _id: Uuid) -> Uuid {
        unimplemented!()
    }
}
