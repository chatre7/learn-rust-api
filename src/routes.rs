use axum::{routing::{get, post}, Router};
use crate::{handlers::book_handlers::{create_book, delete_book, get_book, list_books, update_book}, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/books", post(create_book).get(list_books))
        .route("/books/:id", get(get_book).put(update_book).delete(delete_book))
        .with_state(state)
}
