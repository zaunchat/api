use crate::utils::error::*;
use axum::{http::Request, middleware::Next, response::Response, extract::{RequestParts, Path} };
use crate::structures::{Base, User, Member};

pub async fn handle<B>(req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    let mut req = RequestParts::new(req);
    let Path((server_id, _)) = req.extract::<Path<(u64, u64)>>().await.unwrap();
    let user = req.extensions().get::<User>().unwrap();
    let count = Member::count(|q| q.eq("id", user.id).eq("server_id", server_id)).await;

    if count == 0 {
        return Err(Error::UnknownServer);
    }

    Ok(next.run(req.try_into_request().unwrap()).await)
}
