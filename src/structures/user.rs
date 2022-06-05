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
    pub async fn fetch_sessions(&self) {}
    pub async fn fetch_servers(&self) {}
    pub async fn fetch_bots(&self) {}
    pub async fn fetch_relations(&self) {}
    pub async fn fetch_by_token(token: &str) {}
   
    pub async fn save(&self) {}
    pub async fn delete(&self) {}
    pub fn to_public(&self) {}
}