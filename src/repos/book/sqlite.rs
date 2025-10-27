use async_trait::async_trait;
use sqlx::{Row, sqlite::SqlitePool};
use std::ops::DerefMut;
use uuid::Uuid;

use crate::AppError;

use super::{Book, BookRepo, BookSummary};
#[derive(Clone)]
pub struct SqliteBookRepo(SqlitePool);

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
        .expect("Failed to initialize database");

        SqliteBookRepo(pool)
    }
}

#[async_trait]
impl BookRepo for SqliteBookRepo {
    async fn save_book(&self, book: &super::Book) -> Result<Uuid, AppError> {
        let pool = &self.0;
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
        .execute(pool)
        .await.map_err(|_e| AppError::DatabaseError)?;

        Ok(book.id)
    }

    async fn get_books(
        &self,
        name: Option<String>,
        reading: Option<bool>,
        finished: Option<bool>,
    ) -> Result<Vec<BookSummary>, AppError> {
        let pool = &self.0;

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
            .fetch_all(pool)
            .await
            .map_err(|_e| AppError::DatabaseError)?
            .iter()
            .map(|row| {
                // Note the braces and Ok()
                Ok(BookSummary {
                    id: row
                        .get::<String, _>("id")
                        .parse()
                        .map_err(|_e| AppError::DatabaseError)?,
                    name: row.get("name"),
                    publisher: row.get("publisher"),
                })
            })
            .collect::<Result<Vec<BookSummary>, AppError>>()?;

        Ok(books)
    }

    async fn get_book_by_id(&self, id: Uuid) -> Result<Option<Book>, AppError> {
        let pool = &self.0;
        let book: Option<Book> = sqlx::query_as(
            r#"
            SELECT id, name, year, author, summary, publisher, page_count, read_page, reading, finished, updated_at, inserted_at
            FROM books WHERE id = ?
            "#,
        ).bind(id.to_string()).fetch_optional(pool)
            .await
            .map_err(|_e| AppError::DatabaseError)?;

        Ok(book)
    }

    async fn delete_book(&self, id: Uuid) -> Result<Uuid, AppError> {
        let pool = &self.0;
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(|_e| AppError::DatabaseError)?;

        Ok(id)
    }
}
