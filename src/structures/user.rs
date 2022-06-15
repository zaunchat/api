use super::*;
use crate::database::DB as db;
use crate::utils::{Badges, Result, Error, snowflake};
use serde::{Deserialize, Serialize};

#[crud_table(table_name:users)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, utoipa::Component)]
pub struct User {
    pub id: u64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub password: String,
    pub email: String,
    pub badges: Badges,
    pub verified: bool,
}

impl Base for User {
    fn id(&self) -> u64 {
        self.id
    }
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: snowflake::generate(),
            username,
            email,
            password,
            verified: false,
            ..Default::default()
        }
    }

    pub async fn member_of(&self, server_id: u64) -> Result<()> {
        let exists = Member::count(|q| q.eq("id", self.id).eq("server_id", server_id)).await;

        if !exists {
            return Err(Error::UnknownServer);
        }

        Ok(())
    }

    pub async fn email_taken(email: &String) -> bool {
        User::find_one(|q| q.eq("email", &email)).await.is_some()
    }

    pub async fn fetch_sessions(&self) -> Vec<Session> {
        Session::find(|q| q.eq("user_id", &self.id)).await
    }

    pub async fn fetch_servers(&self) -> Vec<Server> {
        db.fetch("SELECT * FROM servers WHERE owner_id = $1 OR id IN ( SELECT server_id FROM members WHERE id = $1 )", vec![self.id.into()]).await.unwrap()
    }

    pub async fn fetch_bots(&self) -> Vec<Bot> {
        Bot::find(|q| q.eq("owner_id", &self.id)).await
    }

    pub async fn fetch_channels(&self) -> Vec<Channel> {
        db.fetch(
            "SELECT * FROM channels WHERE recipients::jsonb ? $1",
            vec![self.id.into()],
        )
        .await
        .unwrap()
    }

    // pub async fn fetch_relations(&self) {}

    #[sql(crate::database::DB, "SELECT * FROM users LEFT JOIN sessions ON sessions.user_id = users.id WHERE verified = TRUE AND sessions.token = $1 LIMIT 1")]
    pub async fn fetch_by_token(_token: &str) -> Result<User, rbatis::Error> {}

    pub fn to_public(&self) -> Self {
        Self {
            id: self.id,
            username: self.username.clone(),
            avatar: self.avatar.clone(),
            badges: self.badges,
            ..Default::default()
        }
    }
}
