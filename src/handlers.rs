use crate::db::get_user_by_id;
use crate::models::User;
use axum::{
    //axum is the http server framework
    body::Body,
    extract::{Json, Path},
    http::StatusCode,
    Extension,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_postgres::Client;

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
