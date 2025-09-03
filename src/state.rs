use std::sync::Arc;
use crate::{repo::BookRepository, service::book_service::BookService};

#[derive(Clone)]
pub struct AppState {
    pub book_service: Arc<BookService<dyn BookRepository>>,
}
