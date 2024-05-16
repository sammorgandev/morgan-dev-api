use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
}

impl Post {
    //crud functions
    pub async fn new(
        id: i64,
        title: String,
        description: String,
        image: Option<String>,
        client: Arc<Client>,
    ) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO posts (id, title, description, image) VALUES ($1, $2, $3, $4)",
                &[&id, &title, &description, &image.as_deref()],
            )
            .await?;

        Ok(Post {
            id,
            title,
            description,
            image,
        })
    }

    pub async fn update(
        client: Arc<Client>,
        id: i64,
        title: String,
        description: String,
        image: Option<String>,
    ) -> Result<(), tokio_postgres::Error> {
        client
            .execute(
                "UPDATE posts SET title = $1, description = $2, image = $3 WHERE id = $4",
                &[&title, &description, &image.as_deref(), &id],
            )
            .await?;
        Ok(())
    }

    pub async fn delete(client: Arc<Client>, id: i64) -> Result<(), tokio_postgres::Error> {
        client
            .execute("DELETE FROM posts WHERE id = $1", &[&id])
            .await?;
        Ok(())
    }

    pub async fn get(client: Arc<Client>, id: i64) -> Result<Option<Post>, tokio_postgres::Error> {
        let row = client
            .query_opt("SELECT * FROM posts WHERE id = $1", &[&id])
            .await?;

        match row {
            Some(row) => Ok(Some(Post::from(&row))),
            None => Ok(None),
        }
    }

    pub async fn get_all(client: Arc<Client>) -> Result<Vec<Post>, tokio_postgres::Error> {
        let rows = client.query("SELECT * FROM posts", &[]).await?;
        let users: Vec<Post> = rows.iter().map(Post::from).collect();
        Ok(users)
    }

    //convert row to user type (from function)
    pub fn from(row: &tokio_postgres::Row) -> Self {
        Post {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            image: row.try_get("image").ok(),
        }
    }
}
