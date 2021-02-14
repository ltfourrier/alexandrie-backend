use super::app_state;
use crate::routes;

use actix_web::http::StatusCode;
use actix_web::{test, App};
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::name::en::{FirstName, LastName};
use fake::{Dummy, Fake, Faker};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Dummy, Serialize)]
struct CreateUserRequest {
    #[dummy(faker = "Username()")]
    username: String,
    #[dummy(faker = "SafeEmail()")]
    email: String,
    #[dummy(faker = "FirstName()")]
    first_name: String,
    #[dummy(faker = "LastName()")]
    last_name: String,
}

#[actix_rt::test]
async fn test_create_user() {
    let state = app_state().await;
    let mut app = test::init_service(App::new().data(state.clone()).configure(routes::v1)).await;

    let create_user_req: CreateUserRequest = Faker.fake();
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(&create_user_req)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Location").unwrap().to_str().unwrap(),
        format!("/v1/users/{}", create_user_req.username)
    );

    drop_user(&state.db, create_user_req.username).await;
}

async fn drop_user(db: &PgPool, username: String) {
    sqlx::query("DELETE FROM users WHERE username = $1")
        .bind(username)
        .execute(db)
        .await
        .unwrap();
}
