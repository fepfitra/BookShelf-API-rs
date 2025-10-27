use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::{Book, BookRepo, BookSummary};

#[derive(Default, Clone)]
pub struct InMemoryBookRepo(Arc<Mutex<HashMap<Uuid, Book>>>);

#[async_trait]
impl BookRepo for InMemoryBookRepo {
    async fn save_book(&self, book: &Book) -> Uuid {
        self.0.lock().await.insert(book.id, book.clone());
        book.id
    }
    async fn get_books(
        &self,
        name: Option<String>,
        reading: Option<bool>,
        finished: Option<bool>,
    ) -> Vec<BookSummary> {
        let data = self.0.lock().await;
        let mut books: Vec<_> = data.values().collect();

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
    async fn get_book_by_id(&self, id: Uuid) -> Option<Book> {
        self.0.lock().await.get(&id).cloned()
    }
    async fn delete_book(&self, id: Uuid) -> Uuid {
        self.0
            .lock()
            .await
            .remove(&id)
            .map(|book| book.id)
            .unwrap_or(id)
    }
}
