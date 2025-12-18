use crate::errors::AppError;
use crate::models::{CreateUser, User, UserResponse};
use crate::repositories::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, payload: CreateUser) -> Result<UserResponse, AppError> {
        // Future logic (e.g., validation) goes here

        let id = self.repo.create(&payload).await?;

        Ok(UserResponse {
            id,
            username: payload.username,
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repo.find_all().await
    }
}
