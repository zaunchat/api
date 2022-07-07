use crate::structures::*;
use crate::utils::{Error, Ref, Result};
use bitflags::bitflags;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
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
    pub async fn fetch_cached(
        user: &User,
        server: Option<&Server>,
        channel: Option<&Channel>,
    ) -> Result<Permissions> {
        let mut p = Permissions::DEFAULT;

        if let Some(server) = server {
            p.set(Permissions::ADMINISTRATOR, server.owner_id == user.id);
            p.insert(server.permissions);

            if p.contains(Permissions::ADMINISTRATOR) {
                return Ok(p);
            }

            let member = user.id.member(server.id).await?;

            for role in server.fetch_roles().await {
                if member.roles.contains(&role.id) {
                    p.insert(role.permissions);
                }
            }
        }

        if let Some(channel) = channel {
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
                    p.set(Permissions::ADMINISTRATOR, true);
                    return Ok(p);
                }

                let mut member: Option<Member> = None;

                if channel.is_group() {
                    p.insert(channel.permissions.unwrap());
                } else {
                    member = Some(user.id.member(channel.server_id.unwrap()).await?);
                }

                let mut overwrites = channel.overwrites.as_ref().unwrap().0.clone();

                if let Some(parent_id) = channel.parent_id {
                    if let Ok(category) = parent_id.channel(None).await {
                        overwrites.append(category.overwrites.unwrap().0.as_mut());
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

    pub async fn fetch(
        user: &User,
        server_id: Option<i64>,
        channel_id: Option<i64>,
    ) -> Result<Permissions> {
        let server = if let Some(server_id) = server_id {
            Some(server_id.server(user.id.into()).await?)
        } else {
            None
        };

        let channel = if let Some(channel_id) = channel_id {
            Some(channel_id.channel(None).await?)
        } else {
            None
        };

        Permissions::fetch_cached(user, server.as_ref(), channel.as_ref()).await
    }

    pub fn has(&self, bits: Permissions) -> Result<()> {
        if !self.contains(Permissions::ADMINISTRATOR) && !self.contains(bits) {
            return Err(Error::MissingPermissions);
        }

        Ok(())
    }
}

impl Type<Postgres> for Permissions {
    fn type_info() -> PgTypeInfo {
        i64::type_info()
    }
}

impl Encode<'_, Postgres> for Permissions {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        Encode::<Postgres>::encode(self.bits() as i64, buf)
    }
}

impl<'r> Decode<'r, Postgres> for Permissions {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let bits: i64 = Decode::<Postgres>::decode(value)?;
        Ok(Permissions::from_bits(bits as u64).unwrap())
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
        serializer.collect_str(&self.bits())
    }
}

struct PermissionsVisitor;

impl<'de> Visitor<'de> for PermissionsVisitor {
    type Value = Permissions;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid permissions")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u64(v.parse().map_err(E::custom)?)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u64(v.parse().map_err(E::custom)?)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Permissions::from_bits(v) {
            Some(bits) => Ok(bits),
            _ => Err(E::custom("Invalid bits")),
        }
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PermissionsVisitor)
    }
}

use opg::{Components, Model, ModelData, ModelString, ModelType, ModelTypeDescription, OpgModel};

impl OpgModel for Permissions {
    fn get_schema(_cx: &mut Components) -> Model {
        Model {
            description: "Permissions bits".to_string().into(),
            data: ModelData::Single(ModelType {
                nullable: false,
                type_description: ModelTypeDescription::String(ModelString::default()),
            }),
        }
    }

    fn type_name() -> Option<std::borrow::Cow<'static, str>> {
        None
    }
}
