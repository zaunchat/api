use bitflags::bitflags;
use serde::{
    de::{Error, Visitor},
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
    pub struct Badges: i64 {
       const STAFF = 1 << 1;
       const DEVELOPER = 1 << 2;
       const SUPPORTER = 1 << 3;
       const TRANSLATOR = 1 << 4;
    }
}

impl Type<Postgres> for Badges {
    fn type_info() -> PgTypeInfo {
        i64::type_info()
    }
}

impl Encode<'_, Postgres> for Badges {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        Encode::<Postgres>::encode(self.bits(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for Badges {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Badges::from_bits(Decode::<Postgres>::decode(value)?).unwrap())
    }
}

impl Serialize for Badges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.bits())
    }
}

struct BadgesVisitor;

impl<'de> Visitor<'de> for BadgesVisitor {
    type Value = Badges;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid badges")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v.parse().map_err(E::custom)?)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v.parse().map_err(E::custom)?)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Badges::from_bits(v).ok_or_else(|| E::custom("Invalid bits"))
    }
}

impl<'de> Deserialize<'de> for Badges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(BadgesVisitor)
    }
}

use opg::{Components, Model, ModelData, ModelString, ModelType, ModelTypeDescription, OpgModel};

impl OpgModel for Badges {
    fn get_schema(_cx: &mut Components) -> Model {
        Model {
            description: "Badges bits".to_string().into(),
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
