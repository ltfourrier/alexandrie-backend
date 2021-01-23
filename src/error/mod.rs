use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

mod db;

#[derive(Debug, Display, Error)]
pub enum ApplicationError {
    #[display(fmt = "Database error: {}", "_0")]
    Database(#[error(ignore)] String),
}

#[derive(Serialize)]
pub struct ErrorPayload {
    status: u16,
    message: String,
}

impl error::ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApplicationError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponseBuilder::new(status_code).json(ErrorPayload {
            status: status_code.as_u16(),
            message: self.to_string(),
        })
    }
}
