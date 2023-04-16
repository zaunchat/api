use super::*;
use crate::utils::{Permissions, Snowflake, DEFAULT_PERMISSION_DM};
use ormlite::model::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, OpgModel, sqlx::Type,
)]
#[repr(i32)]
pub enum ChannelTypes {
    Unknown = 0,
    Direct = 1,
    Group = 2,
}

impl Default for ChannelTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Model, FromRow, Clone, OpgModel, Debug)]
#[ormlite(table = "channels")]
pub struct Channel {
    pub id: Snowflake,

    pub r#type: ChannelTypes,

    // Group
    pub name: Option<String>,

    // DM/Group
    pub recipients: Option<Vec<Snowflake>>,

    // Group
    #[serde(default)]
    pub owner_id: Option<Snowflake>,

    // Group
    pub permissions: Option<Permissions>,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            id: Snowflake::generate(),
            r#type: ChannelTypes::Unknown,
            name: None,
            recipients: None,
            owner_id: None,
            permissions: None,
        }
    }
}

impl Channel {
    pub fn new_dm(user: Snowflake, target: Snowflake) -> Self {
        Self {
            r#type: ChannelTypes::Direct,
            recipients: Some(vec![user, target]),
            ..Default::default()
        }
    }

    pub fn new_group(user: Snowflake, name: String) -> Self {
        Self {
            name: name.into(),
            r#type: ChannelTypes::Group,
            recipients: Some(vec![user]),
            owner_id: user.into(),
            permissions: Some(*DEFAULT_PERMISSION_DM),
            ..Default::default()
        }
    }

    pub fn is_group(&self) -> bool {
        self.r#type == ChannelTypes::Group
    }

    pub fn is_dm(&self) -> bool {
        self.r#type == ChannelTypes::Direct
    }

    pub async fn fetch_messages(&self, limit: usize) -> Vec<Message> {
        Message::select()
            .filter("channel_id = $1")
            .bind(self.id)
            .limit(limit)
            .fetch_all(pool())
            .await
            .unwrap_or_default()
    }
}

impl Base for Channel {}
