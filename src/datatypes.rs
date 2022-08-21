use serde::{Serialize, Deserialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct CreateCommand {
    pub category: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct Command {
    pub id: i32,
    pub category: String,
    pub description: String,
    pub code: String,
}

impl Command {
    pub fn from_row(row: Row) -> Command {
        Command {
            id: row.get(0),
            category: row.get(1),
            description: row.get(2),
            code: row.get(3),
        }
    }
}

