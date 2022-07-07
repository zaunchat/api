use crate::database::pool;
use crate::structures::*;
use crate::utils::error::*;

#[async_trait]
pub trait Ref {
    fn id(&self) -> i64;

    async fn user(&self) -> Result<User> {
        User::find_one(self.id())
            .await
            .map_err(|_| Error::UnknownUser)
    }

    async fn channel(&self, recipient: Option<i64>) -> Result<Channel> {
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

    async fn server(&self, user_id: Option<i64>) -> Result<Server> {
        let server = if let Some(user_id) = user_id {
            Server::select()
                .filter("id = ( SELECT server_id FROM members WHERE id = $1 AND server_id = $2 )")
                .bind(user_id)
                .bind(self.id())
                .fetch_one(pool())
                .await
        } else {
            Server::find_one(self.id()).await
        };

        server.map_err(|_| Error::UnknownServer)
    }

    async fn role(&self, server_id: i64) -> Result<Role> {
        Role::select()
            .filter("id = $1 AND server_id = $2")
            .bind(self.id())
            .bind(server_id)
            .fetch_optional(pool())
            .await?
            .ok_or(Error::UnknownRole)
    }

    async fn session(&self, user_id: i64) -> Result<Session> {
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

    async fn member(&self, server_id: i64) -> Result<Member> {
        Member::select()
            .filter("id = $1 AND server_id = $2")
            .bind(self.id())
            .bind(server_id)
            .fetch_optional(pool())
            .await?
            .ok_or(Error::UnknownMember)
    }

    async fn invite(&self, server_id: Option<i64>) -> Result<Invite> {
        let invite = if let Some(server_id) = server_id {
            Invite::select()
                .filter("id = $1 AND server_id = $2")
                .bind(self.id())
                .bind(server_id)
                .fetch_one(pool())
                .await
        } else {
            Invite::find_one(self.id()).await
        };

        invite.map_err(|_| Error::UnknownInvite)
    }
}

impl Ref for i64 {
    fn id(&self) -> i64 {
        *self
    }
}

impl Ref for u64 {
    fn id(&self) -> i64 {
        *self as i64
    }
}
