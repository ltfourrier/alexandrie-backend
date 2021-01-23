use super::ApplicationError;

impl From<sqlx::Error> for ApplicationError {
    fn from(err: sqlx::Error) -> Self {
        ApplicationError::Database(err.to_string())
    }
}
