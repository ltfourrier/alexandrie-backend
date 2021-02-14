use super::app_state;
use crate::routes;

use actix_web::{test, App};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetHealthResponse {
    status: String,
}

#[actix_rt::test]
async fn test_get_health() {
    let mut app =
        test::init_service(App::new().data(app_state().await).configure(routes::v1)).await;

    let req = test::TestRequest::get().uri("/v1/health").to_request();
    let resp: GetHealthResponse = test::read_response_json(&mut app, req).await;

    assert_eq!(resp.status, "OK");
}
