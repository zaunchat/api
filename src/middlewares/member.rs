use crate::structures::{Base, Member, User};
use crate::utils::error::*;
use axum::{
    extract::{Path, RequestParts},
    http::Request,
    middleware::Next,
    response::Response,
};

#[derive(Deserialize)]
struct ID {
    server_id: u64,
    #[allow(dead_code)]
    id: Option<u64>,
}

pub async fn handle<B: std::marker::Send>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    let mut req = RequestParts::new(req);
    let Path(ID { server_id, .. }) = req.extract::<Path<ID>>().await.unwrap();
    let user = req.extensions().get::<User>().unwrap();

    let exists = Member::count(&format!("id = {} AND server_id = {}", user.id, server_id)).await;

    if exists == 0 {
        return Err(Error::UnknownServer);
    }

    Ok(next.run(req.try_into_request().unwrap()).await)
}
