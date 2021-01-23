use sqlx::{query, PgPool, Result};

pub struct UserCreation {
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub salt: [u8; 16],
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
pub async fn create_user(pool: &PgPool, user: &UserCreation) -> Result<i64> {
    let created_user = query!(
        r#"
        INSERT INTO users ( username, email, first_name, last_name, salt )
        VALUES ( $1, $2, $3, $4, $5 )
        RETURNING id
        "#,
        user.username,
        user.email,
        user.first_name,
        user.last_name,
        &user.salt as &[u8]
    )
    .fetch_one(pool)
    .await?;

    Ok(created_user.id)
}
