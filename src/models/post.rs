use std::sync::Arc;

use axum::Extension;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use tokio_postgres::types::ToSql;

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
    pub slug: String,
    pub video: Option<String>,
    pub body_2: Option<String>,
    pub body_3: Option<String>,
    pub body_4: Option<String>,
    pub body_5: Option<String>,
    pub body_6: Option<String>,
    pub title_2: Option<String>,
    pub title_3: Option<String>,
    pub list_1_title: Option<String>,
    pub list_1_description: Option<String>,
    pub list_2_title: Option<String>,
    pub list_2_description: Option<String>,
    pub list_3_title: Option<String>,
    pub list_3_description: Option<String>,
    pub quote_text: Option<String>,
    pub quote_author_name: Option<String>,
    pub quote_author_title: Option<String>,
    pub quote_author_role: Option<String>,
    pub quote_author_image: Option<String>,
}

impl Post {
    pub async fn new(post: Post, client: Arc<Client>) -> Result<Self, tokio_postgres::Error> {
        client
            .execute(
                "INSERT INTO posts (id, title, body, image, tags, category, company_name, company_logo, company_description, created_at, slug, video, body_2, body_3, body_4, body_5, body_6, title_2, title_3, list_1_title, list_1_description, list_2_title, list_2_description, list_3_title, list_3_description, quote_text, quote_author_name, quote_author_title, quote_author_role, quote_author_image) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30)",
                &[&post.id, &post.title, &post.body, &post.image, &post.tags, &post.category, &post.company_name, &post.company_logo, &post.company_description, &post.created_at, &post.slug, &post.video as &(dyn ToSql + Sync), &post.body_2, &post.body_3, &post.body_4, &post.body_5, &post.body_6, &post.title_2, &post.title_3, &post.list_1_title, &post.list_1_description, &post.list_2_title, &post.list_2_description, &post.list_3_title, &post.list_3_description, &post.quote_text, &post.quote_author_name, &post.quote_author_title, &post.quote_author_role, &post.quote_author_image],
            )
            .await?;

        Ok(Post {
            id: post.id,
            title: post.title,
            body: post.body,
            image: post.image,
            tags: post.tags,
            category: post.category,
            company_name: post.company_name,
            company_logo: post.company_logo,
            company_description: post.company_description,
            created_at: post.created_at,
            slug: post.slug,
            video: post.video,
            body_2: post.body_2,
            body_3: post.body_3,
            body_4: post.body_4,
            body_5: post.body_5,
            body_6: post.body_6,
            title_2: post.title_2,
            title_3: post.title_3,
            list_1_title: post.list_1_title,
            list_1_description: post.list_1_description,
            list_2_title: post.list_2_title,
            list_2_description: post.list_2_description,
            list_3_title: post.list_3_title,
            list_3_description: post.list_3_description,
            quote_text: post.quote_text,
            quote_author_name: post.quote_author_name,
            quote_author_title: post.quote_author_title,
            quote_author_role: post.quote_author_role,
            quote_author_image: post.quote_author_image,
        })
    }

