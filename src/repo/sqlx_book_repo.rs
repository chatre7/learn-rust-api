use super::BookRepository;
use crate::domain::book::{Book, CreateBook, UpdateBook};
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqlxBookRepo {
    pool: PgPool,
}

impl SqlxBookRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

fn map_row_to_book(row: &sqlx::postgres::PgRow) -> Book {
    Book {
        id: row.get("id"),
        title: row.get("title"),
        author: row.get("author"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[async_trait]
impl BookRepository for SqlxBookRepo {
    async fn create(&self, data: CreateBook) -> AppResult<Book> {
        let rec = sqlx::query(
            r#"INSERT INTO books (id, title, author, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING id, title, author, created_at, updated_at"#,
        )
        .bind(Uuid::new_v4())
        .bind(data.title)
        .bind(data.author)
        .bind(OffsetDateTime::now_utc())
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await?;

        Ok(map_row_to_book(&rec))
    }

    async fn get(&self, id: Uuid) -> AppResult<Book> {
        let rec = sqlx::query(
            r#"SELECT id, title, author, created_at, updated_at
               FROM books WHERE id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(map_row_to_book(&rec))
    }

    async fn list(&self, offset: i64, limit: i64) -> AppResult<Vec<Book>> {
        let rows = sqlx::query(
            r#"SELECT id, title, author, created_at, updated_at
               FROM books
               ORDER BY created_at DESC
               OFFSET $1 LIMIT $2"#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(map_row_to_book).collect())
    }

    async fn update(&self, id: Uuid, data: UpdateBook) -> AppResult<Book> {
        // Fetch existing
        let existing = self.get(id).await?;
        let new_title = data.title.unwrap_or(existing.title);
        let new_author = data.author.unwrap_or(existing.author);

        let rec = sqlx::query(
            r#"UPDATE books SET title = $2, author = $3, updated_at = $4
               WHERE id = $1
               RETURNING id, title, author, created_at, updated_at"#,
        )
        .bind(id)
        .bind(new_title)
        .bind(new_author)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await?;

        Ok(map_row_to_book(&rec))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM books WHERE id = $1").bind(id).execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}

