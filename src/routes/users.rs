use crate::db::users as db;
use crate::error::ApplicationError;
use crate::{utils, AppState};

use actix_web::{post, web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct PostUsers {
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[post("/users")]
async fn post_users(
    req: web::HttpRequest,
    app_state: web::Data<AppState>,
    user: web::Json<PostUsers>,
) -> Result<HttpResponse> {
    let user = user.into_inner();
    let ref db_pool = app_state.db;

    let user_creation = db::UserCreation {
        username: user.username,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        salt: utils::crypto::generate_salt(),
    };

    db::create_user(db_pool, &user_creation)
        .await
        .map_err(|err| -> ApplicationError { err.into() })?;

    Ok(HttpResponse::Created()
        .header(
            "Location",
            format!("{}/{}", req.path(), &user_creation.username),
        )
        .finish())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_users);
}
