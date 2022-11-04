use serde::{Deserialize, Serialize};
use snowflake::SnowflakeIdGenerator;
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use sqlx::Type;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

lazy_static! {
    // Fri, 01 Jan 2021 00:00:00 GMT
    static ref ITCHAT_EPOCH: SystemTime = UNIX_EPOCH + Duration::from_millis(1609459200000);

    static ref GENERATOR: Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::with_epoch(0, 0, *ITCHAT_EPOCH));
}

#[derive(Type, Serialize, Deserialize, opg::OpgModel, Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[serde(transparent)]
#[sqlx(transparent)]
#[serde_as]
pub struct Snowflake(
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[opg(string)]
    pub i64,
);

impl Default for Snowflake {
    fn default() -> Self {
        Self(GENERATOR.lock().unwrap().generate())
    }
}

impl std::ops::Deref for Snowflake {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PgHasArrayType for Snowflake {
    fn array_type_info() -> PgTypeInfo {
        i64::array_type_info()
    }

    fn array_compatible(_: &PgTypeInfo) -> bool {
        true
    }
}

impl TryFrom<String> for Snowflake {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Snowflake(value.parse()?))
    }
}
