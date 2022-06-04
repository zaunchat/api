use serde::{Deserialize, Serialize};
use crate::database::postgres;

#[crud_table(table_name:members)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
   pub id: i64,
   pub nickname: Option<String>,
   pub joined_at: i64,
   pub server_id: i64,
   pub roles: Vec<i64>
}

impl Member {
   pub async fn save(&self) {}
   pub async fn delete(&self) {}
}