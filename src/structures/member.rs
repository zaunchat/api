use super::*;
use crate::utils::Snowflake;
use chrono::{NaiveDateTime, Utc};
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, OpgModel)]
struct MemberRoles(Vec<String>);

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "members")]
pub struct Member {
    pub id: Snowflake,
    pub nickname: Option<String>,
    #[opg(string)]
    pub joined_at: NaiveDateTime,
    pub server_id: Snowflake,
    #[opg(custom = "MemberRoles")]
    pub roles: Vec<Snowflake>,
}

impl Member {
    pub fn new(user_id: Snowflake, server_id: Snowflake) -> Self {
        Self {
            id: user_id,
            nickname: None,
            server_id,
            roles: vec![server_id],
            joined_at: Utc::now().naive_utc(),
        }
    }

    pub async fn fetch_roles(&self) -> Result<Vec<Role>, ormlite::Error> {
        if self.roles.is_empty() {
            return Ok(vec![]);
        }

        Role::select()
            .filter("id = ANY($1) AND server_id = $2")
            .bind(self.roles.clone())
            .bind(self.server_id)
            .fetch_all(pool())
            .await
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let server = Server::faker().await?;
        let member = Self::new(server.owner_id, server.id);

        server.save().await?;

        Ok(member)
    }
}

impl Base for Member {}
