use super::*;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:servers)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, OpgModel)]
pub struct Server {
    pub id: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    pub owner_id: u64,
    pub permissions: Permissions,
}

impl Base for Server {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Server {
    pub fn new(name: String, owner_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            name,
            owner_id,
            permissions: *DEFAULT_PERMISSION_EVERYONE,
            ..Default::default()
        }
    }

    pub async fn fetch_members(&self) -> Vec<Member> {
        Member::find(|q| q.eq("server_id", &self.id)).await
    }

    pub async fn fetch_member(&self, user_id: u64) -> Option<Member> {
        Member::find_one(|q| q.eq("id", user_id).eq("server_id", &self.id)).await
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        Role::find(|q| q.eq("server_id", &self.id)).await
    }

    pub async fn fetch_channels(&self) -> Vec<Channel> {
        Channel::find(|q| q.eq("server_id", &self.id)).await
    }
}
