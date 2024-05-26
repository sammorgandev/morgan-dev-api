mod db;
mod handlers;
mod models;
mod routes;
mod services;

use axum::{
    extract::Extension,
    http::{HeaderName, HeaderValue},
    Router,
};
use db::establish_connection;
use hyper::Method;
use reqwest::Client as HttpClient;
use routes::{get_misc_routes, get_post_routes, get_service_routes, get_user_routes};
use std::sync::Arc;
use tokio_postgres::{Client as DbClient, Error};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

//MAIN FUNCTION
#[tokio::main]
async fn main() -> Result<(), Error> {
    let postgres_client: DbClient = establish_connection()
        .await
        .expect("Failed to connect to database");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_origin(Any);

    let db_client = Arc::new(postgres_client);
    let http_client = Arc::new(HttpClient::new());

    //DEFINE ROUTES

    let app = Router::new()
        .merge(get_user_routes(db_client.clone()))
        .merge(get_post_routes(db_client.clone()))
        .merge(get_misc_routes(db_client.clone()))
        .merge(get_service_routes(http_client.clone()))
        .layer(Extension(db_client))
        .layer(Extension(http_client))
        .layer(cors);

    //START SERVER
    let api_port = std::env::var("API_PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", api_port))
        .await
        .unwrap();

    println!("Server running on port {}", api_port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
