use super::*;
use crate::utils::permissions::*;
use crate::utils::snowflake;
use rbatis::Json;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel)]
#[repr(u8)]
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

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel, Debug)]
#[repr(u8)]
pub enum OverwriteTypes {
    Role = 0,
    Member = 1,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Copy, OpgModel, Debug)]
pub struct Overwrite {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub r#type: OverwriteTypes,
    pub allow: Permissions,
    pub deny: Permissions,
}

#[derive(Serialize, OpgModel)]
pub struct ChannelOverwrites(Option<Vec<Overwrite>>);

#[derive(Serialize, OpgModel)]
pub struct ChannelRecipients(Option<Vec<String>>);

#[crud_table(formats_pg:"id:{}::bigint,server_id:{}::bigint,parent_id:{}::bigint,owner_id:{}::bigint,recipients:{}::bigint[],permissions:{}::bigint" | table_name:channels)]
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, OpgModel, Debug)]
pub struct Channel {
    #[serde_as(as = "snowflake::json::ID")]
    #[opg(string)]
    pub id: u64,
    pub r#type: ChannelTypes,

    // Text/Voice/Category/Group
    pub name: Option<String>,
    // DM/Group
    #[serde_as(as = "Option<Vec<snowflake::json::ID>>")]
    #[opg(custom = "ChannelRecipients")]
    pub recipients: Option<Vec<u64>>,

    // Group/Text/Voice/Category
    #[opg(custom = "ChannelOverwrites")]
    pub overwrites: Json<Option<Vec<Overwrite>>>,

    // For server channels
    #[opg(string, nullable)]
    #[serde_as(as = "Option<snowflake::json::ID>")]
    pub server_id: Option<u64>,

    // Server channels
    #[opg(string, nullable)]
    #[serde_as(as = "Option<snowflake::json::ID>")]
    pub parent_id: Option<u64>,

    // Group
    #[opg(string, nullable)]
    #[serde_as(as = "Option<snowflake::json::ID>")]
    pub owner_id: Option<u64>,

    // Text
    pub topic: Option<String>,

    // Group
    pub permissions: Option<Permissions>,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            id: snowflake::generate(),
            r#type: ChannelTypes::Unknown,
            name: None,
            recipients: None,
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
            r#type: ChannelTypes::Direct,
            recipients: Some(vec![user, target]),
            ..Default::default()
        }
    }

    pub fn new_group(user: u64, name: String) -> Self {
        Self {
            name: name.into(),
            r#type: ChannelTypes::Group,
            recipients: Some(vec![user]),
            owner_id: user.into(),
            permissions: Some(*DEFAULT_PERMISSION_DM),
            overwrites: Some(vec![]).into(),
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

    #[cfg(test)]
    pub async fn faker(r#type: ChannelTypes) -> Self {
        match r#type {
            ChannelTypes::Group => {
                let user = User::faker();

                user.save().await;

                Self::new_group(user.id, "Fake group".to_string())
            }

            ChannelTypes::Direct => {
                let user = User::faker();
                let other = User::faker();

                user.save().await;
                other.save().await;

                Self::new_dm(user.id, other.id)
            }

            ChannelTypes::Text => {
                let server = Server::faker().await;

                server.save().await;

                Self::new_text("Test".to_string(), server.id)
            }

            ChannelTypes::Voice => {
                let server = Server::faker().await;

                server.save().await;

                Self::new_voice("Test".to_string(), server.id)
            }

            ChannelTypes::Category => {
                let server = Server::faker().await;

                server.save().await;

                Self::new_category("Test".to_string(), server.id)
            }

            _ => panic!("Unsupported type"),
        }
    }

    #[cfg(test)]
    pub async fn cleanup(&self) {
        use crate::utils::Ref;

        self.delete().await;

        if self.is_group() || self.is_dm() {
            for id in self.recipients.as_ref().unwrap() {
                id.user().await.unwrap().delete().await;
            }
        }

        if self.in_server() {
            self.server_id
                .unwrap()
                .server()
                .await
                .unwrap()
                .cleanup()
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_group() {
        crate::tests::setup().await;

        let channel = Channel::faker(ChannelTypes::Group).await;

        channel.save().await;
        channel.cleanup().await;
    }

    #[tokio::test]
    async fn create_dm() {
        crate::tests::setup().await;

        let channel = Channel::faker(ChannelTypes::Direct).await;

        channel.save().await;
        channel.cleanup().await;
    }

    #[tokio::test]
    async fn create_text() {
        crate::tests::setup().await;

        let channel = Channel::faker(ChannelTypes::Text).await;

        channel.save().await;
        channel.cleanup().await;
    }

    #[tokio::test]
    async fn create_voice() {
        crate::tests::setup().await;

        let channel = Channel::faker(ChannelTypes::Voice).await;

        channel.save().await;
        channel.cleanup().await;
    }

    #[tokio::test]
    async fn create_category() {
        crate::tests::setup().await;

        let channel = Channel::faker(ChannelTypes::Category).await;

        channel.save().await;
        channel.cleanup().await;
    }
}
