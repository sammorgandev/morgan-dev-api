use crate::models::Post;
use axum::{
    //axum is the http server framework
    body::{to_bytes, Body},
    extract::{Json, Path, Request},
    http::StatusCode,
    response::Response,
    Extension,
};

use chrono::Utc;
use serde_json::{from_slice, json, Value};
use std::sync::Arc;
use tokio_postgres::Client;

use super::get_info_handler;

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
    client: Arc<Client>,
    req: Request<Body>,
) -> Result<Json<Value>, axum::response::Response> {
    let header_map = req.headers().clone();
    let body_bytes = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let post: Post = from_slice(&body_bytes).unwrap();

    let is_authorized = get_info_handler(header_map).await;
    if is_authorized.is_err() {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap());
    } else {
        match Post::new(
            post.id,
            post.title,
            post.body,
            Some(post.image.unwrap_or_else(|| "".to_string())),
            Some(post.tags.unwrap_or_else(|| vec![])),
            Some(post.category.unwrap_or_else(|| "".to_string())),
            Some(post.created_at.unwrap_or_else(Utc::now)),
            client,
        )
        .await
        {
            Ok(_) => {
                let success_message = Json(json!({"message": "Post added successfully"}));
                Ok(success_message)
            }
            Err(e) => {
                let error_message = format!("Failed to decode token: {:?}", e);
                let error_response = json!({"error": error_message});
                let body = Body::from(serde_json::to_string(&error_response).unwrap());
                Err(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(body)
                    .unwrap())
            }
        }
    }
}

pub async fn delete_post(
    req: Request<Body>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Value>, axum::response::Response> {
    let header_map = req.headers().clone();
    let body_bytes = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let post: Post = from_slice(&body_bytes).unwrap();
    let post_id = post.id;

    let is_authorized = get_info_handler(header_map).await;
    if is_authorized.is_err() {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap());
    } else {
        match Post::delete(client, post_id.into()).await {
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
}

pub async fn update_post(
    req: Request<Body>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Value>, Response> {
    let header_map = req.headers().clone();
    let body_bytes = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let post: Post = from_slice(&body_bytes).unwrap();
    let post_id = post.id;
    let is_authorized = get_info_handler(header_map).await;
    if is_authorized.is_err() {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap());
    } else {
        match Post::update(
            client,
            post_id,
            post.title,
            post.body,
            post.image.unwrap(),
            post.tags.unwrap(),
            post.category.unwrap(),
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
}
