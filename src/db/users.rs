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

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::{SafeEmail, Username};
    use fake::faker::name::en::{FirstName, LastName};
    use fake::Fake;

    use crate::db::tests::init_test_db;

    use super::*;

    #[actix_rt::test]
    async fn test_create_user() {
        let db_pool = init_test_db().await.unwrap();

        let username: String = Username().fake();
        let email: String = SafeEmail().fake();
        let first_name: String = FirstName().fake();
        let last_name: String = LastName().fake();

        let user_id = create_user(
            &db_pool,
            CreateUser {
                username: username.clone(),
                email: email.clone(),
                first_name: Some(first_name.clone()),
                last_name: Some(last_name.clone()),
            },
        )
        .await
        .unwrap();

        let user = query!(
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&db_pool)
        .await
        .unwrap();

        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert_eq!(user.first_name, Some(first_name));
        assert_eq!(user.last_name, Some(last_name));
    }
}
