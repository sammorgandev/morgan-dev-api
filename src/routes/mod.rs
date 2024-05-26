pub mod misc;
pub mod post;
pub mod service;
pub mod user;

pub use misc::get_misc_routes;
pub use post::get_post_routes;
pub use service::get_service_routes;
pub use user::get_user_routes;
