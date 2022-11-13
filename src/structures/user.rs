use super::*;
use crate::utils::{Badges, Private, Snowflake};
use ormlite::model::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, OpgModel, sqlx::Type,
)]
#[repr(i32)]
pub enum RelationshipStatus {
    Friend = 0,
    Incoming = 1,
    Outgoing = 2,
    Blocked = 3,
    BlockedByOther = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, OpgModel, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PresenceStatus {
    Offline = 0,
    Online = 1,
    Idle = 2,
    Dnd = 3,
    // BirthDay = 4 (coming feature)
}

#[derive(Serialize, Deserialize, OpgModel, Debug, Clone)]
pub struct Presence {
    pub status: PresenceStatus,
    pub text: Option<String>,
}

impl Default for Presence {
    fn default() -> Self {
        Self {
            status: PresenceStatus::Offline,
            text: None,
        }
    }
}

impl Presence {
    pub fn is_online(&self) -> bool {
        self.status != PresenceStatus::Offline
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow, Model, Clone, OpgModel)]
#[ormlite(table = "users")]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub badges: Badges,
    pub presence: Json<Presence>,
    #[ormlite(skip)]
    #[sqlx(default)]
    pub relationship: Option<RelationshipStatus>,
    #[serde(skip_serializing_if = "Private::is_private")]
    pub relations: Private<Json<HashMap<Snowflake, RelationshipStatus>>>,
    #[serde(skip_serializing_if = "Private::is_private")]
    pub email: Private<String>,
    #[serde(skip_serializing_if = "Private::is_private")]
    pub password: Private<String>,
    #[serde(skip_serializing_if = "Private::is_private")]
    pub verified: Private<bool>,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: Snowflake::generate(),
            username,
            email: email.into(),
            password: password.into(),
            avatar: None,
            relationship: None,
            verified: false.into(),
            presence: Json(Presence::default()),
            badges: Badges::default(),
            relations: Json(HashMap::new()).into(),
        }
    }

    pub fn with_hidden_fields(&self) -> Self {
        let mut u = self.clone();
        u.verified.set_public();
        u.password.set_public();
        u.email.set_public();
        u.relations.set_public();
        u
    }

    pub async fn email_taken(email: &str) -> bool {
        User::select()
            .filter("email = $1")
            .bind(email)
            .fetch_one(pool())
            .await
            .is_ok()
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
        let ids = self.relations.0.keys().copied().collect::<Vec<_>>();

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
            .fetch_one(pool())
            .await
            .ok()
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
        user.verified = true.into();
        user
    }
}

impl Base for User {}
