use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found")] 
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Db(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Db(other.to_string()),
        }
    }
}

#[derive(Serialize)]
struct ErrorBody { message: String }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Db(_) => StatusCode::BAD_GATEWAY,
        };
        let body = Json(ErrorBody { message: self.to_string() });
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
