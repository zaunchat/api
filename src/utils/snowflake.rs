use chrono::{DateTime, TimeZone, Utc};
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
#[opg(string)]
pub struct Snowflake(#[serde_as(as = "serde_with::DisplayFromStr")] i64);

impl Snowflake {
    pub fn generate() -> Self {
        Self(GENERATOR.lock().unwrap().generate())
    }

    pub fn created_at_timestamp(&self) -> Duration {
        Duration::from_millis((self.0 >> 22) as u64)
            + ITCHAT_EPOCH.duration_since(UNIX_EPOCH).unwrap()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        Utc.timestamp(self.created_at_timestamp().as_secs() as i64, 0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_snowflake_generate() {
        let id = Snowflake::generate();
        assert!(id.is_positive());
    }

    #[test]
    fn test_snowflake_created_at() {
        let id = Snowflake::generate();

        sleep(Duration::from_millis(100));

        let now = Utc::now();

        assert!(id.created_at() < now);
        assert!((id.created_at_timestamp().as_millis() as i64) < now.timestamp_millis());
    }
}
