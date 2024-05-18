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
    pub category: String,
    pub created_at: i64,
}

impl Post {
    pub async fn new(
        id: i32,
        title: String,
        body: String,
        image: String,
        tags: Vec<String>,
        category: String,
        created_at: i64,
        client: Arc<Client>,
    ) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO posts (id, title, body, image, tags, category, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[&id, &title, &body, &image, &tags, &category, &created_at],
            )
            .await?;

        Ok(Post {
            id,
            title,
            body,
            image,
            tags,
            category,
            created_at,
        })
    }

    pub async fn update(
        client: Arc<Client>,
        id: i32,
        title: String,
        body: String,
        image: String,
        tags: Vec<String>,
        category: String,
    ) -> Result<(), tokio_postgres::Error> {
        client
        .execute(
            "UPDATE posts SET title = $1, body = $2, image = $3, tags = $4, category = $6 WHERE id = $5",
            &[&title, &body, &image, &tags, &id, &category],
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
                let category: String = row.get(5);
                let created_at: i64 = row.get(6);

                Ok(Some(Post {
                    id,
                    title,
                    body,
                    image,
                    tags,
                    category,
                    created_at,
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
            let category: String = row.get(5);
            let created_at: i64 = row.get(6);

            posts.push(Post {
                id,
                title,
                body,
                image,
                tags,
                category,
                created_at,
            });
        }

        Ok(posts)
    }
}
pub fn _from(row: &tokio_postgres::Row) -> Post {
    let id: i32 = row.get(0);
    let title: String = row.get(1);
    let body: String = row.get(2);
    let image: String = row.get(3);
    let tags: Vec<String> = row.get(4);
    let category: String = row.get(5);
    let created_at: i64 = row.get(6);

    Post {
        id,
        title,
        body,
        image,
        tags,
        category,
        created_at,
    }
}
