use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub id: i32,
    pub category: String,
    pub description: String,
    pub code: String,
}