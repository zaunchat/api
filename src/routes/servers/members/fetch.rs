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

pub async fn fetch_one(Path((server_id, member_id)): Path<(i64, i64)>) -> Result<Json<Member>> {
    Ok(Json(member_id.member(server_id).await?))
}

pub async fn fetch_many(
    Path(server_id): Path<i64>,
    Query(query): Query<FetchMembersOptions>,
) -> Json<Vec<Member>> {
    let limit = query.limit.unwrap_or(100) as usize;
    let members = Member::select()
        .filter("server_id = $1")
        .bind(server_id)
        .limit(limit)
        .fetch_all(pool())
        .await
        .unwrap();
    Json(members)
}
