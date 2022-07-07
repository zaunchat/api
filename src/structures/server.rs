use super::*;
use crate::database::pool;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "servers")]
pub struct Server {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub owner_id: i64,
    pub permissions: Permissions,
}

impl Server {
    pub fn new(name: String, owner_id: i64) -> Self {
        Self {
            id: snowflake::generate(),
            name,
            owner_id,
            permissions: *DEFAULT_PERMISSION_EVERYONE,
            ..Default::default()
        }
    }

    pub async fn fetch_members(&self) -> Vec<Member> {
        Member::select()
            .filter("server_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    pub async fn fetch_member(&self, user_id: i64) -> Option<Member> {
        Member::select()
            .filter("id = $1 AND server_id = $2")
            .bind(user_id)
            .bind(self.id)
            .fetch_optional(pool())
            .await
            .unwrap()
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        Role::select()
            .filter("server_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    pub async fn fetch_channels(&self) -> Vec<Channel> {
        Channel::select()
            .filter("server_id = $1")
            .bind(self.id)
            .fetch_all(pool())
            .await
            .unwrap()
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        let owner = User::faker();
        let server = Self::new("Fake Server".to_string(), owner.id);

        owner.save().await.unwrap();

        server
    }

    #[cfg(test)]
    pub async fn cleanup(self) {
        use crate::utils::Ref;
        self.owner_id.user().await.unwrap().remove().await.unwrap();
    }
}

impl Base for Server {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let server = Server::faker().await;
            let server = server.save().await.unwrap();
            let server = Server::find_one(server.id).await.unwrap();
            server.cleanup().await;
        })
    }
}
