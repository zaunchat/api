use std::env;

fn is_true(v: Result<String, env::VarError>) -> bool {
    if let Ok(mut v) = v {
        v = v.to_lowercase();
        return v == "true" || v == "yes";
    }
    false
}

fn default(v: Result<String, env::VarError>, default_value: &str) -> String {
    if let Ok(v) = v {
        v
    } else {
        default_value.to_string()
    }
}

lazy_static! {
    pub static ref DATABASE_URI: String =
        env::var("DATABASE_URI").expect("DATABASE_URI is required");
    pub static ref CAPTCHA_ENABLED: bool = is_true(env::var("CAPTCHA_ENABLED"));
    pub static ref CAPTCHA_TOKEN: String =
        env::var("CAPTCHA_TOKEN").expect("CAPTCHA_TOKEN is required");
    pub static ref CAPTCHA_KEY: String = env::var("CAPTCHA_KEY").expect("CAPTCHA_KEY is required");
    pub static ref PORT: String = default(env::var("PORT"), "8080");
    pub static ref EMAIL_VERIFICATION: bool = is_true(env::var("EMAIL_VERIFICATION"));
    pub static ref REQUIRE_INVITE_TO_REGISTER: bool =
        is_true(env::var("REQUIRE_INVITE_TO_REGISTER"));
    pub static ref INVITEATION_CODE: String =
        env::var("INVITEATION_CODE").expect("INVITEATION_CODE is required");
    pub static ref SENDINBLUE_API_KEY: String =
        env::var("SENDINBLUE_API_KEY").expect("SENDINBLUE_API_KEY is required");
}
