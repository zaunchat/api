use super::{Base, Role};
use rbatis::types::Timestamp;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:members)]
#[derive(Debug, Serialize, Deserialize, Clone, OpgModel)]
pub struct Member {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[opg(integer)]
    pub joined_at: Timestamp,
    pub server_id: u64,
    pub roles: Vec<i64>,
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
            roles: vec![server_id as i64],
            joined_at: Timestamp::now(),
        }
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        Role::find(|q| q.r#in("id", &self.roles)).await
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
        self.server_id.server().await.unwrap().cleanup().await;
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
        member.cleanup().await;
    }
}
