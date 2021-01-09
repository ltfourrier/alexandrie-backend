mod routes;

use routes::*;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<()> {
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgresql://alexandrie:password@localhost/alexandrie")
        .await?;

    sqlx::migrate!().run(&db_pool).await?;

    HttpServer::new(|| App::new().service(web::scope("/v1").service(get_health)))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
