use std::env;

fn is_true(mut v: String) -> bool {
    v = v.to_lowercase();
    v == "true" || v == "yes"
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
    pub static ref REQUIRE_INVITE_TO_REGISTER: bool =
        is_true(get!("REQUIRE_INVITE_TO_REGISTER", "false"));
    pub static ref SENDINBLUE_API_KEY: String = get!("SENDINBLUE_API_KEY");
    pub static ref TRUST_CLOUDFLARE: bool = is_true(get!("TRUST_CLOUDFLARE", "false"));

    // User related
    pub static ref MAX_FRIENDS: u64 = get!("MAX_FRIENDS", "1000").parse().unwrap();
    pub static ref MAX_BLOCKED: u64 = get!("MAX_BLOCKED", "1000").parse().unwrap();

    // Group related
    pub static ref MAX_GROUPS: u64 = get!("MAX_GROUPS", "100").parse().unwrap();
    pub static ref MAX_GROUP_MEMBERS: u64 = get!("MAX_GROUP_MEMBERS", "50").parse().unwrap();

    // Server related
    pub static ref MAX_SERVERS: u64 = get!("MAX_SERVERS", "100").parse().unwrap();
    pub static ref MAX_SERVER_MEMBERS: u64 = get!("MAX_SERVER_MEMBERS", "10000").parse().unwrap();
    pub static ref MAX_SERVER_CHANNELS: u64 = get!("MAX_SERVER_CHANNELS", "200").parse().unwrap();
    pub static ref MAX_SERVER_ROLES: u64 = get!("MAX_SERVER_ROLES", "200").parse().unwrap();
    pub static ref MAX_SERVER_EMOJIS: u64 = get!("MAX_SERVER_EMOJIS", "150").parse().unwrap();
}
