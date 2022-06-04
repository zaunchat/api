use crate::database::postgres;
use crate::util::permissions::Permissions;
use serde::{Deserialize, Serialize};
use super::{role::Role, member::Member};

#[crud_table(table_name:servers)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub owner_id: i64,
    pub permissions: Permissions,
}

impl Server {
    pub async fn save(&self) {}
    pub async fn delete(&self) {}
}