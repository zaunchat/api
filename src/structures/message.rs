use serde::{Deserialize, Serialize};


#[crud_table(table_name:messages)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i64,
    pub content: Option<String>
}


impl Message {
    pub async fn save(&self) {}
    pub async fn delete(&self) {}
}