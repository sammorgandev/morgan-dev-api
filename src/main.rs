//MODULES
mod db;
mod handlers;
mod models;
mod routes;
use axum::extract::Extension;
use hyper::Method;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

//dotenv is used to load environment variables from a .env file
use axum::Router;
use db::establish_connection;
use routes::{get_misc_routes, get_post_routes, get_user_routes};
use tokio_postgres::Error; //tokio is the async runtime and tokio-postgres is the async postgres driver //std is the standard library //db is a module that contains the get_db_client function

//MAIN FUNCTION
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = establish_connection()
        .await
        .expect("Failed to connect to database");

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let user_client = Arc::new(client);
    let post_client = user_client.clone();
    let misc_client = user_client.clone();
    let layer_client = user_client.clone();

    //DEFINE ROUTES

    let app = Router::new()
        .merge(get_user_routes(user_client))
        .merge(get_post_routes(post_client))
        .merge(get_misc_routes(misc_client))
        .layer(Extension(layer_client))
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
