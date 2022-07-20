use super::*;
use crate::utils::{snowflake, Badges};
use ormlite::model::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel, sqlx::Type)]
#[repr(i32)]
pub enum RelationshipStatus {
    Friend = 0,
    Incoming = 1,
    Outgoing = 2,
    Blocked = 3,
    BlockedByOther = 4,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow, Model, Default, Clone, OpgModel)]
#[ormlite(table = "users")]
pub struct User {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub username: String,
    pub avatar: Option<String>,
    pub badges: Badges,
    #[serde(skip)]
    pub relations: Json<HashMap<i64, RelationshipStatus>>,
    #[ormlite(skip)]
    #[sqlx(default)]
    pub relationship: Option<RelationshipStatus>,
    // Private fields
    #[serde(skip)]
    pub email: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub verified: bool,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: snowflake::generate(),
            username,
            email,
            password,
            ..Default::default()
        }
    }

    pub async fn email_taken(email: &str) -> bool {
        User::select()
            .filter("email = $1")
            .bind(email)
            .fetch_optional(pool())
            .await
            .unwrap()
            .is_some()
    }

    pub async fn fetch_sessions(&self) -> Result<Vec<Session>, ormlite::Error> {
        Session::select()
            .filter("user_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_servers(&self) -> Result<Vec<Server>, ormlite::Error> {
        Server::select()
            .filter("owner_id = $1 OR id IN ( SELECT server_id FROM members WHERE id = $2 )")
            .bind(self.id)
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_bots(&self) -> Result<Vec<Bot>, ormlite::Error> {
        Bot::select()
            .filter("owner_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_channels(&self) -> Result<Vec<Channel>, ormlite::Error> {
        Channel::select()
            .filter("recipients @> ARRAY[$1]::BIGINT[]")
            .bind(self.id)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_relations(&self) -> Result<Vec<User>, ormlite::Error> {
        let ids: Vec<i64> = vec![]; //self.relations.0.keys().copied().collect();

        if ids.is_empty() {
            return Ok(vec![]);
        }

        User::select()
            .filter("id = ANY($1)")
            .bind(ids)
            .fetch_all(pool())
            .await
    }

    pub async fn fetch_by_token(token: &str) -> Option<User> {
        User::select()
            .filter("verified = TRUE AND id = ( SELECT user_id FROM sessions WHERE token = $1 )")
            .bind(token)
            .fetch_optional(pool())
            .await
            .unwrap()
    }

    #[cfg(test)]
    pub fn faker() -> Self {
        use argon2::Config;

        let config = Config::default();
        let salt = nanoid::nanoid!(24);
        let hashed_password =
            argon2::hash_encoded("passw0rd".as_bytes(), salt.as_bytes(), &config).unwrap();

        let email = format!("ghost.{}@example.com", nanoid::nanoid!(6));
        let mut user = Self::new("Ghost".to_string(), email, hashed_password);
        user.verified = true;
        user
    }
}

impl Base for User {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let user = User::faker().save().await.unwrap();
            User::find_one(user.id).await.unwrap();
        });
    }
}
