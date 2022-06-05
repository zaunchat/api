use crate::utils::snowflake::generate_id;

use super::Base;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:invites)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub id: i64,
    pub code: String,
    pub inviter_id: i64,
    pub channel_id: i64,
    pub server_id: Option<i64>,
}

impl Base for Invite {}

impl Invite {
    pub fn new(inviter_id: i64, channel_id: i64, server_id: Option<i64>) -> Self {
        Self {
            id: generate_id(),
            code: nanoid!(8),
            inviter_id,
            channel_id,
            server_id,
        }
    }
}
