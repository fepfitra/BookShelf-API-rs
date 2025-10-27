use std::sync::{Arc, Mutex};

use sqlx::{query, sqlite::SqlitePool};

use super::BookRepo;
#[derive(Clone)]
pub struct SqliteBookRepo(Arc<Mutex<SqlitePool>>);

impl SqliteBookRepo {
    pub async fn new(url: String) -> Self {
        let pool = SqlitePool::connect_lazy(&url).expect("Failed to create SQLite pool");
        SqliteBookRepo(Arc::new(Mutex::new(pool)))
    }
}

impl BookRepo for SqliteBookRepo {}
