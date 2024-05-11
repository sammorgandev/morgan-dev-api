use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
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
