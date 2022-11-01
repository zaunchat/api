use super::*;
use crate::utils::snowflake;
use nanoid::nanoid;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, OpgModel)]
#[ormlite(table = "invites")]
pub struct Invite {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub code: String,
    pub uses: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub inviter_id: i64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub channel_id: i64,
    #[opg(string, nullable)]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub server_id: Option<i64>,
}

impl Invite {
    pub fn new(inviter_id: i64, channel_id: i64, server_id: Option<i64>) -> Self {
        Self {
            id: snowflake::generate(),
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
