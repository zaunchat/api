use crate::utils::error::Error;
use axum::{
    body::HttpBody,
    extract::{FromRequest, Json, RequestParts},
    BoxError,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let data = Json::from_request(req).await;

        if let Ok(data) = data {
            let data: Json<T> = data;

            data.validate()
                .map_err(|error| Error::ValidationFailed { error })?;

            Ok(Self(data.0))
        } else {
            Err(Error::ParseFailed)
        }
    }
}
