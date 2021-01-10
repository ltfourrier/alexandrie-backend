use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Result};

use crate::configuration::DatabaseConfiguration;

pub mod users;

/// Initialize the connection to the database using the given configuration. This function also
/// initializes the database itself if necessary, by running migrations for example.
///
/// # Arguments
///
/// * `config`: The configuration to use to connect and configuration the database.
pub async fn init_db(db_config: &DatabaseConfiguration) -> Result<Pool<Postgres>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .connect(
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                db_config.username,
                db_config.password,
                db_config.host,
                db_config.port,
                db_config.database
            )
                .as_str(),
        )
        .await?;

    sqlx::migrate!().run(&db_pool).await?;

    return Ok(db_pool);
}
