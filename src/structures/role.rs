use super::*;
use crate::utils::{snowflake, Permissions};
use ormlite::model::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Model, FromRow, Clone, Default, OpgModel)]
#[ormlite(table = "roles")]
pub struct Role {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub name: String,
    pub permissions: Permissions,
    pub color: i32,
    pub hoist: bool,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub server_id: i64,
}

impl Role {
    pub fn new(name: String, server_id: i64) -> Self {
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
        let role = Self::new("Mod".to_string(), server.id);
        server.save().await.unwrap();
        role
    }
}

impl Base for Role {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create() {
        run(async {
            let role = Role::faker().await.save().await.unwrap();
            Role::find_one(role.id).await.unwrap();
        })
    }
}
