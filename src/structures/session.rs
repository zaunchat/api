use super::*;
use crate::utils::Snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "sessions")]
pub struct Session {
    pub id: Snowflake,
    #[serde(skip)]
    pub token: String,
    #[opg(string)]
    #[serde(skip)]
    pub user_id: Snowflake,
}

impl Session {
    pub fn new(user_id: Snowflake) -> Self {
        Self {
            id: Snowflake::default(),
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
