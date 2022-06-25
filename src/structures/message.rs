use super::Base;
use crate::utils::snowflake;
use rbatis::types::Timestamp;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:messages | formats_pg:"edited_at:{}::timestamp")]
#[derive(Debug, Serialize, Deserialize, Clone, Default, OpgModel)]
pub struct Message {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub channel_id: u64,
    pub author_id: u64,
    #[opg(integer, nullable)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<Timestamp>, /*
                                      TODO:
                                      pub embeds: Vec<Embed>
                                      pub attachments: Vec<Attachment>
                                      pub mentions: Vec<u64>
                                      pub replies: Vec<Reply>
                                      */
}

impl Base for Message {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Message {
    pub fn new(channel_id: u64, author_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            channel_id,
            author_id,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_none() /* && self.attachments.len() == 0 */
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        use crate::structures::{Channel, ChannelTypes, User};

        let user = User::faker();
        let channel = Channel::faker(ChannelTypes::Group).await;

        channel.save().await;
        user.save().await;

        Self::new(channel.id, user.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.author_id.user().await.unwrap().delete().await;
        self.channel_id.channel(None).await.unwrap().cleanup().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() {
        crate::tests::setup().await;

        let mut msg = Message::faker().await;

        msg.content = "Hello world!".to_string().into();

        msg.save().await;
        msg.cleanup().await;
    }
}
