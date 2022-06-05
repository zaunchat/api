use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

use super::channel::Channel;
use super::member::Member;
use super::role::Role;
use super::Base;

#[crud_table(table_name:servers)]
#[derive(Debug, Validate, Serialize, Deserialize, Clone, Default)]
pub struct Server {
    pub id: i64,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(length(min = 1, max = 1000))]
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub owner_id: i64,
    pub permissions: i64,
}

impl Base for Server {}

impl Server {
    pub fn new(name: String, owner_id: i64) -> Self {
        Self {
            id: generate_id(),
            name,
            owner_id,
            ..Default::default()
        }
    }

    pub async fn fetch_members(&self) -> Vec<Member> {
        Member::find(|q| q.eq("server_id", &self.id)).await
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        Role::find(|q| q.eq("server_id", &self.id)).await
    }

    pub async fn fetch_channels(&self) -> Vec<Channel> {
        Channel::find(|q| q.eq("server_id", &self.id)).await
    }
}
