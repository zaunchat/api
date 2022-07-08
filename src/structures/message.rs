use super::*;
use crate::utils::snowflake;
use chrono::NaiveDateTime;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "messages")]
pub struct Message {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub content: Option<String>,
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
        self.content.is_none() /* && self.attachments.len() == 0 */
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        let user = User::faker();
        let channel = Channel::faker(ChannelTypes::Group).await;
        let mut message = Self::new(channel.id, user.id);

        message.content = "Hello world!".to_string().into();

        channel.save().await.unwrap();
        user.save().await.unwrap();

        message
    }

    #[cfg(test)]
    pub async fn cleanup(self) -> Result<(), crate::utils::Error> {
        use crate::utils::Ref;
        self.author_id.user().await?.remove().await?;
        self.channel_id.channel(None).await?.cleanup().await?;
        Ok(())
    }
}

impl Base for Message {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let msg = Message::faker().await.save().await.unwrap();
            let msg = Message::find_one(msg.id).await.unwrap();
            msg.cleanup().await.unwrap();
        });
    }
}
