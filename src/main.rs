//MODULES
mod db;
mod handlers;
mod models;
mod routes;
use std::sync::Arc;
//dotenv is used to load environment variables from a .env file
use db::get_db_client;
use routes::get_routes;
use tokio_postgres::Error; //tokio is the async runtime and tokio-postgres is the async postgres driver //std is the standard library //db is a module that contains the get_db_client function

//MAIN FUNCTION
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = get_db_client()
        .await
        .expect("Failed to connect to database");

    let shared_client = Arc::new(client);

    //DEFINE ROUTES
    let app = get_routes(shared_client);

    //START SERVER
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
