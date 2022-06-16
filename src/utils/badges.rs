use bitflags::bitflags;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

bitflags! {
    pub struct Badges: u64 {
       const STAFF = 1 << 1;
       const DEVELOPER = 1 << 2;
       const SUPPORTER = 1 << 3;
       const TRANSLATOR = 1 << 4;
       const DEFAULT = 0;
    }
}

impl Default for Badges {
    fn default() -> Self {
        Badges::DEFAULT
    }
}

impl Serialize for Badges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

struct BadgesVisitor;

impl<'de> Visitor<'de> for BadgesVisitor {
    type Value = Badges;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid badges")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(p) = Badges::from_bits(v) {
            return Ok(p);
        }
        Err(E::custom("Invalid Badges"))
    }
}

impl<'de> Deserialize<'de> for Badges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BadgesVisitor)
    }
}

use opg::{Components, Model, ModelData, ModelSimple, ModelType, ModelTypeDescription, OpgModel};

impl OpgModel for Badges {
    fn get_schema(_cx: &mut Components) -> Model {
        Model {
            description: "Badges bits".to_string().into(),
            data: ModelData::Single(ModelType {
                nullable: false,
                type_description: ModelTypeDescription::Number(ModelSimple::default()),
            }),
        }
    }

    fn type_name() -> Option<std::borrow::Cow<'static, str>> {
        None
    }
}
