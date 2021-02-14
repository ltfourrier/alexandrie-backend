use crate::cfg::Configuration;

use actix_web::{App, HttpServer};
use anyhow::Result;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use sqlx::PgPool;
use std::str::FromStr;

mod cfg;
mod db;
mod error;
mod routes;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let config = cfg::read_configuration()?;
    let state = init_app_state(&config).await?;

    HttpServer::new(move || App::new().data(state.clone()).configure(routes::v1))
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

async fn init_app_state(cfg: &Configuration) -> Result<AppState> {
    SimpleLogger::new()
        .with_level(LevelFilter::from_str(cfg.log.level.as_str())?)
        .init()?;

    Ok(AppState {
        db: db::init_db(&cfg.database).await?,
    })
}

#[cfg(test)]
mod tests;
