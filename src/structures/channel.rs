use super::Base;
use crate::utils::snowflake::generate_id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChannelType {
    Direct,
    Group,
    Text,
    Voice,
    Category,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum OverwriteTypes {
    Role,
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Overwrite {
    pub id: i64,
    pub r#type: OverwriteTypes,
    pub allow: i64,
    pub deny: i64,
}

#[crud_table(table_name:channels)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: i64,
    pub r#type: ChannelType,

    pub name: Option<String>,
    // DM/Group
    pub recipients: Option<Vec<i64>>,

    // Group/Text/Voice/Category
    pub overwrites: Option<Vec<Overwrite>>,

    // For server channels
    pub server_id: Option<i64>,

    // Server channels
    pub parent_id: Option<i64>,

    // Group
    pub owner_id: Option<i64>,

    // Text
    pub topic: Option<String>,
}

impl Base for Channel {}

impl Channel {
    pub fn new_dm(user: i64, target: i64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelType::Direct,
            recipients: vec![user, target].into(),
            overwrites: None,
            name: None,
            owner_id: None,
            parent_id: None,
            server_id: None,
            topic: None,
        }
    }

    pub fn new_group(user: i64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelType::Group,
            recipients: vec![user].into(),
            overwrites: None,
            name: None,
            owner_id: None,
            parent_id: None,
            server_id: None,
            topic: None,
        }
    }

    pub fn new_text(name: String, server_id: i64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelType::Text,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            recipients: None,
            owner_id: None,
            parent_id: None,
            topic: None,
        }
    }

    pub fn new_voice(name: String, server_id: i64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelType::Voice,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            recipients: None,
            owner_id: None,
            parent_id: None,
            topic: None,
        }
    }

    pub fn new_category(name: String, server_id: i64) -> Self {
        Self {
            id: generate_id(),
            r#type: ChannelType::Category,
            overwrites: vec![].into(),
            name: name.into(),
            server_id: server_id.into(),
            recipients: None,
            owner_id: None,
            parent_id: None,
            topic: None,
        }
    }
}
