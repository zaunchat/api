use super::*;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:servers | formats_pg:"id:{}::bigint,owner_id:{}::bigint,permissions:{}::bigint")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default, OpgModel)]
pub struct Server {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub owner_id: u64,
    pub permissions: Permissions,
}

impl Base for Server {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Server {
    pub fn new(name: String, owner_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            name,
            owner_id,
            permissions: *DEFAULT_PERMISSION_EVERYONE,
            ..Default::default()
        }
    }

    pub async fn fetch_members(&self) -> Vec<Member> {
        Member::find(|q| q.eq("server_id", &self.id)).await
    }

    pub async fn fetch_member(&self, user_id: u64) -> Option<Member> {
        Member::find_one(|q| q.eq("id", user_id).eq("server_id", &self.id)).await
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        Role::find(|q| q.eq("server_id", &self.id)).await
    }

    pub async fn fetch_channels(&self) -> Vec<Channel> {
        Channel::find(|q| q.eq("server_id", &self.id)).await
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        let owner = User::faker();

        owner.save().await;

        Self::new("Fake Server".to_string(), owner.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.owner_id.user().await.unwrap().delete().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() {
        crate::tests::setup().await;

        let server = Server::faker().await;

        server.save().await;

        let server = Server::find_one_by_id(server.id).await.unwrap();

        server.cleanup().await;
    }
}
