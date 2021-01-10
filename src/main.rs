use std::str::FromStr;

use actix_web::{App, HttpServer, web};
use anyhow::Result;
use log::LevelFilter;
use simple_logger::SimpleLogger;

use configuration::read_configuration;
use routes::*;

mod configuration;
mod db;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = read_configuration()?;

    SimpleLogger::new()
        .with_level(LevelFilter::from_str(config.log.level.as_str())?)
        .init()?;

    db::init_db(&config.database).await?;

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
