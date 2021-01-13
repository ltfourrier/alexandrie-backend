use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Result};

use crate::configuration::DatabaseConfiguration;

pub mod users;

static MIGRATOR: Migrator = sqlx::migrate!();

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

    MIGRATOR.run(&db_pool).await?;

    return Ok(db_pool);
}

#[cfg(test)]
mod tests {
    use sqlx::{Pool, Postgres, Result};

    use crate::configuration::DatabaseConfiguration;
    use crate::db::init_db;

    /// Init the database with the test configuration, running against the development stack.
    pub async fn init_test_db() -> Result<Pool<Postgres>> {
        init_db(&DatabaseConfiguration {
            username: String::from("alexandrie"),
            password: String::from("password"),
            host: String::from("localhost"),
            port: 5432,
            database: String::from("alexandrie"),
            max_connections: 1,
        })
        .await
    }
}
