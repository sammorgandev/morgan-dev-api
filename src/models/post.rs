use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub image: Option<String>,
    pub tags: Option<Vec<String>>,
    pub category: Option<String>,
    pub company_name: Option<String>,
    pub company_logo: Option<String>,
    pub company_description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

impl Post {
    pub async fn new(
        id: i64,
        title: String,
        body: String,
        image: Option<String>,
        tags: Option<Vec<String>>,
        category: Option<String>,
        created_at: Option<DateTime<Utc>>,
        company_name: Option<String>,
        company_logo: Option<String>,
        company_description: Option<String>,
        client: Arc<Client>,
    ) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO posts (id, title, body, image, tags, category, company_name, company_logo, company_description, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[&id, &title, &body, &image, &tags, &category, &company_name, &company_logo, &company_description, &created_at as &(dyn ToSql + Sync)],
            )
            .await?;

        Ok(Post {
            id: id.into(),
            title,
            body,
            image: image.clone(),
            tags: tags.clone(),
            category: category.clone(),
            company_name: company_name.clone(),
            company_logo: company_logo.clone(),
            company_description: company_description.clone(),
            created_at: created_at.into(),
        })
    }

    pub async fn update(
        client: Arc<Client>,
        id: i64,
        title: String,
        body: String,
        image: String,
        tags: Vec<String>,
        category: String,
        company_name: String,
        company_logo: String,
        company_description: String,
    ) -> Result<(), tokio_postgres::Error> {
        client
        .execute(
            "UPDATE posts SET title = $1, body = $2, image = $3, tags = $4, category = $6 WHERE id = $5",
            &[&title, &body, &image, &tags, &id, &category, &company_name, &company_logo, &company_description],
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
            Some(row) => {
                let id: i64 = row.get(0);
                let title: String = row.get(2);
                let body: String = row.get(3);
                let image: Option<String> = row.get(4);
                let tags: Option<Vec<String>> = row.get(5);
                let category: Option<String> = row.get(6);
                let company_name: Option<String> = row.get(8);
                let company_logo: Option<String> = row.get(7);
                let company_description: Option<String> = row.get(9);
                let created_at: DateTime<Utc> = row.get(1);

                Ok(Some(Post {
                    id,
                    title,
                    body,
                    image: image.clone(),
                    tags: tags.clone(),
                    category: category.clone(),
                    company_name: company_name.clone(),
                    company_logo: company_logo.clone(),
                    company_description: company_description.clone(),
                    created_at: created_at.into(),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all(client: Arc<Client>) -> Result<Vec<Post>, tokio_postgres::Error> {
        let rows = client.query("SELECT * FROM posts", &[]).await?;

        let mut posts = Vec::new();

        for row in rows {
            let id: i64 = row.get(0);
            let title: String = row.get(2);
            let body: String = row.get(3);
            let image: Option<String> = row.get(4);
            let tags: Option<Vec<String>> = row.get(5);
            let category: Option<String> = row.get(6);
            let company_name: Option<String> = row.get(8);
            let company_logo: Option<String> = row.get(7);
            let company_description: Option<String> = row.get(9);
            let created_at: DateTime<Utc> = row.get(1);

            posts.push(Post {
                id,
                title,
                body,
                image: image.clone(),
                tags: tags.clone(),
                category: category.clone(),
                company_name: company_name.clone(),
                company_logo: company_logo.clone(),
                company_description: company_description.clone(),
                created_at: created_at.into(),
            });
        }

        Ok(posts)
    }

    pub fn _from(row: &tokio_postgres::Row) -> Post {
        let id: i64 = row.get(0);
        let title: String = row.get(2);
        let body: String = row.get(3);
        let image: String = row.get(4);
        let tags: Vec<String> = row.get(5);
        let category: String = row.get(6);
        let company_name: String = row.get(8);
        let company_logo: String = row.get(7);
        let company_description: String = row.get(9);
        let created_at: DateTime<Utc> = row.get(1);

        Post {
            id,
            title,
            body,
            image: Some(image),
            tags: Some(tags),
            category: Some(category),
            company_name: Some(company_name),
            company_logo: Some(company_logo),
            company_description: Some(company_description),
            created_at: created_at.into(),
        }
    }
}
