use super::*;
use crate::utils::Snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "sessions")]
pub struct Session {
    pub id: Snowflake,
    #[serde(skip)]
    pub token: String,
    #[serde(skip)]
    pub user_id: Option<Snowflake>,
}

impl Session {
    pub fn new(user_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            token: nanoid!(64),
            user_id: Some(user_id),
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
