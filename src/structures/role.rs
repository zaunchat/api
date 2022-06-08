use super::*;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:roles)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub permissions: Permissions,
    pub color: u8,
    pub hoist: bool,
    pub server_id: u64,
}

impl Base for Role {}

impl Role {
    pub fn new(name: String, server_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            name,
            server_id,
            ..Default::default()
        }
    }
}
