use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, sqlite::SqlitePool};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::{Book, BookRepo, BookSummary};
#[derive(Clone)]
pub struct SqliteBookRepo(Arc<Mutex<SqlitePool>>);

impl SqliteBookRepo {
    pub async fn new(url: String) -> Self {
        let pool = SqlitePool::connect_lazy(&url).expect("Failed to create SQLite pool");

        let mut conn = pool
            .acquire()
            .await
            .expect("Failed to acquire connection for migration");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS books (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                year INTEGER NOT NULL,
                author TEXT NOT NULL,
                summary TEXT NOT NULL,
                publisher TEXT NOT NULL,
                page_count INTEGER NOT NULL,
                read_page INTEGER NOT NULL,
                reading BOOLEAN NOT NULL,
                finished BOOLEAN NOT NULL,
                updated_at TEXT NOT NULL,
                inserted_at TEXT NOT NULL
            );
            "#,
        )
        .execute(conn.deref_mut())
        .await
        .expect("Failed to create books table");

        SqliteBookRepo(Arc::new(Mutex::new(pool)))
    }
}

#[async_trait]
impl BookRepo for SqliteBookRepo {
    async fn save_book(&self, book: &super::Book) -> uuid::Uuid {
        let pool = self.0.lock().await;
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO books 
            (id, name, year, author, summary, publisher, page_count, read_page, reading, finished, updated_at, inserted_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(book.id.to_string())
        .bind(&book.name)
        .bind(book.year)
        .bind(&book.author)
        .bind(&book.summary)
        .bind(&book.publisher)
        .bind(book.page_count)
        .bind(book.read_page)
        .bind(book.reading)
        .bind(book.finished)
        .bind(book.updated_at.to_rfc3339())
        .bind(book.inserted_at.to_rfc3339())
        .execute(pool.deref())
        .await.expect("Failed to insert or update book");

        book.id
    }

    async fn get_books(
        &self,
        name: Option<String>,
        reading: Option<bool>,
        finished: Option<bool>,
    ) -> Vec<BookSummary> {
        let pool = self.0.lock().await;

        let mut query = "SELECT id, name, publisher FROM books WHERE 1=1".to_string();
        if name.is_some() {
            query.push_str(" AND name LIKE ?");
        }
        if reading.is_some() {
            query.push_str(" AND reading = ?");
        }
        if finished.is_some() {
            query.push_str(" AND finished = ?");
        }

        let mut query_builder = sqlx::query(&query);

        if let Some(name_val) = name {
            query_builder = query_builder.bind(format!("%{}%", name_val));
        }
        if let Some(reading_val) = reading {
            query_builder = query_builder.bind(reading_val);
        }
        if let Some(finished_val) = finished {
            query_builder = query_builder.bind(finished_val);
        }

        let books = query_builder
            .fetch_all(pool.deref())
            .await
            .expect("Failed to fetch books")
            .iter()
            .map(|row| BookSummary {
                id: row.get::<String, _>("id").parse().unwrap(),
                name: row.get("name"),
                publisher: row.get("publisher"),
            })
            .collect();

        books
    }

    async fn get_book_by_id(&self, id: Uuid) -> Option<Book> {
        let pool = self.0.lock().await;
        let book = sqlx::query("SELECT * FROM books WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool.deref())
            .await
            .expect("Failed to fetch book by id")
            .map(|row| Book {
                id: row.get::<String, _>("id").parse().unwrap(),
                name: row.get("name"),
                year: row.get("year"),
                author: row.get("author"),
                summary: row.get("summary"),
                publisher: row.get("publisher"),
                page_count: row.get("page_count"),
                read_page: row.get("read_page"),
                reading: row.get("reading"),
                finished: row.get("finished"),
                updated_at: row
                    .get::<String, _>("updated_at")
                    .parse()
                    .unwrap_or(Utc::now()),
                inserted_at: row
                    .get::<String, _>("inserted_at")
                    .parse()
                    .unwrap_or(Utc::now()),
            });

        book
    }

    async fn delete_book(&self, id: Uuid) -> Uuid {
        let pool = self.0.lock().await;
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id.to_string())
            .execute(pool.deref())
            .await
            .expect("Failed to delete book");

        id
    }
}
