use crate::cfg::DatabaseConfiguration;

use log::info;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Result};

pub mod users;

static MIGRATOR: Migrator = sqlx::migrate!();

/// Initialize the connection to the database using the given configuration. This function also
/// initializes the database itself if necessary, by running migrations for example.
///
/// # Arguments
///
/// * `config`: The configuration to use to connect and configuration the database.
pub async fn init_db(cfg: &DatabaseConfiguration) -> Result<PgPool> {
    let db_pool = PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .connect(
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                cfg.username, cfg.password, cfg.host, cfg.port, cfg.database
            )
            .as_str(),
        )
        .await?;

    info!("Running migrations...");
    MIGRATOR.run(&db_pool).await?;
    info!("Migrations ran successfully!");

    return Ok(db_pool);
}
