use super::server::*;
use crate::{database::postgres, util::badges::Badges};
use serde::{Deserialize, Serialize};


#[crud_table(table_name:users)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub badges: i64,
}

impl User {
    pub async fn save(&self) {}
    pub async fn delete(&self) {}
}