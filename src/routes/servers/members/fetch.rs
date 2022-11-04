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
    Path((server_id, id)): Path<(Snowflake, Snowflake)>,
) -> Result<Json<Member>> {
    Ok(id.member(server_id).await?.into())
}

pub async fn fetch_many(
    Path(server_id): Path<Snowflake>,
    Query(query): Query<FetchMembersOptions>,
) -> Result<Json<Vec<Member>>> {
    let limit = query.limit.unwrap_or(100) as usize;

    Ok(Member::select()
        .filter("server_id = $1")
        .bind(server_id)
        .limit(limit)
        .fetch_all(pool())
        .await?
        .into())
}
