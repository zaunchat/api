use snowflake::SnowflakeIdGenerator;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref ITCHAT_EPOCH: SystemTime = UNIX_EPOCH + Duration::from_millis(1609459200000);
}

pub fn generate_id() -> u64 {
    let mut generator = SnowflakeIdGenerator::with_epoch(1, 1, *ITCHAT_EPOCH);
    generator.generate() as u64
}
