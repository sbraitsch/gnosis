use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Serialize, Deserialize};
use tokio_postgres::{Row, NoTls};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateCategory {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Snippet {
    pub id: i32,
    pub category: Category,
    pub description: String,
    pub code: String,
}

impl Snippet {
    pub fn from_row(row: Row) -> Snippet {
        Snippet {
            id: row.get("id"),
            category: Category{id: row.get("category_id"), name: row.get("name")},
            description: row.get("description"),
            code: row.get("code"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateSnippet {
    pub category: CreateCategory,
    pub description: String,
    pub code: String,
}

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

