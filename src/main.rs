//IMPORTS
use axum::{
    //axum is the http server framework
    body::Bytes,
    extract::{Json, Path, Query},
    http::StatusCode,
    routing::get,
    Router,
};
use dotenv::dotenv; //dotenv is used to load environment variables from a .env file
use serde_json::{json, Value}; //serde_json is used to serialize and deserialize JSON
use std::{collections::HashMap, env::var, format};
use tokio_postgres::{connect, Error, NoTls}; //tokio is the async runtime and tokio-postgres is the async postgres driver //std is the standard library

//UTILITY & HANDLER FUNCTIONS

//UTILITIES
async fn _path(Path(_user_id): Path<u32>) {} //"Path" give you the path parameters and deserializes them into the type you specify
async fn _query(Query(_params): Query<HashMap<String, String>>) {} //"Query" gives you the query parameters and deserializes them into the type you specify
async fn _json(Json(_payload): Json<serde_json::Value>) {} //buffer the request body and deserialize it as JSON into a serde_json::Value. 'Json' supports any type that implements Deserialize

//HANDLERS
// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn _return_plain_text() -> &'static str {
    "foo"
}

// `Json` gives a content-type of `application/json` and works with any type that implements `serde::Serialize`
async fn _return_json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn _echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    //LOAD ENVIRONMENT VARIABLES
    dotenv().ok();

    //CONSTRUCT DATABASE URL
    let db_user = var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let db_hostname = var("POSTGRES_HOSTNAME").expect("POSTGRES_HOSTNAME must be set");
    let db_port = var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_hostname, db_port, db_name
    );

    //CONNECT TO DATABASE
    let (client, connection) = connect(&database_url, NoTls).await?;

    //Spawn a new task that runs the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statment that just returns its parameter
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    //DEFINE ROUTES
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/hello/:name",
            get(|params: axum::extract::Path<(String,)>| async move {
                format!("Hello, {}", (params.0).0)
            }),
        )
        .route("/plain", get(_return_plain_text))
        .route("/json", get(_return_json));

    //START SERVER
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
