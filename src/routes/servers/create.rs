use crate::config::*;
use crate::database::DB as db;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use rbatis::crud::CRUDMut;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateServerOptions {
    #[validate(length(min = 1, max = 50))]
    name: String,
}

#[utoipa::path(
    post,
    path = "/servers",
    request_body = CreateServerOptions,
    responses((status = 200, body = Server), (status = 400, body = Error))
)]
pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateServerOptions>,
) -> Result<Json<Server>> {
    let count = Member::count(|q| q.eq("id", user.id)).await;

    if count > *MAX_SERVERS {
        return Err(Error::MaximumServers);
    }

    let server = Server::new(data.name, user.id);
    let member = Member::new(user.id, server.id);
    let category = Channel::new_category("General".into(), server.id);
    let mut chat = Channel::new_text("general".into(), server.id);

    chat.parent_id = Some(category.id);

    let mut tx = db.acquire_begin().await.unwrap();

    tx.save(&server, &[]).await.unwrap();
    tx.save(&category, &[]).await.unwrap();
    tx.save(&chat, &[]).await.unwrap();
    tx.save(&member, &[]).await.unwrap();

    tx.commit().await.unwrap();

    Ok(Json(server))
}