use crate::database::DB as db;
use crate::structures::*;
use crate::utils::error::*;

#[async_trait]
pub trait Ref {
    fn id(&self) -> u64;

    async fn user(&self) -> Result<User> {
        User::find_one_by_id(self.id())
            .await
            .ok_or(Error::UnknownUser)
    }

    async fn channel(&self, recipient: Option<u64>) -> Result<Channel> {
        let channel = if let Some(recipient) = recipient {
            db.fetch(
                &format!(
                    "SELECT * FROM channels WHERE id = {} AND recipients @> ARRAY[{}]::BIGINT[]",
                    self.id(),
                    recipient
                ),
                vec![],
            )
            .await
            .unwrap()
        } else {
            Channel::find_one_by_id(self.id()).await
        };

        channel.ok_or(Error::UnknownChannel)
    }

    async fn message(&self) -> Result<Message> {
        Message::find_one_by_id(self.id())
            .await
            .ok_or(Error::UnknownMessage)
    }

    async fn server(&self, user_id: Option<u64>) -> Result<Server> {
        if let Some(user_id) = user_id {
            return db.fetch(
                "SELECT * FROM servers WHERE id = ( SELECT server_id FROM members WHERE id = $1 AND server_id = $2 )",
                vec![user_id.into(), self.id()],
            )
            .await
            .ok_or(Error::UnknownServer)
        }

        Server::find_one_by_id(self.id())
            .await
            .ok_or(Error::UnknownServer)
    }

    async fn role(&self, server_id: u64) -> Result<Role> {
        Role::find_one(|q| q.eq("id", self.id()).eq("server_id", server_id))
             .await
             .ok_or(Error::UnknownRole)
    }

    async fn session(&self, user_id: u64) -> Result<Session> {
        Session::find_one(|q| q.eq("id", self.id()).eq("user_id", user_id))
             .await
             .ok_or(Error::UnknownSession)
    }

    async fn bot(&self) -> Result<Bot> {
        Bot::find_one_by_id(self.id())
            .await
            .ok_or(Error::UnknownBot)
    }

    async fn member(&self, server_id: u64) -> Result<Member> {
        Member::find_one(|q| q.eq("id", self.id()).eq("server_id", server_id))
            .await
            .ok_or(Error::UnknownMember)
    }

    async fn invite(&self, server_id: Option<u64>) -> Result<Invite> {
        let invite = if let Some(server_id) = server_id {
            Invite::find_one(|q| q.eq("id", self.id()).eq("server_id", server_id)).await
        } else {
            Invite::find_one_by_id(self.id()).await
        };

        invite.ok_or(Error::UnknownInvite)
    }
}

impl Ref for u64 {
    fn id(&self) -> u64 {
        *self
    }
}