    pub async fn update(client: Arc<Client>, post: Post) -> Result<(), tokio_postgres::Error> {
        client
        .execute(
            "UPDATE posts SET title = $1, body = $2, image = $3, tags = $4, category = $6 WHERE id = $5",
            &[&post.id, &post.title, &post.body, &post.image, &post.tags, &post.category, &post.company_name, &post.company_logo, &post.company_description, &post.created_at, &post.slug, &post.video as &(dyn ToSql + Sync), &post.body_2, &post.body_3, &post.body_4, &post.body_5, &post.body_6, &post.title_2, &post.title_3, &post.list_1_title, &post.list_1_description, &post.list_2_title, &post.list_2_description, &post.list_3_title, &post.list_3_description, &post.quote_text, &post.quote_author_name, &post.quote_author_title, &post.quote_author_role, &post.quote_author_image],
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

    pub async fn get(
        client: Arc<Client>,
        slug: String,
    ) -> Result<Option<Post>, tokio_postgres::Error> {
        let row = client
            .query_opt("SELECT * FROM posts WHERE slug = $1", &[&slug])
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
                let slug: String = row.get(10);
                let video: Option<String> = row.get(11);
                let body_2: Option<String> = row.get(12);
                let body_3: Option<String> = row.get(13);
                let body_4: Option<String> = row.get(14);
                let body_5: Option<String> = row.get(15);
                let body_6: Option<String> = row.get(16);
                let title_2: Option<String> = row.get(17);
                let title_3: Option<String> = row.get(18);
                let list_1_title: Option<String> = row.get(19);
                let list_1_description: Option<String> = row.get(20);
                let list_2_title: Option<String> = row.get(21);
                let list_2_description: Option<String> = row.get(22);
                let list_3_title: Option<String> = row.get(23);
                let list_3_description: Option<String> = row.get(24);
                let quote_text: Option<String> = row.get(25);
                let quote_author_name: Option<String> = row.get(26);
                let quote_author_title: Option<String> = row.get(27);
                let quote_author_role: Option<String> = row.get(28);
                let quote_author_image: Option<String> = row.get(29);

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
                    slug: slug.into(),
                    video: video.clone(),
                    body_2: body_2.clone(),
                    body_3: body_3.clone(),
                    body_4: body_4.clone(),
                    body_5: body_5.clone(),
                    body_6: body_6.clone(),
                    title_2: title_2.clone(),
                    title_3: title_3.clone(),
                    list_1_title: list_1_title.clone(),
                    list_1_description: list_1_description.clone(),
                    list_2_title: list_2_title.clone(),
                    list_2_description: list_2_description.clone(),
                    list_3_title: list_3_title.clone(),
                    list_3_description: list_3_description.clone(),
                    quote_text: quote_text.clone(),
                    quote_author_name: quote_author_name.clone(),
                    quote_author_title: quote_author_title.clone(),
                    quote_author_role: quote_author_role.clone(),
                    quote_author_image: quote_author_image.clone(),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_category(
        client: Extension<Arc<Client>>,
        category: String,
    ) -> Result<Vec<Post>, tokio_postgres::Error> {
        let parsed_category = category.replace("-", " ");
        let rows = client
            .query(
                "SELECT * FROM posts WHERE category = $1",
                &[&parsed_category],
            )
            .await?;

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
            let slug: String = row.get(10);
            let video: Option<String> = row.get(11);
            let body_2: Option<String> = row.get(12);
            let body_3: Option<String> = row.get(13);
            let body_4: Option<String> = row.get(14);
            let body_5: Option<String> = row.get(15);
            let body_6: Option<String> = row.get(16);
            let title_2: Option<String> = row.get(17);
            let title_3: Option<String> = row.get(18);
            let list_1_title: Option<String> = row.get(19);
            let list_1_description: Option<String> = row.get(20);
            let list_2_title: Option<String> = row.get(21);
            let list_2_description: Option<String> = row.get(22);
            let list_3_title: Option<String> = row.get(23);
            let list_3_description: Option<String> = row.get(24);
            let quote_text: Option<String> = row.get(25);
            let quote_author_name: Option<String> = row.get(26);
            let quote_author_title: Option<String> = row.get(27);
            let quote_author_role: Option<String> = row.get(28);
            let quote_author_image: Option<String> = row.get(29);

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
                slug: slug.into(),
                video: video.clone(),
                body_2: body_2.clone(),
                body_3: body_3.clone(),
                body_4: body_4.clone(),
                body_5: body_5.clone(),
                body_6: body_6.clone(),
                title_2: title_2.clone(),
                title_3: title_3.clone(),
                list_1_title: list_1_title.clone(),
                list_1_description: list_1_description.clone(),
                list_2_title: list_2_title.clone(),
                list_2_description: list_2_description.clone(),
                list_3_title: list_3_title.clone(),
                list_3_description: list_3_description.clone(),
                quote_text: quote_text.clone(),
                quote_author_name: quote_author_name.clone(),
                quote_author_title: quote_author_title.clone(),
                quote_author_role: quote_author_role.clone(),
                quote_author_image: quote_author_image.clone(),
            });
        }

        Ok(posts)
    }

