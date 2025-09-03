use serde::{Deserialize, Serialize};
use time::{OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
}

