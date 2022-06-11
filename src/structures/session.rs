use super::Base;
use crate::utils::snowflake;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:sessions)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, utoipa::Component)]
pub struct Session {
    pub id: u64,
    pub token: String,
    pub user_id: u64,
}

impl Base for Session {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Session {
    pub fn new(user_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            token: nanoid!(64),
            user_id,
        }
    }
}
