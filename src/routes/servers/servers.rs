use crate::database::DB as db;
use crate::extractors::*;
use crate::utils::error::*;
use crate::{structures::*, utils::r#ref::Ref};
use rbatis::crud::CRUDMut;
use serde::Deserialize;
use validator::Validate;

async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Server>> {
    Json(user.fetch_servers().await)
}

async fn fetch_one(Path(server_id): Path<u64>) -> Result<Json<Server>> {
    Ok(Json(server_id.server().await?))
}

#[derive(Deserialize, Validate)]
struct CreateServerOptions {
    #[validate(length(min = 1, max = 50))]
    name: String,
}

async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateServerOptions>,
) -> Result<Json<Server>> {
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

async fn delete_server(Extension(user): Extension<User>, Path(server_id): Path<u64>) -> Result<()> {
    let server = server_id.server().await?;

    if server.owner_id == user.id {
        server.delete().await;
    } else if let Some(member) = server.fetch_member(user.id).await {
        member.delete().await;
    } else {
        return Err(Error::UnknownMember);
    }

    Ok(())
}

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", post(create).get(fetch_many))
        .route("/:server_id", get(fetch_one).delete(delete_server))
}
