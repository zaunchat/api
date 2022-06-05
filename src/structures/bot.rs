use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

use super::Base;

#[crud_table(table_name:bots)]
#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub id: i64,
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    pub owner_id: i64,
    pub verified: bool,
}

#[async_trait]
impl Base for Bot {
    async fn delete(&self) -> bool {
        false
    }
}

impl Bot {
    pub fn new(username: String, owner_id: i64) -> Self {
        Self {
            id: generate_id(),
            username,
            owner_id,
            verified: false,
        }
    }
}
