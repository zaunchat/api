use std::env;

fn is_true(mut v: String) -> bool {
    v = v.to_lowercase();
    return v == "true" || v == "yes";
}

macro_rules! get {
    ($key:expr) => {{
        env::var($key).expect(format!("{} is required", $key).as_str())
    }};
    ($key:expr, $default: expr) => {{
        env::var($key).unwrap_or($default.to_string())
    }};
}

lazy_static! {
    pub static ref DATABASE_URI: String = get!("DATABASE_URI");
    pub static ref CAPTCHA_ENABLED: bool = is_true(get!("CAPTCHA_ENABLED", "false"));
    pub static ref CAPTCHA_TOKEN: String = get!("CAPTCHA_TOKEN");
    pub static ref CAPTCHA_KEY: String = get!("CAPTCHA_KEY");
    pub static ref PORT: String = get!("PORT", "8080");
    pub static ref EMAIL_VERIFICATION: bool = is_true(get!("EMAIL_VERIFICATION", "false"));
    pub static ref REQUIRE_INVITE_TO_REGISTER: bool = is_true(get!("REQUIRE_INVITE_TO_REGISTER", "false"));
    pub static ref SENDINBLUE_API_KEY: String = get!("SENDINBLUE_API_KEY");
    pub static ref TRUST_CLOUDFLARE: bool = is_true(get!("TRUST_CLOUDFLARE", "false"));
}
