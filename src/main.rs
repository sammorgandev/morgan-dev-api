//MODULES
mod db;
mod handlers;
mod models;

//IMPORTS
use axum::{
    //axum is the http server framework
    routing::get,
    Router,
};
use handlers::get_all_users;
//dotenv is used to load environment variables from a .env file
use db::get_db_client;
use tokio_postgres::Error; //tokio is the async runtime and tokio-postgres is the async postgres driver //std is the standard library //db is a module that contains the get_db_client function

//MAIN FUNCTION
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = get_db_client().await?;

    let create_user_table_sql = "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        )";

    client.execute(create_user_table_sql, &[]).await?;

    let create_user = client
        .prepare("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)")
        .await?;

    client
        .execute(
            &create_user,
            &[&"Dori Morgan", &"dori@morgan.dev", &"password"],
        )
        .await?;

    //DEFINE ROUTES
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/hello/:name",
            get(|params: axum::extract::Path<(String,)>| async move {
                format!("Hello, {}", (params.0).0)
            }),
        )
        .route("/plain", get(handlers::_return_plain_text))
        .route("/users", get(get_all_users(&client).await?));

    //START SERVER
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
