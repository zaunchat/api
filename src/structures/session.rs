use serde::{Deserialize, Serialize};

#[crud_table(table_name:sessions)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: i64
}

impl Session {
    pub async fn save(&self) {}
    pub async fn delete(&self) {}
}