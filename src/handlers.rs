// src/handlers.rs
use crate::errors::AppError;
use crate::models::{CreateUser, User, UserResponse}; // Import from sibling module
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::MySqlPool;
use tracing::{info, instrument};

#[instrument(skip(pool))]
pub async fn create_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    info!("Attempting to create user: {}", payload.email);

    let result = sqlx::query("INSERT INTO users (username, email) VALUES (?, ?)")
        .bind(&payload.username)
        .bind(&payload.email)
        .execute(&pool)
        .await?;

    let id = result.last_insert_id();

    info!("User created successfully with ID: {}", id);

    let response = UserResponse {
        id,
        username: payload.username,
    };
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_users(State(pool): State<MySqlPool>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT id, username, email FROM users")
        .fetch_all(&pool)
        .await?;

    Ok(Json(users))
}
