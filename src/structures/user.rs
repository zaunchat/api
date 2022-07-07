use super::*;
use crate::database::pool;
use crate::utils::{snowflake, Badges};
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow, Model, Default, Clone, OpgModel)]
#[ormlite(table = "users")]
pub struct User {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub username: String,
    pub avatar: Option<String>,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub email: String,
    pub badges: Badges,
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
            verified: false,
            ..Default::default()
        }
    }

    pub async fn email_taken(email: &String) -> bool {
        User::select()
            .filter("email = $1")
            .bind(email)
            .fetch_optional(pool())
            .await
            .unwrap()
            .is_some()
    }

    pub async fn fetch_sessions(&self) -> Vec<Session> {
        Session::select()
            .filter("user_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    pub async fn fetch_servers(&self) -> Vec<Server> {
        Server::select()
            .filter("owner_id = $1 OR id IN ( SELECT server_id FROM members WHERE id = $2 )")
            .bind(self.id)
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    pub async fn fetch_bots(&self) -> Vec<Bot> {
        Bot::select()
            .filter("owner_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    pub async fn fetch_channels(&self) -> Vec<Channel> {
        Channel::select()
            .filter("recipients @> ARRAY[$1]::BIGINT[]")
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    // pub async fn fetch_relations(&self) {}

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
        let email = format!("ghost.{}@example.com", nanoid::nanoid!(6));
        let mut user = Self::new("Ghost".to_string(), email, "passw0rd".to_string());
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
            let user = User::faker();
            let user = user.save().await.unwrap();
            let user = User::find_one(user.id).await.unwrap();
            user.remove().await.unwrap();
        })
    }
}
