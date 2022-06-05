use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

use super::Base;
use nanoid::nanoid;

#[crud_table(table_name:sessions)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
}

impl Base for Session {}

impl Session {
    pub fn new(user_id: i64) -> Self {
        Self {
            id: generate_id(),
            token: nanoid!(64),
            user_id,
        }
    }
}
