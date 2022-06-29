use super::*;
use crate::database::DB as db;
use crate::utils::{snowflake, Badges, Error, Result};
use serde::{Deserialize, Serialize};

#[crud_table(table_name:users | formats_pg:"id:{}::bigint,badges:{}::bigint")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default, Clone, OpgModel)]
pub struct User {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub username: String,
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
        let count = Member::count(|q| q.eq("id", self.id).eq("server_id", server_id)).await;

        if count == 0 {
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
            &format!(
                "SELECT * FROM channels WHERE recipients @> ARRAY[{}]::BIGINT[]",
                self.id
            ),
            vec![],
        )
        .await
        .unwrap()
    }

    // pub async fn fetch_relations(&self) {}

    pub async fn fetch_by_token(token: &str) -> Option<User> {
        db.fetch("SELECT * FROM users WHERE verified = TRUE AND id = ( SELECT user_id FROM sessions WHERE token = $1 )", vec![token.into()]).await.ok()
    }

    pub fn to_public(&self) -> Self {
        Self {
            id: self.id,
            username: self.username.clone(),
            avatar: self.avatar.clone(),
            badges: self.badges,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn faker() -> Self {
        let mut email = format!("ghost.{}@example.com", nanoid::nanoid!(6));

        email = email.replace('\u{0000}', "");

        let mut user = Self::new("Ghost".to_string(), email, "passw0rd".to_string());

        user.verified = true;

        user
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let user = User::faker();

            user.save().await;

            let user = User::find_one_by_id(user.id).await.unwrap();

            user.delete().await;
        })
    }
}