    pub async fn get_tag(
        client: Extension<Arc<Client>>,
        tag: String,
    ) -> Result<Vec<Post>, tokio_postgres::Error> {
        let rows = client
            .query("SELECT * FROM posts WHERE $1 = ANY(tags)", &[&tag])
            .await?;

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
            let slug: String = row.get(10);
            let video: Option<String> = row.get(11);
            let body_2: Option<String> = row.get(12);
            let body_3: Option<String> = row.get(13);
            let body_4: Option<String> = row.get(14);
            let body_5: Option<String> = row.get(15);
            let body_6: Option<String> = row.get(16);
            let title_2: Option<String> = row.get(17);
            let title_3: Option<String> = row.get(18);
            let list_1_title: Option<String> = row.get(19);
            let list_1_description: Option<String> = row.get(20);
            let list_2_title: Option<String> = row.get(21);
            let list_2_description: Option<String> = row.get(22);
            let list_3_title: Option<String> = row.get(23);
            let list_3_description: Option<String> = row.get(24);
            let quote_text: Option<String> = row.get(25);
            let quote_author_name: Option<String> = row.get(26);
            let quote_author_title: Option<String> = row.get(27);
            let quote_author_role: Option<String> = row.get(28);
            let quote_author_image: Option<String> = row.get(29);

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
                slug: slug.into(),
                video: video.clone(),
                body_2: body_2.clone(),
                body_3: body_3.clone(),
                body_4: body_4.clone(),
                body_5: body_5.clone(),
                body_6: body_6.clone(),
                title_2: title_2.clone(),
                title_3: title_3.clone(),
                list_1_title: list_1_title.clone(),
                list_1_description: list_1_description.clone(),
                list_2_title: list_2_title.clone(),
                list_2_description: list_2_description.clone(),
                list_3_title: list_3_title.clone(),
                list_3_description: list_3_description.clone(),
                quote_text: quote_text.clone(),
                quote_author_name: quote_author_name.clone(),
                quote_author_title: quote_author_title.clone(),
                quote_author_role: quote_author_role.clone(),
                quote_author_image: quote_author_image.clone(),
            });
        }

        Ok(posts)
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
            let slug: String = row.get(10);
            let video: Option<String> = row.get(11);
            let body_2: Option<String> = row.get(12);
            let body_3: Option<String> = row.get(13);
            let body_4: Option<String> = row.get(14);
            let body_5: Option<String> = row.get(15);
            let body_6: Option<String> = row.get(16);
            let title_2: Option<String> = row.get(17);
            let title_3: Option<String> = row.get(18);
            let list_1_title: Option<String> = row.get(19);
            let list_1_description: Option<String> = row.get(20);
            let list_2_title: Option<String> = row.get(21);
            let list_2_description: Option<String> = row.get(22);
            let list_3_title: Option<String> = row.get(23);
            let list_3_description: Option<String> = row.get(24);
            let quote_text: Option<String> = row.get(25);
            let quote_author_name: Option<String> = row.get(26);
            let quote_author_title: Option<String> = row.get(27);
            let quote_author_role: Option<String> = row.get(28);
            let quote_author_image: Option<String> = row.get(29);

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
                slug: slug.into(),
                video: video.clone(),
                body_2: body_2.clone(),
                body_3: body_3.clone(),
                body_4: body_4.clone(),
                body_5: body_5.clone(),
                body_6: body_6.clone(),
                title_2: title_2.clone(),
                title_3: title_3.clone(),
                list_1_title: list_1_title.clone(),
                list_1_description: list_1_description.clone(),
                list_2_title: list_2_title.clone(),
                list_2_description: list_2_description.clone(),
                list_3_title: list_3_title.clone(),
                list_3_description: list_3_description.clone(),
                quote_text: quote_text.clone(),
                quote_author_name: quote_author_name.clone(),
                quote_author_title: quote_author_title.clone(),
                quote_author_role: quote_author_role.clone(),
                quote_author_image: quote_author_image.clone(),
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
        let slug: String = row.get(10);
        let video: String = row.get(11);
        let body_2: Option<String> = row.get(12);
        let body_3: Option<String> = row.get(13);
        let body_4: Option<String> = row.get(14);
        let body_5: Option<String> = row.get(15);
        let body_6: Option<String> = row.get(16);
        let title_2: Option<String> = row.get(17);
        let title_3: Option<String> = row.get(18);
        let list_1_title: Option<String> = row.get(19);
        let list_1_description: Option<String> = row.get(20);
        let list_2_title: Option<String> = row.get(21);
        let list_2_description: Option<String> = row.get(22);
        let list_3_title: Option<String> = row.get(23);
        let list_3_description: Option<String> = row.get(24);
        let quote_text: Option<String> = row.get(25);
        let quote_author_name: Option<String> = row.get(26);
        let quote_author_title: Option<String> = row.get(27);
        let quote_author_role: Option<String> = row.get(28);
        let quote_author_image: Option<String> = row.get(29);

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
            slug: slug.into(),
            video: Some(video),
            body_2: body_2.clone(),
            body_3: body_3.clone(),
            body_4: body_4.clone(),
            body_5: body_5.clone(),
            body_6: body_6.clone(),
            title_2: title_2.clone(),
            title_3: title_3.clone(),
            list_1_title: list_1_title.clone(),
            list_1_description: list_1_description.clone(),
            list_2_title: list_2_title.clone(),
            list_2_description: list_2_description.clone(),
            list_3_title: list_3_title.clone(),
            list_3_description: list_3_description.clone(),
            quote_text: quote_text.clone(),
            quote_author_name: quote_author_name.clone(),
            quote_author_title: quote_author_title.clone(),
            quote_author_role: quote_author_role.clone(),
            quote_author_image: quote_author_image.clone(),
        }
    }
}
