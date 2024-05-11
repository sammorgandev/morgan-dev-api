//IMPORTS
use axum::{
    //axum is the http server framework
    body::Bytes,
    extract::{Json, Path, Query},
    http::StatusCode,
    routing::get,
    Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
//dotenv is used to load environment variables from a .env file
use serde_json::{json, Value}; //serde_json is used to serialize and deserialize JSON
use std::{collections::HashMap, env::var, format};
use tokio_postgres::{connect, Client, Error, NoTls}; //tokio is the async runtime and tokio-postgres is the async postgres driver //std is the standard library


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
// This function buffers the request body and returns it. If the body is not valid UTF-8, it returns a `400 Bad Request`.
// `Bytes` gives you the raw request body as a `Bytes` instance - this works because 'Bytes' implements FromRequest and therefore can be used as an extractor.
// 'String' and 'StatusCode' both implement 'IntoResponse' and therefore can be used as a response.
async fn _echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}

impl From<&tokio_postgres::Row> for User {
    fn from(row: &tokio_postgres::Row) -> Self {
        User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.get("password"),
        }
    }
}

//MAIN FUNCTION

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
            &[&"Sam Morgan", &"sam@morgan.dev", &"password"],
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
        .route("/plain", get(_return_plain_text))
        .route("/users", get(get_all_users(&client).await?));

    //START SERVER
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

//CUSTOM HANDLERS
async fn get_all_users(client: &Client) -> Result<Json<Value>, tokio_postgres::Error> {
    let rows = client.query("SELECT * FROM users", &[]).await?;
    let users: Vec<User> = rows.iter().map(User::from).collect(); // assuming you have a User struct and a from function to convert a row to a User
    Ok(Json(json!(users)))
}
