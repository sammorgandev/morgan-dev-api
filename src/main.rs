//IMPORTS
//axum is the http server framework
use axum::{
    body::Bytes,
    extract::{Json, Path, Query},
    http::StatusCode,
    routing::get,
    Router,
};

use serde_json::{json, Value};

//standard library
use std::{collections::HashMap, format, string};

//UTILITY & HANDLER FUNCTIONS

//UTILITIES
//"Path" give you the path parameters and deserializes them into the type you specify
async fn _path(Path(_user_id): Path<u32>) {}

//"Query" gives you the query parameters and deserializes them into the type you specify
async fn _query(Query(_params): Query<HashMap<String, String>>) {}

//buffer the request body and deserialize it as JSON into a serde_json::Value. 'Json' supports any type that implements Deserialize
async fn _json(Json(_payload): Json<serde_json::Value>) {}

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
async fn main() {
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
