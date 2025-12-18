// src/handlers.rs
use crate::models::{CreateUser, User, UserResponse}; // Import from sibling module
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::MySqlPool;
use tracing::{error, info, instrument};

#[instrument(skip(pool))]
pub async fn create_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Attempting to create user: {}", payload.email);

    let result = sqlx::query("INSERT INTO users (username, email) VALUES (?, ?)")
        .bind(&payload.username)
        .bind(&payload.email)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_id();
            info!("User created successfully with ID: {}", id);
            let response = UserResponse {
                id,
                username: payload.username,
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get_users(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, username, email FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}
