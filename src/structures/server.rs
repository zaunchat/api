use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

use super::channel::Channel;
use super::member::Member;
use super::role::Role;
use super::Base;
use crate::utils::permissions::DEFAULT_PERMISSION_EVERYONE;

#[crud_table(table_name:servers)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Server {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub owner_id: u64,
    pub permissions: u64,
}

impl Base for Server {}

impl Server {
    pub fn new(name: String, owner_id: u64) -> Self {
        Self {
            id: generate_id(),
            name,
            owner_id,
            permissions: DEFAULT_PERMISSION_EVERYONE.bits(),
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
