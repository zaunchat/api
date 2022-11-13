use super::*;
use crate::utils::{permissions::*, Snowflake};
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "servers")]
pub struct Server {
    pub id: Snowflake,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub owner_id: Snowflake,
    pub permissions: Permissions,
}

impl Server {
    pub fn new(name: String, owner_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            name,
            owner_id,
            permissions: *DEFAULT_PERMISSION_EVERYONE,
            banner: None,
            icon: None,
            description: None,
        }
    }

    pub async fn fetch_members(&self) -> Result<Vec<Member>, ormlite::Error> {
        Member::select()
            .filter("server_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_member(&self, user_id: Snowflake) -> Result<Member, ormlite::Error> {
        Member::select()
            .filter("id = $1 AND server_id = $2")
            .bind(user_id)
            .bind(self.id)
            .fetch_one(pool())
            .await
    }

    pub async fn fetch_roles(&self) -> Result<Vec<Role>, ormlite::Error> {
        Role::select()
            .filter("server_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_channels(&self) -> Result<Vec<Channel>, ormlite::Error> {
        Channel::select()
            .filter("server_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let owner = User::faker();
        let server = Self::new("Fake Server".to_string(), owner.id);

        owner.save().await?;

        Ok(server)
    }
}

impl Base for Server {}
