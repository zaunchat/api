use super::Base;
use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:roles)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub permissions: u64,
    pub color: i32,
    pub hoist: bool,
    pub server_id: u64,
}

impl Base for Role {}

impl Role {
    pub fn new(name: String, server_id: u64) -> Self {
        Self {
            id: generate_id(),
            name,
            server_id,
            ..Default::default()
        }
    }
}
