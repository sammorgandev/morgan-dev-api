use axum::{
    //axum is the http server framework
    body::Bytes,
    extract::{Json, Path, Query},
    http::StatusCode,
};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::models::User;
use tokio_postgres::Client;
// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
pub async fn _return_plain_text() -> &'static str {
    "foo"
}

// `Json` gives a content-type of `application/json` and works with any type that implements `serde::Serialize`
pub async fn _return_json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
// This function buffers the request body and returns it. If the body is not valid UTF-8, it returns a `400 Bad Request`.
// `Bytes` gives you the raw request body as a `Bytes` instance - this works because 'Bytes' implements FromRequest and therefore can be used as an extractor.
// 'String' and 'StatusCode' both implement 'IntoResponse' and therefore can be used as a response.
pub async fn _echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

//UTILITIES
pub async fn _path(Path(_user_id): Path<u32>) {} //"Path" give you the path parameters and deserializes them into the type you specify
pub async fn _query(Query(_params): Query<HashMap<String, String>>) {} //"Query" gives you the query parameters and deserializes them into the type you specify
pub async fn _json(Json(_payload): Json<serde_json::Value>) {} //buffer the request body and deserialize it as JSON into a serde_json::Value. 'Json' supports any type that implements Deserialize

//CUSTOM HANDLERS
pub async fn get_all_users(client: &Client) -> Result<Json<Value>, tokio_postgres::Error> {
    let rows = client.query("SELECT * FROM users", &[]).await?;
    let users: Vec<User> = rows.iter().map(User::from).collect(); // assuming you have a User struct and a from function to convert a row to a User
    Ok(Json(json!(users)))
}
