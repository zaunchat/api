use once_cell::sync::Lazy;
use rbatis::snowflake::Snowflake;

// Fri, 01 Jan 2021 00:00:00 GMT
const ITCHAT_EPOCH: i64 = 1609459200000;

static SNOWFLAKE: Lazy<Snowflake> = Lazy::new(|| {
    let mut snowflake = Snowflake::default();
    snowflake.set_epoch(ITCHAT_EPOCH);
    snowflake
});

pub fn generate() -> u64 {
    SNOWFLAKE.generate() as u64
}

pub mod json {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use serde_with::{DeserializeAs, SerializeAs};
    use std::fmt;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ID(u64);

    impl<'de> DeserializeAs<'de, u64> for ID {
        fn deserialize_as<D>(deserializer: D) -> Result<u64, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize(deserializer)
        }
    }

    impl SerializeAs<u64> for ID {
        fn serialize_as<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize(value, serializer)
        }
    }

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'a> de::Visitor<'a> for Visitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a string containing digits or an int fitting into u64"
                )
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(v as u64)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if (v as i64) < 0 {
                    Err(de::Error::invalid_value(de::Unexpected::Unsigned(v), &self))
                } else {
                    Ok(v)
                }
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                s.parse().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
