use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path((server_id, id)): Path<(i64, i64)>) -> Result<Json<Invite>> {
    Ok(id.invite(server_id.into()).await?.into())
}

pub async fn fetch_many(Path(server_id): Path<i64>) -> Result<Json<Vec<Invite>>> {
    Ok(Invite::select()
        .filter("server_id = $1")
        .bind(server_id)
        .fetch_all(pool())
        .await?
        .into())
}
