use std::env;

lazy_static! {
    pub static ref DATABASE_URI: String =
        env::var("DATABASE_URI").expect("DATABASE_URI is required");
    pub static ref CAPTCHA_ENABLED: bool = env::var("CAPTCHA_ENABLED").is_ok();
    pub static ref CAPTCHA_TOKEN: String =
        env::var("CAPTCHA_TOKEN").expect("CAPTCHA_TOKEN is required");
    pub static ref CAPTCHA_KEY: String = env::var("CAPTCHA_KEY").expect("CAPTCHA_KEY is required");
    pub static ref EMAIL_VERIFICATION: bool = env::var("EMAIL_VERIFICATION").is_ok();
    pub static ref SENDINBLUE_API_KEY: String =
        env::var("SENDINBLUE_API_KEY").expect("SENDINBLUE_API_KEY is required");
}
