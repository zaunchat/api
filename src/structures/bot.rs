use super::*;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:bots)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub id: u64,
    pub username: String,
    pub owner_id: u64,
    pub verified: bool,
}

#[async_trait]
impl Base for Bot {
    fn id(&self) -> u64 { self.id }
}

impl Bot {
    pub fn new(username: String, owner_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            username,
            owner_id,
            verified: false,
        }
    }
}
