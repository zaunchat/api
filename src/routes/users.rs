use crate::extractors::*;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::r#ref::Ref;
use axum::response::IntoResponse;

async fn fetch_me(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(user.to_public())
}

async fn fetch_one(Path(id): Path<u64>) -> Result<Json<User>> {
    Ok(Json(id.user().await?.to_public()))
}

pub fn routes() -> axum::Router {
    use crate::middlewares::*;
    use axum::{middleware, routing::*, Router};

    Router::new()
        .route("/@me", get(fetch_me))
        .route("/:user_id", get(fetch_one))
        .layer(middleware::from_fn(ratelimit::handle!(20, 1000 * 5)))
}
