use crate::structures::{Base, Channel, User};
use crate::utils::error::*;
use rocket::request::FromParam;

pub struct Ref(u64);

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

    pub async fn channel(&self) -> Result<Channel> {
        let channel = Channel::find_one_by_id(self.0).await;
        match channel {
            Some(c) => Ok(c),
            _ => Err(Error::UnknownChannel),
        }
    }
}

impl<'r> FromParam<'r> for Ref {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Ref::from_unchecked(param.into()))
    }
}
