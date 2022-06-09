use crate::database::DB as db;
use crate::structures::*;
use crate::utils::error::*;
use rocket::request::FromParam;

pub struct Ref(pub u64);

impl Ref {
    pub fn from_unchecked(id: String) -> Ref {
        Ref(id.parse().unwrap())
    }

    pub async fn user(&self) -> Result<User> {
        let user = User::find_one_by_id(self.0).await;
        match user {
            Some(u) => Ok(u),
            _ => Err(Error::UnknownUser),
        }
    }

    pub async fn channel(&self, recipient: Option<u64>) -> Result<Channel> {
        let channel = if let Some(recipient) = recipient {
            db.fetch(
                "SELECT * FROM channels WHERE recipients::jsonb ? $1 AND id = $2 LIMIT 1",
                vec![recipient.into(), self.0.into()],
            )
            .await
            .unwrap()
        } else {
            Channel::find_one_by_id(self.0).await
        };

        match channel {
            Some(c) => Ok(c),
            _ => Err(Error::UnknownChannel),
        }
    }

    pub async fn message(&self) -> Result<Message> {
        let message = Message::find_one_by_id(self.0).await;
        match message {
            Some(m) => Ok(m),
            _ => Err(Error::UnknownMessage),
        }
    }

    pub async fn server(&self) -> Result<Server> {
        let server = Server::find_one_by_id(self.0).await;
        match server {
            Some(s) => Ok(s),
            _ => Err(Error::UnknownServer),
        }
    }

    pub async fn role(&self, server_id: u64) -> Result<Role> {
        let role = Role::find_one(|q| q.eq("id", self.0).eq("server_id", server_id)).await;
        match role {
            Some(r) => Ok(r),
            _ => Err(Error::UnknownRole),
        }
    }

    pub async fn session(&self, user_id: u64) -> Result<Session> {
        let session = Session::find_one(|q| q.eq("id", self.0).eq("user_id", user_id)).await;
        match session {
            Some(s) => Ok(s),
            _ => Err(Error::UnknownSession),
        }
    }

    pub async fn bot(&self) -> Result<Bot> {
        let bot = Bot::find_one(|q| q.eq("id", self.0)).await;
        match bot {
            Some(b) => Ok(b),
            _ => Err(Error::UnknownBot),
        }
    }

    pub async fn member(&self, server_id: u64) -> Result<Member> {
        let member = Member::find_one(|q| q.eq("id", self.0).eq("server_id", server_id)).await;
        match member {
            Some(m) => Ok(m),
            _ => Err(Error::UnknownMember),
        }
    }

    pub async fn invite(&self, server_id: Option<u64>) -> Result<Invite> {
        let invite = if let Some(server_id) = server_id {
            Invite::find_one(|q| q.eq("id", self.0).eq("server_id", server_id)).await
        } else {
            Invite::find_one_by_id(self.0).await
        };

        match invite {
            Some(i) => Ok(i),
            _ => Err(Error::UnknownInvite),
        }
    }
}

impl<'r> FromParam<'r> for Ref {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Ref::from_unchecked(param.into()))
    }
}

use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
use schemars::JsonSchema;

impl JsonSchema for Ref {
    fn schema_name() -> String {
        "Id".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Integer))),
            ..Default::default()
        })
    }
}
