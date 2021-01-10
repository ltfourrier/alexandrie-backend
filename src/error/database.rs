use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};
use sqlx::Error;

use crate::error::to_http_response;

#[derive(Debug, Display, Error)]
pub enum DatabaseError {
    #[display(fmt = "Unknown database error: {}", "_0")]
    UnknownError(#[error(ignore)] String),
}

impl error::ResponseError for DatabaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            DatabaseError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        to_http_response(self)
    }
}

impl From<Error> for DatabaseError {
    fn from(err: Error) -> Self {
        DatabaseError::UnknownError(err.to_string())
    }
}
