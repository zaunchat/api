use super::*;
use crate::utils::{Permissions, Snowflake};
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "roles")]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub permissions: Permissions,
    pub color: i32,
    pub hoist: bool,
    pub server_id: Snowflake,
}

impl Role {
    pub fn new(name: String, server_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            name,
            server_id,
            hoist: false,
            color: 0,
            permissions: Permissions::empty(),
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let server = Server::faker().await?;
        let role = Self::new("Mod".to_string(), server.id);
        server.save().await?;
        Ok(role)
    }
}

impl Base for Role {}
