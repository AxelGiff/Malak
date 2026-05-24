use axum::{routing::get, routing::post, Router};
mod db;
use dotenvy::{dotenv, from_filename};
use std::env;
use db::connect;
mod insertPayload;
mod models;

fn build_database_url() -> String {
    if let Ok(url) = env::var("DATABASE_URL") {
        return url;
    }

    let _app_name = env::var("APPLICATION_NAME").unwrap();
    let host = env::var("DB_HOST").unwrap();
    let port = env::var("DB_PORT").unwrap();
    let db_name = env::var("DB_NAME").unwrap();
    let db_password = env::var("DB_PASSWORD").unwrap();
    let db_user = env::var("DB_USER").unwrap_or("malak_user".to_string());

    format!("postgres://{}:{}@{}:{}/{}", db_user, db_password, host, port, db_name)
}

#[tokio::main]
async fn main() {
    from_filename("../.env").ok();
    dotenv().ok();

    let database_url = build_database_url();
    let pool = connect(&database_url).await;
    let state = insertPayload::AppState { db: pool };


    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/metrics", post(insertPayload::insert_to_db))
    .route("/metrics/latest", get(models::get_latest_metrics))
    .route("/machines", get(models::get_machines))
    .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}