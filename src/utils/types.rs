use serde::{Deserialize, Serialize};

use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgValueRef};
use sqlx::{self, Decode, Encode, Postgres};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Private<T: Sized> {
    Marked(T),
    Not(T),
}

use sqlx::postgres::PgTypeInfo;
use sqlx::Type;

impl<T: Type<Postgres>> Type<Postgres> for Private<T> {
    fn type_info() -> PgTypeInfo {
        T::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        T::compatible(ty)
    }
}

impl<'a, T: Encode<'a, Postgres>> Encode<'_, Postgres> for Private<T> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        T::encode_by_ref(self, buf)
    }
}

impl<'r, T: Decode<'r, Postgres>> Decode<'r, Postgres> for Private<T> {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        T::decode(value).map(|v| Self::Marked(v))
    }
}

impl<T> From<T> for Private<T> {
    fn from(d: T) -> Self {
        Self::Marked(d)
    }
}

impl<T: Default> Default for Private<T> {
    fn default() -> Self {
        Self::Marked(T::default())
    }
}

impl<T: Clone> Private<T> {
    pub fn is_private(&self) -> bool {
        matches!(self, Private::Marked(_))
    }

    pub fn set_public(&mut self) {
        *self = Self::Not((**self).clone());
    }

    pub fn set_private(&mut self) {
        *self = Self::Marked((**self).clone());
    }
}

impl<T> std::ops::Deref for Private<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Marked(x) | Self::Not(x) => x,
        }
    }
}

impl<T> std::ops::DerefMut for Private<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Marked(x) | Self::Not(x) => x,
        }
    }
}

impl_opg_model!(generic_simple: Private<T>);
