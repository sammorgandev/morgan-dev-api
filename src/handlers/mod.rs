pub mod post;
pub mod user;

pub use post::{add_post, delete_post, get_all_posts, get_post, update_post};
pub use user::{add_user, delete_user, get_all_users, get_user, update_user};
