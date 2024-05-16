//MODULES
mod db;
mod handlers;
mod models;
mod routes;

use axum::Router;
use std::sync::Arc;
//dotenv is used to load environment variables from a .env file
use db::establish_connection;
use routes::{get_post_routes, get_user_routes};
use tokio_postgres::Error; //tokio is the async runtime and tokio-postgres is the async postgres driver //std is the standard library //db is a module that contains the get_db_client function

//MAIN FUNCTION
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = establish_connection()
        .await
        .expect("Failed to connect to database");

    let shared_client = Arc::new(client);

    //DEFINE ROUTES
    let user_routes = get_user_routes(shared_client.clone());
    let post_routes = get_post_routes(shared_client.clone());

    let app = Router::new().merge(user_routes).merge(post_routes);

    //START SERVER
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
