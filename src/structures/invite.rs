use super::*;
use crate::utils::Snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "invites")]
pub struct Invite {
    pub id: Snowflake,
    pub code: String,
    pub uses: i32,
    pub inviter_id: Snowflake,
    pub channel_id: Snowflake,
    pub server_id: Option<Snowflake>,
}

impl Invite {
    pub fn new(inviter_id: Snowflake, channel_id: Snowflake, server_id: Option<Snowflake>) -> Self {
        Self {
            id: Snowflake::default(),
            code: nanoid!(8),
            inviter_id,
            channel_id,
            server_id,
            uses: 0,
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        use crate::structures::*;

        let user = User::faker();
        let channel = Channel::faker(ChannelTypes::Group).await?;
        let invite = Self::new(user.id, channel.id, None);

        user.save().await?;
        channel.save().await?;

        Ok(invite)
    }
}

impl Base for Invite {}
