use crate::handlers::{add_post, delete_post, get_all_posts, get_post, update_post};
use crate::models::Post;
use axum::body::Body;
use axum::Extension;
use axum::{
    routing::{delete, get, post, put},
    Json, Router,

};
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
        .route("/posts", post({
            let client_clone = client.clone(); // Clone for this closure

            move |req: axum::extract::Request<Body>| {
                async move {
                    add_post(client_clone, req).await
                }
            }
        })).route("/posts/:id", get({
            move |path: axum::extract::Path<i64>, extension: Extension<Arc<Client>>| {
                async move {
                    get_post(path, extension).await
                }
            }
        })).route("/posts/:id", delete({
            let client_clone = Extension(client.clone()); // Clone for this closure

            move |req: axum::extract::Request<Body>| {
                async move {
                    delete_post(req, client_clone).await
                }
            }
        })).route("/posts/:id", put({
            let client_clone_2 = Extension(client.clone()); // Clone for this closure

            move |req: axum::extract::Request<Body>| {
                
                let extension = client_clone_2.clone();
                async move {
                    update_post(req, extension).await
                }
            }
        
        })).layer(Extension(client))
}


