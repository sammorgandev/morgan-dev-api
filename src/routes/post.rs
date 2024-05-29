use crate::handlers::{add_post, delete_post, get_all_posts, auth_handler, get_post, update_post, get_posts_by_category, get_posts_by_tag};
use axum::body::Body;
use axum::response::Response;
use axum::Extension;
use axum::{
    routing::{delete, get, post, put},
    Router,

};
use hyper::StatusCode;
use std::sync::Arc;
use tokio_postgres::Client;


pub fn get_post_routes(client: Arc<Client>) -> Router {
    Router::new()
        .route("/posts", get({
            let client_clone = client.clone(); // Clone for this closure
            move || {
                async move {
                    get_all_posts(client_clone.clone()).await // Clone again for the async block if needed
                }
            }
        }))
        .route("/posts/category/:category_slug", get({
            move |path: axum::extract::Path<String>, extension: Extension<Arc<Client>>| {
                async move {
                    get_posts_by_category(extension, path.to_string()).await
                }
            }
        }))
        .route("/posts/tag/:tag_slug", get({
            move |path: axum::extract::Path<String>, extension: Extension<Arc<Client>>| {
                async move {
                    get_posts_by_tag(extension, path.to_string()).await
                }
            }
        }))
        .route("/posts/:slug", get({
            move |path: axum::extract::Path<String>, extension: Extension<Arc<Client>>| {
                async move {
                    get_post(path, extension).await
                }
            }
        }))
       
        .route("/posts", post({
            let client_clone = client.clone(); // Clone for this closure

            move |req: axum::extract::Request<Body>| {
                async move {
                    match auth_handler(req.headers().clone()).await {
                        Ok(_) => add_post(client_clone, req).await,
                        Err(_) => Err(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
            }
        }))
        .route("/posts/:slug", delete({
            let client_clone = Extension(client.clone()); // Clone for this closure

            move |req: axum::extract::Request<Body>| {
                async move {
                    match auth_handler(req.headers().clone()).await {
                        Ok(_) => delete_post(req, client_clone).await,
                        Err(_) => Err(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
            }
        }))
        .route("/posts/:slug", put({
            let client_clone_2 = Extension(client.clone()); // Clone for this closure

            move |req: axum::extract::Request<Body>| {
                
                let extension = client_clone_2.clone();
                async move {
                    match auth_handler(req.headers().clone()).await {
                        Ok(_) => update_post(req, extension).await,
                        Err(_) => Err(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
            }
        
        }))
        .layer(Extension(client))
}


