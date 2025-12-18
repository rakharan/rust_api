use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::models::CreateUser;
use crate::errors::AppError;
use crate::repositories::UserRepository;
use crate::services::UserService;
use crate::infra::{PrimaryDb, ReplicaDb};

pub async fn create_user(
    State(PrimaryDb(pool)): State<PrimaryDb>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    
    // dependency injection
    let repo = UserRepository::new(pool);
    let service = UserService::new(repo);

    let response = service.create_user(payload).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_users(
    State(ReplicaDb(pool)): State<ReplicaDb>
) -> Result<impl IntoResponse, AppError> {
    let repo = UserRepository::new(pool);
    let service = UserService::new(repo);

    let users = service.get_all_users().await?;

    Ok(Json(users))
}