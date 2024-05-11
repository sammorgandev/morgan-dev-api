use crate::db::get_user_by_id;
use crate::models::User;
use axum::{
    //axum is the http server framework
    body::{Body, Bytes},
    extract::{Json, Path, Query},
    http::StatusCode,
    Extension,
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
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
pub async fn _path(Path(_path): Path<String>) {} //"Path" give you the path parameters and deserializes them into the type you specify
pub async fn _query(Query(_params): Query<HashMap<String, String>>) {} //"Query" gives you the query parameters and deserializes them into the type you specify
pub async fn _json(Json(_payload): Json<serde_json::Value>) {} //buffer the request body and deserialize it as JSON into a serde_json::Value. 'Json' supports any type that implements Deserialize

//CUSTOM HANDLERS
pub async fn get_all_users(client: Arc<Client>) -> Result<Json<Value>, axum::response::Response> {
    let result = client.query("SELECT * FROM users", &[]).await;
    match result {
        Ok(rows) => {
            let users: Vec<User> = rows.iter().map(User::from).collect();
            Ok(Json(json!(users)))
        }
        Err(e) => {
            let error_message = format!("Failed to fetch users: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(axum::response::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn get_user(
    Path(user_id): Path<i32>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<User>, StatusCode> {
    match get_user_by_id(&client, user_id).await {
        Ok(Some(row)) => Ok(Json(User::from(&row))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_new_user(
    Json(user): Json<User>,
    client: Arc<Client>,
) -> Result<Json<Value>, axum::response::Response> {
    match User::new(user.id, user.name, user.email, user.password, client).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "User added successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to add user: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(axum::response::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}
