use actix_web::{post, web, HttpResponse, Result};
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::users as db;
use crate::error::DatabaseError;

#[derive(Deserialize)]
struct PostUsers {
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[post("/users")]
async fn post_users(
    db_pool: web::Data<PgPool>,
    user: web::Json<PostUsers>,
) -> Result<HttpResponse> {
    let user = user.into_inner();

    db::create_user(
        db_pool.get_ref(),
        db::CreateUser {
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        },
    )
    .await
    .map_err(|err| -> DatabaseError { err.into() })?;

    Ok(HttpResponse::Created().finish())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_users);
}
