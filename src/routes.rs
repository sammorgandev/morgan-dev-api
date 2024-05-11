use crate::handlers::{add_new_user, get_all_users};
use crate::models::User;
use axum::{
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio_postgres::Client;

pub fn get_routes(client: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello! You've reached the root directory of api.morgan.dev. This is a private api. Go away." }))
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
                    add_new_user(Json(user), client_clone.clone()).await // Clone again for the async block if needed
                }
            }
        }))
}
