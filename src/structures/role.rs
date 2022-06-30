use super::*;
use crate::utils::{snowflake, Permissions};
use serde::{Deserialize, Serialize};

#[crud_table(table_name:roles | formats_pg:"id:{}::bigint,permissions:{}::bigint,server_id:{}::bigint")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default, OpgModel)]
pub struct Role {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub name: String,
    pub permissions: Permissions,
    pub color: u8,
    pub hoist: bool,
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub server_id: u64,
}

impl Base for Role {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Role {
    pub fn new(name: String, server_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            name,
            server_id,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub async fn faker() -> Self {
        let server = Server::faker().await;
        server.save().await;
        Self::new("Mod".to_string(), server.id)
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;
        self.delete().await;
        self.server_id.server(None).await.unwrap().delete().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() {
        crate::tests::setup().await;

        let role = Role::faker().await;

        role.save().await;

        let role = Role::find_one_by_id(role.id).await.unwrap();

        role.cleanup().await;
    }
}
