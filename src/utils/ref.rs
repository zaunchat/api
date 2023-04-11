use crate::structures::*;
use crate::utils::error::*;
use crate::utils::Snowflake;

#[async_trait]
pub trait Ref {
    fn id(&self) -> Snowflake;

    async fn user(&self) -> Result<User> {
        User::find_one(self.id())
            .await
            .map_err(|_| Error::UnknownUser)
    }

    async fn channel(&self, recipient: Option<Snowflake>) -> Result<Channel> {
        let channel = if let Some(recipient) = recipient {
            Channel::select()
                .filter("id = $1 AND recipients @> ARRAY[$2]::BIGINT[]")
                .bind(self.id())
                .bind(recipient)
                .fetch_one(pool())
                .await
        } else {
            Channel::find_one(self.id()).await
        };

        channel.map_err(|_| Error::UnknownChannel)
    }

    async fn message(&self) -> Result<Message> {
        Message::find_one(self.id())
            .await
            .map_err(|_| Error::UnknownMessage)
    }

    async fn session(&self, user_id: Snowflake) -> Result<Session> {
        Session::select()
            .filter("id = $1 AND user_id = $2")
            .bind(self.id())
            .bind(user_id)
            .fetch_optional(pool())
            .await?
            .ok_or(Error::UnknownSession)
    }

    async fn bot(&self) -> Result<Bot> {
        Bot::find_one(self.id())
            .await
            .map_err(|_| Error::UnknownBot)
    }
}

impl Ref for Snowflake {
    fn id(&self) -> Snowflake {
        *self
    }
}
