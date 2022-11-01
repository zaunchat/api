use super::*;
use crate::utils::snowflake;
use chrono::NaiveDateTime;
use ormlite::model::*;
use ormlite::types::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, OpgModel)]
struct MessageAttachments(Vec<Attachment>);

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "messages")]
pub struct Message {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub content: Option<String>,
    #[opg(custom = "MessageAttachments")]
    pub attachments: Json<Vec<Attachment>>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub channel_id: i64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub author_id: i64,
    #[opg(string, nullable)]
    pub edited_at: Option<NaiveDateTime>,
}

impl Message {
    pub fn new(channel_id: i64, author_id: i64) -> Self {
        Self {
            id: snowflake::generate(),
            channel_id,
            author_id,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() && self.attachments.0.is_empty()
    }

    #[cfg(test)]
    pub async fn faker() -> Result<Self, Error> {
        let user = User::faker();
        let channel = Channel::faker(ChannelTypes::Group).await?;
        let mut message = Self::new(channel.id, user.id);

        message.content = "Hello world!".to_string().into();

        channel.save().await?;
        user.save().await?;

        Ok(message)
    }
}

impl Base for Message {}
