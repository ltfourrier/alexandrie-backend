use actix_web::dev::HttpResponseBuilder;
use actix_web::{error, HttpResponse};
use serde::Serialize;

pub use database::*;

mod database;

#[derive(Serialize)]
pub struct ErrorPayload {
    status: u16,
    message: String,
}

/// Translate a given error response into a HTTP response.
///
/// # Arguments
///
/// * `err`: The error response to translate.
///
/// # Return value
///
/// This function returns the translated error response.
pub fn to_http_response(err: &impl error::ResponseError) -> HttpResponse {
    let status_code = err.status_code();
    HttpResponseBuilder::new(status_code).json(ErrorPayload {
        status: status_code.as_u16(),
        message: err.to_string(),
    })
}
