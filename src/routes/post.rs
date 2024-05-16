use crate::handlers::{add_post, delete_post, get_all_posts, get_post, update_post};
use crate::models::Post;
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
            move |Json(post): Json<Post>| {
                async move {
                    add_post(Json(post), client_clone.clone()).await // Clone again for the async block if needed
                }
            }
        })).route("/posts/:id", get({
            move |path: axum::extract::Path<i64>, extension: Extension<Arc<Client>>| {
                async move {
                    get_post(path, extension).await
                }
            }
        })).route("/posts/:id", delete({
            move |path: axum::extract::Path<i64>, extension: Extension<Arc<Client>>| {
                async move {
                    delete_post(path, extension).await
                }
            }
        })).route("/post/:id", put({
            let client_clone = client.clone(); // Clone for this closure
            move |path: axum::extract::Path<i64>, Json(post): Json<Post>| {
                let extension = Extension(client_clone);
                let title = post.title.clone();
                let description = post.description.clone();
                let image = post.image.clone();
                
                async move {
                    update_post(path, extension, title, description, image).await
                }
            }
        
        }))}

                        


