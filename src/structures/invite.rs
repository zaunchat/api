use serde::{Deserialize, Serialize};
use crate::util::permissions::Permissions;

#[crud_table(table_name:invites)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub id: i64,
    pub code: String,
    pub inviter_id: i64,
    pub channel_id: i64,
    pub server_id: Option<i64>
}

impl Invite {
    pub async fn save($self) {}
    pub async fn delete(&self) {}
}