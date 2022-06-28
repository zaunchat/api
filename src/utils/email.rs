use crate::config::*;
use crate::database::redis::{connection, AsyncCommands};
use crate::database::DB as db;
use crate::structures::{Base, User};
use nanoid::nanoid;
use rbatis::crud::CRUD;
use regex::Regex;
use serde_json::json;

const THREE_HOURS_IN_SECONDS: usize = 10800;

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new("([^@]+)(@.+)").unwrap();
    static ref SYMBOL_REGEX: Regex = Regex::new("\\+.+|\\.").unwrap();
}

#[crud_table(table_name:account_invites)]
pub struct Invite {
    pub code: String,
    pub used: bool,
    pub taken_by: Option<u64>,
}

#[async_trait]
impl Base for Invite {
    fn id(&self) -> u64 {
        unreachable!()
    }

    async fn update(&self) {
        db.update_by_column("code", &self)
            .await
            .expect("Couldn't update account invite");
    }
}

pub fn normalize(email: String) -> String {
    let split = SPLIT_REGEX.captures(&email).unwrap();
    let mut clean = SYMBOL_REGEX
        .replace_all(split.get(1).unwrap().as_str(), "")
        .to_string();
    clean.push_str(split.get(2).unwrap().as_str());
    clean.to_lowercase()
}

pub async fn send(user: &User) -> bool {
    let mut content = include_str!("../../assets/templates/verify.html").to_string();
    let code = nanoid!(10);

    content = content
        .replace("%%EMAIL%%", user.email.as_str())
        .replace("%%CODE%%", code.as_str())
        .replace("%%USER_ID%%", user.id.to_string().as_str());

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
        .await
        .unwrap();

    if res.status().is_success() {
        let mut con = connection().await;
        con.set_ex::<String, String, u32>(user.id.to_string(), code, THREE_HOURS_IN_SECONDS)
            .await
            .is_ok()
    } else {
        false
    }
}

pub async fn verify(user_id: u64, code: &str) -> bool {
    let mut con = connection().await;

    match con.get::<String, String>(user_id.to_string()).await {
        Ok(token) if code == token => {
            con.del::<String, u32>(user_id.to_string()).await.ok();
            true
        }
        _ => false,
    }
}
