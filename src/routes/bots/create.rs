use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

#[utoipa::path(
    post,
    path = "/bots",
    responses((status = 400, body = Error))
)]
pub async fn create() -> Result<Json<Bot>> {
    todo!()
}