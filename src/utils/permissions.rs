use crate::structures::*;
use crate::utils::{Error, Ref, Result};
use bitflags::bitflags;
use serde::{
    de::{Error as SerdeError, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct Permissions: i64 {
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
    }
}

macro_rules! bits {
    (ALL) => {{ Permissions::all() }};
    ($flag:ident) => {{ Permissions::$flag }};
    ($($flag:ident),*) => {{
        #[allow(unused_mut)]
        let mut bits = Permissions::default();
        $( bits.insert(Permissions::$flag); )*
        bits
    }};
}

pub(crate) use bits;

lazy_static! {
    pub static ref DEFAULT_PERMISSION_DM: Permissions = bits![
        VIEW_CHANNEL,
        SEND_MESSAGES,
        EMBED_LINKS,
        UPLOAD_FILES,
        READ_MESSAGE_HISTORY
    ];
    pub static ref DEFAULT_PERMISSION_EVERYONE: Permissions = bits![
        VIEW_CHANNEL,
        SEND_MESSAGES,
        EMBED_LINKS,
        UPLOAD_FILES,
        READ_MESSAGE_HISTORY
    ];
}

impl Permissions {
    pub async fn fetch_cached(
        user: &User,
        server: Option<&Server>,
        channel: Option<&Channel>,
    ) -> Result<Permissions> {
        let mut p = bits![];

        if let Some(server) = server {
            if server.owner_id == user.id {
                p = bits![ALL];
            }

            p.insert(server.permissions);

            if p.is_all() {
                return Ok(p);
            }

            let member = server.fetch_member(user.id).await?;

            for role in server.fetch_roles().await? {
                if member.roles.contains(&role.id) {
                    p.insert(role.permissions);
                }
            }
        }

        if let Some(channel) = channel {
            if p.is_all() {
                return Ok(p);
            }

            if channel.is_dm() {
                p.insert(*DEFAULT_PERMISSION_DM);

                let recipients = channel.recipients.as_ref().unwrap();
                let is_notes = || recipients[0] == recipients[1];

                if !is_notes() {
                    let status = user.relations.0.get(&recipients[1]).unwrap();
                    if status != &RelationshipStatus::Friend {
                        p.remove(bits![SEND_MESSAGES]);
                    }
                }
            }

            if channel.is_group()
                || channel.is_text()
                || channel.is_voice()
                || channel.is_category()
            {
                // for group owners
                if channel.owner_id == Some(user.id) {
                    p = bits![ALL];
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

    pub fn has(self, bits: Permissions) -> Result<()> {
        if self.is_all() {
            return Ok(());
        }

        if !self.contains(bits) {
            return Err(Error::MissingPermissions(self.difference(bits)));
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
        Encode::<Postgres>::encode(self.bits(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for Permissions {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Permissions::from_bits(Decode::<Postgres>::decode(value)?).unwrap())
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
        E: SerdeError,
    {
        self.visit_str(&v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        self.visit_i64(v.parse().map_err(E::custom)?)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Permissions::from_bits(v).ok_or_else(|| E::custom("Invalid bits"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        assert!(bits![ALL].is_all());
    }

    #[test]
    fn default() {
        assert_eq!(bits![], Permissions::default());
    }

    #[test]
    fn one_parameter() {
        assert!(Permissions::VIEW_CHANNEL.contains(bits![VIEW_CHANNEL]));
    }

    #[test]
    fn multiple_parameters() {
        let p = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;
        assert!(p.contains(bits![VIEW_CHANNEL, SEND_MESSAGES]));
    }
}
