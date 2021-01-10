use std::str::FromStr;

use actix_web::{App, HttpServer, web};
use anyhow::Result;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPoolOptions;

use configuration::read_configuration;
use routes::*;

mod configuration;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = read_configuration()?;

    SimpleLogger::new()
        .with_level(LevelFilter::from_str(config.log.level.as_str())?)
        .init()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                config.database.username,
                config.database.password,
                config.database.host,
                config.database.port,
                config.database.database
            )
                .as_str(),
        )
        .await?;

    sqlx::migrate!().run(&db_pool).await?;

    HttpServer::new(|| App::new().service(web::scope("/v1").service(get_health)))
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
