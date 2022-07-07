use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use crypto::util::fixed_time_eq;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct DeleteSessionOptions {
    token: String,
}

pub async fn delete(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
    ValidatedJson(data): ValidatedJson<DeleteSessionOptions>,
) -> Result<()> {
    let session = id.session(user.id).await?;

    if !fixed_time_eq(session.token.as_bytes(), data.token.as_bytes()) {
        return Err(Error::InvalidToken);
    }

    session.delete(pool()).await.unwrap();

    Ok(())
}
