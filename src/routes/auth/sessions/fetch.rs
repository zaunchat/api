use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(id): Path<u64>,
) -> Result<Json<Session>> {
    Ok(Json(id.session(user.id).await?))
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Session>> {
    Json(Session::find(|q| q.eq("user_id", &user.id)).await)
}
