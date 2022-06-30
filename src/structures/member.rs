use super::{Base, Role};
use crate::utils::snowflake;
use rbatis::types::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, OpgModel)]
struct MemberRoles(Vec<String>);

#[crud_table(table_name:members | formats_pg:"id:{}::bigint,server_id:{}::bigint,roles:{}::bigint[]")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, OpgModel)]
pub struct Member {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub nickname: Option<String>,
    #[opg(string)]
    pub joined_at: Timestamp,
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub server_id: u64,
    #[serde_as(as = "Vec<snowflake::json::ID>")]
    #[opg(custom = "MemberRoles")]
    pub roles: Vec<u64>,
}

impl Base for Member {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Member {
    pub fn new(user_id: u64, server_id: u64) -> Self {
        Self {
            id: user_id,
            nickname: None,
            server_id,
            roles: vec![server_id],
            joined_at: Timestamp::now(),
        }
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        if self.roles.is_empty() {
            return vec![];
        }

        Role::find(|q| q.eq("server_id", &self.server_id).r#in("id", &self.roles)).await
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        use crate::structures::Server;

        let server = Server::faker().await;

        server.save().await;

        Self::new(server.owner_id, server.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.server_id.server(None).await.unwrap().cleanup().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() {
        crate::tests::setup().await;

        let member = Member::faker().await;

        member.save().await;

        let member = Member::find_one(|q| q.eq("id", member.id).eq("server_id", member.server_id))
            .await
            .unwrap();

        member.cleanup().await;
    }
}
