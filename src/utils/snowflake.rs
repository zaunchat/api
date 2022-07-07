use snowflake::SnowflakeIdGenerator;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

lazy_static! {
    // Fri, 01 Jan 2021 00:00:00 GMT
    static ref ITCHAT_EPOCH: SystemTime = UNIX_EPOCH + Duration::from_millis(1609459200000);

    static ref SNOWFLAKE: Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::with_epoch(0, 0, *ITCHAT_EPOCH));
}

pub fn generate() -> i64 {
    SNOWFLAKE.lock().unwrap().generate()
}
