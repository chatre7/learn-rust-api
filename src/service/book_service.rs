use crate::domain::book::{Book, CreateBook, UpdateBook};
use crate::error::{AppError, AppResult};
use crate::repo::BookRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BookService<R: BookRepository + ?Sized> {
    repo: Arc<R>,
}

impl<R: BookRepository + ?Sized> BookService<R> {
    pub fn new(repo: Arc<R>) -> Self { Self { repo } }

    pub async fn create(&self, data: CreateBook) -> AppResult<Book> {
        validate_title(&data.title)?;
        validate_author(&data.author)?;
        self.repo.create(data).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<Book> { self.repo.get(id).await }

    pub async fn list(&self, offset: i64, limit: i64) -> AppResult<Vec<Book>> {
        let limit = limit.clamp(1, 100);
        let offset = offset.max(0);
        self.repo.list(offset, limit).await
    }

    pub async fn update(&self, id: Uuid, data: UpdateBook) -> AppResult<Book> {
        if let Some(title) = &data.title { validate_title(title)?; }
        if let Some(author) = &data.author { validate_author(author)?; }
        self.repo.update(id, data).await
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> { self.repo.delete(id).await }
}

fn validate_title(title: &str) -> AppResult<()> {
    if title.trim().is_empty() { return Err(AppError::Validation("title cannot be empty".into())); }
    if title.len() > 200 { return Err(AppError::Validation("title too long".into())); }
    Ok(())
}

fn validate_author(author: &str) -> AppResult<()> {
    if author.trim().is_empty() { return Err(AppError::Validation("author cannot be empty".into())); }
    if author.len() > 100 { return Err(AppError::Validation("author too long".into())); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use time::OffsetDateTime;

    struct InMemoryRepo { store: Mutex<HashMap<Uuid, Book>> }

    #[async_trait]
    impl BookRepository for InMemoryRepo {
        async fn create(&self, data: CreateBook) -> AppResult<Book> {
            let b = Book {
                id: Uuid::new_v4(),
                title: data.title,
                author: data.author,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            };
            self.store.lock().unwrap().insert(b.id, b.clone());
            Ok(b)
        }
        async fn get(&self, id: Uuid) -> AppResult<Book> {
            self.store.lock().unwrap().get(&id).cloned().ok_or(AppError::NotFound)
        }
        async fn list(&self, _offset: i64, _limit: i64) -> AppResult<Vec<Book>> {
            Ok(self.store.lock().unwrap().values().cloned().collect())
        }
        async fn update(&self, id: Uuid, data: UpdateBook) -> AppResult<Book> {
            let mut g = self.store.lock().unwrap();
            let b = g.get_mut(&id).ok_or(AppError::NotFound)?;
            if let Some(t) = data.title { b.title = t; }
            if let Some(a) = data.author { b.author = a; }
            b.updated_at = OffsetDateTime::now_utc();
            Ok(b.clone())
        }
        async fn delete(&self, id: Uuid) -> AppResult<()> {
            self.store.lock().unwrap().remove(&id).map(|_|()).ok_or(AppError::NotFound)
        }
    }

    fn svc() -> BookService<InMemoryRepo> {
        BookService::new(Arc::new(InMemoryRepo { store: Mutex::new(HashMap::new()) }))
    }

    #[tokio::test]
    async fn create_validates() {
        let s = svc();
        let err = s.create(CreateBook { title: "".into(), author: "A".into() }).await.unwrap_err();
        matches!(err, AppError::Validation(_));
        let ok = s.create(CreateBook { title: "Title".into(), author: "Auth".into() }).await.unwrap();
        assert_eq!(ok.title, "Title");
    }

    #[tokio::test]
    async fn crud_flow() {
        let s = svc();
        let b = s.create(CreateBook { title: "T1".into(), author: "A1".into() }).await.unwrap();
        let fetched = s.get(b.id).await.unwrap();
        assert_eq!(b, fetched);
        let updated = s.update(b.id, UpdateBook { title: Some("T2".into()), author: None }).await.unwrap();
        assert_eq!(updated.title, "T2");
        s.delete(b.id).await.unwrap();
        assert!(matches!(s.get(b.id).await, Err(AppError::NotFound)));
    }
}
