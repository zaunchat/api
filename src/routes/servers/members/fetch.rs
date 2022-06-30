use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct FetchMembersOptions {
    #[validate(range(min = 2, max = 1000))]
    limit: Option<u64>,
}

pub async fn fetch_one(Path((server_id, member_id)): Path<(u64, u64)>) -> Result<Json<Member>> {
    Ok(Json(member_id.member(server_id).await?))
}

pub async fn fetch_many(
    Path(server_id): Path<u64>,
    Query(query): Query<FetchMembersOptions>,
) -> Json<Vec<Member>> {
    let limit = query.limit.unwrap_or(100);
    Json(Member::find(|q| q.eq("server_id", server_id).limit(limit)).await)
}
