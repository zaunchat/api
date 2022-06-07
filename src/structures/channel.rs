use super::Base;
use crate::utils::permissions::DEFAULT_PERMISSION_DM;
use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ChannelTypes {
    Direct,
    Group,
    Text,
    Voice,
    Category,
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
    pub allow: u64,
    pub deny: u64,
}

#[crud_table(table_name:channels)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: u64,
    pub r#type: ChannelTypes,

    pub name: Option<String>,
    // DM/Group
    pub recipients: Option<Vec<u64>>,

    // Group/Text/Voice/Category
    pub overwrites: Option<Vec<Overwrite>>,

    // For server channels
    pub server_id: Option<u64>,

    // Server channels
    pub parent_id: Option<u64>,

    // Group
    pub owner_id: Option<u64>,

    // Text
    pub topic: Option<String>,

    // Group
    pub permissions: Option<u64>,
}

impl Base for Channel {}

impl Channel {
    pub fn new_dm(user: u64, target: u64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelTypes::Direct,
            recipients: vec![user, target].into(),
            permissions: None,
            overwrites: None,
            name: None,
            owner_id: None,
            parent_id: None,
            server_id: None,
            topic: None,
        }
    }

    pub fn new_group(user: u64, name: String) -> Self {
        Self {
            id: generate_id(),
            name: Some(name),
            r#type: ChannelTypes::Group,
            recipients: vec![user].into(),
            permissions: Some(DEFAULT_PERMISSION_DM.bits()),
            overwrites: None,
            owner_id: None,
            parent_id: None,
            server_id: None,
            topic: None,
        }
    }

    pub fn new_text(name: String, server_id: u64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelTypes::Text,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            permissions: None,
            recipients: None,
            owner_id: None,
            parent_id: None,
            topic: None,
        }
    }

    pub fn new_voice(name: String, server_id: u64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelTypes::Voice,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            permissions: None,
            recipients: None,
            owner_id: None,
            parent_id: None,
            topic: None,
        }
    }

    pub fn new_category(name: String, server_id: u64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelTypes::Category,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            permissions: None,
            recipients: None,
            owner_id: None,
            parent_id: None,
            topic: None,
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
