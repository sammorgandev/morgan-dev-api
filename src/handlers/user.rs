use crate::models::User;
use axum::{
    //axum is the http server framework
    body::Body,
    extract::{Json, Path},
    http::StatusCode,
    response::Response,
    Extension,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_postgres::Client;

//CUSTOM HANDLERS
pub async fn get_all_users(client: Arc<Client>) -> Result<Json<Value>, Response> {
    let result = User::get_all(client).await;
    match result {
        Ok(users) => Ok(Json(json!({ "users": users }))),
        Err(e) => {
            let error_message = format!("Failed to fetch users: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
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
    match User::get(client, user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_user(
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
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn delete_user(
    Path(user_id): Path<i32>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Value>, axum::response::Response> {
    match User::delete(client, user_id).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "User deleted successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to delete user: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}
pub async fn update_user(
    Path(user_id): Path<i32>,
    Extension(client): Extension<Arc<Client>>,
    name: String,
    email: String,
    password: Option<String>,
) -> Result<Json<Value>, Response> {
    match User::update(client, user_id, name, email, password).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "User updated successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to update user: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}
