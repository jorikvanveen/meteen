use std::sync::Arc;

use axum::{http::HeaderValue, routing::{delete, get, patch, post}, Router};
use color_eyre::eyre::Context;
use tower_http::cors::{Any, CorsLayer};

mod cfg;
mod entity;
mod routes;
mod utils;

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    let config = cfg::load_config().await.wrap_err("Failed to load config")?;
    let db = sea_orm::Database::connect(config.db_path).await.unwrap();

    let state = Arc::new(AppState { db });

    let cors_layer = CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/category", post(routes::create_category))
        .route("/category", get(routes::list_categories))
        .route("/category/{id}/tasks", get(routes::list_category_tasks))
        .route("/category/{id}", delete(routes::delete_category))
        .route("/category/{id}/nuke", delete(routes::nuke_category))
        .route("/task", post(routes::create_task))
        .route("/task/{id}", patch(routes::patch_task))
        .route("/task/{id}", get(routes::get_task))
        .with_state(state)
        .layer(cors_layer);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
