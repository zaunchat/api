use super::*;
use crate::utils::{snowflake, Permissions, DEFAULT_PERMISSION_DM};
use ormlite::model::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::types::Json;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel, sqlx::Type)]
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

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, OpgModel, Debug)]
#[repr(i32)]
pub enum OverwriteTypes {
    Role = 0,
    Member = 1,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Copy, OpgModel, Debug)]
pub struct Overwrite {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,
    pub r#type: OverwriteTypes,
    pub allow: Permissions,
    pub deny: Permissions,
}

#[derive(Serialize, OpgModel)]
pub struct ChannelOverwrites(Option<Vec<Overwrite>>);

#[derive(Serialize, OpgModel)]
pub struct ChannelRecipients(Option<Vec<String>>);

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Model, FromRow, Clone, OpgModel, Debug)]
#[ormlite(table = "channels")]
pub struct Channel {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub id: i64,

    pub r#type: ChannelTypes,

    // Text/Voice/Category/Group
    pub name: Option<String>,
    // DM/Group
    #[serde_as(as = "Option<Vec<serde_with::DisplayFromStr>>")]
    #[opg(custom = "ChannelRecipients")]
    pub recipients: Option<Vec<i64>>,

    // Group/Text/Voice/Category
    #[opg(custom = "ChannelOverwrites")]
    pub overwrites: Option<Json<Vec<Overwrite>>>,

    // For server channels
    #[opg(string, nullable)]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(default)]
    pub server_id: Option<i64>,

    // Server channels
    #[opg(string, nullable)]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(default)]
    pub parent_id: Option<i64>,

    // Group
    #[opg(string, nullable)]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(default)]
    pub owner_id: Option<i64>,

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
    pub fn new_dm(user: i64, target: i64) -> Self {
        Self {
            r#type: ChannelTypes::Direct,
            recipients: Some(vec![user, target]),
            ..Default::default()
        }
    }

    pub fn new_group(user: i64, name: String) -> Self {
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

    pub fn new_text(name: String, server_id: i64) -> Self {
        Self {
            r#type: ChannelTypes::Text,
            overwrites: Some(Json(vec![])),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_voice(name: String, server_id: i64) -> Self {
        Self {
            r#type: ChannelTypes::Voice,
            overwrites: Some(Json(vec![])),
            name: name.into(),
            server_id: server_id.into(),
            ..Default::default()
        }
    }

    pub fn new_category(name: String, server_id: i64) -> Self {
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
    pub async fn faker(r#type: ChannelTypes) -> Self {
        match r#type {
            ChannelTypes::Group => {
                let user = User::faker();
                let channel = Self::new_group(user.id, "Fake group".to_string());

                user.insert(pool()).await.unwrap();

                channel
            }

            ChannelTypes::Direct => {
                let user = User::faker();
                let other = User::faker();
                let channel = Self::new_dm(user.id, other.id);

                user.insert(pool()).await.unwrap();
                other.insert(pool()).await.unwrap();

                channel
            }

            ChannelTypes::Text => {
                let server = Server::faker().await;
                let channel = Self::new_text("Test".to_string(), server.id);

                server.insert(pool()).await.unwrap();

                channel
            }

            ChannelTypes::Voice => {
                let server = Server::faker().await;
                let channel = Self::new_voice("Test".to_string(), server.id);

                server.insert(pool()).await.unwrap();

                channel
            }

            ChannelTypes::Category => {
                let server = Server::faker().await;
                let channel = Self::new_category("Test".to_string(), server.id);

                server.insert(pool()).await.unwrap();

                channel
            }

            _ => panic!("Unsupported type"),
        }
    }

    #[cfg(test)]
    pub async fn cleanup(self) {
        use crate::utils::Ref;

        if self.is_group() || self.is_dm() {
            for id in self.recipients.as_ref().unwrap() {
                id.user().await.unwrap().delete(pool()).await.unwrap();
            }

            if self.owner_id.is_none() {
                self.delete(pool()).await.unwrap();
            }
        } else if self.in_server() {
            self.server_id
                .unwrap()
                .server(None)
                .await
                .unwrap()
                .cleanup()
                .await;
        }
    }
}

impl Base for Channel {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;

    #[test]
    fn create_group() {
        run(async {
            let channel = Channel::faker(ChannelTypes::Group).await;
            let channel = channel.insert(pool()).await.unwrap();
            let channel = Channel::get_one(channel.id, pool()).await.unwrap();
            channel.cleanup().await;
        });
    }

    #[test]
    fn create_dm() {
        run(async {
            let channel = Channel::faker(ChannelTypes::Direct).await;
            let channel = channel.insert(pool()).await.unwrap();
            let channel = Channel::get_one(channel.id, pool()).await.unwrap();

            channel.cleanup().await;
        });
    }

    #[test]
    fn create_text() {
        run(async {
            let channel = Channel::faker(ChannelTypes::Text).await;
            let channel = channel.insert(pool()).await.unwrap();
            let channel = Channel::get_one(channel.id, pool()).await.unwrap();

            channel.cleanup().await;
        });
    }

    #[test]
    fn create_voice() {
        run(async {
            let channel = Channel::faker(ChannelTypes::Voice).await;
            let channel = channel.insert(pool()).await.unwrap();
            let channel = Channel::get_one(channel.id, pool()).await.unwrap();

            channel.cleanup().await;
        });
    }

    #[test]
    fn create_category() {
        run(async {
            let channel = Channel::faker(ChannelTypes::Category).await;
            let channel = channel.insert(pool()).await.unwrap();
            let channel = Channel::get_one(channel.id, pool()).await.unwrap();

            channel.cleanup().await;
        });
    }
}
