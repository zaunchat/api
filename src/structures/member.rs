use super::{Base, Role};
use serde::{Deserialize, Serialize};

#[crud_table(table_name:members)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Member {
    pub id: u64,
    pub nickname: Option<String>,
    pub joined_at: u64,
    pub server_id: u64,
    pub roles: Vec<u64>,
}

impl Base for Member {
    fn id(&self) -> u64 { self.id }
}

impl Member {
    pub fn new(user_id: u64, server_id: u64) -> Self {
        Self {
            id: user_id,
            server_id,
            ..Default::default()
        }
    }

    pub async fn fetch_roles(&self) -> Vec<Role> {
        Role::find(|q| q.r#in("id", &self.roles)).await
    }
}
