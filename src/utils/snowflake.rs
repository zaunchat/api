use once_cell::sync::Lazy;
use rbatis::snowflake::Snowflake;

const ITCHAT_EPOCH: i64 = 1609459200000;

static SNOWFLAKE: Lazy<Snowflake> = Lazy::new(|| {
    let mut snowflake = Snowflake::default();
    snowflake.set_epoch(ITCHAT_EPOCH);
    snowflake
});

pub fn generate() -> u64 {
    SNOWFLAKE.generate() as u64
}
