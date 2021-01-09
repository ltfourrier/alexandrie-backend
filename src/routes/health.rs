use actix_web::{get, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Health {
    status: &'static str,
}

#[get("/health")]
pub async fn get_health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Health {
        status: "OK",
    }))
}