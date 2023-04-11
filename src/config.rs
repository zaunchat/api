macro_rules! config {
    ($($name:ident $t:tt $($default:expr)?),+) => {
            lazy_static! {
                $(
                 pub static ref $name: $t = std::env::var(stringify!($name))
                    .unwrap_or_else(|_| {
                        $( if true { return $default.to_string(); } )?
                        panic!("{} is required", stringify!($name));
                    })
                    .parse::<$t>()
                    .expect("Failed to parse value");
                )+
            }
    };
}

config! {
    // Networking
    PORT u16 8080,
    TRUST_CLOUDFLARE bool false,

    // Database
    DATABASE_URI String "postgres://postgres:postgres@localhost",
    REDIS_URI String "redis://localhost:6379",
    REDIS_POOL_SIZE usize 100,
    DATABASE_POOL_SIZE u32 100,

    // Captcha
    CAPTCHA_ENABLED bool false,
    CAPTCHA_TOKEN String,
    CAPTCHA_KEY String,

    // Email
    SENDINBLUE_API_KEY String,
    EMAIL_VERIFICATION bool false,
    REQUIRE_INVITE_TO_REGISTER bool false,

    // User related
    MAX_FRIENDS u64 1000,
    MAX_BLOCKED u64 1000,
    MAX_FRIEND_REQUESTS u64 100,

    // Group related
    MAX_GROUPS u64 100,
    MAX_GROUP_MEMBERS u64 50
}
