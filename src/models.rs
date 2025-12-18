use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}