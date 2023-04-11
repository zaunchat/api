use super::*;
use crate::utils::Snowflake;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Model, FromRow, Serialize, Deserialize, Clone, OpgModel)]
#[ormlite(table = "bots")]
pub struct Bot {
    pub id: Snowflake,
    pub username: String,
    pub owner_id: Snowflake,
    pub verified: bool,
}

impl Bot {
    pub fn new(username: String, owner_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            username,
            owner_id,
            verified: false,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let owner = User::faker();
        let bot = Self::new("Ghost Bot".to_string(), owner.id);

        owner.save().await?;

        Ok(bot)
    }
}

impl Base for Bot {}
