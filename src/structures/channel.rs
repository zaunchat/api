use serde::{Deserialize, Serialize};

#[crud_table(table_name:channels)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: i64,
    pub r#type: String   
}

impl Channel {
    pub async fn save(&self) {}
    pub async fn delete(&self) {}
}