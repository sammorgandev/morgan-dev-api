use crate::handlers::{add_user, delete_user, get_all_users, get_user, update_user};
use crate::models::User;
use axum::Extension;
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use std::sync::Arc;
use tokio_postgres::Client;


pub fn get_user_routes(client: Arc<Client>) -> Router {
    Router::new()
        .route("/users", get({
            let client_clone = client.clone(); // Clone for this closure
            move || {
                async move {
                    get_all_users(client_clone.clone()).await // Clone again for the async block if needed
                }
            }
        }))
        .route("/users", post({
            let client_clone = client.clone(); // Clone for this closure
            move |Json(user): Json<User>| {
                async move {
                    add_user(Json(user), client_clone.clone()).await // Clone again for the async block if needed
                }
            }
        })).route("/users/:id", get({
            move |path: axum::extract::Path<i32>, extension: Extension<Arc<Client>>| {
                async move {
                    get_user(path, extension).await
                }
            }
        })).route("/users/:id", delete({
            move |path: axum::extract::Path<i32>, extension: Extension<Arc<Client>>| {
                async move {
                    delete_user(path, extension).await
                }
            }
        })).route("/users/:id", put({
            let client_clone = client.clone(); // Clone for this closure
            move |path: axum::extract::Path<i32>, Json(user): Json<User>| {
                let extension = Extension(client_clone);
                let name = user.name.clone();
                let email = user.email.clone();
                let password = user.password.clone();
                
                async move {
                    update_user(path, extension, name, email, password).await
                }
            }
        })).layer(Extension(client))
}


