use crate::handlers::auth::{auth_handler, login_handler};
use crate::services::create_contact;
use axum::{
    body::Body,
    response::Response,
    routing::{get, post},
    Extension, Router,
};
use hyper::StatusCode;
use reqwest::Client;
use std::sync::Arc;

pub fn get_service_routes(client: Arc<Client>) -> Router {
    Router::new().route(
        "/contacts",
        post({
            let client_clone = client.clone(); // Clone for this closure
            move |req: axum::extract::Request<Body>| async move {
                match auth_handler(req.headers().clone()).await {
                    Ok(_) => match create_contact(req, Extension(client_clone)).await {
                        Ok(value) => {
                            Response::new(Body::from(serde_json::to_string(&value).unwrap()))
                        }
                        Err(_) => Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap(),
                    },
                    Err(_) => Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::empty())
                        .unwrap(),
                }
            }
        }),
    )
}
