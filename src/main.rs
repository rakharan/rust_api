// src/main.rs
use axum::{
    routing::{post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;

// 1. Declare the modules
mod handlers;
mod models;

// 2. Use the functions from the handlers module
use handlers::{create_user, get_users};

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://rust:rust@localhost:3306/rust_api")
        .await
        .expect("Failed to connect to MySQL");

    // Table creation omitted for brevity, assumes table exists

    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000...");
    axum::serve(listener, app).await.unwrap();
}