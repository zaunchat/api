use crate::structures::*;
use crate::utils::error::*;
use crate::utils::Snowflake;

#[async_trait]
pub trait Ref {
    fn id(&self) -> Snowflake;

    async fn user(&self) -> Result<User> {
        User::find_by_id(self.id())
            .await
            .map_err(|_| Error::UnknownUser)
    }

    async fn channel(&self, recipient: Option<Snowflake>) -> Result<Channel> {
        let channel = if let Some(recipient) = recipient {
            Channel::find_one(
                "id = $1 AND recipients @> ARRAY[$2]::BIGINT[]",
                vec![self.id(), recipient],
            )
            .await
        } else {
            Channel::find_by_id(self.id()).await
        };

        channel.map_err(|_| Error::UnknownChannel)
    }

    async fn message(&self) -> Result<Message> {
        Message::find_by_id(self.id())
            .await
            .map_err(|_| Error::UnknownMessage)
    }

    async fn session(&self, user_id: Snowflake) -> Result<Session> {
        Session::find_one("id = $1 AND user_id = $2", vec![self.id(), user_id])
            .await
            .map_err(|_| Error::UnknownSession)
    }

    async fn bot(&self) -> Result<Bot> {
        Bot::find_by_id(self.id())
            .await
            .map_err(|_| Error::UnknownBot)
    }
}

impl Ref for Snowflake {
    fn id(&self) -> Snowflake {
        *self
    }
}
