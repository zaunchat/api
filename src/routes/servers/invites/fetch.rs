use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path((server_id, invite_id)): Path<(i64, i64)>) -> Result<Json<Invite>> {
    Ok(Json(invite_id.invite(server_id.into()).await?))
}

pub async fn fetch_many(Path(server_id): Path<i64>) -> Json<Vec<Invite>> {
    Json(
        Invite::select()
            .filter("server_id = $1")
            .bind(server_id)
            .fetch_all(pool())
            .await
            .unwrap(),
    )
}
