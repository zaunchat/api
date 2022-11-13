use super::*;
use crate::utils::{Permissions, Snowflake, DEFAULT_PERMISSION_DM};
use ormlite::model::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::types::Json;

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, OpgModel, sqlx::Type,
)]
#[repr(i32)]
pub enum ChannelTypes {
    Unknown = 0,
    Direct = 1,
    Group = 2,
    Category = 3,
    Text = 4,
    Voice = 5,
}

impl Default for ChannelTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, OpgModel, Debug)]
#[repr(i32)]
pub enum OverwriteTypes {
    Role = 0,
    Member = 1,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Copy, OpgModel, Debug)]
pub struct Overwrite {
    pub id: Snowflake,
    pub r#type: OverwriteTypes,
    pub allow: Permissions,
    pub deny: Permissions,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Model, FromRow, Clone, OpgModel, Debug)]
#[ormlite(table = "channels")]
pub struct Channel {
    pub id: Snowflake,

    pub r#type: ChannelTypes,

    // Text/Voice/Category/Group
    pub name: Option<String>,

    // DM/Group
    pub recipients: Option<Vec<Snowflake>>,

    // Group/Text/Voice/Category
    pub overwrites: Option<Json<Vec<Overwrite>>>,

    // For server channels
    #[serde(default)]
    pub server_id: Option<Snowflake>,

    // Server channels
    #[serde(default)]
    pub parent_id: Option<Snowflake>,

    // Group
    #[serde(default)]
    pub owner_id: Option<Snowflake>,

    // Text
    pub topic: Option<String>,

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
            overwrites: None,
            server_id: None,
            parent_id: None,
            owner_id: None,
            topic: None,
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
            overwrites: Some(Json(vec![])),
            ..Default::default()
        }
    }

    pub fn new_text(name: String, server_id: Snowflake) -> Self {
        Self {
            r#type: ChannelTypes::Text,
            overwrites: Some(Json(vec![])),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_voice(name: String, server_id: Snowflake) -> Self {
        Self {
            r#type: ChannelTypes::Voice,
            overwrites: Some(Json(vec![])),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_category(name: String, server_id: Snowflake) -> Self {
        Self {
            r#type: ChannelTypes::Category,
            overwrites: Some(Json(vec![])),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn is_group(&self) -> bool {
        self.r#type == ChannelTypes::Group
    }

    pub fn is_text(&self) -> bool {
        self.r#type == ChannelTypes::Text
    }

    pub fn is_dm(&self) -> bool {
        self.r#type == ChannelTypes::Direct
    }

    pub fn is_category(&self) -> bool {
        self.r#type == ChannelTypes::Category
    }

    pub fn is_voice(&self) -> bool {
        self.r#type == ChannelTypes::Voice
    }

    pub fn in_server(&self) -> bool {
        self.server_id.is_some()
    }

    #[cfg(test)]
    pub async fn faker(r#type: ChannelTypes) -> Result<Self, Error> {
        let channel;

        match r#type {
            ChannelTypes::Group => {
                let user = User::faker();
                channel = Self::new_group(user.id, "Fake group".to_string());

                user.save().await?;
            }

            ChannelTypes::Direct => {
                let user = User::faker();
                let other = User::faker();
                channel = Self::new_dm(user.id, other.id);

                user.save().await?;
                other.save().await?;
            }

            ChannelTypes::Text => {
                let server = Server::faker().await?;
                channel = Self::new_text("Test".to_string(), server.id);

                server.save().await?;
            }

            ChannelTypes::Voice => {
                let server = Server::faker().await?;
                channel = Self::new_voice("Test".to_string(), server.id);

                server.save().await?;
            }

            ChannelTypes::Category => {
                let server = Server::faker().await?;
                channel = Self::new_category("Test".to_string(), server.id);

                server.save().await?;
            }
            _ => panic!("Unsupported type"),
        }

        Ok(channel)
    }
}

impl Base for Channel {}
