use super::{Base, Role};
use serde::{Deserialize, Serialize};

#[crud_table(table_name:members)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Member {
    pub id: i64,
    pub nickname: Option<String>,
    pub joined_at: i64,
    pub server_id: i64,
    pub roles: Vec<i64>,
}

impl Base for Member {}

impl Member {
    pub fn new(user_id: i64, server_id: i64) -> Self {
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
