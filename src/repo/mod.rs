pub mod sqlx_book_repo;

use crate::domain::book::{Book, CreateBook, UpdateBook};
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BookRepository: Send + Sync + 'static {
    async fn create(&self, data: CreateBook) -> AppResult<Book>;
    async fn get(&self, id: Uuid) -> AppResult<Book>;
    async fn list(&self, offset: i64, limit: i64) -> AppResult<Vec<Book>>;
    async fn update(&self, id: Uuid, data: UpdateBook) -> AppResult<Book>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

