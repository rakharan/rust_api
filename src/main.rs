use axum::{Router, routing::post};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

// 1. Declare the modules
mod handlers;
mod models;

// 2. Use the functions from the handlers module
use handlers::{create_user, get_users};

// Logging imports
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Initialize Tracing
    // This reads the RUST_LOG environment variable to decide what to log
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let database_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let database_user = env::var("DB_USER").unwrap_or_else(|_| "rust".to_string());
    let database_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "rust".to_string());
    let database_name = env::var("DB_NAME").unwrap_or_else(|_| "rust_api".to_string());
    let database_port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "mysql://{}:{}@{}:{}/{}",
            database_user, database_password, database_host, database_port, database_name
        ))
        .await
        .expect("Failed to connect to MySQL");

    println!("Connected to MySQL database.");


    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
