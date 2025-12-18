use sqlx::MySqlPool;
use crate::models::{User, CreateUser};
use crate::errors::AppError;

// This struct "holds" the connection
#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    // Constructor
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, payload: &CreateUser) -> Result<u64, AppError> {
        let result = sqlx::query("INSERT INTO users (username, email) VALUES (?, ?)")
            .bind(&payload.username)
            .bind(&payload.email)
            .execute(&self.pool)
            .await?; // The ? automatically converts sqlx::Error to AppError

        Ok(result.last_insert_id())
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>("SELECT id, username, email FROM users")
            .fetch_all(&self.pool)
            .await?;
            
        Ok(users)
    }
}