use super::*;
use crate::utils::snowflake;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Model, FromRow, Serialize, Deserialize, Clone, OpgModel)]
#[ormlite(table = "bots")]
pub struct Bot {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub username: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub owner_id: i64,
    pub verified: bool,
}

impl Bot {
    pub fn new(username: String, owner_id: i64) -> Self {
        Self {
            id: snowflake::generate(),
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
