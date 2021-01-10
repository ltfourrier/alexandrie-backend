use sqlx::{query, PgPool, Result};

pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Create a new user in the database.
///
/// # Arguments
///
/// * `pool`: The connection pool to the database.
/// * `create_user`: The structure containing the data for the user to create.
///
/// # Return value
///
/// This function returns the primary key of the created user.
pub async fn create_user(pool: &PgPool, create_user: CreateUser) -> Result<i64> {
    let created_user = query!(
        r#"
        INSERT INTO users ( username, email, first_name, last_name )
        VALUES ( $1, $2, $3, $4 )
        RETURNING id
        "#,
        create_user.username,
        create_user.email,
        create_user.first_name,
        create_user.last_name
    )
    .fetch_one(pool)
    .await?;

    Ok(created_user.id)
}
