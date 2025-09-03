use axum::{extract::{Path, Query, State}, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::book::{Book, CreateBook, UpdateBook}, error::AppResult, state::AppState};

#[derive(Deserialize)]
pub struct ListParams { pub offset: Option<i64>, pub limit: Option<i64> }

pub async fn create_book(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> AppResult<Json<Book>> {
    let book = state.book_service.create(payload).await?;
    Ok(Json(book))
}

pub async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Book>> {
    let book = state.book_service.get(id).await?;
    Ok(Json(book))
}

pub async fn list_books(
    State(state): State<AppState>,
    Query(q): Query<ListParams>,
) -> AppResult<Json<Vec<Book>>> {
    let books = state.book_service.list(q.offset.unwrap_or(0), q.limit.unwrap_or(20)).await?;
    Ok(Json(books))
}

pub async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBook>,
) -> AppResult<Json<Book>> {
    let book = state.book_service.update(id, payload).await?;
    Ok(Json(book))
}

pub async fn delete_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    state.book_service.delete(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::{Body, to_bytes}, http::{Request, StatusCode}, routing::{get, post}, Router};
    use serde_json::json;
    use std::{collections::HashMap, sync::{Arc, Mutex}};
    use time::OffsetDateTime;
    use tower::ServiceExt;
    use crate::domain::book::Book;
    use crate::error::AppError;

    struct InMemoryRepo { store: Mutex<HashMap<Uuid, Book>> }
    #[axum::async_trait]
    impl crate::repo::BookRepository for InMemoryRepo {
        async fn create(&self, data: CreateBook) -> crate::error::AppResult<Book> {
            let b = Book { id: Uuid::new_v4(), title: data.title, author: data.author, created_at: OffsetDateTime::now_utc(), updated_at: OffsetDateTime::now_utc() };
            self.store.lock().unwrap().insert(b.id, b.clone());
            Ok(b)
        }
        async fn get(&self, id: Uuid) -> crate::error::AppResult<Book> { self.store.lock().unwrap().get(&id).cloned().ok_or(AppError::NotFound) }
        async fn list(&self, _o: i64, _l: i64) -> crate::error::AppResult<Vec<Book>> { Ok(self.store.lock().unwrap().values().cloned().collect()) }
        async fn update(&self, id: Uuid, data: UpdateBook) -> crate::error::AppResult<Book> {
            let mut g = self.store.lock().unwrap();
            let b = g.get_mut(&id).ok_or(AppError::NotFound)?;
            if let Some(t) = data.title { b.title = t; }
            if let Some(a) = data.author { b.author = a; }
            Ok(b.clone())
        }
        async fn delete(&self, id: Uuid) -> crate::error::AppResult<()> { self.store.lock().unwrap().remove(&id).map(|_|()).ok_or(AppError::NotFound) }
    }

    fn app() -> Router {
        let repo: Arc<dyn crate::repo::BookRepository> = Arc::new(InMemoryRepo { store: Mutex::new(HashMap::new()) });
        let svc = crate::service::book_service::BookService::<dyn crate::repo::BookRepository>::new(repo);
        let state = crate::state::AppState { book_service: Arc::new(svc) };
        Router::new()
            .route("/books", post(super::create_book).get(super::list_books))
            .route("/books/:id", get(super::get_book).put(super::update_book).delete(super::delete_book))
            .with_state(state)
    }

    #[tokio::test]
    async fn handlers_crud() {
        let app = app();
        // create
        let res = app.clone().oneshot(Request::post("/books").header("content-type", "application/json").body(Body::from(serde_json::to_vec(&json!({"title":"T","author":"A"})).unwrap())).unwrap()).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let b: Book = serde_json::from_slice(&body).unwrap();

        // get
        let res = app.clone().oneshot(Request::get(format!("/books/{}", b.id)).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // update
        let res = app.clone().oneshot(Request::put(format!("/books/{}", b.id)).header("content-type","application/json").body(Body::from(serde_json::to_vec(&json!({"title":"T2"})).unwrap())).unwrap()).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let b2: Book = serde_json::from_slice(&body).unwrap();
        assert_eq!(b2.title, "T2");

        // list
        let res = app.clone().oneshot(Request::get("/books").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // delete
        let res = app.clone().oneshot(Request::delete(format!("/books/{}", b.id)).body(Body::empty()).unwrap()).await.unwrap();
        assert!(res.status().is_success());
    }
}
