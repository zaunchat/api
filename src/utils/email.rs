use crate::config::*;
use crate::database::DB as db;
use crate::structures::User;
use nanoid::nanoid;
use rbatis::crud::CRUD;
use serde_json::json;

#[crud_table(table_name:pending_accounts)]
struct PendingVerification {
    user_id: u64,
    code: String,
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
        let p = PendingVerification {
            user_id: user.id,
            code,
        };
        db.save(&p, &[]).await.is_ok()
    } else {
        false
    }
}

pub async fn verify(user_id: u64, code: &str) -> bool {
    let p: Option<PendingVerification> = db
        .fetch(
            "SELECT * FROM pending_accounts WHERE user_id = $1 AND code = $2",
            vec![user_id.into(), code.into()],
        )
        .await
        .unwrap();

    match p {
        Some(_) => {
            db.remove_by_column::<PendingVerification, u64>("user_id", user_id)
                .await
                .ok();
            true
        }
        _ => false,
    }
}
