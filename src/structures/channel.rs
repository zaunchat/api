use super::*;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use rbatis::Json;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel)]
#[repr(u8)]
pub enum ChannelTypes {
    Unknown,
    Direct,
    Group,
    Text,
    Voice,
    Category,
}

impl Default for ChannelTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel, Debug)]
#[repr(u8)]
pub enum OverwriteTypes {
    Role,
    Member,
}

#[derive(Serialize, Deserialize, Clone, Copy, OpgModel, Debug)]
pub struct Overwrite {
    pub id: u64,
    pub r#type: OverwriteTypes,
    pub allow: Permissions,
    pub deny: Permissions,
}

#[crud_table(formats_pg:"server_id:{}::bigint,parent_id:{}::bigint,owner_id:{}::bigint" | table_name:channels)]
#[derive(Serialize, Deserialize, Clone, OpgModel, Debug)]
pub struct Channel {
    pub id: u64,
    pub r#type: ChannelTypes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // DM/Group
    #[opg(any)]
    pub recipients: Json<Option<Vec<u64>>>,

    // Group/Text/Voice/Category
    #[opg(any)]
    pub overwrites: Json<Option<Vec<Overwrite>>>,

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

impl Default for Channel {
    fn default() -> Self {
        Self {
            id: snowflake::generate(),
            r#type: ChannelTypes::Unknown,
            name: None,
            recipients: None.into(),
            overwrites: None.into(),
            server_id: None,
            parent_id: None,
            owner_id: None,
            topic: None,
            permissions: None,
        }
    }
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
            recipients: Some(vec![user, target]).into(),
            ..Default::default()
        }
    }

    pub fn new_group(user: u64, name: String) -> Self {
        Self {
            id: snowflake::generate(),
            name: name.into(),
            r#type: ChannelTypes::Group,
            recipients: Some(vec![user]).into(),
            permissions: Some(*DEFAULT_PERMISSION_DM),
            ..Default::default()
        }
    }

    pub fn new_text(name: String, server_id: u64) -> Self {
        Self {
            r#type: ChannelTypes::Text,
            overwrites: Some(vec![]).into(),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_voice(name: String, server_id: u64) -> Self {
        Self {
            r#type: ChannelTypes::Voice,
            overwrites: Some(vec![]).into(),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_category(name: String, server_id: u64) -> Self {
        Self {
            r#type: ChannelTypes::Category,
            overwrites: Some(vec![]).into(),
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::database::postgres;

//     #[tokio::test]
//     async fn create_channel() {
//         dotenv::dotenv().ok();
//         env_logger::init();
//         postgres::connect().await;

//         let channel = Channel::new_group(1, "Test group".to_string());

//         channel.save().await;
//         // channel.delete().await;
//     }

//     #[test]
//     fn serialize() {
//         let channel = Channel::new_group(1, "Test group".to_string());

//         let x = serde_json::to_string_pretty(&channel).unwrap();

//         println!("{}", x);
//     }

//     // #[test]
//     // fn deserialize() {
//     //     let channel: Channel = serde_json::from_str(r#"{
//     //         "id": 194565395108204544,
//     //         "type": 2,
//     //         "name": "Test group",
//     //         "recipients": null,
//     //         "overwrites": null,
//     //         "server_id": null,
//     //         "parent_id": null,
//     //         "owner_id": null,
//     //         "topic": null,
//     //         "permissions": 62
//     //       }"#).unwrap();

//     //       println!("{:?}", channel);
//     // }
// }
