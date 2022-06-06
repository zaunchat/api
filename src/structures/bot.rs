use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

use super::Base;

#[crud_table(table_name:bots)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub id: u64,
    pub username: String,
    pub owner_id: u64,
    pub verified: bool,
}

#[async_trait]
impl Base for Bot {}

impl Bot {
    pub fn new(username: String, owner_id: u64) -> Self {
        Self {
            id: generate_id(),
            username,
            owner_id,
            verified: false,
        }
    }
}
