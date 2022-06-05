use serde::{Deserialize, Serialize};


#[crud_table(table_name:bots)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub id: i64,
    pub username: String,
    pub owner_id: i64,
    pub verified: bool
}

impl Bot {
    pub async fn save($self) {}
    pub async fn delete(&self) {}
}