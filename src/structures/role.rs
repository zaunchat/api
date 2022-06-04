use serde::{Deserialize, Serialize};
use crate::util::permissions::Permissions;

#[crud_table(table_name:roles)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
   pub id: i64,
   pub name: String,
   pub permissions: Permissions,
   pub color: i32,
   pub hoist: bool,
   pub server_id: i64
}

impl Role {
   pub async fn save(&self) {}
   pub async fn delete(&self) {}
}