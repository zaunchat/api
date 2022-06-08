use crate::utils::snowflake;

use super::Base;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:invites)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub id: u64,
    pub code: String,
    pub uses: u64,
    pub inviter_id: u64,
    pub channel_id: u64,
    pub server_id: Option<u64>,
}

impl Base for Invite {}

impl Invite {
    pub fn new(inviter_id: u64, channel_id: u64, server_id: Option<u64>) -> Self {
        Self {
            id: snowflake::generate(),
            code: nanoid!(8),
            inviter_id,
            channel_id,
            server_id,
            uses: 0,
        }
    }
}
