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

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
) -> Result<Json<Member>> {
    user.member_of(server_id).await?;
    Ok(Json(member_id.member(server_id).await?))
}

pub async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    Query(query): Query<FetchMembersOptions>,
) -> Result<Json<Vec<Member>>> {
    user.member_of(server_id).await?;

    let limit = query.limit.unwrap_or(100);
    let members = Member::find(|q| q.eq("server_id", server_id).limit(limit)).await;

    Ok(Json(members))
}
