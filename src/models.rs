use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn new(
        id: i32,
        name: String,
        email: String,
        password: String,
        client: Arc<Client>,
    ) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
                &[&id, &name, &email, &password],
            )
            .await?;

        Ok(User {
            id,
            name,
            email,
            password,
        })
    }
}

impl From<&tokio_postgres::Row> for User {
    fn from(row: &tokio_postgres::Row) -> Self {
        User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.get("password"),
        }
    }
}
