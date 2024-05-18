use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

impl User {
    //crud functions
    pub async fn new(
        id: i64,
        name: String,
        email: String,
        password: Option<String>,
        client: Arc<Client>,
    ) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
                &[&id, &name, &email, &password.as_deref()],
            )
            .await?;

        Ok(User {
            id,
            name,
            email,
            password,
        })
    }

    pub async fn update(
        client: Arc<Client>,
        id: i64,
        name: String,
        email: String,
        password: Option<String>,
    ) -> Result<(), tokio_postgres::Error> {
        client
            .execute(
                "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
                &[&name, &email, &password.as_deref(), &id],
            )
            .await?;
        Ok(())
    }

    pub async fn delete(client: Arc<Client>, id: i64) -> Result<(), tokio_postgres::Error> {
        client
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .await?;
        Ok(())
    }

    pub async fn get(client: Arc<Client>, id: i64) -> Result<Option<User>, tokio_postgres::Error> {
        let row = client
            .query_opt("SELECT * FROM users WHERE id = $1", &[&id])
            .await?;

        match row {
            Some(row) => Ok(Some(User::from(&row))),
            None => Ok(None),
        }
    }

    pub async fn get_all(client: Arc<Client>) -> Result<Vec<User>, tokio_postgres::Error> {
        let rows = client.query("SELECT * FROM users", &[]).await?;
        let users: Vec<User> = rows.iter().map(User::from).collect();
        Ok(users)
    }

    //convert row to user type (from function)
    pub fn from(row: &tokio_postgres::Row) -> Self {
        User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            password: row.try_get("password").ok(),
        }
    }
}
