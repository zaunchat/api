use crate::structures::{Base, Member, User};
use crate::utils::error::*;
use crate::utils::Snowflake;
use axum::{
    extract::{Extension, Path},
    http::Request,
    middleware::Next,
    response::Response,
};

#[derive(Deserialize)]
pub struct ID {
    server_id: Snowflake,
    #[allow(dead_code)]
    id: Option<Snowflake>,
}

pub async fn handle<B: std::marker::Send>(
    Extension(user): Extension<User>,
    Path(ID { server_id, .. }): Path<ID>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    let exists =
        Member::count(&format!("id = {} AND server_id = {}", *user.id, *server_id)).await?;

    if exists == 0 {
        return Err(Error::UnknownServer);
    }

    Ok(next.run(req).await)
}
