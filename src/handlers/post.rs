use crate::models::Post;
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
pub async fn get_all_posts(client: Arc<Client>) -> Result<Json<Value>, Response> {
    let result = Post::get_all(client).await;
    match result {
        Ok(posts) => Ok(Json(json!({ "posts": posts }))),
        Err(e) => {
            let error_message = format!("Failed to fetch posts: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn get_post(
    Path(post_id): Path<i64>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Post>, StatusCode> {
    match Post::get(client, post_id).await {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_post(
    Json(post): Json<Post>,
    client: Arc<Client>,
) -> Result<Json<Value>, axum::response::Response> {
    match Post::new(post.id, post.title, post.description, post.image, client).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "Post added successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to add post: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn delete_post(
    Path(post_id): Path<i64>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Value>, axum::response::Response> {
    match Post::delete(client, post_id.try_into().unwrap()).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "Post deleted successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to delete post: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}
pub async fn update_post(
    Path(post_id): Path<i64>,
    Extension(client): Extension<Arc<Client>>,
    title: String,
    description: String,
    image: Option<String>,
) -> Result<Json<Value>, Response> {
    match Post::update(
        client,
        post_id.try_into().unwrap(),
        title,
        description,
        image,
    )
    .await
    {
        Ok(_) => {
            let success_message = Json(json!({"message": "Post updated successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to update post: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}
