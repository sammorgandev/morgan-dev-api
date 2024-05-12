pub async fn root() -> &'static str {
    "Hello! You've reached the root directory of api.morgan.dev. This is a private api. Go away."
}

pub async fn health_check() -> &'static str {
    "Healthy"
}
