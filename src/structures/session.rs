use super::*;
use crate::utils::snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "sessions")]
pub struct Session {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    #[serde(skip)]
    pub token: String,
    #[opg(string)]
    #[serde(skip)]
    pub user_id: i64,
}

impl Session {
    pub fn new(user_id: i64) -> Self {
        Self {
            id: snowflake::generate(),
            token: nanoid!(64),
            user_id,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let user = User::faker();
        let session = Self::new(user.id);

        user.save().await?;

        Ok(session)
    }
}

impl Base for Session {}
