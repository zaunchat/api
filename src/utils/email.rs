use crate::config::*;
use crate::database::redis::*;
use crate::structures::{Base, User};
use crate::utils::Snowflake;
use lazy_regex::regex;

use serde_json::json;
use sqlx::types::Uuid;
use sqlx::{postgres::PgArguments, Arguments, FromRow};

const THREE_HOURS_IN_SECONDS: i64 = 10800;

#[derive(FromRow)]
pub struct AccountInvite {
    pub code: Uuid,
    pub used: bool,
    pub taken_by: Option<Snowflake>,
}

impl Base<'_, Uuid> for AccountInvite {
    fn id(&self) -> Uuid {
        self.code
    }

    fn primary_key() -> &'static str {
        "code"
    }

    fn fields(&self) -> (Vec<&str>, PgArguments) {
        let mut values = PgArguments::default();

        values.add(self.code);
        values.add(self.used);
        values.add(self.taken_by);

        (vec!["code", "used", "taken_by"], values)
    }

    fn table_name() -> &'static str {
        "account_invites"
    }
}

pub fn normalize(email: String) -> Option<String> {
    let split = regex!("([^@]+)(@.+)").captures(&email)?;
    let mut clean = regex!("\\+.+|\\.")
        .replace_all(split.get(1)?.as_str(), "")
        .to_string();
    clean.push_str(split.get(2)?.as_str());

    Some(clean.to_lowercase())
}

pub async fn send(user: &User) -> bool {
    let mut content = include_str!("../../assets/templates/verify.html").to_string();
    let code = Uuid::new_v4();

    content = content
        .replace("%%EMAIL%%", &user.email)
        .replace("%%CODE%%", &code.to_string())
        .replace("%%USER_ID%%", &user.id.to_string());

    let body = json!({
        "subject": "Verify your ItChat account",
        "sender": { "email": "noreply@itchat.world" },
        "to": [{ "email": user.email }],
        "type": "classic",
        "htmlContent": content,
    });

    let res = reqwest::Client::new()
        .post("https://api.sendinblue.com/v3/smtp/email")
        .header("api-key", (*SENDINBLUE_API_KEY).clone())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body.to_string())
        .send()
        .await;

    if res.map(|r| r.status().is_success()).unwrap_or(false) {
        REDIS
            .set::<(), _, _>(
                *user.id,
                code.to_string(),
                Expiration::EX(THREE_HOURS_IN_SECONDS).into(),
                None,
                false,
            )
            .await
            .is_ok()
    } else {
        false
    }
}

pub async fn verify(user_id: Snowflake, code: Uuid) -> bool {
    match REDIS.get::<String, _>(user_id.to_string()).await {
        Ok(token) if code.to_string() == token => {
            REDIS.del::<u32, _>(user_id.to_string()).await.ok();
            true
        }
        _ => false,
    }
}
