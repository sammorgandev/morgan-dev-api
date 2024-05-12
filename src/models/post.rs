use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub image: String,
    pub tags: Vec<String>,
}

//create post table if it doesn't exist
pub async fn create_table(client: Arc<Client>) -> Result<(), tokio_postgres::Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            image TEXT NOT NULL,
            tags TEXT[] NOT NULL
        )",
            &[],
        )
        .await?;
    Ok(())
}

impl Post {
    pub async fn new(
        id: i32,
        title: String,
        body: String,
        image: String,
        tags: Vec<String>,
        client: Arc<Client>,
    ) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO posts (id, title, body, image, tags) VALUES ($1, $2, $3, $4, $5)",
                &[&id, &title, &body, &image, &tags],
            )
            .await?;

        Ok(Post {
            id,
            title,
            body,
            image,
            tags,
        })
    }
}

pub async fn update(
    client: Arc<Client>,
    id: i32,
    title: String,
    body: String,
    image: String,
    tags: Vec<String>,
) -> Result<(), tokio_postgres::Error> {
    client
        .execute(
            "UPDATE posts SET title = $1, body = $2, image = $3, tags = $4 WHERE id = $5",
            &[&title, &body, &image, &tags, &id],
        )
        .await?;
    Ok(())
}

pub async fn delete(client: Arc<Client>, id: i32) -> Result<(), tokio_postgres::Error> {
    client
        .execute("DELETE FROM posts WHERE id = $1", &[&id])
        .await?;
    Ok(())
}

pub async fn get(client: Arc<Client>, id: i32) -> Result<Option<Post>, tokio_postgres::Error> {
    let row = client
        .query_opt("SELECT * FROM posts WHERE id = $1", &[&id])
        .await?;

    match row {
        Some(row) => {
            let id: i32 = row.get(0);
            let title: String = row.get(1);
            let body: String = row.get(2);
            let image: String = row.get(3);
            let tags: Vec<String> = row.get(4);

            Ok(Some(Post {
                id,
                title,
                body,
                image,
                tags,
            }))
        }
        None => Ok(None),
    }
}

pub async fn get_all(client: Arc<Client>) -> Result<Vec<Post>, tokio_postgres::Error> {
    let rows = client.query("SELECT * FROM posts", &[]).await?;

    let mut posts = Vec::new();

    for row in rows {
        let id: i32 = row.get(0);
        let title: String = row.get(1);
        let body: String = row.get(2);
        let image: String = row.get(3);
        let tags: Vec<String> = row.get(4);

        posts.push(Post {
            id,
            title,
            body,
            image,
            tags,
        });
    }

    Ok(posts)
}

pub fn from(row: &tokio_postgres::Row) -> Post {
    let id: i32 = row.get(0);
    let title: String = row.get(1);
    let body: String = row.get(2);
    let image: String = row.get(3);
    let tags: Vec<String> = row.get(4);

    Post {
        id,
        title,
        body,
        image,
        tags,
    }
}
