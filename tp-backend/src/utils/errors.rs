use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("Database error")]
    DbError(#[from] sea_orm::DbErr),

    #[error("The requested resource could not be found")]
    NotFound,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        match self {
            APIError::DbError(db_err) => {
                eprintln!("{:?}", db_err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            APIError::NotFound => {
                (StatusCode::NOT_FOUND, format!("{}", self)).into_response()
            },
        }
    }
}
