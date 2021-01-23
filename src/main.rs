use actix_web::{web, App, HttpServer};
use anyhow::Result;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::str::FromStr;

mod cfg;
mod db;
mod error;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = cfg::read_configuration()?;

    SimpleLogger::new()
        .with_level(LevelFilter::from_str(config.log.level.as_str())?)
        .init()?;

    let db_pool = db::init_db(&config.database).await?;

    HttpServer::new(move || {
        App::new().data(db_pool.clone()).service(
            web::scope("/v1")
                .service(routes::health::get_health)
                .configure(routes::users::init),
        )
    })
    .bind(
        format!(
            "{}:{}",
            config.server.listen_address, config.server.listen_port
        )
        .as_str(),
    )?
    .run()
    .await?;

    Ok(())
}
