use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::AppError;

use super::{Book, BookRepo, BookSummary};

#[derive(Default, Clone)]
pub struct InMemoryBookRepo(Arc<Mutex<HashMap<Uuid, Book>>>);

#[async_trait]
impl BookRepo for InMemoryBookRepo {
    async fn save_book(&self, book: &Book) -> Result<Uuid, AppError> {
        self.0.lock().await.insert(book.id, book.clone());
        Ok(book.id)
    }
    async fn get_books(
        &self,
        name: Option<String>,
        reading: Option<bool>,
        finished: Option<bool>,
    ) -> Result<Vec<BookSummary>, AppError> {
        let all_books: Vec<Book> = self.0.lock().await.values().cloned().collect();

        let books = all_books
            .into_iter()
            .filter(|book| {
                name.as_ref().map_or(true, |name_filter| {
                    book.name
                        .to_lowercase()
                        .contains(&name_filter.to_lowercase())
                })
            })
            .filter(|book| reading.map_or(true, |reading_filter| book.reading == reading_filter))
            .filter(|book| {
                finished.map_or(true, |finished_filter| book.finished == finished_filter)
            });

        Ok(books
            .map(|book| BookSummary {
                id: book.id,
                name: book.name.clone(),
                publisher: book.publisher.clone(),
            })
            .collect())
    }
    async fn get_book_by_id(&self, id: Uuid) -> Result<Option<Book>, AppError> {
        Ok(self.0.lock().await.get(&id).cloned())
    }
    async fn delete_book(&self, id: Uuid) -> Result<Uuid, AppError> {
        self.0.lock().await.remove(&id);
        Ok(id)
    }
}
