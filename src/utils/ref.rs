use crate::database::DB as db;
use crate::structures::*;
use crate::utils::error::*;

#[async_trait]
pub trait Ref {
    fn id(&self) -> u64;

    async fn user(&self) -> Result<User> {
        let user = User::find_one_by_id(self.id()).await;
        match user {
            Some(u) => Ok(u),
            _ => Err(Error::UnknownUser),
        }
    }

    async fn channel(&self, recipient: Option<u64>) -> Result<Channel> {
        let channel = if let Some(recipient) = recipient {
            db.fetch(
                "SELECT * FROM channels WHERE recipients::jsonb ? $1 AND id = $2 LIMIT 1",
                vec![recipient.into(), self.id().into()],
            )
            .await
            .unwrap()
        } else {
            Channel::find_one_by_id(self.id()).await
        };

        match channel {
            Some(c) => Ok(c),
            _ => Err(Error::UnknownChannel),
        }
    }

    async fn message(&self) -> Result<Message> {
        let message = Message::find_one_by_id(self.id()).await;
        match message {
            Some(m) => Ok(m),
            _ => Err(Error::UnknownMessage),
        }
    }

    async fn server(&self) -> Result<Server> {
        let server = Server::find_one_by_id(self.id()).await;
        match server {
            Some(s) => Ok(s),
            _ => Err(Error::UnknownServer),
        }
    }

    async fn role(&self, server_id: u64) -> Result<Role> {
        let role = Role::find_one(|q| q.eq("id", self.id()).eq("server_id", server_id)).await;
        match role {
            Some(r) => Ok(r),
            _ => Err(Error::UnknownRole),
        }
    }

    async fn session(&self, user_id: u64) -> Result<Session> {
        let session = Session::find_one(|q| q.eq("id", self.id()).eq("user_id", user_id)).await;
        match session {
            Some(s) => Ok(s),
            _ => Err(Error::UnknownSession),
        }
    }

    async fn bot(&self) -> Result<Bot> {
        let bot = Bot::find_one(|q| q.eq("id", self.id())).await;
        match bot {
            Some(b) => Ok(b),
            _ => Err(Error::UnknownBot),
        }
    }

    async fn member(&self, server_id: u64) -> Result<Member> {
        let member = Member::find_one(|q| q.eq("id", self.id()).eq("server_id", server_id)).await;
        match member {
            Some(m) => Ok(m),
            _ => Err(Error::UnknownMember),
        }
    }

    async fn invite(&self, server_id: Option<u64>) -> Result<Invite> {
        let invite = if let Some(server_id) = server_id {
            Invite::find_one(|q| q.eq("id", self.id()).eq("server_id", server_id)).await
        } else {
            Invite::find_one_by_id(self.id()).await
        };

        match invite {
            Some(i) => Ok(i),
            _ => Err(Error::UnknownInvite),
        }
    }
}

impl Ref for u64 {
    fn id(&self) -> u64 {
        *self
    }
}
