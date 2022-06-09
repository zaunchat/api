use super::*;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ChannelTypes {
    Direct,
    Group,
    Text,
    Voice,
    Category,
    Unknown
}

impl Default for ChannelTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum OverwriteTypes {
    Role,
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Overwrite {
    pub id: u64,
    pub r#type: OverwriteTypes,
    pub allow: Permissions,
    pub deny: Permissions,
}

#[crud_table(table_name:channels)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Channel {
    pub id: u64,
    pub r#type: ChannelTypes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // DM/Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<u64>>,

    // Group/Text/Voice/Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrites: Option<Vec<Overwrite>>,

    // For server channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<u64>,

    // Server channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,

    // Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<u64>,

    // Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    // Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
}

impl Base for Channel {
    fn id(&self) -> u64 {
        self.id
    }
}


impl Channel {
    pub fn new_dm(user: u64, target: u64) -> Self {
        Self {
            id: snowflake::generate(),
            r#type: ChannelTypes::Direct,
            recipients: vec![user, target].into(),
            ..Default::default()
        }
    }

    pub fn new_group(user: u64, name: String) -> Self {
        Self {
            id: snowflake::generate(),
            name: Some(name),
            r#type: ChannelTypes::Group,
            recipients: vec![user].into(),
            permissions: Some(*DEFAULT_PERMISSION_DM),
            ..Default::default()
        }
    }

    pub fn new_text(name: String, server_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            r#type: ChannelTypes::Text,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_voice(name: String, server_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            r#type: ChannelTypes::Voice,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_category(name: String, server_id: u64) -> Self {
        Self {
            id: snowflake::generate(),
            r#type: ChannelTypes::Category,
            overwrites: vec![].into(),
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
}
