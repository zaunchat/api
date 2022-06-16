use crate::structures::*;
use crate::utils::error::*;
use bitflags::bitflags;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

bitflags! {
    pub struct Permissions: u64 {
          const ADMINISTRATOR = 1 << 0;
          const VIEW_CHANNEL = 1 << 1;
          const SEND_MESSAGES = 1 << 2;
          const READ_MESSAGE_HISTORY = 1 << 3;
          const EMBED_LINKS = 1 << 4;
          const UPLOAD_FILES = 1 << 5;
          const MANAGE_SERVER = 1 << 6;
          const MANAGE_CHANNELS = 1 << 7;
          const MANAGE_MESSAGES = 1 << 8;
          const MANAGE_ROLES = 1 << 9;
          const MANAGE_INVITES = 1 << 10;
          const MANAGE_NICKNAMES = 1 << 11;
          const BAN_MEMBERS = 1 << 12;
          const KICK_MEMBERS = 1 << 13;
          const CHANGE_NICKNAME = 1 << 14;
          const INVITE_OTHERS = 1 << 15;
          const DEFAULT = 0;
    }
}

lazy_static! {
    pub static ref DEFAULT_PERMISSION_DM: Permissions = Permissions::DEFAULT
        | Permissions::VIEW_CHANNEL
        | Permissions::SEND_MESSAGES
        | Permissions::EMBED_LINKS
        | Permissions::UPLOAD_FILES
        | Permissions::READ_MESSAGE_HISTORY;
    pub static ref DEFAULT_PERMISSION_EVERYONE: Permissions = Permissions::DEFAULT
        | Permissions::VIEW_CHANNEL
        | Permissions::SEND_MESSAGES
        | Permissions::EMBED_LINKS
        | Permissions::UPLOAD_FILES
        | Permissions::READ_MESSAGE_HISTORY;
}

impl Permissions {
    pub async fn fetch(
        user: &User,
        server_id: Option<u64>,
        channel_id: Option<u64>,
    ) -> Result<Permissions> {
        let mut p = Permissions::DEFAULT;
        let admin = || Permissions::ADMINISTRATOR;

        if let Some(id) = server_id {
            let server = Server::find_one_by_id(id).await;

            if server.is_none() {
                return Err(Error::UnknownServer);
            }

            let server = server.unwrap();

            if server.owner_id == user.id {
                return Ok(admin());
            }

            p.set(Permissions::ADMINISTRATOR, server.owner_id == user.id);
            p.insert(server.permissions);

            if p.contains(Permissions::ADMINISTRATOR) {
                return Ok(p);
            }

            let member = server.fetch_member(user.id).await.unwrap();
            let roles = server.fetch_roles().await;

            for role in roles {
                if member.roles.contains(&role.id) {
                    p.insert(role.permissions);
                }
            }
        }

        if p.contains(Permissions::ADMINISTRATOR) {
            return Ok(p);
        }

        if let Some(id) = channel_id {
            let channel = Channel::find_one_by_id(id).await;

            if channel.is_none() {
                return Err(Error::UnknownChannel);
            }

            let channel = channel.unwrap();

            if channel.is_dm() {
                p.insert(*DEFAULT_PERMISSION_DM);
                // TODO: Check user relations
            }

            if channel.is_group()
                || channel.is_text()
                || channel.is_voice()
                || channel.is_category()
            {
                // for group owners
                if channel.owner_id == Some(user.id) {
                    return Ok(admin());
                }

                let mut member: Option<Member> = None;

                if channel.is_group() {
                    p.insert(channel.permissions.unwrap());
                } else {
                    member = Member::find_one(|q| {
                        q.eq("id", user.id)
                            .eq("server_id", channel.server_id.unwrap())
                    })
                    .await;
                }

                let mut overwrites = channel.overwrites.unwrap();

                if let Some(parent_id) = channel.parent_id {
                    let category = Channel::find_one_by_id(parent_id).await;
                    if let Some(category) = category {
                        overwrites.append(category.overwrites.unwrap().as_mut());
                    }
                }

                for overwrite in overwrites {
                    if overwrite.r#type == OverwriteTypes::Member && overwrite.id == user.id {
                        p.insert(overwrite.allow);
                        p.remove(overwrite.deny);
                    }

                    if overwrite.r#type == OverwriteTypes::Role
                        && member.as_ref().unwrap().roles.contains(&overwrite.id)
                    {
                        p.insert(overwrite.allow);
                        p.remove(overwrite.deny);
                    }
                }
            }
        }

        Ok(p)
    }

    pub fn has(&self, bits: Permissions): Result<()> {
        if !self.contains(bits) {
            return Err(Error::MissingPermissions)
        }
        Ok(())
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::DEFAULT
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

struct PermissionsVisitor;

impl<'de> Visitor<'de> for PermissionsVisitor {
    type Value = Permissions;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid permissions")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(p) = Permissions::from_bits(v) {
            return Ok(p);
        }
        Err(E::custom("Invalid Permissions"))
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(PermissionsVisitor)
    }
}

use utoipa::openapi::{schema::Component, ComponentType, PropertyBuilder};

impl utoipa::Component for Permissions {
    fn component() -> Component {
        PropertyBuilder::new()
            .component_type(ComponentType::Integer)
            .into()
    }
}
