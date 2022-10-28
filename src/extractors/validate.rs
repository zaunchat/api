use crate::utils::error::Error;
use axum::{
    extract::{FromRequest, Json},
    http::Request,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: Send + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<S, B>,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let data = Json::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        data.validate()
            .map_err(|_| IntoResponse::into_response(Error::InvalidBody))?;

        Ok(Self(data.0))
    }
}
