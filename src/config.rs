use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

lazy_static! {
    // Fri, 01 Jan 2021 00:00:00 GMT
    pub static ref ITCHAT_EPOCH: SystemTime = UNIX_EPOCH + Duration::from_millis(1609459200000);

    pub static ref DATABASE_URI: String = env::var("DATABASE_URI").expect("DATABASE_URI is required");

    pub static ref SMTP_ENABLED: bool = env::var("SMTP_ENABLED").is_ok();
    pub static ref SMTP_HOST: String = env::var("SMTP_HOST").expect("SMTP_HOST is required");
    pub static ref SMTP_USERNAME: String = env::var("SMTP_USERNAME").expect("SMTP_USERNAME is required");
    pub static ref SMTP_PASSWORD: String = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is required");

    pub static ref CAPTCHA_ENABLED: bool = env::var("CAPTCHA_ENABLED").is_ok();
    pub static ref CAPTCHA_TOKEN: String = env::var("CAPTCHA_TOKEN").expect("CAPTCHA_TOKEN is required");
    pub static ref CAPTCHA_KEY: String = env::var("CAPTCHA_KEY").expect("CAPTCHA_KEY is required");
}
