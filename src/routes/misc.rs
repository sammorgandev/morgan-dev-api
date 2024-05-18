use axum::{routing::get, Extension, Router};
use std::sync::Arc;
use tokio_postgres::Client;

async fn root() -> &'static str {
    "Hello! You've reached the root directory of api.morgan.dev. This is a private api. Go away."
}

async fn health_check() -> &'static str {
    "Healthy"
}

pub fn get_misc_routes(client: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .layer(Extension(client))
}
